#[derive(clap::Parser)]
struct Cli {
    track_description_file_path: std::path::PathBuf,
    output_directory: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    use anyhow::Context as _;
    use clap::Parser as _;

    let start_time = std::time::Instant::now();

    let data_directory = std::env::current_exe()?;
    let data_directory = data_directory
        .parent()
        .with_context(|| format!("Could not get parent directory of {}", data_directory.display()))?;
    let data_directory = data_directory.join("data");

    let cli = Cli::parse();

    let track_description_file_path = cli
        .track_description_file_path
        .canonicalize()
        .with_context(|| format!("Invalid file path {}", cli.track_description_file_path.display()))?;

    let output_directory = cli
        .output_directory
        .canonicalize()
        .with_context(|| format!("Invalid file path {}", cli.output_directory.display()))?;

    make_track::make_track(&data_directory, &track_description_file_path, &output_directory)?;

    println!("Time taken: {} seconds", start_time.elapsed().as_secs_f32());

    Ok(())
}
