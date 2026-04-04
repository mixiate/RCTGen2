fn test_make_track(track_name: &str) {
    let make_track_directory = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
    let data_directory = make_track_directory.parent().unwrap().join("data");
    let test_files_directory = make_track_directory.join("tests").join("files");

    let track_description_directory = test_files_directory.join("src").join(track_name);
    let track_description_file_path = track_description_directory.join("track").with_extension("json");

    let output_directory = tempfile::tempdir().unwrap();
    let expected_directory = test_files_directory.join("output").join(track_name);

    make_track::make_track(
        &data_directory,
        &track_description_file_path,
        output_directory.path(),
        false,
    )
    .unwrap();

    let output_sprites_directory = output_directory.path().join("track").join(track_name);
    let expected_sprites_directory = expected_directory.join("track").join(track_name);

    {
        let output_file_count = std::fs::read_dir(&output_sprites_directory).unwrap().count();
        let expected_file_count = std::fs::read_dir(&output_sprites_directory).unwrap().count();
        assert!(output_file_count == expected_file_count);
    }

    let mut failed = false;
    for entry in std::fs::read_dir(&output_sprites_directory).unwrap() {
        let entry = entry.unwrap();

        let output_file_path = entry.path();
        let output_file = renderer::image::IndexedImage::load(&output_file_path, &renderer::palette::PALETTE_FLAT)
            .unwrap_or_else(|_| panic!("Could not open {output_file_path:?}"));

        let expected_file_path = expected_sprites_directory.join(entry.file_name());
        let expected_file = renderer::image::IndexedImage::load(&expected_file_path, &renderer::palette::PALETTE_FLAT)
            .unwrap_or_else(|_| panic!("Could not open {expected_file_path:?}"));

        let file_name = output_file_path.file_name().unwrap();

        if output_file.width() != expected_file.width() {
            println!(
                "{file_name:?} width {} != {}",
                output_file.width(),
                expected_file.width()
            );
            failed = true;
        }
        if output_file.height() != expected_file.height() {
            println!(
                "{file_name:?} height {} != {}",
                output_file.height(),
                expected_file.height()
            );
            failed = true;
        }
        if output_file.as_raw() != expected_file.as_raw() {
            println!("{file_name:?} pixels mismatch");
            failed = true;
        }
    }

    assert!(!failed, "Images did not match");

    let output_sprites_json = std::fs::read(output_directory.path().join("sprites").with_extension("json")).unwrap();
    let output_sprites_json: serde_json::Value = serde_json::from_slice(&output_sprites_json).unwrap();
    let expected_sprites_json = std::fs::read(expected_directory.join("sprites").with_extension("json")).unwrap();
    let expected_sprites_json: serde_json::Value = serde_json::from_slice(&expected_sprites_json).unwrap();

    assert!(output_sprites_json == expected_sprites_json);
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
