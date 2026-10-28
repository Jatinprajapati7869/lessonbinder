//! Library-wide "continue learning" target selection.

use crate::db::DbError;
use crate::db::VaultDb;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinueTarget {
    pub course_id: String,
    pub lesson_id: String,
}

/// Lesson to resume: newest in-progress resume write, else first `not_started` in library order.
pub fn continue_target(db: &VaultDb) -> Result<Option<ContinueTarget>, DbError> {
    if let Some(target) = db.most_recent_in_progress_continue_target()? {
        return Ok(Some(target));
    }
    db.first_not_started_continue_target()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryContinueRow {
    pub course_id: String,
    pub course_title: String,
    pub lesson_id: String,
    pub lesson_title: String,
    pub resume_position_ms: i64,
    pub watched_percent: Option<i64>,
}

pub fn library_continue_row(db: &VaultDb) -> Result<Option<LibraryContinueRow>, DbError> {
    let Some(target) = continue_target(db)? else {
        return Ok(None);
    };
    let course_title = db.course_title(&target.course_id)?;
    let lesson_title = db.lesson_display_title(&target.lesson_id)?;
    let progress = db.progress_for_lesson(&target.lesson_id)?;
    let watched_percent = db.watched_percent_for_lesson(&target.lesson_id)?;
    Ok(Some(LibraryContinueRow {
        course_id: target.course_id,
        course_title,
        lesson_id: target.lesson_id,
        lesson_title,
        resume_position_ms: progress.resume_position_ms,
        watched_percent,
    }))
}
