pub struct Light {
    pub diffuse_strength: f32,
    pub specular_strength: f32,
    pub direction: glam::Vec3,
    pub shadow: bool,
}

#[derive(Clone, Copy)]
pub(crate) enum EdgeType {
    Light,
    Dark,
}

fn calculate_ao_factor(
    scene: &crate::raytrace::Scene,
    position: &glam::Vec3,
    normal: &glam::Vec3,
    samples_x: u32,
    samples_y: u32,
    rng: &mut rand_pcg::Pcg32,
) -> f32 {
    use rand_pcg::rand_core::RngCore as _;

    let tangent = if normal.x.abs() > normal.y.abs() {
        glam::Vec3::new(normal.z, 0.0, -normal.x) * (1.0 / (normal.x * normal.x + normal.z * normal.z).sqrt())
    } else {
        glam::Vec3::new(0.0, -normal.z, normal.y) * (1.0 / (normal.y * normal.y + normal.z * normal.z).sqrt())
    };
    let bitangent = normal.cross(tangent);

    let mut not_occluded_samples = 0;
    for x in 0..samples_x {
        for y in 0..samples_y {
            let random_x = rng.next_u32() as f32 / u32::MAX as f32;
            let random_y = rng.next_u32() as f32 / u32::MAX as f32;

            let theta = 2.0 * std::f32::consts::PI * ((x as f32 + random_x) / samples_x as f32);
            let phi = (1.0 - ((y as f32 + random_y) / samples_y as f32)).asin();

            let phi_cos = phi.cos();
            let local_sample_direction = glam::Vec3::new(phi_cos * theta.sin(), phi_cos * theta.cos(), phi.sin());
            let sample_direction = (normal * local_sample_direction.z)
                + (tangent * local_sample_direction.x)
                + (bitangent * local_sample_direction.y);

            if !scene.trace_occlusion_ray(position, &sample_direction) {
                not_occluded_samples += 1;
            }
        }
    }

    not_occluded_samples as f32 / (samples_x * samples_y) as f32
}

