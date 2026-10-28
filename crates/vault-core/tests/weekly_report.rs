use std::fs;

use tempfile::tempdir;
use vault_core::import::import_folder;
use vault_core::study_day::machine_local_date_string;
use vault_core::weekly_report::{
    current_weekly_report, local_week_bounds_for_date, record_card_grade,
    watched_ranges_store_session_time,
};
use vault_core::VaultDb;

#[test]
fn local_week_is_monday_through_sunday() {
    let bounds = local_week_bounds_for_date("2026-10-28").unwrap();
    assert_eq!(bounds.monday, "2026-10-26");
    assert_eq!(bounds.sunday, "2026-11-01");
}

#[test]
fn weekly_report_counts_lessons_marked_done_and_cards_graded_in_week() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("clip.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = import_folder(&db, dir.path()).unwrap().lesson_ids[0].clone();
    let today = machine_local_date_string();

    db.mark_complete(&lesson_id).unwrap();
    record_card_grade(&db).unwrap();
    record_card_grade(&db).unwrap();

    let report = current_weekly_report(&db, &today).unwrap();
    let bounds = local_week_bounds_for_date(&today).unwrap();

    assert_eq!(report.lessons_marked_done, 1);
    assert_eq!(report.cards_graded, 2);
    assert_eq!(report.week_monday, bounds.monday);
    assert_eq!(report.week_sunday, bounds.sunday);
}

#[test]
fn weekly_report_omits_watched_minutes_without_session_time_on_ranges() {
    assert!(!watched_ranges_store_session_time());

    let db = VaultDb::open_in_memory().unwrap();
    let report = current_weekly_report(&db, "2026-10-28").unwrap();

    assert!(report.watched_minutes.is_none());
}

#[test]
fn record_card_grade_increments_weekly_card_count() {
    let db = VaultDb::open_in_memory().unwrap();
    let today = machine_local_date_string();

    record_card_grade(&db).unwrap();

    let report = current_weekly_report(&db, &today).unwrap();
    assert_eq!(report.cards_graded, 1);
}
