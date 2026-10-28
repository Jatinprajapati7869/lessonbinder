use std::fs;

use tempfile::tempdir;
use vault_core::course::course_detail;
use vault_core::import::import_folder;
use vault_core::library::{list_courses, CourseLibraryVisibility};
use vault_core::study_day::{
    current_streak_days, has_study_day, list_study_days, record_study_day,
};
use vault_core::VaultDb;

const TODAY: &str = "2026-10-28";
const YESTERDAY: &str = "2026-10-27";

#[test]
fn records_local_date_when_study_day_is_logged() {
    let db = VaultDb::open_in_memory().unwrap();

    record_study_day(&db, "2026-10-28").unwrap();
    record_study_day(&db, "2026-10-28").unwrap();

    assert!(has_study_day(&db, "2026-10-28").unwrap());
    assert!(!has_study_day(&db, "2026-10-27").unwrap());
    assert_eq!(
        list_study_days(&db).unwrap(),
        vec!["2026-10-28".to_string()]
    );
}

#[test]
fn marking_lesson_complete_records_study_day_for_local_date() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("clip.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = import_folder(&db, dir.path()).unwrap().lesson_ids[0].clone();

    db.mark_complete_with_local_study_day(&lesson_id, "2026-10-28")
        .unwrap();

    assert!(has_study_day(&db, "2026-10-28").unwrap());
}

#[test]
fn resume_position_does_not_record_study_day() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("clip.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = import_folder(&db, dir.path()).unwrap().lesson_ids[0].clone();

    db.set_resume_position(&lesson_id, 5_000).unwrap();

    assert!(list_study_days(&db).unwrap().is_empty());
}

#[test]
fn opening_course_without_play_or_grade_leaves_today_study_day_unchanged() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("clip.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, dir.path()).unwrap();
    record_study_day(&db, YESTERDAY).unwrap();

    assert!(!has_study_day(&db, TODAY).unwrap());
    let before = list_study_days(&db).unwrap();
    assert_eq!(current_streak_days(&db, TODAY).unwrap(), 0);

    assert!(list_courses(&db, None, CourseLibraryVisibility::Active).is_ok());
    let detail = course_detail(&db, &report.course_id).unwrap();
    assert!(!detail.lessons.is_empty());

    assert_eq!(list_study_days(&db).unwrap(), before);
    assert!(!has_study_day(&db, TODAY).unwrap());
    assert_eq!(current_streak_days(&db, TODAY).unwrap(), 0);
}

#[test]
fn gap_day_without_study_breaks_streak() {
    let db = VaultDb::open_in_memory().unwrap();

    record_study_day(&db, "2026-10-26").unwrap();
    record_study_day(&db, "2026-10-27").unwrap();
    record_study_day(&db, "2026-10-29").unwrap();

    assert_eq!(
        vault_core::study_day::current_streak_days(&db, "2026-10-29").unwrap(),
        1
    );
    assert_eq!(
        vault_core::study_day::current_streak_days(&db, "2026-10-28").unwrap(),
        0
    );
}

#[test]
fn consecutive_study_days_increase_current_streak() {
    let db = VaultDb::open_in_memory().unwrap();

    record_study_day(&db, "2026-10-26").unwrap();
    record_study_day(&db, "2026-10-27").unwrap();
    record_study_day(&db, "2026-10-28").unwrap();

    assert_eq!(
        vault_core::study_day::current_streak_days(&db, "2026-10-28").unwrap(),
        3
    );
}

#[test]
fn card_graded_records_study_day_for_local_date() {
    let db = VaultDb::open_in_memory().unwrap();

    vault_core::study_day::record_card_graded_study_day(&db, "2026-10-29").unwrap();

    assert!(has_study_day(&db, "2026-10-29").unwrap());
}

#[test]
fn heatmap_52_weeks_has_364_days_and_counts_study_in_window() {
    let db = VaultDb::open_in_memory().unwrap();

    record_study_day(&db, "2026-10-26").unwrap();
    record_study_day(&db, "2026-10-28").unwrap();
    record_study_day(&db, "2025-01-01").unwrap();

    let heatmap = vault_core::study_day::study_heatmap_52_weeks(&db, "2026-10-28").unwrap();

    assert_eq!(heatmap.total_days, 364);
    assert_eq!(heatmap.weeks.len(), 52);
    assert!(heatmap.weeks.iter().all(|week| week.len() == 7));
    assert_eq!(heatmap.counted_days, 2);
    assert!(heatmap
        .weeks
        .last()
        .unwrap()
        .iter()
        .any(|day| { day.local_date == "2026-10-28" && day.counted }));
}
