use std::fs;

use tempfile::tempdir;
use vault_core::import::{import_folder, ImportError};
use vault_core::library::{list_courses, CourseLibraryVisibility};
use vault_core::VaultDb;

#[test]
fn creates_lessons_for_eligible_extensions() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("a.mp4"), b"").unwrap();
    fs::write(root.join("b.mkv"), b"").unwrap();
    fs::write(root.join("notes.md"), b"").unwrap();
    fs::write(root.join(".hidden.mp4"), b"").unwrap();
    let node_modules = root.join("node_modules");
    fs::create_dir(&node_modules).unwrap();
    fs::write(node_modules.join("evil.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();

    assert_eq!(db.lesson_count_for_course(&report.course_id).unwrap(), 3);
    assert_eq!(
        db.lesson_stable_keys_for_course(&report.course_id).unwrap(),
        vec!["a.mp4", "b.mkv", "notes.md"]
    );
}

#[test]
fn idempotent_reimport() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("one.mp4"), b"").unwrap();
    fs::write(root.join("two.txt"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let first = import_folder(&db, root).unwrap();
    let second = import_folder(&db, root).unwrap();

    assert_eq!(first.course_id, second.course_id);
    assert_eq!(first.lesson_ids, second.lesson_ids);
    assert_eq!(db.lesson_count_for_course(&first.course_id).unwrap(), 2);
}

#[test]
fn attaches_subtitle_by_stem() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("lecture.mp4"), b"").unwrap();
    fs::write(root.join("lecture.srt"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();

    assert_eq!(db.lesson_count_for_course(&report.course_id).unwrap(), 1);
    let subtitle = db
        .subtitle_relative_path_for_lesson(&report.lesson_ids[0])
        .unwrap();
    assert_eq!(subtitle.as_deref(), Some("lecture.srt"));
}

#[test]
fn empty_folder_creates_no_course() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::create_dir(root.join("empty-sub")).unwrap();
    fs::write(root.join("readme.doc"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let err = import_folder(&db, root).unwrap_err();
    assert!(matches!(err, ImportError::NoLessonFiles));
    assert_eq!(
        list_courses(&db, None, CourseLibraryVisibility::Active)
            .unwrap()
            .len(),
        0
    );
}
