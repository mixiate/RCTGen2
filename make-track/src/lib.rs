mod track_desc;

pub fn make_track(track_description_file_path: &std::path::Path) -> anyhow::Result<()> {
    let desc = track_desc::Desc::load(track_description_file_path)?;
    println!("{desc:?}");
    Ok(())
}
