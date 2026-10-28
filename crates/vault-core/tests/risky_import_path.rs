#[cfg(unix)]
use std::os::unix::fs::symlink;

use std::fs;
use std::path::Path;

use tempfile::tempdir;
use vault_core::import::{import_folder, ImportError};
use vault_core::VaultDb;

#[test]
fn rejects_filesystem_root() {
    let db = VaultDb::open_in_memory().unwrap();
    let err = import_folder(&db, Path::new("/")).unwrap_err();
    assert!(matches!(err, ImportError::RiskyImportPath));
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
