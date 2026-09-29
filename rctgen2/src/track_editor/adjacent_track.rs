use make_track::track_sections::TRACK_SECTIONS;
use nonempty_collections::NEVec;

#[derive(serde::Deserialize)]
struct AdjacentTrackSectionDesc<'a> {
    name: &'a str,
    coords: [i16; 3],
    #[serde(default)]
    rotation: u8,
}

type AdjacentTrackSectionsDesc<'a> = indexmap::IndexMap<String, NEVec<heapless::Vec<AdjacentTrackSectionDesc<'a>, 2>>>;

pub struct AdjacentTrackSection {
    pub track_section: &'static make_track::track_sections::TrackSection,
    pub coords: [i16; 3],
    pub rotation: u8,
}

impl std::convert::TryFrom<AdjacentTrackSectionDesc<'_>> for AdjacentTrackSection {
    type Error = anyhow::Error;

    fn try_from(section_desc: AdjacentTrackSectionDesc<'_>) -> Result<Self, Self::Error> {
        use anyhow::Context as _;
        let track_section = TRACK_SECTIONS
            .iter()
            .find(|x| x.name == section_desc.name)
            .with_context(|| format!("Unknown adjacent track section {}", section_desc.name,))?;
        Ok(AdjacentTrackSection {
            track_section,
            coords: section_desc.coords,
            rotation: section_desc.rotation,
        })
    }
}

pub type AdjacentTrackSections = heapless::Vec<AdjacentTrackSection, 2>;
pub type AdjacentTrackSectionsByName = indexmap::IndexMap<String, NEVec<AdjacentTrackSections>>;

pub fn load_adjacent_track_sections(path: &std::path::Path) -> anyhow::Result<AdjacentTrackSectionsByName> {
    use anyhow::Context as _;
    use nonempty_collections::iter::{IntoNonEmptyIterator as _, NonEmptyIterator as _};

    let json = std::fs::read_to_string(path).with_context(|| format!("Could not read file {}", path.display()))?;
    let sections_desc = serde_json::from_str::<AdjacentTrackSectionsDesc>(&json)
        .with_context(|| format!("Could not parse json in file {}", path.display()))?;

    let mut sections = indexmap::IndexMap::with_capacity(sections_desc.len());
    for (name, section_descs) in sections_desc {
        anyhow::ensure!(
            TRACK_SECTIONS.iter().find(|x| x.name == name).is_some(),
            "Unknown track section {name} in {}",
            path.display()
        );

        let adjacent_sections = section_descs
            .into_nonempty_iter()
            .map(|x| {
                x.into_iter()
                    .map(AdjacentTrackSection::try_from)
                    .collect::<Result<heapless::Vec<AdjacentTrackSection, 2>, anyhow::Error>>()
                    .with_context(|| format!("Error in track section {} in {}", name, path.display()))
            })
            .collect::<Result<NEVec<heapless::Vec<AdjacentTrackSection, 2>>, anyhow::Error>>()?;
        sections.insert(name, adjacent_sections);
    }
    Ok(sections)
}
