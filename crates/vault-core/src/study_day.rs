//! Local calendar days that count toward streaks (offline; machine local date only).

use std::collections::HashSet;

use ::time::{Date, Duration, Month};

use crate::db::{DbError, VaultDb};

#[derive(Debug, thiserror::Error)]
pub enum StudyDayError {
    #[error(transparent)]
    Db(#[from] DbError),
    #[error("invalid local date: {0}")]
    InvalidLocalDate(String),
}

pub fn record_study_day(db: &VaultDb, local_date: &str) -> Result<(), StudyDayError> {
    validate_local_date(local_date)?;
    db.insert_study_day(local_date)?;
    Ok(())
}

pub fn record_card_graded_study_day(db: &VaultDb, local_date: &str) -> Result<(), StudyDayError> {
    record_study_day(db, local_date)
}

pub fn has_study_day(db: &VaultDb, local_date: &str) -> Result<bool, StudyDayError> {
    validate_local_date(local_date)?;
    db.study_day_exists(local_date).map_err(StudyDayError::from)
}

pub fn list_study_days(db: &VaultDb) -> Result<Vec<String>, StudyDayError> {
    db.list_study_days().map_err(StudyDayError::from)
}

pub fn machine_local_date_string() -> String {
    let now =
        ::time::OffsetDateTime::now_local().unwrap_or_else(|_| ::time::OffsetDateTime::now_utc());
    format!(
        "{:04}-{:02}-{:02}",
        now.year(),
        u8::from(now.month()),
        now.day()
    )
}

pub fn record_study_day_today(db: &VaultDb) -> Result<(), StudyDayError> {
    record_study_day(db, &machine_local_date_string())
}

/// Consecutive counted days ending at `today` (inclusive). A calendar day without study breaks the streak.
pub fn current_streak_days(db: &VaultDb, today: &str) -> Result<u32, StudyDayError> {
    current_streak_days_for_dates(&list_study_days(db)?, today)
}

pub fn current_streak_days_for_dates(
    study_days: &[String],
    today: &str,
) -> Result<u32, StudyDayError> {
    let set: HashSet<&str> = study_days.iter().map(String::as_str).collect();
    let mut cursor = parse_local_date(today)?;
    let mut count = 0u32;
    loop {
        let key = format_local_date(cursor);
        if set.contains(key.as_str()) {
            count += 1;
            cursor = cursor
                .checked_sub(Duration::days(1))
                .ok_or_else(|| StudyDayError::InvalidLocalDate(today.to_string()))?;
        } else {
            break;
        }
    }
    Ok(count)
}

pub const STUDY_HEATMAP_WEEKS: u32 = 52;
pub const STUDY_HEATMAP_DAYS: u32 = STUDY_HEATMAP_WEEKS * 7;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudyHeatmapDay {
    pub local_date: String,
    pub counted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudyHeatmap52Weeks {
    pub weeks: Vec<Vec<StudyHeatmapDay>>,
    pub counted_days: u32,
    pub total_days: u32,
}

pub fn study_heatmap_52_weeks(
    db: &VaultDb,
    today: &str,
) -> Result<StudyHeatmap52Weeks, StudyDayError> {
    study_heatmap_52_weeks_for_dates(&list_study_days(db)?, today)
}

pub fn study_heatmap_52_weeks_for_dates(
    study_days: &[String],
    today: &str,
) -> Result<StudyHeatmap52Weeks, StudyDayError> {
    let days_span = i64::from(STUDY_HEATMAP_DAYS);
    let set: HashSet<&str> = study_days.iter().map(String::as_str).collect();
    let end = parse_local_date(today)?;
    let start = end
        .checked_sub(Duration::days(days_span - 1))
        .ok_or_else(|| StudyDayError::InvalidLocalDate(today.to_string()))?;

    let mut days = Vec::with_capacity(STUDY_HEATMAP_DAYS as usize);
    let mut counted_days = 0u32;
    let mut cursor = start;
    loop {
        let key = format_local_date(cursor);
        let counted = set.contains(key.as_str());
        if counted {
            counted_days += 1;
        }
        days.push(StudyHeatmapDay {
            local_date: key,
            counted,
        });
        if cursor == end {
            break;
        }
        cursor = cursor
            .checked_add(Duration::days(1))
            .ok_or_else(|| StudyDayError::InvalidLocalDate(today.to_string()))?;
    }

    let weeks = days.chunks(7).map(|chunk| chunk.to_vec()).collect();

    Ok(StudyHeatmap52Weeks {
        weeks,
        counted_days,
        total_days: STUDY_HEATMAP_DAYS,
    })
}

pub(crate) fn parse_local_date(local_date: &str) -> Result<Date, StudyDayError> {
    validate_local_date(local_date)?;
    let year: i32 = local_date[0..4]
        .parse()
        .map_err(|_| StudyDayError::InvalidLocalDate(local_date.to_string()))?;
    let month: u8 = local_date[5..7]
        .parse()
        .map_err(|_| StudyDayError::InvalidLocalDate(local_date.to_string()))?;
    let day: u8 = local_date[8..10]
        .parse()
        .map_err(|_| StudyDayError::InvalidLocalDate(local_date.to_string()))?;
    let month = Month::try_from(month)
        .map_err(|_| StudyDayError::InvalidLocalDate(local_date.to_string()))?;
    Date::from_calendar_date(year, month, day)
        .map_err(|_| StudyDayError::InvalidLocalDate(local_date.to_string()))
}

pub(crate) fn format_local_date(date: Date) -> String {
    format!(
        "{:04}-{:02}-{:02}",
        date.year(),
        u8::from(date.month()),
        date.day()
    )
}

fn validate_local_date(local_date: &str) -> Result<(), StudyDayError> {
    let bytes = local_date.as_bytes();
    let valid = bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[..4].iter().all(|b| b.is_ascii_digit())
        && bytes[5..7].iter().all(|b| b.is_ascii_digit())
        && bytes[8..10].iter().all(|b| b.is_ascii_digit());
    if valid {
        Ok(())
    } else {
        Err(StudyDayError::InvalidLocalDate(local_date.to_string()))
    }
}
