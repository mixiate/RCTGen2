pub struct IndexedImage {
    pub pixels: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl IndexedImage {
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
