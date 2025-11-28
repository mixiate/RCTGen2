pub struct Light {
    pub diffuse: bool,
    pub specular: bool,
    pub direction: glam::Vec3,
    pub strength: f32,
    pub shadow: bool,
}

pub struct Framebuffer {
    pub buffer: Vec<Option<glam::Vec3>>,
    pub width: usize,
    pub height: usize,
}

impl Framebuffer {
    #[allow(dead_code)]
    pub fn to_indexed_image(&self) -> IndexedImage {
        let pixels = self
            .buffer
            .iter()
            .map(|x| x.map_or(0, |x| crate::palette::get_nearest_colour(&x).index))
            .collect::<Vec<u8>>();

        IndexedImage {
            pixels,
            width: self.width,
            height: self.height,
        }
    }

    pub fn to_cropped_indexed_image(&self) -> IndexedImage {
        let (min_x, min_y, max_x, max_y) = {
            let mut min_x = self.width;
            let mut min_y = self.height;
            let mut max_x = 0;
            let mut max_y = 0;
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.buffer[y * self.width + x].is_some() {
                        min_x = std::cmp::min(min_x, x);
                        min_y = std::cmp::min(min_y, y);
                        max_x = std::cmp::max(max_x, x + 1);
                        max_y = std::cmp::max(max_y, y + 1);
                    }
                }
            }
            (min_x, min_y, max_x, max_y)
        };
        let cropped_width = max_x - min_x;
        let cropped_height = max_y - min_y;

        let mut pixels = Vec::with_capacity(cropped_width * cropped_height);
        for y in min_y..max_y {
            for x in min_x..max_x {
                let sample = &self.buffer[y * self.width + x];
                pixels.push(sample.map_or(0, |x| crate::palette::get_nearest_colour(&x).index));
            }
        }

        IndexedImage {
            pixels,
            width: cropped_width,
            height: cropped_height,
        }
    }
}

pub struct IndexedImage {
    pub pixels: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

pub fn render_scene(
    scene: &crate::raytrace::Scene,
    camera: &glam::Mat4,
    lights: &[Light],
    multi_samples_x: usize,
    multi_samples_y: usize,
) -> Framebuffer {
    let scene_bounds = scene.get_scene_screen_bounds(camera);
    let offset = glam::Vec3::new(scene_bounds[0] as f32 - 0.5, scene_bounds[1] as f32, 0.0);

    let camera_inverse = camera.inverse();

    let width = (scene_bounds[2] - scene_bounds[0]) as usize + 1;
    let height = (scene_bounds[3] - scene_bounds[1]) as usize;
    let mut buffer = vec![None; width * height];

    let multi_sample_count = multi_samples_x * multi_samples_y;

    for y in 0..height {
        for x in 0..width {
            let origin = glam::Vec3::new(x as f32, (height - 1 - y) as f32, -512.0) + offset;
            let direction = glam::Vec3::new(0.0, 0.0, 1.0);
            let direction = camera_inverse.transform_vector3(direction).normalize();

            {
                let origin = camera_inverse.transform_point3(origin);
                if scene.trace_ray(&origin, &direction).is_none() {
                    continue;
                }
            }

            let mut sub_samples = vec![None; multi_sample_count];

            for sub_y in 0..multi_samples_y {
                for sub_x in 0..multi_samples_x {
                    let origin = origin
                        + glam::Vec3::new(
                            (sub_x as f32 + 0.5) / multi_samples_x as f32 - 0.5,
                            (sub_y as f32 + 0.5) / multi_samples_y as f32 - 0.5,
                            0.0,
                        );
                    let origin = camera_inverse.transform_point3(origin);

                    if let Some(hit) = scene.trace_ray(&origin, &direction) {
                        let mut sample = None;
                        for light in lights {
                            if light.shadow && scene.trace_occlusion_ray(&hit.position, &light.direction) {
                                continue;
                            }
                            if light.diffuse {
                                let light = hit.normal.dot(light.direction).max(0.0) * light.strength;
                                *sample.get_or_insert_default() += light * hit.material.diffuse;
                            }
                            if light.specular {
                                let reflected_direction = hit.normal * (2.0 * light.direction.dot(hit.normal));
                                let reflected_direction = reflected_direction - light.direction;
                                let angle = reflected_direction.dot(-direction).max(0.0);
                                let specular_factor = light.strength * angle.powf(hit.material.specular_exponent);
                                *sample.get_or_insert_default() += specular_factor * hit.material.specular;
                            }
                        }
                        sub_samples[sub_y * multi_samples_x + sub_x] = sample;
                    }
                }
            }

            let sample = sub_samples.iter().flatten().sum::<glam::Vec3>();
            let sub_sample_count = sub_samples.iter().flatten().count();
            buffer[y * width + x] = Some(sample / sub_sample_count as f32);
        }
    }

    Framebuffer { buffer, width, height }
}