pub fn render_scene(
    scene: &crate::raytrace::Scene,
    camera: &glam::Mat4,
    lights: &[Light],
    multi_samples_x: usize,
    multi_samples_y: usize,
) -> crate::Framebuffer {
    const EDGE_DISTANCE: f32 = 4.0 / 13.713_586; // what is this? scale of the camera?

    use rand_pcg::rand_core::SeedableRng as _;
    let mut rng = rand_pcg::Pcg32::seed_from_u64(1);

    let scene_bounds = scene.get_scene_screen_bounds(camera);
    let ray_origin_offset = glam::Vec3::new(scene_bounds[0] as f32 - 0.5, scene_bounds[1] as f32, 0.0);

    let camera_inverse = camera.inverse();
    let ray_direction = camera_inverse.transform_vector3(glam::Vec3::new(0.0, 0.0, 1.0)).normalize();

    let width = usize::try_from(scene_bounds[2] - scene_bounds[0]).unwrap() + 1;
    let height = usize::try_from(scene_bounds[3] - scene_bounds[1]).unwrap();
    let mut buffer = vec![crate::framebuffer::Fragment::default(); width * height];

    let multi_sample_count = multi_samples_x * multi_samples_y;

    for y in 0..height {
        for x in 0..width {
            let ray_origin = glam::Vec3::new(x as f32, y as f32, -512.0) + ray_origin_offset;

            let (depth, edge_type, palette_region_type, is_mask) = {
                let ray_origin = camera_inverse.transform_vector3(ray_origin);
                match scene.trace_ray(&ray_origin, &ray_direction) {
                    Some(crate::raytrace::RayHit::Mesh(hit)) => (
                        hit.depth,
                        hit.mesh.material.edge_type,
                        Some(hit.mesh.material.palette_region_type),
                        false,
                    ),
                    Some(crate::raytrace::RayHit::Mask) => (f32::INFINITY, None, None, true),
                    _ => (f32::INFINITY, None, None, false),
                }
            };

            let mut samples = vec![crate::framebuffer::Fragment::default(); multi_sample_count];

            for sub_x in 0..multi_samples_x {
                for sub_y in 0..multi_samples_y {
                    let ray_origin = glam::Vec3::new(
                        (sub_x as f32 + 0.5) / multi_samples_x as f32 - 0.5,
                        (sub_y as f32 + 0.5) / multi_samples_y as f32 - 0.5,
                        0.0,
                    ) + ray_origin;
                    let ray_origin = camera_inverse.transform_vector3(ray_origin);

                    match scene.trace_ray(&ray_origin, &ray_direction) {
                        Some(crate::raytrace::RayHit::Mesh(hit)) => {
                            let fragment = &mut samples[sub_y * multi_samples_x + sub_x];
                            let material = &hit.mesh.material;

                            fragment.depth = hit.depth;
                            fragment.edge_type = material.edge_type;
                            fragment.palette_region_type = Some(material.palette_region_type);

                            let diffuse = match &material.diffuse {
                                crate::model::MaterialColour::Colour(colour) => *colour,
                                crate::model::MaterialColour::Texture(texture) => {
                                    let uvs = [
                                        hit.mesh.uvs[hit.indices.0 as usize] * (1.0 - hit.u - hit.v),
                                        hit.mesh.uvs[hit.indices.1 as usize] * hit.u,
                                        hit.mesh.uvs[hit.indices.2 as usize] * hit.v,
                                    ];
                                    let uv = uvs.iter().sum::<glam::Vec2>();
                                    texture.sample_wrapped(uv)
                                }
                            };
                            // move this to material/texture load?
                            let diffuse = if material.palette_region_type.is_diffuse_greyscale() {
                                let max = diffuse.x.max(diffuse.y.max(diffuse.z));
                                glam::Vec3::new(max, max, max)
                            } else {
                                diffuse
                            };

                            for light in lights {
                                if light.shadow && scene.trace_occlusion_ray(&hit.position, &light.direction) {
                                    continue;
                                }
                                if light.diffuse_strength > 0.0 {
                                    let light = hit.normal.dot(light.direction).max(0.0) * light.diffuse_strength;
                                    fragment.colour += light * diffuse;
                                }
                                if light.specular_strength > 0.0 {
                                    let reflected_direction = hit.normal * (2.0 * light.direction.dot(hit.normal));
                                    let reflected_direction = reflected_direction - light.direction;
                                    let angle = reflected_direction.dot(-ray_direction).max(0.0);
                                    let specular_factor =
                                        light.specular_strength * angle.powf(material.specular_exponent);
                                    fragment.colour += specular_factor * material.specular;
                                }
                            }

                            if material.use_ao {
                                fragment.colour *=
                                    calculate_ao_factor(scene, &hit.position, &hit.normal, 8, 4, &mut rng);
                            }
                        }
                        Some(crate::raytrace::RayHit::Mask) => {
                            let fragment = &mut samples[sub_y * multi_samples_x + sub_x];
                            fragment.is_mask = true;
                        }
                        _ => {}
                    }
                }
            }

            let samples = samples;

            let (closest_sample, min_depth) = {
                let mut closest_sample = None;
                let mut min_depth = f32::INFINITY;
                for sample in samples.iter().filter(|x| x.edge_type.is_some()) {
                    if sample.depth < min_depth {
                        closest_sample = Some(sample);
                        min_depth = sample.depth;
                    }
                }
                (closest_sample, min_depth)
            };

            let (depth, edge_type, palette_region_type) = if let Some(closest_sample) = closest_sample
                && (min_depth < depth - EDGE_DISTANCE && !is_mask)
            {
                let mut inside_count = 0;
                for sample in samples.iter().filter(|x| !x.is_mask) {
                    if sample.depth <= min_depth + EDGE_DISTANCE && sample.palette_region_type.is_some() {
                        inside_count += 1;
                    }
                }
                if inside_count > 3 {
                    (min_depth, closest_sample.edge_type, closest_sample.palette_region_type)
                } else {
                    (depth, edge_type, palette_region_type)
                }
            } else {
                (depth, edge_type, palette_region_type)
            };

            if palette_region_type.is_some() {
                let fragment = &mut buffer[y * width + x];
                fragment.palette_region_type = palette_region_type;

                if let Some(edge_type) = edge_type {
                    let sample_count = samples.iter().filter(|x| !x.is_mask).count();
                    let sample_weight = 1.0 / sample_count as f32;
                    let mut colour = glam::Vec3::new(0.0, 0.0, 0.0);
                    let mut weight = 0.0;
                    let mut total_weight = 0.0;
                    for sample in samples.iter().filter(|x| !x.is_mask) {
                        // sample.depth <= depth + EDGE_DISTANCE should be sample.ghost_depth
                        if !(sample.depth <= depth + EDGE_DISTANCE && sample.depth > depth + EDGE_DISTANCE) {
                            if sample.depth <= depth + EDGE_DISTANCE && sample.palette_region_type.is_some() {
                                colour += sample.colour * sample_weight;
                                weight += sample_weight;
                            }
                            total_weight += sample_weight;
                        }
                    }
                    match edge_type {
                        crate::renderer::EdgeType::Light => fragment.colour = colour,
                        crate::renderer::EdgeType::Dark => {
                            fragment.colour = colour * (0.5 + (0.5 * (weight / total_weight)));
                        }
                    }
                } else {
                    let colour = samples
                        .iter()
                        .filter(|x| x.palette_region_type.is_some())
                        .map(|x| x.colour)
                        .sum::<glam::Vec3>();
                    let sample_count = samples.iter().filter(|x| x.palette_region_type.is_some()).count();

                    fragment.colour = colour / sample_count as f32;
                }
            }
        }
    }

    crate::Framebuffer { buffer, width, height }
}
