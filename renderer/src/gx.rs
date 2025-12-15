struct Entry {
    data_offset: u32,
    width: i16,
    height: i16,
    offset_x: i16,
    offset_y: i16,
    flags: u16,
    zoom_offset: u16,
}

// This should be added to all non palette sprites, as is the case for all vanilla sprites.
// It's not actually used in OpenRCT2 to signify transparency. It's either vestigial or incorrectly named.
const ENTRY_FLAG_TRANSPARENT: u16 = 1;

const ENTRY_FLAG_RLE: u16 = 4;

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
            flags: ENTRY_FLAG_TRANSPARENT,
            zoom_offset: 0,
        });
        self.data.extend(image.as_raw());
    }

    pub fn add_indexed_image_rle(&mut self, image: &crate::image::IndexedImage) {
        self.entries.push(Entry {
            data_offset: u32::try_from(self.data.len()).unwrap(),
            width: i16::try_from(image.width()).unwrap(),
            height: i16::try_from(image.height()).unwrap(),
            offset_x: i16::try_from(image.offset().x).unwrap(),
            offset_y: i16::try_from(image.offset().y).unwrap(),
            flags: ENTRY_FLAG_TRANSPARENT | ENTRY_FLAG_RLE,
            zoom_offset: 0,
        });

        let mut row_offsets = Vec::with_capacity(image.height());
        let mut rle_data = Vec::new();

        for y in 0..image.height() {
            row_offsets.push(i16::try_from((image.height() * 2) + rle_data.len()).unwrap());

            let mut last_count_index = None;
            let mut x_offset = 0;
            let mut pixel_count = 0;
            for x in 0..image.width() {
                if image.get_pixel(x, y) == 0 {
                    if pixel_count != 0 {
                        last_count_index = Some(rle_data.len());
                        rle_data.push(u8::try_from(pixel_count).unwrap());
                        rle_data.push(u8::try_from(x_offset).unwrap());
                        for i in 0..pixel_count {
                            rle_data.push(image.get_pixel(x_offset + i, y));
                        }
                        x_offset = 0;
                        pixel_count = 0;
                    }
                } else {
                    if pixel_count == 0 {
                        x_offset = x;
                    }
                    pixel_count += 1;
                }

                if pixel_count == 127 {
                    last_count_index = Some(rle_data.len());
                    rle_data.push(u8::try_from(pixel_count).unwrap());
                    rle_data.push(u8::try_from(x_offset).unwrap());
                    for i in 0..pixel_count {
                        rle_data.push(image.get_pixel(x_offset + i, y));
                    }
                    x_offset = 0;
                    pixel_count = 0;
                }
            }

            if pixel_count > 0 || last_count_index.is_none() {
                rle_data.push(u8::try_from(pixel_count).unwrap() | 0b1000_0000);
                rle_data.push(u8::try_from(x_offset).unwrap());
                for i in 0..pixel_count {
                    rle_data.push(image.get_pixel(x_offset + i, y));
                }
            } else if let Some(last_count_index) = last_count_index {
                rle_data[last_count_index] |= 0b1000_0000;
            }
        }

        for row_offset in row_offsets {
            self.data.extend(row_offset.to_le_bytes());
        }

        self.data.extend(rle_data);
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
