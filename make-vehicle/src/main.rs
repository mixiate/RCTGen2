#[derive(clap::Parser)]
struct Cli {
    ride_description_file_path: std::path::PathBuf,
    #[arg(long, value_enum, default_value_t = make_vehicle::ImageOutputType::Packed)]
    image_output_type: make_vehicle::ImageOutputType,
}

fn main() -> anyhow::Result<()> {
    use clap::Parser as _;

    let start_time = std::time::Instant::now();

    let cli = Cli::parse();

    make_vehicle::make_vehicle(&cli.ride_description_file_path, cli.image_output_type)?;

    println!("Time taken: {} seconds", start_time.elapsed().as_secs_f32());

    Ok(())
}
