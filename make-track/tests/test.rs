fn test_make_track(track_name: &str) {
    let make_track_directory = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
    let data_directory = make_track_directory.parent().unwrap().join("data");
    let test_files_directory = make_track_directory.join("tests").join("files");

    let track_description_directory = test_files_directory.join("src").join(track_name);
    let track_description_file_path = track_description_directory.join("track").with_extension("json");

    let output_directory = tempfile::tempdir().unwrap();

    make_track::make_track(&data_directory, &track_description_file_path, output_directory.path()).unwrap();

    let output_directory = output_directory.path().join("track").join(track_name);
    let expected_directory = test_files_directory.join("output").join(track_name).join("track").join(track_name);

    {
        let output_file_count = std::fs::read_dir(&output_directory).unwrap().count();
        let expected_file_count = std::fs::read_dir(&output_directory).unwrap().count();
        assert!(output_file_count == expected_file_count);
    }

    for entry in std::fs::read_dir(&output_directory).unwrap() {
        let entry = entry.unwrap();

        let output_file_path = entry.path();
        let output_file = renderer::image::IndexedImage::load(&output_file_path, &renderer::palette::PALETTE_FLAT)
            .expect(&format!("Could not open {output_file_path:?}"));

        let expected_file_path = expected_directory.join(entry.file_name());
        let expected_file = renderer::image::IndexedImage::load(&expected_file_path, &renderer::palette::PALETTE_FLAT)
            .expect(&format!("Could not open {expected_file_path:?}"));

        assert!(
            output_file.as_raw() == expected_file.as_raw(),
            "{output_file_path:?} != {expected_file_path:?}"
        );
    }
}

#[test]
fn test_track() {
    test_make_track("test-track");
}

#[test]
fn test_track_offsets() {
    test_make_track("test-track-offsets");
}

#[test]
fn test_track_tie() {
    test_make_track("test-track-tie");
}

#[test]
fn test_track_alt() {
    // this only tests the flat track because the alt track mesh system is quite flawed
    test_make_track("test-track-alt");
}

#[test]
fn test_track_boundary_tie() {
    test_make_track("test-track-boundary-tie");
}

#[test]
fn test_track_semi_flat_shaded() {
    test_make_track("test-track-semi-flat-shaded");
}

#[test]
fn test_track_supports() {
    test_make_track("test-track-supports");
}
