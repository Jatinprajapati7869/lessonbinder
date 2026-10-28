use std::fs;

use tempfile::tempdir;
use vault_core::import::import_folder;
use vault_core::{LessonStatus, ProgressPersistenceError, VaultDb};

fn lesson_id(db: &VaultDb, root: &std::path::Path) -> String {
    import_folder(db, root).unwrap().lesson_ids[0].clone()
}

fn fixture_course() -> tempfile::TempDir {
    let course = tempdir().unwrap();
    fs::write(course.path().join("clip.mp4"), b"").unwrap();
    course
}

#[test]
fn manual_done() {
    let db = VaultDb::open_in_memory().unwrap();
    let course = fixture_course();
    let lesson_id = lesson_id(&db, course.path());

    db.set_resume_position(&lesson_id, 9_000).unwrap();
    db.mark_complete(&lesson_id).unwrap();

    let progress = db.progress_for_lesson(&lesson_id).unwrap();
    assert_eq!(progress.status, LessonStatus::Done);
    assert_eq!(progress.resume_position_ms, 9_000);
    assert!(db.marked_done_at(&lesson_id).unwrap().is_some());
}

#[test]
fn incomplete_from_done_with_resume() {
    let db = VaultDb::open_in_memory().unwrap();
    let course = fixture_course();
    let lesson_id = lesson_id(&db, course.path());

    db.set_resume_position(&lesson_id, 1_500).unwrap();
    db.mark_complete(&lesson_id).unwrap();
    db.mark_incomplete(&lesson_id).unwrap();

    let progress = db.progress_for_lesson(&lesson_id).unwrap();
    assert_eq!(progress.status, LessonStatus::InProgress);
    assert_eq!(progress.resume_position_ms, 1_500);
    assert!(db.marked_done_at(&lesson_id).unwrap().is_none());
}

#[test]
fn incomplete_from_done_without_resume() {
    let db = VaultDb::open_in_memory().unwrap();
    let course = fixture_course();
    let lesson_id = lesson_id(&db, course.path());

    db.mark_complete(&lesson_id).unwrap();
    db.mark_incomplete(&lesson_id).unwrap();

    let progress = db.progress_for_lesson(&lesson_id).unwrap();
    assert_eq!(progress.status, LessonStatus::NotStarted);
    assert_eq!(progress.resume_position_ms, 0);
}

#[test]
fn broken_cannot_complete() {
    let db = VaultDb::open_in_memory().unwrap();
    let course = fixture_course();
    let lesson_id = lesson_id(&db, course.path());

    db.mark_source_missing(&lesson_id).unwrap();
    let err = db.mark_complete(&lesson_id).unwrap_err();
    assert!(matches!(
        err,
        ProgressPersistenceError::IllegalTransition { .. }
    ));
    assert_eq!(
        db.progress_for_lesson(&lesson_id).unwrap().status,
        LessonStatus::Broken
    );
}
