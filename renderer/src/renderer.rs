pub struct Light {
    pub diffuse_strength: f32,
    pub specular_strength: f32,
    pub direction: glam::Vec3,
    pub shadow: bool,
}

pub struct Framebuffer {
    pub buffer: Vec<Option<glam::Vec3>>,
    pub width: usize,
    pub height: usize,
}

impl Framebuffer {
    fn bounds(&self) -> [usize; 4] {
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
        [min_x, min_y, max_x, max_y]
    }

    pub fn to_image(&self) -> crate::image::Image {
        let pixels = self
            .buffer
            .iter()
            .flat_map(|x| x.map_or([0; 3], |x| crate::palette::vec_to_colour(&x)))
            .collect::<Vec<u8>>();

        crate::image::Image {
            pixels,
            width: self.width,
            height: self.height,
        }
    }

    fn into_indexed_image_inner(mut self, dither: bool, bounds: &[usize; 4]) -> crate::image::IndexedImage {
        let [min_x, min_y, max_x, max_y] = *bounds;
        let width = max_x - min_x;
        let height = max_y - min_y;
        let mut pixels = vec![0; width * height];

        for y in min_y..max_y {
            for x in (min_x..max_x).rev() {
                let sample = &mut self.buffer[y * self.width + x];
                *sample = sample.as_ref().map(|x| crate::palette::colour_to_vec(&crate::palette::vec_to_colour(x)));

                let image_index = (x - min_x) + (y - min_y) * width;
                pixels[image_index] = sample.map_or(0, |x| crate::palette::get_nearest_colour(&x).index);

                if dither && let Some(sample) = &mut self.buffer[y * self.width + x] {
                    let nearest_colour = crate::palette::get_nearest_colour(sample);

                    let points = [[x - 1, y], [x + 1, y + 1], [x, y + 1], [x - 1, y + 1]];
                    let weights: [f32; 4] = [7.0 / 16.0, 3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0];

                    for (point, weight) in points.iter().zip(weights) {
                        if let Some(point_sample) = &mut self.buffer[point[1] * self.width + point[0]] {
                            *point_sample += nearest_colour.error * (0.3 * weight);
                        }
                    }
                }
            }
        }

        crate::image::IndexedImage { pixels, width, height }
    }

    pub fn into_indexed_image(self, dither: bool) -> crate::image::IndexedImage {
        let bounds = [0, 0, self.width, self.height];
        self.into_indexed_image_inner(dither, &bounds)
    }

    pub fn into_cropped_indexed_image(self, dither: bool) -> crate::image::IndexedImage {
        let bounds = self.bounds();
        self.into_indexed_image_inner(dither, &bounds)
    }
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

    let width = usize::try_from(scene_bounds[2] - scene_bounds[0]).unwrap() + 1;
    let height = usize::try_from(scene_bounds[3] - scene_bounds[1]).unwrap();
    let mut buffer = vec![None; width * height];

    let multi_sample_count = multi_samples_x * multi_samples_y;

    for y in 0..height {
        for x in 0..width {
            let origin = glam::Vec3::new(x as f32, y as f32, -512.0) + offset;
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
                                sample.get_or_insert(glam::Vec3::new(0.0, 0.0, 0.0));
                                continue;
                            }
                            if light.diffuse_strength > 0.0 {
                                let light = hit.normal.dot(light.direction).max(0.0) * light.diffuse_strength;
                                *sample.get_or_insert_default() += light * hit.material.diffuse;
                            }
                            if light.specular_strength > 0.0 {
                                let reflected_direction = hit.normal * (2.0 * light.direction.dot(hit.normal));
                                let reflected_direction = reflected_direction - light.direction;
                                let angle = reflected_direction.dot(-direction).max(0.0);
                                let specular_factor =
                                    light.specular_strength * angle.powf(hit.material.specular_exponent);
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
