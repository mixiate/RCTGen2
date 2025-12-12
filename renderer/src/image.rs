pub struct Image {
    pub pixels: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn save(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let image_file = std::fs::File::create(path)?;
        let w = std::io::BufWriter::new(image_file);

        let mut encoder = png::Encoder::new(w, self.width.try_into()?, self.height.try_into()?);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.pixels)?;

        Ok(())
    }
}
pub struct IndexedImage {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
    offset: glam::IVec2,
}

impl IndexedImage {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![0; width * height],
            width,
            height,
            offset: glam::IVec2::new(0, 0),
        }
    }

    pub fn with_offset(width: usize, height: usize, offset: glam::IVec2) -> Self {
        Self {
            pixels: vec![0; width * height],
            width,
            height,
            offset,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: u8) {
        self.pixels[x + (y * self.width)] = pixel;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn offset(&self) -> &glam::IVec2 {
        &self.offset
    }

    pub fn save(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let image_file = std::fs::File::create(path)?;
        let w = std::io::BufWriter::new(image_file);

        let mut encoder = png::Encoder::new(w, self.width.try_into()?, self.height.try_into()?);
        encoder.set_color(png::ColorType::Indexed);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_palette(&crate::palette::PALETTE_FLAT);
        encoder.set_trns(&[0]);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.pixels)?;

        Ok(())
    }
}

fn pack_rects_fixed(images: &[IndexedImage], width: usize, height: usize, coords: &mut [glam::IVec2]) -> bool {
    let mut rect_packer = rect_packer::DensePacker::new(width.try_into().unwrap(), height.try_into().unwrap());
    for (image, coord) in images.iter().zip(coords.iter_mut()) {
        if let Some(rect) = rect_packer.pack(image.width.try_into().unwrap(), image.height.try_into().unwrap(), false) {
            coord.x = rect.x;
            coord.y = rect.y;
        } else {
            return false;
        }
    }
    true
}

struct PackedRects {
    width: usize,
    height: usize,
    coords: Vec<glam::IVec2>,
}

fn pack_rects(images: &[IndexedImage]) -> PackedRects {
    let mut size = 256;
    let mut coords = vec![glam::IVec2::new(0, 0); images.len()];

    while !pack_rects_fixed(images, size, size, &mut coords) {
        size *= 2;
    }
    let size = size;

    // Use binary search to find smallest square that can fit the images
    let mut lower_size = size / 2;
    let mut upper_size = size;
    while upper_size - lower_size > 2 {
        let mid_size = (upper_size + lower_size) / 2;
        if pack_rects_fixed(images, mid_size, mid_size, &mut coords) {
            upper_size = mid_size;
        } else {
            lower_size = mid_size;
        }
    }

    // Use binary search to determine how much the height can be reduced
    let mut upper_height = upper_size;
    let mut lower_height = 0;
    while upper_height - lower_height > 2 {
        let mid_height = (upper_height + lower_height) / 2;
        if pack_rects_fixed(images, upper_size, mid_height, &mut coords) {
            upper_height = mid_height;
        } else {
            lower_height = mid_height;
        }
    }

    // Use binary search to determine how much the width can be reduced
    let mut upper_width = upper_size;
    let mut lower_width = 0;
    while upper_width - lower_width > 2 {
        let mid_width = (upper_width + lower_width) / 2;
        if pack_rects_fixed(images, mid_width, upper_height, &mut coords) {
            upper_width = mid_width;
        } else {
            lower_width = mid_width;
        }
    }

    let (width, height) = if upper_width < upper_height {
        (upper_width, upper_size)
    } else {
        (upper_size, upper_height)
    };

    if !pack_rects_fixed(images, width, height, &mut coords) {
        panic!();
    }

    PackedRects { width, height, coords }
}

pub fn create_atlas(images: &[IndexedImage]) -> (IndexedImage, Vec<glam::IVec2>) {
    let PackedRects { width, height, coords } = pack_rects(images);

    let mut atlas_image = IndexedImage::new(width, height);
    for (image, coord) in images.iter().zip(coords.iter()) {
        let rect_x = usize::try_from(coord.x).unwrap();
        let rect_y = usize::try_from(coord.y).unwrap();
        for y in 0..image.height {
            for x in 0..image.width {
                atlas_image.pixels[(rect_x + x) + (width * (rect_y + y))] = image.pixels[x + (image.width * y)];
            }
        }
    }

    (atlas_image, coords)
}
