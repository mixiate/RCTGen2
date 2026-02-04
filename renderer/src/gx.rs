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

pub struct EncodedSprite {
    row_offsets: Vec<i16>,
    data: Vec<u8>,
}

impl EncodedSprite {
    pub fn new(pixels: &[u8], width: usize, height: usize) -> Self {
        let mut row_offsets = Vec::with_capacity(height);
        let mut data = Vec::new();

        for y in 0..height {
            row_offsets.push(i16::try_from((height * 2) + data.len()).unwrap());

            let mut last_count_index = None;
            let mut x_offset = 0;
            let mut pixel_count = 0;

            let mut push_run = |last_count_index: &mut Option<usize>, x_offset: &mut usize, pixel_count: &mut usize| {
                *last_count_index = Some(data.len());
                data.push(u8::try_from(*pixel_count).unwrap());
                data.push(u8::try_from(*x_offset).unwrap());
                for i in 0..*pixel_count {
                    let x = *x_offset + i;
                    data.push(pixels[x + y * width]);
                }
                *x_offset = 0;
                *pixel_count = 0;
            };

            for x in 0..width {
                if pixels[x + y * width] == 0 {
                    if pixel_count != 0 {
                        push_run(&mut last_count_index, &mut x_offset, &mut pixel_count);
                    }
                } else {
                    if pixel_count == 0 {
                        x_offset = x;
                    }
                    pixel_count += 1;
                }

                if pixel_count == 127 {
                    push_run(&mut last_count_index, &mut x_offset, &mut pixel_count);
                }
            }

            if pixel_count > 0 || last_count_index.is_none() {
                push_run(&mut last_count_index, &mut x_offset, &mut pixel_count);
            }

            if let Some(last_count_index) = last_count_index {
                data[last_count_index] |= 0b1000_0000;
            }
        }

        Self { row_offsets, data }
    }
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

    pub fn add_sprite(&mut self, pixels: &[u8], width: usize, height: usize, x: i32, y: i32) {
        self.entries.push(Entry {
            data_offset: u32::try_from(self.data.len()).unwrap(),
            width: i16::try_from(width).unwrap(),
            height: i16::try_from(height).unwrap(),
            offset_x: i16::try_from(x).unwrap(),
            offset_y: i16::try_from(y).unwrap(),
            flags: ENTRY_FLAG_TRANSPARENT,
            zoom_offset: 0,
        });
        self.data.extend(pixels);
    }

    pub fn add_encoded_sprite(&mut self, encoded_sprite: &EncodedSprite, width: usize, height: usize, x: i32, y: i32) {
        self.entries.push(Entry {
            data_offset: u32::try_from(self.data.len()).unwrap(),
            width: i16::try_from(width).unwrap(),
            height: i16::try_from(height).unwrap(),
            offset_x: i16::try_from(x).unwrap(),
            offset_y: i16::try_from(y).unwrap(),
            flags: ENTRY_FLAG_TRANSPARENT | ENTRY_FLAG_RLE,
            zoom_offset: 0,
        });

        for row_offset in &encoded_sprite.row_offsets {
            self.data.extend(row_offset.to_le_bytes());
        }

        self.data.extend(&encoded_sprite.data);
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_archive() {
        let test_files_directory = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
        let test_files_directory = test_files_directory.join("tests").join("files").join("gx archive");

        let expected_file_path = test_files_directory.join("images").with_extension("dat");

        let test_image_path = test_files_directory.join("rle_test").with_extension("png");
        let test_image = crate::image::IndexedImage::load(&test_image_path, &crate::palette::PALETTE_FLAT).unwrap();
        let mut archive = crate::gx::Archive::with_capacity(2);
        archive.add_sprite(
            test_image.as_raw(),
            test_image.width(),
            test_image.height(),
            test_image.offset.x,
            test_image.offset.y,
        );
        let encoded_sprite =
            crate::gx::EncodedSprite::new(test_image.as_raw(), test_image.width(), test_image.height());
        archive.add_encoded_sprite(
            &encoded_sprite,
            test_image.width(),
            test_image.height(),
            test_image.offset.x,
            test_image.offset.y,
        );

        let temp_dir = tempfile::tempdir().unwrap();
        let temp_file_path = temp_dir.path().join("images").with_extension("dat");
        archive.save(&temp_file_path).unwrap();

        let output_file_bytes = std::fs::read(&temp_file_path).unwrap();
        let expected_file_bytes = std::fs::read(&expected_file_path).unwrap();

        assert_eq!(output_file_bytes, expected_file_bytes);
    }
}
