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
#[expect(unused)]
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

pub struct View {
    #[expect(unused)]
    pub image: renderer::image::IndexedImage,
    pub extrude_behind: bool,
    pub extrude_ahead: bool,
}

impl View {
    fn new(view_desc: &ViewDesc, directory: &std::path::Path) -> anyhow::Result<View> {
        use anyhow::Context as _;

        let image_path = directory.join(&view_desc.mask);
        let image = renderer::image::IndexedImage::load(&image_path, &PALETTE_FLAT)
            .with_context(|| format!("Could not load {}", image_path.display()))?;

        Ok(View {
            image,
            extrude_behind: view_desc.extrude_behind,
            extrude_ahead: view_desc.extrude_in_front,
        })
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
