use std::fs;

use tempfile::tempdir;
use vault_core::import::{import_folder, scan_folder};
use vault_core::VaultDb;

#[test]
fn scan_folder_groups_matching_stem_sidecars_as_lesson_resources() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("lecture.mp4"), b"").unwrap();
    fs::write(root.join("lecture.pdf"), b"").unwrap();
    fs::write(root.join("lecture.png"), b"").unwrap();

    let lessons = scan_folder(root).unwrap();
    assert_eq!(lessons.len(), 1);
    assert_eq!(lessons[0].relative_path, "lecture.mp4");
    assert_eq!(
        lessons[0]
            .resources
            .iter()
            .map(|r| (r.relative_path.as_str(), r.kind.as_str()))
            .collect::<Vec<_>>(),
        vec![("lecture.pdf", "pdf"), ("lecture.png", "image"),],
    );
}

#[test]
fn scan_folder_keeps_unmatched_markdown_as_its_own_lesson() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("notes.md"), b"# Notes").unwrap();

    let lessons = scan_folder(root).unwrap();
    assert_eq!(lessons.len(), 1);
    assert_eq!(lessons[0].relative_path, "notes.md");
    assert!(lessons[0].resources.is_empty());
}

#[test]
fn import_persists_lesson_resources_on_disk() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("lecture.mp4"), b"").unwrap();
    fs::write(root.join("lecture.pdf"), b"").unwrap();
    fs::write(root.join("lecture.png"), b"").unwrap();
    fs::write(root.join("notes.md"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();

    assert_eq!(db.lesson_count_for_course(&report.course_id).unwrap(), 2);
    assert_eq!(
        db.lesson_stable_keys_for_course(&report.course_id).unwrap(),
        vec!["lecture.mp4", "notes.md"],
    );

    let lecture_id = db
        .lesson_id_for_stable_key(&report.course_id, "lecture.mp4")
        .unwrap();
    assert_eq!(
        db.lesson_resources_for_lesson(&lecture_id).unwrap(),
        vec![
            ("lecture.pdf".to_string(), "pdf".to_string()),
            ("lecture.png".to_string(), "image".to_string()),
        ],
    );
}
