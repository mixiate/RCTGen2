#[derive(Clone, Copy, clap::ValueEnum)]
pub enum ImageOutputType {
    Dat,
    Packed,
    Grid,
}

#[derive(clap::Parser)]
struct Cli {
    ride_description_file_path: std::path::PathBuf,
    #[arg(long, value_enum, default_value_t = ImageOutputType::Dat)]
    image_output_type: ImageOutputType,
}

fn main() -> anyhow::Result<()> {
    use clap::Parser as _;

    let start_time = std::time::Instant::now();

    let cli = Cli::parse();

    let image_output_type = match cli.image_output_type {
        ImageOutputType::Dat => make_vehicle::ImageOutputType::Dat,
        ImageOutputType::Packed => make_vehicle::ImageOutputType::Atlas(make_vehicle::AtlasType::Packed),
        ImageOutputType::Grid => make_vehicle::ImageOutputType::Atlas(make_vehicle::AtlasType::Grid),
    };
    make_vehicle::make_vehicle(&cli.ride_description_file_path, image_output_type)?;

    println!("Time taken: {} seconds", start_time.elapsed().as_secs_f32());

    Ok(())
}
