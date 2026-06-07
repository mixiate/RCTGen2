#![expect(clippy::map_err_ignore)]

#[binrw::binrw]
pub struct Entry {
    data_offset: u32,
    pub width: u16,
    pub height: u16,
    pub offset_x: i16,
    pub offset_y: i16,
    pub flags: u16,
    zoom_offset: u16,
}

// This should be added to all non palette sprites, as is the case for all vanilla sprites.
// It's not actually used in OpenRCT2 to signify transparency. It's either vestigial or incorrectly named.
const ENTRY_FLAG_TRANSPARENT: u16 = 1;

const ENTRY_FLAG_RLE: u16 = 4;

pub struct EncodedSprite {
    width: u16,
    height: u16,
    row_offsets: Vec<i16>,
    data: Vec<u8>,
}

impl EncodedSprite {
    pub fn new(pixels: &[u8], width: u16, height: u16) -> Self {
        let mut row_offsets = Vec::with_capacity(height.into());
        let mut data = Vec::new();

        for y in 0..usize::from(height) {
            row_offsets.push(i16::try_from((usize::from(height) * 2) + data.len()).unwrap());

            let mut last_count_index = None;
            let mut x_offset = 0;
            let mut pixel_count = 0;

            let mut push_run = |last_count_index: &mut Option<usize>, x_offset: &mut usize, pixel_count: &mut usize| {
                *last_count_index = Some(data.len());
                data.push(u8::try_from(*pixel_count).unwrap());
                data.push(u8::try_from(*x_offset).unwrap());
                for i in 0..*pixel_count {
                    let x = *x_offset + i;
                    data.push(pixels[x + y * usize::from(width)]);
                }
                *x_offset = 0;
                *pixel_count = 0;
            };

            for x in 0..usize::from(width) {
                if pixels[x + y * usize::from(width)] == 0 {
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

        Self {
            width,
            height,
            row_offsets,
            data,
        }
    }
}

pub enum Pixels<'a> {
    Uncompressed(&'a [u8]),
    Compressed(Vec<u8>),
}

#[binrw::binrw]
pub struct Archive {
    #[bw(try_calc(u32::try_from(entries.len())))]
    entry_count: u32,
    #[bw(try_calc(u32::try_from(data.len())))]
    data_size: u32,
    #[br(count = entry_count)]
    entries: Vec<Entry>,
    #[br(count = data_size)]
    data: Vec<u8>,
}

impl Archive {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            data: Vec::new(),
        }
    }

    pub fn add_sprite(&mut self, pixels: &[u8], width: u16, height: u16, x: i32, y: i32) {
        assert!(pixels.len() == usize::from(width) * usize::from(height));
        self.entries.push(Entry {
            data_offset: u32::try_from(self.data.len()).unwrap(),
            width,
            height,
            offset_x: i16::try_from(x).unwrap(),
            offset_y: i16::try_from(y).unwrap(),
            flags: ENTRY_FLAG_TRANSPARENT,
            zoom_offset: 0,
        });
        self.data.extend(pixels);
    }

    pub fn add_encoded_sprite(&mut self, encoded_sprite: &EncodedSprite, x: i32, y: i32) {
        self.entries.push(Entry {
            data_offset: u32::try_from(self.data.len()).unwrap(),
            width: encoded_sprite.width,
            height: encoded_sprite.height,
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

    pub fn decode_sprite(&self, entry: &Entry) -> Option<Vec<u8>> {
        use byteorder::ReadBytesExt as _;
        use std::io::Read as _;

        assert!(entry.flags & ENTRY_FLAG_RLE != 0);

        let width = usize::from(entry.width);
        let height = usize::from(entry.height);
        let data_index = usize::try_from(entry.data_offset).ok()?;
        let mut row_offsets = {
            let size = height * std::mem::size_of::<u16>();
            self.data.get(data_index..(data_index + size))?
        };

        let mut pixels = vec![0; width * height];
        for y in 0..height {
            let row_offset = usize::from(row_offsets.read_u16::<byteorder::LittleEndian>().ok()?);
            let mut data = self.data.get(data_index + row_offset..)?;

            loop {
                let (pixel_count, end) = {
                    let byte = data.read_u8().ok()?;
                    (byte & 0b0111_1111, (byte & 0b1000_0000) != 0)
                };
                let x = usize::from(data.read_u8().ok()?);

                let pixel_buffer = {
                    let index = (y * width) + x;
                    pixels.get_mut(index..(index + usize::from(pixel_count)))?
                };
                data.read_exact(pixel_buffer).ok()?;

                if end {
                    break;
                }
            }
        }

        Some(pixels)
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn get_pixels(&'_ self, entry: &Entry) -> Option<Pixels<'_>> {
        if entry.flags & ENTRY_FLAG_RLE != 0 {
            Some(Pixels::Compressed(self.decode_sprite(entry)?))
        } else if entry.flags & ENTRY_FLAG_TRANSPARENT != 0 {
            let index = usize::try_from(entry.data_offset).ok()?;
            let data_size = usize::from(entry.width * entry.height);
            Some(Pixels::Uncompressed(self.data.get(index..(index + data_size))?))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn load(path: &std::path::Path) -> binrw::BinResult<Self> {
        use binrw::BinReaderExt as _;

        let file = std::fs::File::open(path)?;
        let mut reader = binrw::io::BufReader::new(&file);
        reader.read_le()
    }

    pub fn save(&self, path: &std::path::Path) -> binrw::BinResult<()> {
        use binrw::BinWriterExt as _;

        let mut file = std::fs::File::create(path)?;
        file.write_le(self)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_archive() {
        let test_files_directory = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
        let test_files_directory = test_files_directory.join("tests").join("files").join("csg");

        let expected_file_path = test_files_directory.join("images").with_extension("dat");

        let test_image_path = test_files_directory.join("rle_test").with_extension("png");
        let test_image =
            renderer::image::IndexedImage::load(&test_image_path, &renderer::palette::PALETTE_FLAT).unwrap();
        let mut archive = crate::csg::Archive::with_capacity(2);
        archive.add_sprite(
            test_image.as_raw(),
            test_image.width(),
            test_image.height(),
            test_image.offset.x,
            test_image.offset.y,
        );
        let encoded_sprite =
            crate::csg::EncodedSprite::new(test_image.as_raw(), test_image.width(), test_image.height());
        archive.add_encoded_sprite(&encoded_sprite, test_image.offset.x, test_image.offset.y);

        let temp_dir = tempfile::tempdir().unwrap();
        let temp_file_path = temp_dir.path().join("images").with_extension("dat");
        archive.save(&temp_file_path).unwrap();

        let output_file_bytes = std::fs::read(&temp_file_path).unwrap();
        let expected_file_bytes = std::fs::read(&expected_file_path).unwrap();

        assert_eq!(output_file_bytes, expected_file_bytes);

        let output_archive = crate::csg::Archive::load(&temp_file_path).unwrap();
        assert_eq!(output_archive.entries.len(), 2);

        let entries = output_archive.entries();
        if let crate::csg::Pixels::Uncompressed(pixels) = output_archive.get_pixels(&entries[0]).unwrap() {
            assert_eq!(pixels, test_image.as_raw());
        } else {
            panic!();
        }
        if let crate::csg::Pixels::Compressed(pixels) = output_archive.get_pixels(&entries[1]).unwrap() {
            assert_eq!(pixels, test_image.as_raw());
        } else {
            panic!();
        }
    }
}
