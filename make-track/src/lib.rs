mod track_desc;

pub fn make_track(track_description_file_path: &std::path::Path) -> anyhow::Result<()> {
    use anyhow::Context as _;

    let desc = track_desc::Desc::load(track_description_file_path)?;
    println!("{desc:?}");

    let base_directory = track_description_file_path.parent().with_context(|| {
        format!(
            "Could not get parent directory of {}",
            track_description_file_path.display()
        )
    })?;

    for track in desc.tracks {
        let models = track.models.load(base_directory)?;
        println!("{} {}", models.track.meshes.len(), models.mask.meshes.len());
    }

    Ok(())
}
