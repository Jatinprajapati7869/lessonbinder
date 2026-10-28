use std::fs;

use tempfile::tempdir;
use vault_core::import::import_folder;
use vault_core::read_model::guard_local_media_for_resume;
use vault_core::{LessonStatus, VaultDb};

#[test]
fn missing_file_on_resume_marks_broken_not_done() {
    let course = tempdir().unwrap();
    fs::write(course.path().join("lecture.mp4"), b"video").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = import_folder(&db, course.path()).unwrap().lesson_ids[0].clone();

    db.set_resume_position(&lesson_id, 4_500).unwrap();
    assert_eq!(
        db.progress_for_lesson(&lesson_id).unwrap().status,
        LessonStatus::InProgress
    );

    fs::remove_file(course.path().join("lecture.mp4")).unwrap();

    let status = guard_local_media_for_resume(&db, &lesson_id).unwrap();
    assert_eq!(status, LessonStatus::Broken);

    let progress = db.progress_for_lesson(&lesson_id).unwrap();
    assert_eq!(progress.status, LessonStatus::Broken);
    assert_eq!(progress.resume_position_ms, 4_500);
    assert!(db.marked_done_at(&lesson_id).unwrap().is_none());
}
