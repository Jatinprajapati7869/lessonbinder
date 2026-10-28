use anki_bridge::{
    commit_card_draft, open_profile, read_media, screenshot_filename_for_bytes,
    AuraVaultProfilePaths, ReadMediaError,
};
use vault_core::card::build_card_draft;
use vault_core::ocr::FakeOcrEngine;
use vault_core::VaultDb;

fn fixture_lesson(db: &VaultDb) -> String {
    let course_id = db.insert_course("Course", "/courses/c").unwrap();
    db.insert_local_file_lesson(&course_id, "lecture.mp4", "Lecture", 0)
        .unwrap()
}

#[test]
fn read_media_returns_png_written_by_commit() {
    let profile = tempfile::tempdir().unwrap();
    let mut col = open_profile(profile.path()).unwrap();
    let paths = AuraVaultProfilePaths::in_dir(profile.path());
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = fixture_lesson(&db);
    let png = b"\x89PNG\r\n\0read-media-test-bytes";
    let engine = FakeOcrEngine::fixed("Front", 1.0);
    let draft = build_card_draft(&db, &engine, &lesson_id, 1_000, png, None, None).unwrap();

    commit_card_draft(
        col.collection_mut(),
        &paths.media_folder,
        &db,
        &lesson_id,
        1_000,
        &draft,
        png,
        None,
    )
    .unwrap();

    let filename = screenshot_filename_for_bytes(png);
    let bytes = read_media(&paths.media_folder, &filename).unwrap();
    assert_eq!(bytes, png);
}

#[cfg(unix)]
#[test]
fn read_media_rejects_symlink_outside_media_root() {
    use std::os::unix::fs::symlink;

    let profile = tempfile::tempdir().unwrap();
    let paths = AuraVaultProfilePaths::in_dir(profile.path());
    std::fs::create_dir_all(&paths.media_folder).unwrap();
    let outside = profile.path().join("outside-secret.png");
    std::fs::write(&outside, b"secret").unwrap();
    symlink(&outside, paths.media_folder.join("escape.png")).unwrap();

    let err = read_media(&paths.media_folder, "escape.png").unwrap_err();
    assert!(matches!(err, ReadMediaError::InvalidFilename));
}

#[test]
fn read_media_rejects_path_escape() {
    let profile = tempfile::tempdir().unwrap();
    let paths = AuraVaultProfilePaths::in_dir(profile.path());
    std::fs::create_dir_all(&paths.media_folder).unwrap();

    let err = read_media(&paths.media_folder, "../outside.png").unwrap_err();
    assert!(matches!(err, ReadMediaError::InvalidFilename));
}
