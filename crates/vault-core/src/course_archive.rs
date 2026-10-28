//! Hide courses from the library without deleting lessons, progress, or Anki links.

use crate::db::DbError;
use crate::db::VaultDb;

#[derive(Debug, thiserror::Error)]
pub enum CourseArchiveError {
    #[error(transparent)]
    Db(#[from] DbError),
    #[error("course not found: {0}")]
    CourseNotFound(String),
}

pub fn archive_course(db: &VaultDb, course_id: &str) -> Result<(), CourseArchiveError> {
    if !db.archive_course_row(course_id)? {
        return Err(CourseArchiveError::CourseNotFound(course_id.to_string()));
    }
    Ok(())
}

pub fn unarchive_course(db: &VaultDb, course_id: &str) -> Result<(), CourseArchiveError> {
    if !db.unarchive_course_row(course_id)? {
        return Err(CourseArchiveError::CourseNotFound(course_id.to_string()));
    }
    Ok(())
}
