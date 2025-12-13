pub struct Image {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn from_raw(width: usize, height: usize, pixels: Vec<u8>) -> Self {
        assert!(pixels.len() == width * height * 3);
        Self { pixels, width, height }
    }

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

    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        self.pixels[x + (y * self.width)]
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

    pub fn blit(&mut self, image: &IndexedImage, dest_x: i32, dest_y: i32) {
        let dest_x = usize::try_from(dest_x).unwrap();
        let dest_y = usize::try_from(dest_y).unwrap();
        for y in 0..image.height() {
            for x in 0..image.width() {
                self.set_pixel(dest_x + x, dest_y + y, image.get_pixel(x, y));
            }
        }
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
