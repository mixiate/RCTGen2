pub struct Image {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn from_raw(width: usize, height: usize, pixels: Vec<u8>) -> Self {
        assert!(pixels.len() == width * height * 4);
        Self { pixels, width, height }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn as_raw(&self) -> &[u8] {
        &self.pixels
    }

    pub fn save(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let image_file = std::fs::File::create(path)?;
        let w = std::io::BufWriter::new(image_file);

        let mut encoder = png::Encoder::new(w, self.width.try_into()?, self.height.try_into()?);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.pixels)?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct IndexedImage {
    pixels: Vec<u8>,
    width: u16,
    height: u16,
    pub offset: glam::IVec2,
}

impl IndexedImage {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            pixels: vec![0; usize::from(width) * usize::from(height)],
            width,
            height,
            offset: glam::IVec2::new(0, 0),
        }
    }

    pub fn with_offset(width: u16, height: u16, offset: glam::IVec2) -> Self {
        Self {
            pixels: vec![0; usize::from(width) * usize::from(height)],
            width,
            height,
            offset,
        }
    }

    pub fn with_buffer(pixels: Vec<u8>, width: u16, height: u16) -> Self {
        assert!(pixels.len() == usize::from(width) * usize::from(height));
        Self {
            pixels,
            width,
            height,
            offset: glam::IVec2::new(0, 0),
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        self.pixels[x + (y * usize::from(self.width))]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: u8) {
        self.pixels[x + (y * usize::from(self.width))] = pixel;
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn blit(&mut self, image: &IndexedImage, dest_x: usize, dest_y: usize) {
        for y in 0..usize::from(image.height()) {
            for x in 0..usize::from(image.width()) {
                self.set_pixel(dest_x + x, dest_y + y, image.get_pixel(x, y));
            }
        }
    }

    pub fn crop(&mut self) {
        let (min_x, min_y, max_x, max_y) = {
            let mut min_x = usize::from(self.width);
            let mut min_y = usize::from(self.height);
            let mut max_x = 0;
            let mut max_y = 0;
            for y in 0..usize::from(self.height) {
                for x in 0..usize::from(self.width) {
                    if self.get_pixel(x, y) != 0 {
                        min_x = std::cmp::min(min_x, x);
                        min_y = std::cmp::min(min_y, y);
                        max_x = std::cmp::max(max_x, x + 1);
                        max_y = std::cmp::max(max_y, y + 1);
                    }
                }
            }
            (min_x, min_y, max_x, max_y)
        };

        if max_x < min_x {
            self.width = 1;
            self.height = 1;
            self.offset = glam::IVec2::splat(0);
            self.pixels.truncate(1);
        } else {
            let stride = usize::from(self.width);
            self.offset += glam::IVec2::new(min_x.try_into().unwrap(), min_y.try_into().unwrap());
            self.width = u16::try_from(max_x - min_x).unwrap();
            self.height = u16::try_from(max_y - min_y).unwrap();

            for y in 0..usize::from(self.height) {
                for x in 0..usize::from(self.width) {
                    self.pixels[x + y * usize::from(self.width)] = self.pixels[(x + min_x) + (y + min_y) * stride];
                }
            }

            self.pixels.truncate(usize::from(self.width) * usize::from(self.height));
        }
    }

    pub fn as_raw(&self) -> &[u8] {
        &self.pixels
    }

    pub fn load(path: &std::path::Path, expected_palette: &[u8]) -> anyhow::Result<Self> {
        use anyhow::Context as _;

        let file = std::fs::File::open(path)?;
        let decoder = png::Decoder::new(std::io::BufReader::new(file));
        let mut reader = decoder.read_info()?;

        let palette = reader
            .info()
            .palette
            .as_ref()
            .with_context(|| format!("Error reading {}, image has no palette", path.display()))?;
        anyhow::ensure!(
            *palette == expected_palette,
            "Error reading {}, image palette is incorrect",
            path.display()
        );

        let buffer_size = reader
            .output_buffer_size()
            .with_context(|| format!("Error reading {} buffer size", path.display()))?;
        let mut buffer = vec![0; buffer_size];
        let info = reader.next_frame(&mut buffer)?;
        buffer.truncate(info.buffer_size());

        anyhow::ensure!(
            info.bit_depth == png::BitDepth::Eight,
            "Error reading {}, image bit depth is not 8",
            path.display()
        );
        anyhow::ensure!(
            info.color_type == png::ColorType::Indexed,
            "Error reading {}, image is not indexed",
            path.display()
        );

        let width = u16::try_from(info.width)?;
        let height = u16::try_from(info.height)?;

        Ok(Self {
            pixels: buffer,
            width,
            height,
            offset: glam::IVec2::new(0, 0),
        })
    }

    pub fn save(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let image_file = std::fs::File::create(path)?;
        let w = std::io::BufWriter::new(image_file);

        let mut encoder = png::Encoder::new(w, self.width.into(), self.height.into());
        encoder.set_color(png::ColorType::Indexed);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_palette(&crate::palette::PALETTE_FLAT);
        encoder.set_trns(&[0]);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.pixels)?;

        Ok(())
    }

    pub fn water_colours_to_regular_colours(&mut self) {
        const WATER_FIRST_INDEX: u8 = 230;
        const WATER_LAST_INDEX: u8 = 239;

        for y in 0..usize::from(self.height()) {
            for x in 0..usize::from(self.width()) {
                let pixel = self.get_pixel(x, y);
                if (WATER_FIRST_INDEX..=WATER_LAST_INDEX).contains(&pixel) {
                    let colour = crate::palette::srgb_to_linear_rgb(&crate::palette::PALETTE[usize::from(pixel)]);
                    let nearest_colour =
                        crate::palette::get_nearest_colour(&colour, crate::palette::RegionType::NoRemaps);
                    self.set_pixel(x, y, nearest_colour.index);
                }
            }
        }
    }
}
