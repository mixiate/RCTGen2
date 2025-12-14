fn main() -> anyhow::Result<()> {
    use anyhow::Context as _;

    let start_time = std::time::Instant::now();

    let args: Vec<String> = std::env::args().collect();

    let ride_description_path = std::path::PathBuf::from(args.get(1).context("No description file path argument")?);

    make_vehicle::make_vehicle(&ride_description_path)?;

    println!("Time taken: {} seconds", start_time.elapsed().as_secs_f32());

    Ok(())
}
