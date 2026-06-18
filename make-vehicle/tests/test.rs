struct RideObjectFiles {
    object_json: serde_json::Value,
    car_0_image: renderer::image::IndexedImage,
    preview_image: renderer::image::IndexedImage,
}

fn read_ride_object_files(object_directory: &std::path::Path) -> RideObjectFiles {
    let object_json_file_path = object_directory.join("object").with_extension("json");
    let object_json = std::fs::read(&object_json_file_path).unwrap();
    let object_json = serde_json::from_slice(&object_json).unwrap();

    let car_0_image_file_path = object_directory.join("images").join("car_0").with_extension("png");
    let car_0_image = renderer::image::IndexedImage::load(&car_0_image_file_path, &renderer::palette::PALETTE_FLAT)
        .unwrap_or_else(|_| panic!("Could not open {car_0_image_file_path:?}"));

    let preview_image_file_path = object_directory.join("images").join("preview").with_extension("png");
    let preview_image = renderer::image::IndexedImage::load(&preview_image_file_path, &renderer::palette::PALETTE_FLAT)
        .unwrap_or_else(|_| panic!("Could not open {preview_image_file_path:?}"));

    RideObjectFiles {
        object_json,
        car_0_image,
        preview_image,
    }
}

#[test]
fn test_make_vehicle() {
    let cargo_path = std::env!("CARGO_MANIFEST_DIR");
    let test_files_directory = std::path::PathBuf::from(cargo_path).join("tests").join("files");

    let temp_dir = tempfile::tempdir().unwrap();

    let ride_description_directory = test_files_directory.join("testvehicle");
    let ride_description_file_path = ride_description_directory.join("ride").with_extension("json");
    make_vehicle::make_vehicle(
        &ride_description_file_path,
        temp_dir.path(),
        temp_dir.path(),
        make_vehicle::ImageOutputType::Atlas,
    )
    .unwrap();

    let output_directory = temp_dir.path().join("mix.ride.rctgen2_make_vehicle_test");
    let output_files = read_ride_object_files(&output_directory);

    let expected_directory = test_files_directory.join("mix.ride.rctgen2_make_vehicle_test");
    let expected_files = read_ride_object_files(&expected_directory);

    assert!(output_files.object_json == expected_files.object_json);
    assert!(output_files.preview_image.as_raw() == expected_files.preview_image.as_raw());

    const PIXEL_DIFF_TOLERANCE: usize = 10;
    let pixel_diff_count = output_files
        .car_0_image
        .as_raw()
        .iter()
        .zip(expected_files.car_0_image.as_raw().iter())
        .filter(|(a, b)| a != b)
        .count();
    assert!(
        pixel_diff_count <= PIXEL_DIFF_TOLERANCE,
        "car_0.png: {pixel_diff_count} pixels differ (tolerance {PIXEL_DIFF_TOLERANCE})"
    );
}
