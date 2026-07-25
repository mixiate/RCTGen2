use make_track::track_desc::TrackSectionSprites;
use make_track::track_sections::TRACK_SECTIONS;

#[derive(serde::Deserialize)]
struct AdjacentTrackSectionDesc<'a> {
    name: &'a str,
    coords: [i16; 3],
    #[serde(default)]
    rotation: u8,
}

type AdjacentTrackSectionsDesc<'a> = indexmap::IndexMap<String, heapless::Vec<AdjacentTrackSectionDesc<'a>, 2>>;

pub struct AdjacentTrackSection {
    pub track_section: &'static make_track::track_sections::TrackSection,
    pub coords: [i16; 3],
    pub rotation: u8,
}

pub type AdjacentTrackSections = indexmap::IndexMap<String, heapless::Vec<AdjacentTrackSection, 2>>;

pub fn load_adjacent_track_sections(path: &std::path::Path) -> anyhow::Result<AdjacentTrackSections> {
    use anyhow::Context as _;
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

        let mut adjacent_sections = heapless::Vec::new();
        for section_desc in section_descs {
            let track_section = TRACK_SECTIONS.iter().find(|x| x.name == section_desc.name).with_context(|| {
                format!(
                    "Unknown track section {} referenced by {name} in {}",
                    section_desc.name,
                    path.display()
                )
            })?;
            let _ignore_full = adjacent_sections.push(AdjacentTrackSection {
                track_section,
                coords: section_desc.coords,
                rotation: section_desc.rotation,
            });
        }
        sections.insert(name, adjacent_sections);
    }
    Ok(sections)
}

pub struct TrackSectionWithSprites<'a> {
    pub track_section: &'static make_track::track_sections::TrackSection,
    pub coords: [i16; 3],
    pub rotation: u8,
    pub sprites: &'a TrackSectionSprites,
}

pub fn list_track_sections<'a>(
    track_section_name: &str,
    adjacent_sections: &AdjacentTrackSections,
    sprites: &'a indexmap::IndexMap<String, TrackSectionSprites>,
) -> heapless::Vec<TrackSectionWithSprites<'a>, 2> {
    if let Some(adjacent_sections) = adjacent_sections.get(track_section_name) {
        let mut track_sections = heapless::Vec::new();
        for adjacent_section in adjacent_sections {
            if let Some(sprites) = sprites.get(adjacent_section.track_section.name) {
                let _ignore_full = track_sections.push(TrackSectionWithSprites {
                    track_section: adjacent_section.track_section,
                    coords: adjacent_section.coords,
                    rotation: adjacent_section.rotation,
                    sprites,
                });
            }
        }
        track_sections
    } else {
        heapless::Vec::new()
    }
}
