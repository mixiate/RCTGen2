#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaletteType {
    Keep,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Sprite {
    path: String,
    x: Option<i32>,
    y: Option<i32>,
    palette: Option<PaletteType>,
}

impl Sprite {
    pub fn new(path: &str, offset: glam::IVec2) -> Self {
        Self {
            path: path.to_owned(),
            x: Some(offset.x),
            y: Some(offset.y),
            palette: Some(PaletteType::Keep),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Sprites {
    pub sprites: Vec<Sprite>,
}

impl Sprites {
    pub fn save(&self, path: &std::path::Path) -> anyhow::Result<()> {
        use anyhow::Context as _;
        use serde::Serialize as _;

        let mut buffer = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut serializer = serde_json::Serializer::with_formatter(&mut buffer, formatter);
        self.sprites.serialize(&mut serializer).unwrap();
        buffer.push(b'\n');

        std::fs::write(path, buffer).with_context(|| format!("Could not save file {}", path.display()))
    }
}
