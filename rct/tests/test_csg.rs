#[test]
fn test_archive() {
    let test_files_directory = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
    let test_files_directory = test_files_directory.join("tests").join("files").join("csg");

    let expected_file_path = test_files_directory.join("archive").with_extension("dat");

    let test_image_path = test_files_directory.join("sprite").with_extension("png");
    let test_image = renderer::image::IndexedImage::load(&test_image_path, &renderer::palette::PALETTE_FLAT).unwrap();
    let mut archive = rct::csg::Archive::with_capacity(2);
    archive.add_sprite(
        test_image.as_raw(),
        test_image.width(),
        test_image.height(),
        test_image.offset.x,
        test_image.offset.y,
    );
    let encoded_sprite = rct::csg::EncodedSprite::new(test_image.as_raw(), test_image.width(), test_image.height());
    archive.add_encoded_sprite(&encoded_sprite, test_image.offset.x, test_image.offset.y);

    let temp_dir = tempfile::tempdir().unwrap();
    let temp_file_path = temp_dir.path().join("archive").with_extension("dat");
    archive.save(&temp_file_path).unwrap();

    let output_file_bytes = std::fs::read(&temp_file_path).unwrap();
    let expected_file_bytes = std::fs::read(&expected_file_path).unwrap();

    assert_eq!(output_file_bytes, expected_file_bytes);

    let output_archive = rct::csg::Archive::load(&temp_file_path).unwrap();
    assert_eq!(output_archive.entries().len(), 2);

    let entries = output_archive.entries();
    if let rct::csg::Pixels::Uncompressed(pixels) = output_archive.get_pixels(&entries[0]).unwrap() {
        assert_eq!(pixels, test_image.as_raw());
    } else {
        panic!();
    }
    if let rct::csg::Pixels::Compressed(pixels) = output_archive.get_pixels(&entries[1]).unwrap() {
        assert_eq!(pixels, test_image.as_raw());
    } else {
        panic!();
    }
}
