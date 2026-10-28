use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};

use tempfile::tempdir;
use vault_core::import::{import_folder_with_cancel, ImportError};
use vault_core::library::{list_courses, CourseLibraryVisibility};
use vault_core::VaultDb;

#[test]
fn cancel_after_first_directory_entry_leaves_no_course() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("one.mp4"), b"").unwrap();
    fs::write(root.join("two.mp4"), b"").unwrap();
    fs::write(root.join("three.mp4"), b"").unwrap();

    let entry_checks = AtomicUsize::new(0);
    let should_cancel = || entry_checks.fetch_add(1, Ordering::Relaxed) > 0;

    let db = VaultDb::open_in_memory().unwrap();
    let err = import_folder_with_cancel(&db, root, &should_cancel).unwrap_err();
    assert!(matches!(err, ImportError::Cancelled));

    assert!(list_courses(&db, None, CourseLibraryVisibility::Active)
        .unwrap()
        .is_empty());
}
