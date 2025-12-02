pub struct Light {
    pub diffuse_strength: f32,
    pub specular_strength: f32,
    pub direction: glam::Vec3,
    pub shadow: bool,
}

pub fn render_scene(
    scene: &crate::raytrace::Scene,
    camera: &glam::Mat4,
    lights: &[Light],
    multi_samples_x: usize,
    multi_samples_y: usize,
) -> crate::Framebuffer {
    let scene_bounds = scene.get_scene_screen_bounds(camera);
    let offset = glam::Vec3::new(scene_bounds[0] as f32 - 0.5, scene_bounds[1] as f32, 0.0);

    let camera_inverse = camera.inverse();

    let width = usize::try_from(scene_bounds[2] - scene_bounds[0]).unwrap() + 1;
    let height = usize::try_from(scene_bounds[3] - scene_bounds[1]).unwrap();
    let mut buffer = vec![None; width * height];

    let multi_sample_count = multi_samples_x * multi_samples_y;

    for y in 0..height {
        for x in 0..width {
            let origin = glam::Vec3::new(x as f32, y as f32, -512.0) + offset;
            let direction = glam::Vec3::new(0.0, 0.0, 1.0);
            let direction = camera_inverse.transform_vector3(direction).normalize();

            let palette_region_type = {
                let origin = camera_inverse.transform_vector3(origin);
                if let Some(hit) = scene.trace_ray(&origin, &direction) {
                    hit.material.palette_region_type
                } else {
                    continue;
                }
            };

            let mut samples = vec![None; multi_sample_count];

            for sub_x in 0..multi_samples_x {
                for sub_y in 0..multi_samples_y {
                    let origin = glam::Vec3::new(
                        (sub_x as f32 + 0.5) / multi_samples_x as f32 - 0.5,
                        (sub_y as f32 + 0.5) / multi_samples_y as f32 - 0.5,
                        0.0,
                    ) + origin;
                    let origin = camera_inverse.transform_vector3(origin);

                    if let Some(hit) = scene.trace_ray(&origin, &direction) {
                        let mut fragment: Option<crate::framebuffer::Fragment> = None;
                        for light in lights {
                            if light.shadow && scene.trace_occlusion_ray(&hit.position, &light.direction) {
                                fragment.get_or_insert_default();
                                continue;
                            }
                            if light.diffuse_strength > 0.0 {
                                let diffuse = hit.material.diffuse;
                                let diffuse = if hit.material.palette_region_type.is_diffuse_greyscale() {
                                    let max = diffuse.x.max(diffuse.y.max(diffuse.z));
                                    glam::Vec3::new(max, max, max)
                                } else {
                                    diffuse
                                };
                                let light = hit.normal.dot(light.direction).max(0.0) * light.diffuse_strength;
                                fragment.get_or_insert_default().colour += light * diffuse;
                            }
                            if light.specular_strength > 0.0 {
                                let reflected_direction = hit.normal * (2.0 * light.direction.dot(hit.normal));
                                let reflected_direction = reflected_direction - light.direction;
                                let angle = reflected_direction.dot(-direction).max(0.0);
                                let specular_factor =
                                    light.specular_strength * angle.powf(hit.material.specular_exponent);
                                fragment.get_or_insert_default().colour += specular_factor * hit.material.specular;
                            }
                        }
                        samples[sub_y * multi_samples_x + sub_x] = fragment;
                    }
                }
            }

            let colour = samples.iter().flatten().map(|x| x.colour).sum::<glam::Vec3>();
            let sample_count = samples.iter().flatten().count();
            buffer[y * width + x] = Some(crate::framebuffer::Fragment {
                colour: colour / sample_count as f32,
                palette_region_type,
            });
        }
    }

    crate::Framebuffer { buffer, width, height }
}
