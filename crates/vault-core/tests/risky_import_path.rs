#[cfg(unix)]
use std::os::unix::fs::symlink;

use std::fs;
use std::path::Path;

use tempfile::tempdir;
use vault_core::import::{import_folder, validate_import_folder_path, ImportError};
use vault_core::VaultDb;

#[test]
fn validate_import_folder_path_guard_blocks_drop_on_risky_root() {
    let db = VaultDb::open_in_memory().unwrap();
    let err = validate_import_folder_path(Path::new("/")).unwrap_err();
    assert!(matches!(err, ImportError::RiskyImportPath));
    assert!(import_folder(&db, Path::new("/")).is_err());
}

#[test]
fn rejects_home_directory() {
    let home = std::env::var_os("HOME").expect("HOME for risky import test");
    let db = VaultDb::open_in_memory().unwrap();
    let err = import_folder(&db, Path::new(&home)).unwrap_err();
    assert!(matches!(err, ImportError::RiskyImportPath));
}

#[test]
fn allows_normal_course_folder() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("lesson.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, dir.path()).unwrap();

    assert_eq!(db.lesson_count_for_course(&report.course_id).unwrap(), 1);
}

#[test]
fn rejects_file_as_import_root() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("lesson.mp4");
    fs::write(&file, b"").unwrap();
    let db = VaultDb::open_in_memory().unwrap();
    let err = import_folder(&db, &file).unwrap_err();
    assert!(matches!(err, ImportError::InvalidPath(_)));
}

#[cfg(unix)]
#[test]
fn rejects_symlink_that_escapes_picked_directory() {
    let base = tempdir().unwrap();
    let outside = tempdir().unwrap();
    fs::write(outside.path().join("lesson.mp4"), b"").unwrap();
    symlink(outside.path(), base.path().join("course")).unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let err = import_folder(&db, &base.path().join("course")).unwrap_err();
    assert!(matches!(err, ImportError::RiskyImportPath));
}
