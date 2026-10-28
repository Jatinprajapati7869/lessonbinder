use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};

use tempfile::tempdir;
use vault_core::import::{import_folder_with_cancel, ImportError};
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
