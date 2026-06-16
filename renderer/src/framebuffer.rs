#[derive(Clone, Copy)]
pub(crate) struct Fragment {
    pub(crate) colour: glam::Vec3,
    pub(crate) depth: f32,
    pub(crate) ghost_depth: f32,
    pub(crate) edge_type: Option<crate::renderer::EdgeType>,
    pub(crate) palette_region_type: Option<crate::palette::RegionType>,
    pub(crate) is_mask: bool,
    pub(crate) no_bleed: bool,
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
            no_bleed: false,
        }
    }
}

pub struct Framebuffer {
    buffer: Vec<Fragment>,
    width: usize,
    height: usize,
    offset: glam::Vec2,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, offset: glam::Vec2) -> Self {
        let buffer = vec![Fragment::default(); width * height];

        Self {
            buffer,
            width,
            height,
            offset,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub(crate) fn get_fragment(&self, x: usize, y: usize) -> &Fragment {
        &self.buffer[x + (y * self.width)]
    }

    pub(crate) fn get_fragment_mut(&mut self, x: usize, y: usize) -> &mut Fragment {
        &mut self.buffer[x + (y * self.width)]
    }

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
        let offset = glam::IVec2::new(
            min_x as i32 + self.offset.x.floor() as i32,
            // ORIGINAL COMMENT: y - 1 compensates for error not sure why it's needed TODO work out why it's needed
            min_y as i32 + self.offset.y.floor() as i32 - 1,
        );
        let mut image =
            crate::image::IndexedImage::with_offset(width.try_into().unwrap(), height.try_into().unwrap(), offset);

        for y in min_y..max_y {
            for x in (min_x..max_x).rev() {
                let fragment = &self.buffer[y * self.width + x];
                let no_bleed = fragment.no_bleed;
                if let Some(palette_region_type) = fragment.palette_region_type {
                    // not sure about this
                    let colour =
                        crate::palette::srgb_to_linear_rgb(&crate::palette::linear_to_srgb_rgb(&fragment.colour));
                    let nearest_colour = crate::palette::get_nearest_colour(&colour, palette_region_type);

                    image.set_pixel(x - min_x, y - min_y, nearest_colour.index);

                    if dither && !no_bleed {
                        let mut distribute_error = |x: usize, y: usize, weight: f32| {
                            let next_fragment = &mut self.buffer[x + (y * self.width)];
                            if !next_fragment.no_bleed {
                                next_fragment.colour += nearest_colour.error * (0.3 * weight);
                            }
                        };

                        if x > min_x {
                            distribute_error(x - 1, y, 7.0 / 16.0);
                        }
                        if x < max_x - 1 && y < max_y - 1 {
                            distribute_error(x + 1, y + 1, 3.0 / 16.0);
                        }
                        if y < max_y - 1 {
                            distribute_error(x, y + 1, 5.0 / 16.0);
                        }
                        if x > min_x && y < max_y - 1 {
                            distribute_error(x - 1, y + 1, 1.0 / 16.0);
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

    pub fn to_cropped_depth(&self) -> DepthBuffer {
        if let Some(bounds) = self.bounds() {
            let [min_x, min_y, max_x, max_y] = bounds;
            let width = max_x - min_x;
            let height = max_y - min_y;
            let offset = glam::IVec2::new(
                min_x as i32 + self.offset.x.floor() as i32,
                // ORIGINAL COMMENT: y - 1 compensates for error not sure why it's needed TODO work out why it's needed
                min_y as i32 + self.offset.y.floor() as i32 - 1,
            );
            let mut depth_buffer = DepthBuffer::new(width, height, offset);
            for y in min_y..max_y {
                for x in min_x..max_x {
                    depth_buffer.set_depth(x - min_x, y - min_y, self.get_fragment(x, y).depth);
                }
            }
            depth_buffer
        } else {
            DepthBuffer::new(1, 1, glam::IVec2::default())
        }
    }
}

pub struct DepthBuffer {
    buffer: Vec<f32>,
    width: usize,
    height: usize,
    pub offset: glam::IVec2,
}

impl DepthBuffer {
    pub fn new(width: usize, height: usize, offset: glam::IVec2) -> Self {
        let buffer = vec![f32::INFINITY; width * height];

        Self {
            buffer,
            width,
            height,
            offset,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_depth(&self, x: usize, y: usize) -> f32 {
        self.buffer[x + y * self.width]
    }

    pub fn set_depth(&mut self, x: usize, y: usize, depth: f32) {
        self.buffer[x + y * self.width] = depth;
    }
}
