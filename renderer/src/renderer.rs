#[derive(Clone, Copy)]
pub struct Light {
    pub diffuse_strength: f32,
    pub specular_strength: f32,
    pub direction: glam::Vec3,
    pub shadow: bool,
}

impl Light {
    pub fn transform(&self, transform: &glam::Mat4) -> Self {
        let mut light = *self;
        light.direction = transform.transform_vector3(light.direction).normalize();
        light
    }
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
    use rand_pcg::rand_core::Rng as _;

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

fn sample_mesh(
    scene: &crate::raytrace::Scene,
    rng: &mut rand_pcg::Pcg32,
    ray_direction: &glam::Vec3,
    hit: &crate::raytrace::RayHitMesh,
    lights: &[Light],
    fragment: &mut crate::framebuffer::Fragment,
) {
    let material = &hit.mesh.material;

    fragment.depth = hit.depth;
    fragment.ghost_depth = hit.ghost_depth;
    fragment.edge_type = material.edge_type;
    fragment.palette_region_type = Some(material.palette_region_type);
    fragment.no_bleed = material.no_bleed;

    let uv = {
        let uvs = [
            hit.mesh.uvs[hit.indices[0] as usize] * (1.0 - hit.u - hit.v),
            hit.mesh.uvs[hit.indices[1] as usize] * hit.u,
            hit.mesh.uvs[hit.indices[2] as usize] * hit.v,
        ];
        uvs.iter().sum::<glam::Vec2>()
    };

    let diffuse = match &material.diffuse {
        crate::model::MaterialColour::Colour(colour) => *colour,
        crate::model::MaterialColour::Texture(texture) => texture.sample_wrapped(uv),
    };

    let specular = match &material.specular {
        crate::model::MaterialColour::Colour(colour) => *colour,
        crate::model::MaterialColour::Texture(texture) => texture.sample_wrapped(uv),
    };

    for light in lights {
        if material.shadows && light.shadow && scene.trace_occlusion_ray(&hit.position, &light.direction) {
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
            let specular_factor = light.specular_strength * angle.powf(material.specular_exponent);
            fragment.colour += specular_factor * specular;
        }
    }

    if material.use_ao {
        fragment.colour *= calculate_ao_factor(scene, &hit.position, &hit.normal, 8, 4, rng);
    }
}

#[expect(clippy::too_many_arguments)]
fn sample_point(
    scene: &crate::raytrace::Scene,
    mesh_types: &[crate::raytrace::MeshType],
    camera_inverse: &glam::Mat4,
    lights: &[Light],
    multi_samples_x: usize,
    multi_samples_y: usize,
    edge_distance: f32,
    fragment: &mut crate::framebuffer::Fragment,
    rng: &mut rand_pcg::Pcg32,
    ray_origin_offset: &glam::Vec3,
    ray_direction: &glam::Vec3,
    x: usize,
    y: usize,
) {
    let ray_origin = glam::Vec3::new(x as f32, y as f32, -512.0) + ray_origin_offset;

    let (depth, ghost_depth, edge_type, palette_region_type, is_mask, no_bleed) = {
        let ray_origin = camera_inverse.transform_vector3(ray_origin);
        match scene.trace_ray(mesh_types, &ray_origin, ray_direction) {
            Some(crate::raytrace::RayHit::Mesh(hit)) => (
                hit.depth,
                hit.ghost_depth,
                hit.mesh.material.edge_type,
                Some(hit.mesh.material.palette_region_type),
                false,
                hit.mesh.material.no_bleed,
            ),
            Some(crate::raytrace::RayHit::Mask) => (f32::INFINITY, f32::INFINITY, None, None, true, false),
            Some(crate::raytrace::RayHit::Ghost(ghost_depth)) => (f32::INFINITY, ghost_depth, None, None, false, false),
            _ => (f32::INFINITY, f32::INFINITY, None, None, false, false),
        }
    };

    let mut samples = vec![crate::framebuffer::Fragment::default(); multi_samples_x * multi_samples_y];

    for sub_x in 0..multi_samples_x {
        for sub_y in 0..multi_samples_y {
            let ray_origin = glam::Vec3::new(
                (sub_x as f32 + 0.5) / multi_samples_x as f32 - 0.5,
                (sub_y as f32 + 0.5) / multi_samples_y as f32 - 0.5,
                0.0,
            ) + ray_origin;
            let ray_origin = camera_inverse.transform_vector3(ray_origin);

            match scene.trace_ray(mesh_types, &ray_origin, ray_direction) {
                Some(crate::raytrace::RayHit::Mesh(hit)) => {
                    let fragment = &mut samples[sub_y * multi_samples_x + sub_x];
                    sample_mesh(scene, rng, ray_direction, &hit, lights, fragment);
                }
                Some(crate::raytrace::RayHit::Mask) => {
                    let fragment = &mut samples[sub_y * multi_samples_x + sub_x];
                    fragment.is_mask = true;
                }
                Some(crate::raytrace::RayHit::Ghost(depth)) => {
                    let fragment = &mut samples[sub_y * multi_samples_x + sub_x];
                    fragment.ghost_depth = depth;
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
        && (min_depth < ghost_depth - edge_distance && !is_mask)
    {
        let mut inside_count = 0;
        for sample in samples.iter().filter(|x| !x.is_mask) {
            if sample.depth <= min_depth + edge_distance && sample.palette_region_type.is_some() {
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
        fragment.palette_region_type = palette_region_type;

        fragment.depth = samples.iter().fold(f32::INFINITY, |a, &b| a.min(b.depth));

        if let Some(edge_type) = edge_type {
            let sample_count = samples.iter().filter(|x| !x.is_mask).count();
            let sample_weight = 1.0 / sample_count as f32;
            let mut colour = glam::Vec3::new(0.0, 0.0, 0.0);
            let mut weight = 0.0;
            let mut total_weight = 0.0;
            for sample in samples.iter().filter(|x| !x.is_mask) {
                if (!sample.no_bleed || no_bleed)
                    && !(sample.ghost_depth <= depth + edge_distance && sample.depth > depth + edge_distance)
                {
                    if sample.depth <= depth + edge_distance && sample.palette_region_type.is_some() {
                        colour += sample.colour * sample_weight;
                        weight += sample_weight;
                    }
                    total_weight += sample_weight;
                }
            }
            colour *= 1.0 / total_weight;
            match edge_type {
                crate::renderer::EdgeType::Light => fragment.colour = colour,
                crate::renderer::EdgeType::Dark => {
                    fragment.colour = colour * (0.5 + (0.5 * (weight / total_weight)));
                }
            }
        } else {
            let colour = samples
                .iter()
                .filter(|x| x.palette_region_type.is_some() && (!x.no_bleed || no_bleed))
                .map(|x| x.colour)
                .sum::<glam::Vec3>();
            let sample_count = samples.iter().filter(|x| x.palette_region_type.is_some()).count();

            fragment.colour = colour / sample_count as f32;
        }
    }
}

pub fn render_scene(
    scene: &crate::raytrace::Scene,
    mesh_types: &[crate::raytrace::MeshType],
    camera: &glam::Mat4,
    lights: &[Light],
    multi_samples_x: usize,
    multi_samples_y: usize,
    edge_distance: f32,
) -> crate::Framebuffer {
    use rand_pcg::rand_core::SeedableRng as _;
    let mut rng = rand_pcg::Pcg32::seed_from_u64(1);

    let scene_bounds = scene.get_scene_screen_bounds(camera, mesh_types);
    let framebuffer_offset = glam::Vec2::new(scene_bounds[0] as f32 - 0.5, scene_bounds[1] as f32);
    let ray_origin_offset = framebuffer_offset.extend(0.0);

    let camera_inverse = camera.inverse();
    let ray_direction = camera_inverse.transform_vector3(glam::Vec3::new(0.0, 0.0, 1.0)).normalize();

    let mut framebuffer = {
        let width = usize::try_from(scene_bounds[2] - scene_bounds[0]).unwrap() + 1;
        let height = usize::try_from(scene_bounds[3] - scene_bounds[1]).unwrap();
        crate::Framebuffer::new(width, height, framebuffer_offset)
    };

    for y in 0..framebuffer.height() {
        for x in 0..framebuffer.width() {
            let fragment = framebuffer.get_fragment_mut(x, y);
            sample_point(
                scene,
                mesh_types,
                &camera_inverse,
                lights,
                multi_samples_x,
                multi_samples_y,
                edge_distance,
                fragment,
                &mut rng,
                &ray_origin_offset,
                &ray_direction,
                x,
                y,
            );
        }
    }

    framebuffer
}

pub fn render_scene_depth(
    scene: &crate::raytrace::Scene,
    mesh_types: &[crate::raytrace::MeshType],
    camera: &glam::Mat4,
    multi_samples_x: usize,
    multi_samples_y: usize,
) -> crate::DepthBuffer {
    let scene_bounds = scene.get_scene_screen_bounds(camera, mesh_types);
    let offset = glam::Vec2::new(scene_bounds[0] as f32 - 0.5, scene_bounds[1] as f32);
    let ray_origin_offset = offset.extend(0.0);

    let camera_inverse = camera.inverse();
    let ray_direction = camera_inverse.transform_vector3(glam::Vec3::new(0.0, 0.0, 1.0)).normalize();

    let mut depth_buffer = {
        let width = usize::try_from(scene_bounds[2] - scene_bounds[0]).unwrap() + 1;
        let height = usize::try_from(scene_bounds[3] - scene_bounds[1]).unwrap();
        let offset = glam::IVec2::new(offset.x.floor() as i32, offset.y.floor() as i32 - 1);
        crate::DepthBuffer::new(width, height, offset)
    };

    for y in 0..depth_buffer.height() {
        for x in 0..depth_buffer.width() {
            let ray_origin = glam::Vec3::new(x as f32, y as f32, -512.0) + ray_origin_offset;

            let mut depth = f32::INFINITY;
            for sub_x in 0..multi_samples_x {
                for sub_y in 0..multi_samples_y {
                    let ray_origin = glam::Vec3::new(
                        (sub_x as f32 + 0.5) / multi_samples_x as f32 - 0.5,
                        (sub_y as f32 + 0.5) / multi_samples_y as f32 - 0.5,
                        0.0,
                    ) + ray_origin;
                    let ray_origin = camera_inverse.transform_vector3(ray_origin);

                    depth = depth.min(scene.trace_depth_ray(&ray_origin, &ray_direction));
                }
            }

            depth_buffer.set_depth(x, y, depth);
        }
    }

    depth_buffer
}
