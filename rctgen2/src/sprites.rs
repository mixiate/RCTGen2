use crate::render::Texture;
use eframe::egui;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn load_texture(archive: &rct::csg::Archive, index: usize, egui_context: &egui::Context) -> anyhow::Result<Texture> {
    use anyhow::Context as _;

    let entry = archive.entries().get(index).with_context(|| format!("Sprite {index} not in archive"))?;
    let entry_pixels = archive.get_pixels(entry).with_context(|| format!("Error loading sprite {index} in archive"))?;
    let pixels = match &entry_pixels {
        rct::csg::Pixels::Uncompressed(pixels) => *pixels,
        rct::csg::Pixels::Compressed(pixels) => pixels,
    };
    let pixels: Vec<_> = pixels
        .iter()
        .flat_map(|x| {
            if *x == 0 {
                [0; 4]
            } else {
                let colour = renderer::palette::PALETTE[usize::from(*x)];
                [colour[0], colour[1], colour[2], 255]
            }
        })
        .collect();
    let egui_image = egui::ColorImage::from_rgba_unmultiplied([entry.width.into(), entry.height.into()], &pixels);
    let handle = egui_context.load_texture(index.to_string(), egui_image, egui::TextureOptions::default());
    Ok(Texture {
        handle,
        offset: glam::IVec2::new(entry.offset_x.into(), entry.offset_y.into()),
    })
}

pub struct Sprites {
    archive: rct::csg::Archive,
    sprites: HashMap<u32, Texture>,
}

impl Sprites {
    pub fn try_new(path: &std::path::Path) -> anyhow::Result<Self> {
        use anyhow::Context as _;
        let archive = rct::csg::Archive::load(path).with_context(|| format!("Could not load {}", path.display()))?;

        Ok(Self {
            archive,
            sprites: HashMap::new(),
        })
    }

    pub fn get_sprite(&mut self, index: u32, egui_context: &egui::Context) -> Option<&Texture> {
        match self.sprites.entry(index) {
            Entry::Occupied(entry) => Some(entry.into_mut()),
            Entry::Vacant(entry) => {
                if let Ok(texture) = load_texture(&self.archive, index as usize, egui_context) {
                    Some(entry.insert_entry(texture).into_mut())
                } else {
                    None
                }
            }
        }
    }
}
