use std::fs;

#[cfg(unix)]
use std::os::unix::fs::symlink;

use tempfile::tempdir;
use vault_core::course_folder::{lesson_file_directory_for_open, CourseFolderOpenError};
use vault_core::import::import_folder;
use vault_core::VaultDb;

#[test]
fn resolves_directory_for_lesson_media_under_course_root() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::create_dir(root.join("week-one")).unwrap();
    fs::write(root.join("week-one/lecture.mp4"), b"bytes").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let lesson_id = db
        .lesson_id_for_stable_key(&report.course_id, "week-one/lecture.mp4")
        .unwrap();

    let open_dir = lesson_file_directory_for_open(&db, &lesson_id).unwrap();
    assert_eq!(open_dir, fs::canonicalize(root.join("week-one")).unwrap());
}

#[test]
#[cfg(unix)]
fn rejects_lesson_media_that_escapes_course_root_via_symlink() {
    let course_dir = tempdir().unwrap();
    let outside_dir = tempdir().unwrap();
    fs::write(outside_dir.path().join("secret.mp4"), b"outside").unwrap();
    symlink(
        outside_dir.path().join("secret.mp4"),
        course_dir.path().join("escape.mp4"),
    )
    .unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, course_dir.path()).unwrap();
    let lesson_id = db
        .lesson_id_for_stable_key(&report.course_id, "escape.mp4")
        .unwrap();

    let err = lesson_file_directory_for_open(&db, &lesson_id).unwrap_err();
    assert!(matches!(err, CourseFolderOpenError::PathOutsideCourseRoot));
}
