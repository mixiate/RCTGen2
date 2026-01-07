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
    pub extrude_behind: bool,
    pub extrude_ahead: bool,
}

pub struct Masks {
    track_sections: std::collections::HashMap<String, Vec<View>>,
}

impl Masks {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Masks> {
        let desc = MasksDesc::load(path)?;

        let mut track_sections = std::collections::HashMap::new();

        for (name, views) in desc.track_sections {
            let views = views
                .iter()
                .map(|view_desc| View {
                    extrude_behind: view_desc.extrude_behind,
                    extrude_ahead: view_desc.extrude_in_front,
                })
                .collect();
            track_sections.insert(name, views);
        }

        Ok(Masks { track_sections })
    }

    pub fn get_views(&self, track_section_name: &str) -> Option<&[View]> {
        self.track_sections.get(track_section_name).map(|x| x.as_slice())
    }
}
