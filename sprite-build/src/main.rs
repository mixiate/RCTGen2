#[derive(clap::Parser)]
struct Cli {
    sprites_json_path: std::path::PathBuf,
    output_file_path: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    use anyhow::Context as _;
    use clap::Parser as _;

    let start_time = std::time::Instant::now();

    let cli = Cli::parse();

    let sprites_json_path = cli
        .sprites_json_path
        .canonicalize()
        .with_context(|| format!("Invalid file path {}", cli.sprites_json_path.display()))?;

    let output_file_path = std::path::absolute(&cli.output_file_path)
        .with_context(|| format!("Invalid file path {}", cli.output_file_path.display()))?;

    sprite_build::build(&sprites_json_path, &output_file_path)?;

    println!("Time taken: {} seconds", start_time.elapsed().as_secs_f32());

    Ok(())
}
