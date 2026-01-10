#[derive(Debug, serde::Deserialize)]
#[expect(unused)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
enum OperationDesc {
    Split(Vec<bool>),
    SplitEnds(bool),
    Transfer(Vec<bool>),
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ViewDesc {
    mask: std::path::PathBuf,
    #[serde(default)]
    mirror: bool,
    #[serde(default)]
    offset: Vec<[i32; 2]>,
    #[serde(default)]
    extrude_behind: bool,
    #[serde(default)]
    extrude_in_front: bool,
    #[serde(flatten)]
    operation: Option<OperationDesc>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
struct MasksDesc {
    track_sections: std::collections::HashMap<String, Vec<ViewDesc>>,
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

        let mut image = renderer::image::IndexedImage::load(path, &PALETTE_FLAT)
            .with_context(|| format!("Could not load {}", path.display()))?;

        let mut section_count = 0;
        let mut origin = None;
        for y in 0..image.height() {
            for x in 0..image.width() {
                let pixel = image.get_pixel(x, y);
                section_count = std::cmp::max(section_count, pixel & PRIMARY_INDEX_MASK);
                section_count = std::cmp::max(section_count, (pixel & SECONDARY_INDEX_MASK) >> SECONDARY_INDEX_SHIFT);

                if pixel & ORIGIN_MASK != 0 {
                    if origin.is_none() {
                        origin = Some(glam::IVec2::new(x.try_into().unwrap(), y.try_into().unwrap()));
                    } else {
                        anyhow::bail!("More than one origin in {}", path.display());
                    }
                }
            }
        }

        let origin = origin.with_context(|| format!("No origin found in {}", path.display()))?;
        image.set_offset(origin);

        Ok(MaskImage {
            image,
            section_count: section_count.into(),
        })
    }
}

#[expect(unused)]
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

pub struct View {
    image: MaskImage,
    mirror: bool,
    pub sprites: Vec<Sprite>,
    pub extrude_behind: bool,
    pub extrude_ahead: bool,
}

impl View {
    fn new(view_desc: &ViewDesc, directory: &std::path::Path) -> anyhow::Result<View> {
        let image = MaskImage::new(&directory.join(&view_desc.mask))?;

        let sprites = match &view_desc.operation {
            None => {
                let section_count = std::cmp::max(image.section_count, view_desc.offset.len());

                let mut sprites = Vec::with_capacity(section_count * 2);
                for i in 0..section_count {
                    sprites.push(Sprite {
                        index: (i + 1).try_into().unwrap(),
                        offset: (*view_desc.offset.get(i).unwrap_or(&[0, 0])).into(),
                        operation: None,
                    });
                }
                sprites
            }
            Some(OperationDesc::Split(splits)) => {
                let section_count = std::cmp::max(image.section_count, view_desc.offset.len());
                let section_count = std::cmp::max(section_count, splits.len());

                let mut sprites = Vec::with_capacity(section_count * 2);
                for i in 0..section_count {
                    let index = (i + 1).try_into().unwrap();
                    let offset = (*view_desc.offset.get(i).unwrap_or(&[0, 0])).into();

                    if *splits.get(i).unwrap_or(&false) {
                        sprites.push(Sprite {
                            index,
                            offset,
                            operation: Some(Operation::Intersect),
                        });
                        sprites.push(Sprite {
                            index,
                            offset,
                            operation: Some(Operation::Difference),
                        });
                    } else {
                        sprites.push(Sprite {
                            index,
                            offset,
                            operation: None,
                        });
                    }
                }
                sprites
            }
            Some(OperationDesc::SplitEnds(true)) => {
                let section_count = std::cmp::max(image.section_count, view_desc.offset.len());

                let mut sprites = Vec::with_capacity(section_count);
                sprites.push(Sprite {
                    index: 1,
                    offset: (*view_desc.offset.first().unwrap_or(&[0, 0])).into(),
                    operation: Some(Operation::Intersect),
                });
                for i in 1..(section_count - 1) {
                    sprites.push(Sprite {
                        index: (i + 1).try_into().unwrap(),
                        offset: (*view_desc.offset.get(i).unwrap_or(&[0, 0])).into(),
                        operation: None,
                    });
                }
                sprites.push(Sprite {
                    index: 1,
                    offset: (*view_desc.offset.last().unwrap_or(&[0, 0])).into(),
                    operation: Some(Operation::Difference),
                });
                sprites
            }
            _ => Vec::new(),
        };

        Ok(View {
            image,
            mirror: view_desc.mirror,
            sprites,
            extrude_behind: view_desc.extrude_behind,
            extrude_ahead: view_desc.extrude_in_front,
        })
    }

    pub fn sample_primary(&self, x: i32, y: i32, index: u8) -> bool {
        let x = if self.mirror { -x - 1 } else { x };

        let x = x + self.image.image.offset().x;
        let y = y + self.image.image.offset().y;

        let x = usize::try_from(x.clamp(0, (self.image.image.width() - 1).try_into().unwrap())).unwrap();
        let y = usize::try_from(y.clamp(0, (self.image.image.height() - 1).try_into().unwrap())).unwrap();

        self.image.image.get_pixel(x, y) & PRIMARY_INDEX_MASK == index
    }

    pub fn sample_secondary(&self, x: i32, y: i32, index: u8) -> bool {
        let x = if self.mirror { -x - 1 } else { x };

        let x = x + self.image.image.offset().x;
        let y = y + self.image.image.offset().y;

        let x = usize::try_from(x.clamp(0, (self.image.image.width() - 1).try_into().unwrap())).unwrap();
        let y = usize::try_from(y.clamp(0, (self.image.image.height() - 1).try_into().unwrap())).unwrap();

        (self.image.image.get_pixel(x, y) & SECONDARY_INDEX_MASK) >> SECONDARY_INDEX_SHIFT == index
    }
}

pub struct Masks {
    track_sections: std::collections::HashMap<String, Vec<View>>,
}

impl Masks {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Masks> {
        use anyhow::Context as _;

        let directory =
            path.parent().with_context(|| format!("Could not get parent directory of {}", path.display()))?;

        let desc = MasksDesc::load(path)?;

        let mut track_sections = std::collections::HashMap::new();

        for (name, views) in desc.track_sections {
            let views = views.iter().map(|x| View::new(x, directory)).collect::<anyhow::Result<Vec<View>>>()?;
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
