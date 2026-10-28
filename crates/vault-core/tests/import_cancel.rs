use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};

use tempfile::tempdir;
use vault_core::import::{import_folder, import_folder_with_cancel, ImportError};
use vault_core::library::{list_courses, CourseLibraryVisibility};
use vault_core::LessonStatus;
use vault_core::VaultDb;

fn total_lesson_count(db: &VaultDb) -> usize {
    list_courses(db, None, CourseLibraryVisibility::Active)
        .unwrap()
        .into_iter()
        .map(|course| {
            db.lesson_ids_in_learning_order(&course.course_id)
                .unwrap()
                .len()
        })
        .sum()
}

#[test]
fn cancel_during_scan_leaves_no_course() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("one.mp4"), b"").unwrap();
    fs::write(root.join("two.mp4"), b"").unwrap();
    fs::write(root.join("three.mp4"), b"").unwrap();

    let entry_checks = AtomicUsize::new(0);
    let should_cancel = || entry_checks.fetch_add(1, Ordering::Relaxed) > 0;

    let db = VaultDb::open_in_memory().unwrap();
    let err = import_folder_with_cancel(&db, root, &should_cancel, None).unwrap_err();
    assert!(matches!(err, ImportError::Cancelled));

    assert!(list_courses(&db, None, CourseLibraryVisibility::Active)
        .unwrap()
        .is_empty());
}

#[test]
fn cancel_after_first_lesson_keeps_that_lesson_and_stops_import() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("one.mp4"), b"").unwrap();
    fs::write(root.join("two.mp4"), b"").unwrap();
    fs::write(root.join("three.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let should_cancel = || total_lesson_count(&db) >= 1;

    let err = import_folder_with_cancel(&db, root, &should_cancel, None).unwrap_err();
    assert!(matches!(err, ImportError::Cancelled));

    let courses = list_courses(&db, None, CourseLibraryVisibility::Active).unwrap();
    assert_eq!(courses.len(), 1);
    let lesson_ids = db
        .lesson_ids_in_learning_order(&courses[0].course_id)
        .unwrap();
    assert_eq!(lesson_ids.len(), 1);
}

#[test]
fn cancel_after_first_of_two_files_does_not_mark_that_lesson_done_or_broken() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("one.mp4"), b"").unwrap();
    fs::write(root.join("two.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let should_cancel = || total_lesson_count(&db) >= 1;

    let err = import_folder_with_cancel(&db, root, &should_cancel, None).unwrap_err();
    assert!(matches!(err, ImportError::Cancelled));

    let courses = list_courses(&db, None, CourseLibraryVisibility::Active).unwrap();
    assert_eq!(courses.len(), 1);
    let lesson_id = db
        .lesson_ids_in_learning_order(&courses[0].course_id)
        .unwrap()
        .into_iter()
        .next()
        .expect("first lesson");
    let progress = db.progress_for_lesson(&lesson_id).unwrap();
    assert_eq!(progress.status, LessonStatus::NotStarted);
    assert_ne!(progress.status, LessonStatus::Done);
    assert_ne!(progress.status, LessonStatus::Broken);
}

#[test]
fn reimport_after_cancelled_partial_import_finishes_without_duplicating_first_lesson() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("one.mp4"), b"").unwrap();
    fs::write(root.join("two.mp4"), b"").unwrap();
    fs::write(root.join("three.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let should_cancel = || total_lesson_count(&db) >= 1;

    let err = import_folder_with_cancel(&db, root, &should_cancel, None).unwrap_err();
    assert!(matches!(err, ImportError::Cancelled));

    let courses = list_courses(&db, None, CourseLibraryVisibility::Active).unwrap();
    assert_eq!(courses.len(), 1);
    let course_id = courses[0].course_id.clone();
    let first_lesson_id = db
        .lesson_ids_in_learning_order(&course_id)
        .unwrap()
        .into_iter()
        .next()
        .expect("partial import wrote one lesson");

    let finished = import_folder(&db, root).unwrap();
    assert_eq!(finished.course_id, course_id);
    assert_eq!(db.lesson_count_for_course(&course_id).unwrap(), 3);
    let mut stable_keys = db.lesson_stable_keys_for_course(&course_id).unwrap();
    assert_eq!(stable_keys.len(), 3);
    stable_keys.sort();
    let mut expected_keys = vec!["one.mp4", "two.mp4", "three.mp4"];
    expected_keys.sort();
    assert_eq!(stable_keys, expected_keys);

    let lesson_ids = db.lesson_ids_in_learning_order(&course_id).unwrap();
    assert_eq!(lesson_ids.len(), 3);
    assert_eq!(lesson_ids[0], first_lesson_id);
    assert_eq!(
        db.lesson_id_for_stable_key(&course_id, "one.mp4").unwrap(),
        first_lesson_id
    );
}
