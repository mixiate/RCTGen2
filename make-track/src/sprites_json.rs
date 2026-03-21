#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct Sprites {
    pub sprites: Vec<openrct2::objects::image::ImageFile>,
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
