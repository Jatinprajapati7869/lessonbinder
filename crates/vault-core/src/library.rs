//! Unified library course list (folder + playlist).

use crate::db::DbError;
use crate::db::VaultDb;
use crate::read_model::course_progress;
use crate::time::{course_remaining_time, RemainingStudyTime};
use crate::DerivedStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourseSourceKind {
    Folder,
    Playlist,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourseLibraryVisibility {
    Active,
    Archived,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseListEntry {
    pub course_id: String,
    pub title: String,
    pub source_kind: CourseSourceKind,
    pub status: DerivedStatus,
    pub remaining: RemainingStudyTime,
    pub thumbnail_path: Option<String>,
}

/// All courses in import order, optionally filtered by derived status.
pub fn list_courses(
    db: &VaultDb,
    status_filter: Option<DerivedStatus>,
    visibility: CourseLibraryVisibility,
) -> Result<Vec<CourseListEntry>, DbError> {
    let rows = db.list_course_rows(visibility)?;
    let mut entries = Vec::with_capacity(rows.len());
    for row in rows {
        let status = course_progress(db, &row.id)?;
        if status_filter.is_some_and(|filter| filter != status) {
            continue;
        }
        let remaining = course_remaining_time(db, &row.id)?;
        let thumbnail_path = db.first_lesson_thumbnail_path_for_course(&row.id)?;
        entries.push(CourseListEntry {
            course_id: row.id,
            title: row.title,
            source_kind: parse_source_kind(&row.source_kind)?,
            status,
            remaining,
            thumbnail_path,
        });
    }
    Ok(entries)
}

fn parse_source_kind(value: &str) -> Result<CourseSourceKind, DbError> {
    match value {
        "folder" => Ok(CourseSourceKind::Folder),
        "playlist" => Ok(CourseSourceKind::Playlist),
        other => Err(DbError::InvalidData(format!(
            "unknown course source_kind: {other}"
        ))),
    }
}
