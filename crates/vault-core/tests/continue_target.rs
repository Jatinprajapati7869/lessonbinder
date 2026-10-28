use std::fs;

use tempfile::tempdir;
use vault_core::continue_target;
use vault_core::import::import_folder;
use vault_core::library_continue_row;
use vault_core::VaultDb;

fn import_single_lesson(db: &VaultDb, name: &str) -> (String, String) {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join(name), b"").unwrap();
    let report = import_folder(db, dir.path()).unwrap();
    (report.course_id, report.lesson_ids[0].clone())
}

#[test]
fn prefers_in_progress_lesson_with_newest_resume_write() {
    let db = VaultDb::open_in_memory().unwrap();
    let (_course_a, lesson_a) = import_single_lesson(&db, "a.mp4");
    let (_course_b, lesson_b) = import_single_lesson(&db, "b.mp4");

    db.set_resume_position(&lesson_a, 1_000).unwrap();
    db.set_resume_position(&lesson_b, 2_000).unwrap();
    db.set_progress_last_engaged_at(&lesson_a, "2026-01-01T10:00:00Z")
        .unwrap();
    db.set_progress_last_engaged_at(&lesson_b, "2026-01-02T10:00:00Z")
        .unwrap();

    let target = continue_target(&db).unwrap().expect("continue target");
    assert_eq!(target.lesson_id, lesson_b);

    let row = library_continue_row(&db).unwrap().expect("continue row");
    assert_eq!(row.lesson_id, lesson_b);
    assert_eq!(row.resume_position_ms, 2_000);
}

#[test]
fn else_first_not_started_in_library_lesson_order() {
    let db = VaultDb::open_in_memory().unwrap();
    let dir_old = tempdir().unwrap();
    fs::write(dir_old.path().join("done.mp4"), b"").unwrap();
    let old = import_folder(&db, dir_old.path()).unwrap();
    db.mark_complete(&old.lesson_ids[0]).unwrap();

    let dir_new = tempdir().unwrap();
    fs::write(dir_new.path().join("z-last.mp4"), b"").unwrap();
    fs::write(dir_new.path().join("a-first.mp4"), b"").unwrap();
    let new = import_folder(&db, dir_new.path()).unwrap();
    let first_id = db
        .lesson_id_for_stable_key(&new.course_id, "a-first.mp4")
        .unwrap();

    let target = continue_target(&db).unwrap().expect("continue target");
    assert_eq!(target.lesson_id, first_id);
    assert_eq!(target.course_id, new.course_id);
}

#[test]
fn returns_none_when_no_in_progress_or_not_started() {
    let db = VaultDb::open_in_memory().unwrap();
    let (_course, lesson) = import_single_lesson(&db, "only.mp4");
    db.mark_complete(&lesson).unwrap();

    assert!(continue_target(&db).unwrap().is_none());
}

#[test]
fn skips_broken_and_done_when_finding_not_started() {
    let db = VaultDb::open_in_memory().unwrap();
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("broken.mp4"), b"").unwrap();
    fs::write(dir.path().join("next.mp4"), b"").unwrap();
    let report = import_folder(&db, dir.path()).unwrap();
    let broken_id = db
        .lesson_id_for_stable_key(&report.course_id, "broken.mp4")
        .unwrap();
    let next_id = db
        .lesson_id_for_stable_key(&report.course_id, "next.mp4")
        .unwrap();
    db.mark_complete(&broken_id).unwrap();
    db.mark_source_missing(&next_id).unwrap();

    assert!(continue_target(&db).unwrap().is_none());
}
