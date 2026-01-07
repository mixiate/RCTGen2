#[derive(Debug, serde::Deserialize)]
#[expect(unused)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    Split(Vec<bool>),
    SplitEnds(bool),
    Transfer(Vec<bool>),
}

#[derive(Debug, serde::Deserialize)]
#[expect(unused)]
#[serde(deny_unknown_fields)]
pub struct View {
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
    operation: Option<Operation>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
pub struct Masks {
    pub track_sections: std::collections::HashMap<String, Vec<View>>,
}

impl Masks {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Masks> {
        use anyhow::Context as _;
        let json = std::fs::read_to_string(path).with_context(|| format!("Could not read file {}", path.display()))?;
        serde_json::from_str::<Masks>(&json).with_context(|| format!("Could not parse json in file {}", path.display()))
    }
}
