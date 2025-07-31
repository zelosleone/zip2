// \\?\[..]\[long_path..]\[long_path..] is not a valid path.
// path components must be shorter than 255 chars.

#[test]
fn long_path_creation() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path();
    let long_path = "a".repeat(260);

    let archive_path = path.join("archive.zip");
    let archive_file = std::fs::File::create(&archive_path).unwrap();

    let mut archive = zip::ZipWriter::new(archive_file);
    // This should always succeed regardless of platform
    archive.start_file::<_, ()>(&long_path, zip::write::FileOptions::default()).unwrap();
    archive.finish().unwrap();

    let archive_file = std::fs::File::open(&archive_path).unwrap();
    let mut archive = zip::ZipArchive::new(archive_file).unwrap();
    
    // Verify the archive was created successfully
    assert_eq!(archive.len(), 1);
    let file = archive.by_index(0).unwrap();
    assert_eq!(file.name(), long_path);
}