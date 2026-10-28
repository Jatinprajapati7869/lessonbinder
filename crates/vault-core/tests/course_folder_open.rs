use std::fs;

use tempfile::tempdir;
use vault_core::course_folder::{course_folder_path_for_open, CourseFolderOpenError};
use vault_core::import::{canonical_import_folder_path, import_folder, ImportError};
use vault_core::playlist::import_playlist_definition;
use vault_core::VaultDb;

#[test]
fn resolves_stored_folder_root_after_risky_path_check() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("lecture.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let stored = db.course_root_path(&report.course_id).unwrap().unwrap();
    let expected = canonical_import_folder_path(root).unwrap();
    assert_eq!(stored, expected.to_string_lossy());

    let open_path = course_folder_path_for_open(&db, &report.course_id).unwrap();
    assert_eq!(open_path, expected);
}

#[test]
fn rejects_playlist_course() {
    let db = VaultDb::open_in_memory().unwrap();
    let fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/playlists/two_items.json");
    let report = import_playlist_definition(&db, &fixture).unwrap();

    let err = course_folder_path_for_open(&db, &report.course_id).unwrap_err();
    assert!(matches!(err, CourseFolderOpenError::NotFolderCourse));
}

#[test]
fn rejects_unknown_course_id() {
    let db = VaultDb::open_in_memory().unwrap();
    let err = course_folder_path_for_open(&db, "missing-course").unwrap_err();
    assert!(matches!(err, CourseFolderOpenError::CourseNotFound));
}

#[test]
fn rejects_stored_root_that_fails_risky_import_check() {
    let db = VaultDb::open_in_memory().unwrap();
    let course_id = db.insert_course("Risky", "/").unwrap();

    let err = course_folder_path_for_open(&db, &course_id).unwrap_err();
    assert!(matches!(
        err,
        CourseFolderOpenError::Import(ImportError::RiskyImportPath)
    ));
}
