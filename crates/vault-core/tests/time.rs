use std::fs;

use tempfile::tempdir;
use vault_core::extract::{ExtractorOutcome, SubtitleExtract};
use vault_core::import::import_folder;
use vault_core::roadmap::{add_roadmap_step, create_roadmap};
use vault_core::time::{course_remaining_time, roadmap_remaining_time};
use vault_core::VaultDb;

fn empty_subtitle() -> SubtitleExtract {
    SubtitleExtract {
        cue_count: ExtractorOutcome::unknown("test", "none"),
        first_timestamp_ms: ExtractorOutcome::unknown("test", "none"),
        last_timestamp_ms: ExtractorOutcome::unknown("test", "none"),
    }
}

fn set_duration(db: &VaultDb, lesson_id: &str, ms: i64) {
    db.save_lesson_metadata(
        lesson_id,
        &ExtractorOutcome::known("test", ms),
        &ExtractorOutcome::unknown("test", "n/a"),
        &ExtractorOutcome::unknown("test", "n/a"),
        &empty_subtitle(),
    )
    .unwrap();
}

fn three_lesson_course(db: &VaultDb) -> (String, Vec<String>) {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.mp4"), b"").unwrap();
    fs::write(dir.path().join("b.mp4"), b"").unwrap();
    fs::write(dir.path().join("c.mp4"), b"").unwrap();
    let report = import_folder(db, dir.path()).unwrap();
    (report.course_id, report.lesson_ids)
}

#[test]
fn remaining_excludes_unknown() {
    let db = VaultDb::open_in_memory().unwrap();
    let (course_id, lesson_ids) = three_lesson_course(&db);

    set_duration(&db, &lesson_ids[0], 60_000);
    set_duration(&db, &lesson_ids[1], 120_000);
    // lesson_ids[2] left without duration metadata

    db.mark_complete(&lesson_ids[1]).unwrap();

    let estimate = course_remaining_time(&db, &course_id).unwrap();
    assert_eq!(estimate.remaining_ms, 60_000);
    assert_eq!(estimate.unknown_lessons, 1);

    let roadmap_id = create_roadmap(&db, "Study path").unwrap();
    add_roadmap_step(&db, &roadmap_id, &course_id, None).unwrap();
    let roadmap_estimate = roadmap_remaining_time(&db, &roadmap_id).unwrap();
    assert_eq!(roadmap_estimate.remaining_ms, 60_000);
    assert_eq!(roadmap_estimate.unknown_lessons, 1);
}

#[test]
fn done_course_zero() {
    let db = VaultDb::open_in_memory().unwrap();
    let (course_id, lesson_ids) = three_lesson_course(&db);

    set_duration(&db, &lesson_ids[0], 90_000);
    set_duration(&db, &lesson_ids[1], 90_000);
    set_duration(&db, &lesson_ids[2], 90_000);

    for id in &lesson_ids {
        db.mark_complete(id).unwrap();
    }

    let estimate = course_remaining_time(&db, &course_id).unwrap();
    assert_eq!(estimate.remaining_ms, 0);
    assert_eq!(estimate.unknown_lessons, 0);
}
