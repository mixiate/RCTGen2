struct RideObjectFiles {
    object_json: serde_json::Value,
    car_0_image: Vec<u8>,
    preview_image: Vec<u8>,
}

fn read_ride_object_files(object_directory: &std::path::Path) -> RideObjectFiles {
    let object_json_file_path = object_directory.join("object").with_extension("json");
    let object_json = std::fs::read(&object_json_file_path).unwrap();
    let object_json = serde_json::from_slice(&object_json).unwrap();

    let car_0_image_file_path = object_directory.join("images").join("car_0").with_extension("png");
    let car_0_image = std::fs::read(&car_0_image_file_path).unwrap();

    let preview_image_file_path = object_directory.join("images").join("preview").with_extension("png");
    let preview_image = std::fs::read(&preview_image_file_path).unwrap();

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
        make_vehicle::ImageOutputType::Atlas(make_vehicle::AtlasType::Grid),
    )
    .unwrap();

    let output_directory = temp_dir.path().join("mix.ride.rctgen2_make_vehicle_test");
    let output_files = read_ride_object_files(&output_directory);

    let expected_directory = test_files_directory.join("mix.ride.rctgen2_make_vehicle_test");
    let expected_files = read_ride_object_files(&expected_directory);

    assert!(output_files.object_json == expected_files.object_json);
    assert!(output_files.car_0_image == expected_files.car_0_image);
    assert!(output_files.preview_image == expected_files.preview_image);
}
