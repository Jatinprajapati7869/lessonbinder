use std::fs;

use tempfile::tempdir;
use vault_core::import::import_folder;
use vault_core::read_model::{course_progress, library_summary, refresh_sources};
use vault_core::{DerivedStatus, VaultDb};

fn three_lesson_course_with_db(db: &VaultDb) -> (tempfile::TempDir, String) {
    let course = tempdir().unwrap();
    fs::write(course.path().join("a.mp4"), b"a").unwrap();
    fs::write(course.path().join("b.mp4"), b"b").unwrap();
    fs::write(course.path().join("c.mp4"), b"c").unwrap();
    let report = import_folder(db, course.path()).unwrap();
    (course, report.course_id)
}

#[test]
fn library_summary_by_status() {
    let db = VaultDb::open_in_memory().unwrap();

    let course_a = tempdir().unwrap();
    fs::write(course_a.path().join("one.mp4"), b"").unwrap();

    let course_b = tempdir().unwrap();
    fs::write(course_b.path().join("one.mp4"), b"").unwrap();
    fs::write(course_b.path().join("two.mp4"), b"").unwrap();

    let _id_a = import_folder(&db, course_a.path()).unwrap().course_id;
    let report_b = import_folder(&db, course_b.path()).unwrap();

    db.set_resume_position(&report_b.lesson_ids[0], 500)
        .unwrap();

    let summary = library_summary(&db).unwrap();
    assert_eq!(summary.not_started, 1);
    assert_eq!(summary.in_progress, 1);
    assert_eq!(summary.done, 0);
    assert_eq!(summary.broken, 0);

    db.mark_complete(&report_b.lesson_ids[0]).unwrap();
    db.mark_complete(&report_b.lesson_ids[1]).unwrap();
    let summary = library_summary(&db).unwrap();
    assert_eq!(summary.not_started, 1);
    assert_eq!(summary.in_progress, 0);
    assert_eq!(summary.done, 1);
    assert_eq!(summary.broken, 0);
}

#[test]
fn refresh_marks_broken() {
    let db = VaultDb::open_in_memory().unwrap();
    let (course, course_id) = three_lesson_course_with_db(&db);

    assert_eq!(
        course_progress(&db, &course_id).unwrap(),
        DerivedStatus::NotStarted
    );

    fs::remove_file(course.path().join("b.mp4")).unwrap();
    refresh_sources(&db).unwrap();

    assert_eq!(
        course_progress(&db, &course_id).unwrap(),
        DerivedStatus::Broken
    );

    let summary = library_summary(&db).unwrap();
    assert_eq!(summary.broken, 1);
    assert_eq!(summary.not_started, 0);
}
