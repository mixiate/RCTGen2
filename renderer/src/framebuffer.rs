#[derive(Clone, Copy)]
pub(crate) struct Fragment {
    pub(crate) colour: glam::Vec3,
    pub(crate) depth: f32,
    pub(crate) ghost_depth: f32,
    pub(crate) edge_type: Option<crate::renderer::EdgeType>,
    pub(crate) palette_region_type: Option<crate::palette::RegionType>,
    pub(crate) is_mask: bool,
}

impl Default for Fragment {
    fn default() -> Self {
        Self {
            colour: glam::Vec3::new(0.0, 0.0, 0.0),
            depth: f32::INFINITY,
            ghost_depth: f32::INFINITY,
            edge_type: None,
            palette_region_type: None,
            is_mask: false,
        }
    }
}

pub struct Framebuffer {
    pub(crate) buffer: Vec<Fragment>,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) offset: glam::Vec2,
}

impl Framebuffer {
    fn bounds(&self) -> Option<[usize; 4]> {
        let mut found_pixel = false;
        let mut min_x = self.width;
        let mut min_y = self.height;
        let mut max_x = 0;
        let mut max_y = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.buffer[y * self.width + x].palette_region_type.is_some() {
                    found_pixel = true;
                    min_x = std::cmp::min(min_x, x);
                    min_y = std::cmp::min(min_y, y);
                    max_x = std::cmp::max(max_x, x + 1);
                    max_y = std::cmp::max(max_y, y + 1);
                }
            }
        }
        found_pixel.then_some([min_x, min_y, max_x, max_y])
    }

    fn get_offset(&self, x: usize, y: usize) -> glam::IVec2 {
        // ORIGINAL COMMENT: y - 1 compensates for error not sure why it's needed TODO work out why it's needed
        glam::IVec2::new(
            x as i32 + self.offset.x.floor() as i32,
            y as i32 + self.offset.y.floor() as i32 - 1,
        )
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
        crate::image::Image::from_raw(self.width, self.height, pixels)
    }

    fn into_indexed_image_inner(mut self, dither: bool, bounds: &[usize; 4]) -> crate::image::IndexedImage {
        let [min_x, min_y, max_x, max_y] = *bounds;
        let width = max_x - min_x;
        let height = max_y - min_y;
        let mut image = crate::image::IndexedImage::with_offset(width, height, self.get_offset(min_x, min_y));

        for y in min_y..max_y {
            for x in (min_x..max_x).rev() {
                let fragment = &self.buffer[y * self.width + x];
                if let Some(palette_region_type) = fragment.palette_region_type {
                    // not sure about this
                    let colour =
                        crate::palette::srgb_to_linear_rgb(&crate::palette::linear_to_srgb_rgb(&fragment.colour));
                    let nearest_colour = crate::palette::get_nearest_colour(&colour, palette_region_type);

                    image.set_pixel(x - min_x, y - min_y, nearest_colour.index);

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

        image
    }

    pub fn into_indexed_image(self, dither: bool) -> crate::image::IndexedImage {
        let bounds = [0, 0, self.width, self.height];
        self.into_indexed_image_inner(dither, &bounds)
    }

    pub fn into_cropped_indexed_image(self, dither: bool) -> crate::image::IndexedImage {
        if let Some(bounds) = self.bounds() {
            self.into_indexed_image_inner(dither, &bounds)
        } else {
            // Output a 1x1 transparent pixel here as the game often requires sprites even if they are empty
            crate::image::IndexedImage::new(1, 1)
        }
    }
}
