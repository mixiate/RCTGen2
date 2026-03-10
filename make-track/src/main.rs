#[derive(clap::Parser)]
struct Cli {
    track_description_file_path: std::path::PathBuf,
    output_directory: std::path::PathBuf,
    #[arg(long)]
    skip_empty_sprites: bool,
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

    let output_directory = std::path::absolute(&cli.output_directory)
        .with_context(|| format!("Invalid file path {}", cli.output_directory.display()))?;

    make_track::make_track(
        &data_directory,
        &track_description_file_path,
        &output_directory,
        cli.skip_empty_sprites,
    )?;

    println!("Time taken: {} seconds", start_time.elapsed().as_secs_f32());

    Ok(())
}
