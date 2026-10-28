//! Read-only summary for the current local calendar week (Monday–Sunday).

use ::time::{Duration, OffsetDateTime, PrimitiveDateTime, Time};

use crate::db::{DbError, VaultDb};
use crate::study_day::{format_local_date, parse_local_date, StudyDayError};

#[derive(Debug, thiserror::Error)]
pub enum WeeklyReportError {
    #[error(transparent)]
    Db(#[from] DbError),
    #[error(transparent)]
    StudyDay(#[from] StudyDayError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalWeekBounds {
    pub monday: String,
    pub sunday: String,
    pub start_unix: i64,
    pub end_unix: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeeklyReport {
    pub week_monday: String,
    pub week_sunday: String,
    pub lessons_marked_done: u32,
    pub cards_graded: u32,
    pub watched_minutes: Option<u32>,
}

/// Watched ranges store timeline spans only, not when playback happened.
pub fn watched_ranges_store_session_time() -> bool {
    false
}

pub fn local_week_bounds_for_date(today: &str) -> Result<LocalWeekBounds, WeeklyReportError> {
    let date = parse_local_date(today)?;
    let monday = date - Duration::days(date.weekday().number_days_from_monday() as i64);
    let sunday = monday + Duration::days(6);
    let offset = OffsetDateTime::now_local()
        .map(|dt| dt.offset())
        .unwrap_or_else(|_| OffsetDateTime::now_utc().offset());
    let start = PrimitiveDateTime::new(monday, Time::MIDNIGHT).assume_offset(offset);
    let end = PrimitiveDateTime::new(
        sunday,
        Time::from_hms(23, 59, 59).expect("valid end-of-day time"),
    )
    .assume_offset(offset);
    Ok(LocalWeekBounds {
        monday: format_local_date(monday),
        sunday: format_local_date(sunday),
        start_unix: start.unix_timestamp(),
        end_unix: end.unix_timestamp(),
    })
}

pub fn current_weekly_report(db: &VaultDb, today: &str) -> Result<WeeklyReport, WeeklyReportError> {
    let bounds = local_week_bounds_for_date(today)?;
    let lessons_marked_done =
        db.count_lessons_marked_done_between(bounds.start_unix, bounds.end_unix)?;
    let cards_graded = db.count_card_grades_between(bounds.start_unix, bounds.end_unix)?;
    let watched_minutes = if watched_ranges_store_session_time() {
        Some(total_watched_minutes(db)?)
    } else {
        None
    };
    Ok(WeeklyReport {
        week_monday: bounds.monday,
        week_sunday: bounds.sunday,
        lessons_marked_done,
        cards_graded,
        watched_minutes,
    })
}

pub fn record_card_grade(db: &VaultDb) -> Result<(), WeeklyReportError> {
    db.insert_card_grade_event(&activity_unix_timestamp())?;
    crate::study_day::record_study_day_today(db)?;
    Ok(())
}

fn total_watched_minutes(db: &VaultDb) -> Result<u32, WeeklyReportError> {
    let ms = db.total_watched_ms_all_lessons()?;
    Ok(u32::try_from(ms / 60_000).unwrap_or(u32::MAX))
}

fn activity_unix_timestamp() -> String {
    format!(
        "{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_secs()
    )
}
