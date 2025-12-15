struct Entry {
    data_offset: u32,
    width: i16,
    height: i16,
    offset_x: i16,
    offset_y: i16,
    flags: u16,
    zoom_offset: u16,
}

pub struct Archive {
    entries: Vec<Entry>,
    data: Vec<u8>,
}

impl Archive {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            data: Vec::new(),
        }
    }

    pub fn add_indexed_image(&mut self, image: &crate::image::IndexedImage) {
        self.entries.push(Entry {
            data_offset: u32::try_from(self.data.len()).unwrap(),
            width: i16::try_from(image.width()).unwrap(),
            height: i16::try_from(image.height()).unwrap(),
            offset_x: i16::try_from(image.offset().x).unwrap(),
            offset_y: i16::try_from(image.offset().y).unwrap(),
            flags: 1,
            zoom_offset: 0,
        });
        self.data.extend(image.as_raw());
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        use std::io::Write as _;

        let file = std::fs::File::create(path)?;
        let mut writer = std::io::BufWriter::new(file);

        let entry_count = u32::try_from(self.entries.len()).unwrap();
        let data_size = u32::try_from(self.data.len()).unwrap();

        writer.write_all(&entry_count.to_le_bytes())?;
        writer.write_all(&data_size.to_le_bytes())?;

        for entry in &self.entries {
            writer.write_all(&entry.data_offset.to_le_bytes())?;
            writer.write_all(&entry.width.to_le_bytes())?;
            writer.write_all(&entry.height.to_le_bytes())?;
            writer.write_all(&entry.offset_x.to_le_bytes())?;
            writer.write_all(&entry.offset_y.to_le_bytes())?;
            writer.write_all(&entry.flags.to_le_bytes())?;
            writer.write_all(&entry.zoom_offset.to_le_bytes())?;
        }

        writer.write_all(&self.data)?;

        Ok(())
    }
}
