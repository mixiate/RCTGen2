use renderer::image::IndexedImage;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn load_sprite(archive: &rct::csg::Archive, index: usize) -> anyhow::Result<IndexedImage> {
    use anyhow::Context as _;

    let entry = archive.entries().get(index).with_context(|| format!("Sprite {index} not in archive"))?;
    let entry_pixels = archive.get_pixels(entry).with_context(|| format!("Error loading sprite {index} in archive"))?;
    let mut image = match entry_pixels {
        rct::csg::Pixels::Uncompressed(pixels) => IndexedImage::with_buffer(pixels.to_vec(), entry.width, entry.height),
        rct::csg::Pixels::Compressed(pixels) => IndexedImage::with_buffer(pixels, entry.width, entry.height),
    };
    image.offset = glam::IVec2::new(entry.offset_x.into(), entry.offset_y.into());
    Ok(image)
}

pub struct SpriteCache {
    archive: rct::csg::Archive,
    sprites: HashMap<u32, IndexedImage>,
}

impl SpriteCache {
    pub fn try_new(path: &std::path::Path) -> anyhow::Result<Self> {
        use anyhow::Context as _;
        let archive = rct::csg::Archive::load(path).with_context(|| format!("Could not load {}", path.display()))?;

        Ok(Self {
            archive,
            sprites: HashMap::new(),
        })
    }

    pub fn get(&mut self, index: u32) -> Option<&IndexedImage> {
        match self.sprites.entry(index) {
            Entry::Occupied(entry) => Some(entry.into_mut()),
            Entry::Vacant(entry) => {
                if let Ok(sprite) = load_sprite(&self.archive, index as usize) {
                    Some(entry.insert_entry(sprite).into_mut())
                } else {
                    None
                }
            }
        }
    }
}
