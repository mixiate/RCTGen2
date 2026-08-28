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
pub struct ViewDesc {
    mask: std::path::PathBuf,
    #[serde(default)]
    mirror: bool,
    #[serde(default)]
    offset: heapless::Vec<[i16; 3], MAX_SECTION_COUNT>,
    #[serde(default)]
    tiles: heapless::Vec<u8, MAX_SECTION_COUNT>,
    #[serde(default)]
    empty: heapless::Vec<bool, MAX_SECTION_COUNT>,
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
pub enum ViewsDescType {
    Two([ViewDesc; 2]),
    Four([ViewDesc; 4]),
}

impl ViewsDescType {
    pub fn load(&self, directory: &std::path::Path, tiles: &[[i16; 3]]) -> anyhow::Result<[View; 4]> {
        Ok(match self {
            ViewsDescType::Two(views) => [
                View::load(&views[0], directory, tiles, 0, false)?,
                View::load(&views[1], directory, tiles, 1, false)?,
                View::load(&views[0], directory, tiles, 2, true)?,
                View::load(&views[1], directory, tiles, 3, true)?,
            ],
            ViewsDescType::Four(views) => [
                View::load(&views[0], directory, tiles, 0, false)?,
                View::load(&views[1], directory, tiles, 1, false)?,
                View::load(&views[2], directory, tiles, 2, false)?,
                View::load(&views[3], directory, tiles, 3, false)?,
            ],
        })
    }
}

pub type Masks = std::collections::HashMap<String, ViewsDescType>;

pub fn load_masks(path: &std::path::Path) -> anyhow::Result<std::collections::HashMap<String, ViewsDescType>> {
    use anyhow::Context as _;
    let json = std::fs::read_to_string(path).with_context(|| format!("Could not read {}", path.display()))?;
    serde_json::from_str::<Masks>(&json).with_context(|| format!("Could not parse json in {}", path.display()))
}

const PRIMARY_INDEX_MASK: u8 = 0b00_000_111;
const SECONDARY_INDEX_MASK: u8 = 0b00_111_000;
const SECONDARY_INDEX_SHIFT: u8 = 3;
const ORIGIN_MASK: u8 = 0b01_000_000;

pub struct MaskImage {
    image: renderer::image::IndexedImage,
    sections: u8,
}

impl MaskImage {
    fn new(path: &std::path::Path) -> anyhow::Result<MaskImage> {
        use anyhow::Context as _;

        let image = renderer::image::IndexedImage::load(path, &PALETTE_FLAT)
            .with_context(|| format!("Could not load {}", path.display()))?;

        let mut sections = 0;
        let mut origin = None;
        for y in 0..image.height() {
            for x in 0..image.width() {
                let pixel = image.get_pixel(x.into(), y.into());
                let indices = [
                    pixel & PRIMARY_INDEX_MASK,
                    (pixel & SECONDARY_INDEX_MASK) >> SECONDARY_INDEX_SHIFT,
                ];
                for index in indices {
                    if index > 0 {
                        sections |= 0b1 << (index - 1);
                    }
                }

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

        Ok(MaskImage { image, sections })
    }

    fn has_section(&self, index: usize) -> bool {
        (self.sections & (0b1 << index)) != 0
    }
}

impl Default for MaskImage {
    fn default() -> Self {
        MaskImage {
            image: renderer::image::IndexedImage::new(0, 0),
            sections: 0,
        }
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
    fn new(index: usize, offset: glam::IVec2, operation: Option<Operation>) -> Self {
        Sprite {
            index: index.try_into().unwrap(),
            offset,
            operation,
        }
    }
}

#[derive(Default)]
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
    fn load(
        view_desc: &ViewDesc,
        directory: &std::path::Path,
        tiles: &[[i16; 3]],
        rotation: usize,
        optional: bool,
    ) -> anyhow::Result<View> {
        let image = {
            let mut image = MaskImage::new(&directory.join(&view_desc.mask))?;
            for (index, empty) in view_desc.empty.iter().enumerate() {
                if *empty {
                    image.sections |= 0b1 << index;
                }
            }
            image
        };
        let section_count = usize::try_from(image.sections.count_ones())?;

        let sprites = match &view_desc.operation {
            None | Some(OperationDesc::SplitEnds(false)) => {
                let mut sprites = Vec::with_capacity(section_count);
                for i in 0..MAX_SECTION_COUNT {
                    if image.has_section(i) {
                        let coordinates = get_coordinates(i, &view_desc.tiles, tiles)?;
                        let offset = calculate_tile_image_offset(coordinates, view_desc.offset.get(i), rotation);
                        sprites.push(Sprite::new(i + 1, offset, None));
                    }
                }
                sprites
            }
            Some(OperationDesc::Split(splits)) => {
                let mut sprites = Vec::with_capacity(section_count * 2);
                for i in 0..MAX_SECTION_COUNT {
                    if image.has_section(i) {
                        let coordinates = get_coordinates(i, &view_desc.tiles, tiles)?;
                        let offset = calculate_tile_image_offset(coordinates, view_desc.offset.get(i), rotation);
                        if *splits.get(i).unwrap_or(&false) {
                            sprites.push(Sprite::new(i + 1, offset, Some(Operation::Intersect)));
                            sprites.push(Sprite::new(i + 1, offset, Some(Operation::Difference)));
                        } else {
                            sprites.push(Sprite::new(i + 1, offset, None));
                        }
                    }
                }
                sprites
            }
            Some(OperationDesc::SplitEnds(true)) => {
                let mut sprites = Vec::with_capacity(section_count);

                let first_index = image.sections.lowest_one().map(|x| x as usize).unwrap_or_default();

                {
                    let coordinates = get_coordinates(first_index, &view_desc.tiles, tiles)?;
                    let offset = calculate_tile_image_offset(coordinates, view_desc.offset.get(first_index), rotation);
                    sprites.push(Sprite::new(first_index + 1, offset, Some(Operation::Intersect)));
                }

                for i in (first_index + 1)..MAX_SECTION_COUNT {
                    if image.has_section(i) {
                        let coordinates = get_coordinates(i, &view_desc.tiles, tiles)?;
                        let offset = calculate_tile_image_offset(coordinates, view_desc.offset.get(i), rotation);
                        sprites.push(Sprite::new(i + 1, offset, None));
                    }
                }

                {
                    let coordinates = tiles.last().expect("Track section has no tiles");
                    let offset = calculate_tile_image_offset(coordinates, view_desc.offset.last(), rotation);
                    sprites.push(Sprite::new(first_index + 1, offset, Some(Operation::Difference)));
                }

                sprites
            }
            Some(OperationDesc::Transfer(transfers)) => {
                if transfers.last() == Some(&true) {
                    anyhow::bail!("Cannot use transfer on the last sprite");
                }

                let mut previous_transfer = false;
                let mut sprites = Vec::with_capacity(section_count);
                for i in 0..MAX_SECTION_COUNT {
                    let transfer = *transfers.get(i).unwrap_or(&false);

                    if image.has_section(i) {
                        let operation = if previous_transfer {
                            Some(Operation::Difference)
                        } else if transfer {
                            Some(Operation::TransferNext)
                        } else {
                            None
                        };

                        let coordinates = get_coordinates(i, &view_desc.tiles, tiles)?;
                        let offset = calculate_tile_image_offset(coordinates, view_desc.offset.get(i), rotation);
                        sprites.push(Sprite::new(i + 1, offset, operation));
                    }

                    previous_transfer = transfer;
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

fn get_coordinates<'a>(index: usize, tile_map: &[u8], tiles: &'a [[i16; 3]]) -> anyhow::Result<&'a [i16; 3]> {
    use anyhow::Context as _;
    let index = tile_map.get(index).map(|x| usize::from(*x)).unwrap_or(index);
    tiles.get(index).with_context(|| "More sections than tiles in mask")
}

fn calculate_tile_image_offset(coordinates: &[i16; 3], offset: Option<&[i16; 3]>, rotation: usize) -> glam::IVec2 {
    let [x, y, z] = *coordinates;
    let (x, y) = match rotation {
        1 => (y, -x),
        2 => (-x, -y),
        3 => (-y, x),
        _ => (x, y),
    };
    let offset = offset.unwrap_or(&[0; 3]);
    let x = x + offset[0];
    let y = y + offset[1];
    let z = z + offset[2];

    let offset_x = x - y;
    let offset_y = (-(x + y) / 2) + z;
    glam::IVec2::new(offset_x.into(), offset_y.into())
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
    [83, 83, 83],
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
