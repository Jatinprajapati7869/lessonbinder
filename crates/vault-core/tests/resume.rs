use std::fs;

use tempfile::tempdir;
use vault_core::import::import_folder;
use vault_core::{LessonStatus, ProgressPersistenceError, VaultDb};

fn lesson_id_from_import(db: &VaultDb, root: &std::path::Path) -> String {
    let report = import_folder(db, root).unwrap();
    report.lesson_ids[0].clone()
}

#[test]
fn resume_sets_in_progress() {
    let db = VaultDb::open_in_memory().unwrap();
    let course = tempdir().unwrap();
    fs::write(course.path().join("clip.mp4"), b"").unwrap();
    let lesson_id = lesson_id_from_import(&db, course.path());

    db.set_resume_position(&lesson_id, 4_500).unwrap();

    let progress = db.progress_for_lesson(&lesson_id).unwrap();
    assert_eq!(progress.resume_position_ms, 4_500);
    assert_eq!(progress.status, LessonStatus::InProgress);
}

#[test]
fn resume_round_trip() {
    let data = tempdir().unwrap();
    let db_path = data.path().join("vault.db");
    let course = tempdir().unwrap();
    fs::write(course.path().join("clip.mp4"), b"").unwrap();

    let lesson_id = {
        let db = VaultDb::open(&db_path).unwrap();
        let id = lesson_id_from_import(&db, course.path());
        db.set_resume_position(&id, 12_345).unwrap();
        id
    };

    let db = VaultDb::open(&db_path).unwrap();
    assert_eq!(db.resume_position(&lesson_id).unwrap(), 12_345);
}

#[test]
fn resume_does_not_mark_done() {
    let db = VaultDb::open_in_memory().unwrap();
    let course = tempdir().unwrap();
    fs::write(course.path().join("clip.mp4"), b"").unwrap();
    let lesson_id = lesson_id_from_import(&db, course.path());

    db.set_resume_position(&lesson_id, 2_000).unwrap();
    let progress = db.progress_for_lesson(&lesson_id).unwrap();
    assert_ne!(progress.status, LessonStatus::Done);
}

#[test]
fn resume_rejects_negative_position() {
    let db = VaultDb::open_in_memory().unwrap();
    let course = tempdir().unwrap();
    fs::write(course.path().join("clip.mp4"), b"").unwrap();
    let lesson_id = lesson_id_from_import(&db, course.path());

    let err = db.set_resume_position(&lesson_id, -1).unwrap_err();
    assert!(matches!(
        err,
        ProgressPersistenceError::IllegalTransition { .. }
    ));
    assert_eq!(db.resume_position(&lesson_id).unwrap(), 0);
}
