use std::fs;

use tempfile::tempdir;
use vault_core::extract::{ExtractorOutcome, SubtitleExtract};
use vault_core::import::{import_folder, rescan};
use vault_core::lesson_note::{create_lesson_note, list_lesson_notes};
use vault_core::order::{learning_order, reorder_lessons};
use vault_core::{record_playback, LessonStatus, VaultDb};

fn set_duration(db: &VaultDb, lesson_id: &str, ms: i64) {
    let empty_subtitle = SubtitleExtract {
        cue_count: ExtractorOutcome::unknown("test", "none"),
        first_timestamp_ms: ExtractorOutcome::unknown("test", "none"),
        last_timestamp_ms: ExtractorOutcome::unknown("test", "none"),
    };
    db.save_lesson_metadata(
        lesson_id,
        &ExtractorOutcome::known("test", ms),
        &ExtractorOutcome::unknown("test", "n/a"),
        &ExtractorOutcome::unknown("test", "n/a"),
        &empty_subtitle,
    )
    .unwrap();
}

#[test]
fn first_course_sitting() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("alpha.mp4"), b"alpha-bytes").unwrap();
    fs::write(root.join("beta.mp4"), b"beta-bytes").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let alpha = db
        .lesson_id_for_stable_key(&report.course_id, "alpha.mp4")
        .unwrap();
    let beta = db
        .lesson_id_for_stable_key(&report.course_id, "beta.mp4")
        .unwrap();

    reorder_lessons(&db, &report.course_id, &[beta.clone(), alpha.clone()]).unwrap();

    db.set_resume_position(&alpha, 8_000).unwrap();
    create_lesson_note(&db, &alpha, "Sitting note", Some(1_000)).unwrap();
    db.mark_complete(&beta).unwrap();

    set_duration(&db, &alpha, 60_000);
    record_playback(&db, &alpha, 0, 60_000).unwrap();
    assert_eq!(
        db.watched_percent_for_lesson(&alpha).unwrap(),
        Some(100),
        "watched percent is tracked separately from completion"
    );
    assert_eq!(
        db.progress_for_lesson(&alpha).unwrap().status,
        LessonStatus::InProgress,
        "watch percent does not set done"
    );
    assert_eq!(
        db.progress_for_lesson(&beta).unwrap().status,
        LessonStatus::Done
    );

    fs::write(root.join("gamma.mp4"), b"gamma-bytes").unwrap();
    rescan(&db, &report.course_id).unwrap();

    let order = learning_order(&db, &report.course_id).unwrap();
    assert_eq!(order.len(), 3);
    assert_eq!(order[0..2], [beta.clone(), alpha.clone()]);
    assert_eq!(
        list_lesson_notes(&db, &alpha).unwrap()[0].text,
        "Sitting note"
    );
    assert_eq!(
        db.progress_for_lesson(&beta).unwrap().status,
        LessonStatus::Done
    );
    assert_eq!(
        db.progress_for_lesson(&alpha).unwrap().resume_position_ms,
        8_000
    );
    assert_eq!(
        db.progress_for_lesson(&alpha).unwrap().status,
        LessonStatus::InProgress
    );
}
