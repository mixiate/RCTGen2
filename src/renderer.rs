pub struct Light {
    pub diffuse: bool,
    pub specular: bool,
    pub direction: glam::Vec3,
    pub strength: f32,
    pub shadow: bool,
}

pub struct Framebuffer {
    pub buffer: Vec<[f32; 3]>,
    pub width: usize,
    pub height: usize,
}

pub fn render_scene(scene: &crate::raytrace::Scene, camera: &glam::Mat4, lights: &[Light]) -> Framebuffer {
    let scene_bounds = scene.get_scene_screen_bounds(camera);
    let offset = glam::Vec3::new(scene_bounds[0] as f32 - 0.5, scene_bounds[1] as f32, 0.0);

    let camera_inverse = camera.inverse();

    let width = (scene_bounds[2] - scene_bounds[0]) as usize + 1;
    let height = (scene_bounds[3] - scene_bounds[1]) as usize;
    let mut buffer = vec![[1.0; 3]; width * height];

    for y in 0..height {
        for x in 0..width {
            let origin = glam::Vec3::new(x as f32, (height - 1 - y) as f32, -512.0) + offset;
            let origin = camera_inverse.transform_point3(origin);
            let direction = glam::Vec3::new(0.0, 0.0, 1.0);
            let direction = camera_inverse.transform_vector3(direction).normalize();

            if let Some(hit) = scene.trace_ray(&origin, &direction) {
                let mut sample = glam::Vec3::new(0.0, 0.0, 0.0);
                for light in lights {
                    if light.shadow && scene.trace_occlusion_ray(&hit.position, &light.direction) {
                        continue;
                    }
                    if light.diffuse {
                        let light = hit.normal.dot(light.direction).max(0.0) * light.strength;
                        sample += light * hit.material.diffuse;
                    }
                    if light.specular {
                        let reflected_direction = hit.normal * (2.0 * light.direction.dot(hit.normal));
                        let reflected_direction = reflected_direction - light.direction;
                        let angle = reflected_direction.dot(-direction).max(0.0);
                        let specular_factor = light.strength * angle.powf(hit.material.specular_exponent);
                        sample += specular_factor * hit.material.specular;
                    }
                }
                buffer[y * width + x] = sample.into();
            }
        }
    }

    Framebuffer { buffer, width, height }
}
