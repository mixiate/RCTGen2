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

#[derive(Clone)]
pub struct IndexedImage {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
    pub offset: glam::IVec2,
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

    pub fn blit(&mut self, image: &IndexedImage, dest_x: i32, dest_y: i32) {
        let dest_x = usize::try_from(dest_x).unwrap();
        let dest_y = usize::try_from(dest_y).unwrap();
        for y in 0..image.height() {
            for x in 0..image.width() {
                self.set_pixel(dest_x + x, dest_y + y, image.get_pixel(x, y));
            }
        }
    }

    pub fn crop(&mut self) {
        let (min_x, min_y, max_x, max_y) = {
            let mut min_x = self.width;
            let mut min_y = self.height;
            let mut max_x = 0;
            let mut max_y = 0;
            for y in 0..self.height {
                for x in 0..self.width {
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
            let stride = self.width;
            self.offset += glam::IVec2::new(min_x.try_into().unwrap(), min_y.try_into().unwrap());
            self.width = max_x - min_x;
            self.height = max_y - min_y;

            for y in 0..self.height {
                for x in 0..self.width {
                    self.pixels[x + y * self.width] = self.pixels[(x + min_x) + (y + min_y) * stride];
                }
            }

            self.pixels.truncate(self.width * self.height);
        }
    }

    pub fn as_raw(&self) -> &Vec<u8> {
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
            info.color_type == png::ColorType::Indexed,
            "Error reading {}, image is not indexed",
            path.display()
        );

        let width = usize::try_from(info.width)?;
        let height = usize::try_from(info.height)?;

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

        let mut encoder = png::Encoder::new(w, self.width.try_into()?, self.height.try_into()?);
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

        for y in 0..self.height() {
            for x in 0..self.width() {
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
