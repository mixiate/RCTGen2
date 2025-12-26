#[derive(clap::Parser)]
struct Cli {
    track_description_file_path: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    use anyhow::Context as _;
    use clap::Parser as _;

    let start_time = std::time::Instant::now();

    let cli = Cli::parse();

    let track_description_file_path = cli
        .track_description_file_path
        .canonicalize()
        .with_context(|| format!("Invalid file path {}", cli.track_description_file_path.display()))?;

    make_track::make_track(&track_description_file_path)?;

    println!("Time taken: {} seconds", start_time.elapsed().as_secs_f32());

    Ok(())
}
