use std::fs;

use tempfile::tempdir;
use vault_core::import::import_folder;
use vault_core::sections::{folder_lesson_section, list_sections};
use vault_core::VaultDb;

#[test]
fn list_sections_two_folders_and_one_root_file() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::create_dir(root.join("alpha")).unwrap();
    fs::create_dir(root.join("beta")).unwrap();
    fs::write(root.join("alpha").join("a1.mp4"), b"").unwrap();
    fs::write(root.join("beta").join("b1.mp4"), b"").unwrap();
    fs::write(root.join("overview.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();

    let keys_before = db.lesson_stable_keys_for_course(&report.course_id).unwrap();
    let sections = list_sections(&db, &report.course_id).unwrap();
    let keys_after = db.lesson_stable_keys_for_course(&report.course_id).unwrap();

    assert_eq!(keys_before, keys_after);
    assert_eq!(
        keys_before,
        vec!["alpha/a1.mp4", "beta/b1.mp4", "overview.mp4"],
    );
    assert_eq!(sections, vec!["alpha".to_string(), "beta".to_string()]);

    assert_eq!(folder_lesson_section("overview.mp4"), None);
    assert_eq!(
        folder_lesson_section("alpha/a1.mp4").as_deref(),
        Some("alpha")
    );
    assert_eq!(
        folder_lesson_section("beta/b1.mp4").as_deref(),
        Some("beta")
    );
}
