const MAX_SECTION_COUNT: usize = 7;

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
enum OperationDesc {
    Split(heapless::Vec<bool, MAX_SECTION_COUNT>),
    SplitEnds(bool),
    Transfer(heapless::Vec<bool, MAX_SECTION_COUNT>),
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ViewDesc {
    mask: std::path::PathBuf,
    #[serde(default)]
    mirror: bool,
    #[serde(default)]
    offset: heapless::Vec<[i32; 2], MAX_SECTION_COUNT>,
    #[serde(default)]
    extrude_behind: bool,
    #[serde(default)]
    extrude_in_front: bool,
    #[serde(default)]
    mask_end: bool,
    #[serde(flatten)]
    operation: Option<OperationDesc>,
}

#[expect(clippy::large_enum_variant)]
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
enum ViewsDescType {
    Two([ViewDesc; 2]),
    Four([ViewDesc; 4]),
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
struct MasksDesc {
    track_sections: std::collections::HashMap<String, ViewsDescType>,
}

impl MasksDesc {
    fn load(path: &std::path::Path) -> anyhow::Result<MasksDesc> {
        use anyhow::Context as _;
        let json = std::fs::read_to_string(path).with_context(|| format!("Could not read {}", path.display()))?;
        serde_json::from_str::<MasksDesc>(&json).with_context(|| format!("Could not parse json in {}", path.display()))
    }
}

const PRIMARY_INDEX_MASK: u8 = 0b00_000_111;
const SECONDARY_INDEX_MASK: u8 = 0b00_111_000;
const SECONDARY_INDEX_SHIFT: u8 = 3;
const ORIGIN_MASK: u8 = 0b01_000_000;

pub struct MaskImage {
    image: renderer::image::IndexedImage,
    section_count: usize,
}

impl MaskImage {
    fn new(path: &std::path::Path) -> anyhow::Result<MaskImage> {
        use anyhow::Context as _;

        let image = renderer::image::IndexedImage::load(path, &PALETTE_FLAT)
            .with_context(|| format!("Could not load {}", path.display()))?;

        let mut section_count = 0;
        let mut origin = None;
        for y in 0..image.height() {
            for x in 0..image.width() {
                let pixel = image.get_pixel(x.into(), y.into());
                section_count = std::cmp::max(section_count, pixel & PRIMARY_INDEX_MASK);
                section_count = std::cmp::max(section_count, (pixel & SECONDARY_INDEX_MASK) >> SECONDARY_INDEX_SHIFT);

                if pixel & ORIGIN_MASK != 0 {
                    if origin.is_none() {
                        origin = Some(glam::IVec2::new(x.into(), y.into()));
                    } else {
                        anyhow::bail!("More than one origin in {}", path.display());
                    }
                }
            }
        }

        let origin = origin.with_context(|| format!("No origin found in {}", path.display()))?;
        let mut image = image;
        image.offset = origin;

        Ok(MaskImage {
            image,
            section_count: section_count.into(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Difference,
    Intersect,
    TransferNext,
}

pub struct Sprite {
    pub index: u8,
    pub offset: glam::IVec2,
    pub operation: Option<Operation>,
}

impl Sprite {
    fn new(index: usize, offset: Option<&[i32; 2]>, operation: Option<Operation>) -> Self {
        Sprite {
            index: index.try_into().unwrap(),
            offset: (*offset.unwrap_or(&[0, 0])).into(),
            operation,
        }
    }
}

pub struct View {
    image: MaskImage,
    mirror: bool,
    pub sprites: Vec<Sprite>,
    pub requires_track_mask: bool,
    pub extrude_behind_type: Option<renderer::MeshType>,
    pub extrude_ahead_type: Option<renderer::MeshType>,
    pub optional: bool,
}

impl View {
    fn new(view_desc: &ViewDesc, directory: &std::path::Path, optional: bool) -> anyhow::Result<View> {
        let image = MaskImage::new(&directory.join(&view_desc.mask))?;

        let sprites = match &view_desc.operation {
            None | Some(OperationDesc::SplitEnds(false)) => {
                let section_count = std::cmp::max(image.section_count, view_desc.offset.len());

                let mut sprites = Vec::with_capacity(section_count * 2);
                for i in 0..section_count {
                    sprites.push(Sprite::new(i + 1, view_desc.offset.get(i), None));
                }
                sprites
            }
            Some(OperationDesc::Split(splits)) => {
                let section_count = std::cmp::max(image.section_count, view_desc.offset.len());
                let section_count = std::cmp::max(section_count, splits.len());

                let mut sprites = Vec::with_capacity(section_count * 2);
                for i in 0..section_count {
                    if *splits.get(i).unwrap_or(&false) {
                        sprites.push(Sprite::new(i + 1, view_desc.offset.get(i), Some(Operation::Intersect)));
                        sprites.push(Sprite::new(i + 1, view_desc.offset.get(i), Some(Operation::Difference)));
                    } else {
                        sprites.push(Sprite::new(i + 1, view_desc.offset.get(i), None));
                    }
                }
                sprites
            }
            Some(OperationDesc::SplitEnds(true)) => {
                let section_count = std::cmp::max(image.section_count, view_desc.offset.len());

                let mut sprites = Vec::with_capacity(section_count);
                sprites.push(Sprite::new(1, view_desc.offset.first(), Some(Operation::Intersect)));
                for i in 1..(section_count - 1) {
                    sprites.push(Sprite::new(i + 1, view_desc.offset.get(i), None));
                }
                sprites.push(Sprite::new(1, view_desc.offset.last(), Some(Operation::Difference)));
                sprites
            }
            Some(OperationDesc::Transfer(transfers)) => {
                let section_count = std::cmp::max(image.section_count, view_desc.offset.len());
                let section_count = std::cmp::max(section_count, transfers.len());

                if transfers.len() == section_count && transfers.last() == Some(&true) {
                    anyhow::bail!("Cannot use transfer on the last sprite");
                }

                let mut previous_transfer = false;
                let mut sprites = Vec::with_capacity(section_count * 2);
                for i in 0..section_count {
                    let transfer = *transfers.get(i).unwrap_or(&false);
                    anyhow::ensure!(
                        !(transfer && previous_transfer),
                        "Cannot use transfer on consecutive sprites"
                    );

                    let operation = if previous_transfer {
                        Some(Operation::Difference)
                    } else if transfer {
                        Some(Operation::TransferNext)
                    } else {
                        None
                    };
                    previous_transfer = transfer;

                    sprites.push(Sprite::new(i + 1, view_desc.offset.get(i), operation));
                }
                sprites
            }
        };

        let extrude_behind_type = if view_desc.extrude_behind {
            Some(renderer::MeshType::Normal)
        } else {
            None
        };
        let extrude_ahead_type = if view_desc.extrude_in_front {
            Some(renderer::MeshType::Normal)
        } else if view_desc.mask_end {
            Some(renderer::MeshType::Mask)
        } else {
            None
        };

        Ok(View {
            image,
            mirror: view_desc.mirror,
            sprites,
            requires_track_mask: view_desc.operation.is_some(),
            extrude_behind_type,
            extrude_ahead_type,
            optional,
        })
    }

    fn translate_coords(&self, x: i32, y: i32) -> (usize, usize) {
        let x = if self.mirror { -x - 1 } else { x };

        let x = x + self.image.image.offset.x;
        let y = y + self.image.image.offset.y;

        (
            x.clamp(0, i32::from(self.image.image.width()) - 1) as usize,
            y.clamp(0, i32::from(self.image.image.height()) - 1) as usize,
        )
    }

    pub fn sample_primary(&self, x: i32, y: i32, index: u8) -> bool {
        let (x, y) = self.translate_coords(x, y);
        self.image.image.get_pixel(x, y) & PRIMARY_INDEX_MASK == index
    }

    pub fn sample_secondary(&self, x: i32, y: i32, index: u8) -> bool {
        let (x, y) = self.translate_coords(x, y);
        (self.image.image.get_pixel(x, y) & SECONDARY_INDEX_MASK) >> SECONDARY_INDEX_SHIFT == index
    }
}

pub struct Masks {
    track_sections: std::collections::HashMap<String, [View; 4]>,
}

impl Masks {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Masks> {
        use anyhow::Context as _;

        let directory =
            path.parent().with_context(|| format!("Could not get parent directory of {}", path.display()))?;

        let desc = MasksDesc::load(path)?;

        let mut track_sections = std::collections::HashMap::new();

        for (name, views) in desc.track_sections {
            let views = match &views {
                ViewsDescType::Two(views) => [
                    View::new(&views[0], directory, false)?,
                    View::new(&views[1], directory, false)?,
                    View::new(&views[0], directory, true)?,
                    View::new(&views[1], directory, true)?,
                ],
                ViewsDescType::Four(views) => [
                    View::new(&views[0], directory, false)?,
                    View::new(&views[1], directory, false)?,
                    View::new(&views[2], directory, false)?,
                    View::new(&views[3], directory, false)?,
                ],
            };
            track_sections.insert(name, views);
        }

        Ok(Masks { track_sections })
    }

    pub fn get_views(&self, track_section_name: &str) -> Option<&[View]> {
        self.track_sections.get(track_section_name).map(|x| x.as_slice())
    }
}

pub const PALETTE: [[u8; 3]; 128] = [
    [0, 0, 0],
    [192, 0, 0],
    [0, 192, 0],
    [0, 0, 192],
    [192, 192, 0],
    [0, 192, 192],
    [192, 0, 192],
    [192, 192, 192],
    [64, 0, 0],
    [255, 0, 0],
    [64, 192, 0],
    [64, 0, 192],
    [255, 192, 0],
    [64, 192, 192],
    [255, 0, 192],
    [255, 192, 192],
    [0, 64, 0],
    [192, 64, 0],
    [0, 255, 0],
    [0, 64, 192],
    [192, 255, 0],
    [0, 255, 192],
    [192, 64, 192],
    [192, 255, 192],
    [0, 0, 64],
    [192, 0, 64],
    [0, 192, 64],
    [0, 0, 255],
    [192, 192, 64],
    [0, 192, 255],
    [192, 0, 255],
    [192, 192, 255],
    [64, 64, 0],
    [255, 64, 0],
    [64, 255, 0],
    [64, 64, 192],
    [255, 255, 0],
    [64, 255, 192],
    [255, 64, 192],
    [255, 255, 192],
    [0, 64, 64],
    [192, 64, 64],
    [0, 255, 64],
    [0, 64, 255],
    [192, 255, 64],
    [0, 255, 255],
    [192, 64, 255],
    [192, 255, 255],
    [64, 0, 64],
    [255, 0, 64],
    [64, 192, 64],
    [64, 0, 255],
    [255, 192, 64],
    [64, 192, 255],
    [255, 0, 255],
    [255, 192, 255],
    [64, 64, 64],
    [255, 64, 64],
    [64, 255, 64],
    [64, 64, 255],
    [255, 255, 64],
    [64, 255, 255],
    [255, 64, 255],
    [255, 255, 255],
    [0, 0, 0],
    [96, 0, 0],
    [0, 96, 0],
    [0, 0, 96],
    [96, 96, 0],
    [0, 96, 96],
    [96, 0, 96],
    [96, 96, 96],
    [32, 0, 0],
    [128, 0, 0],
    [32, 96, 0],
    [32, 0, 96],
    [128, 96, 0],
    [32, 96, 96],
    [128, 0, 96],
    [128, 96, 96],
    [0, 32, 0],
    [96, 32, 0],
    [0, 128, 0],
    [0, 32, 96],
    [96, 128, 0],
    [0, 128, 96],
    [96, 32, 96],
    [96, 128, 96],
    [0, 0, 32],
    [96, 0, 32],
    [0, 96, 32],
    [0, 0, 128],
    [96, 96, 32],
    [0, 96, 128],
    [96, 0, 128],
    [96, 96, 128],
    [32, 32, 0],
    [128, 32, 0],
    [32, 128, 0],
    [32, 32, 96],
    [128, 128, 0],
    [32, 128, 96],
    [128, 32, 96],
    [128, 128, 96],
    [0, 32, 32],
    [96, 32, 32],
    [0, 128, 32],
    [0, 32, 128],
    [96, 128, 32],
    [0, 128, 128],
    [96, 32, 128],
    [96, 128, 128],
    [32, 0, 32],
    [128, 0, 32],
    [32, 96, 32],
    [32, 0, 128],
    [128, 96, 32],
    [32, 96, 128],
    [128, 0, 128],
    [128, 96, 128],
    [32, 32, 32],
    [128, 32, 32],
    [32, 128, 32],
    [32, 32, 128],
    [128, 128, 32],
    [32, 128, 128],
    [128, 32, 128],
    [128, 128, 128],
];

pub const PALETTE_FLAT: [u8; 128 * 3] = const {
    let mut palette_flat = [0; 128 * 3];
    let mut i = 0;
    while i < 128 {
        palette_flat[i * 3] = PALETTE[i][0];
        palette_flat[i * 3 + 1] = PALETTE[i][1];
        palette_flat[i * 3 + 2] = PALETTE[i][2];
        i += 1;
    }
    palette_flat
};
