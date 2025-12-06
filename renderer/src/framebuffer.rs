#[derive(Clone, Copy, Default)]
pub(crate) struct Fragment {
    pub(crate) colour: glam::Vec3,
    pub(crate) palette_region_type: Option<crate::palette::RegionType>,
}

pub struct Framebuffer {
    pub(crate) buffer: Vec<Fragment>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl Framebuffer {
    fn bounds(&self) -> [usize; 4] {
        let mut min_x = self.width;
        let mut min_y = self.height;
        let mut max_x = 0;
        let mut max_y = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.buffer[y * self.width + x].palette_region_type.is_some() {
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
            .flat_map(|x| {
                if x.palette_region_type.is_some() {
                    crate::palette::linear_to_srgb_rgb(&x.colour)
                } else {
                    [255; 3]
                }
            })
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
                let fragment = &self.buffer[y * self.width + x];
                if let Some(palette_region_type) = fragment.palette_region_type {
                    // not sure about this
                    let colour =
                        crate::palette::srgb_to_linear_rgb(&crate::palette::linear_to_srgb_rgb(&fragment.colour));
                    let nearest_colour = crate::palette::get_nearest_colour(&colour, palette_region_type);

                    pixels[(x - min_x) + (y - min_y) * width] = nearest_colour.index;

                    if dither {
                        let points = [[x - 1, y], [x + 1, y + 1], [x, y + 1], [x - 1, y + 1]];
                        let weights: [f32; 4] = [7.0 / 16.0, 3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0];

                        for (point, weight) in points.iter().zip(weights) {
                            self.buffer[point[1] * self.width + point[0]].colour +=
                                nearest_colour.error * (0.3 * weight);
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
