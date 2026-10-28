//! Derived progress read model: per-course status and library rollups.

use crate::db::DbError;
use crate::db::VaultDb;
use crate::progress_persist::ProgressPersistenceError;
use crate::rollup::derive_course_status;
use crate::DerivedStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LibrarySummary {
    pub not_started: u32,
    pub in_progress: u32,
    pub done: u32,
    pub broken: u32,
}

impl LibrarySummary {
    fn record_course(&mut self, status: DerivedStatus) {
        match status {
            DerivedStatus::NotStarted => self.not_started += 1,
            DerivedStatus::InProgress => self.in_progress += 1,
            DerivedStatus::Done => self.done += 1,
            DerivedStatus::Broken => self.broken += 1,
        }
    }
}

pub fn course_progress(db: &VaultDb, course_id: &str) -> Result<DerivedStatus, DbError> {
    let statuses = db.lesson_statuses_for_course(course_id)?;
    Ok(derive_course_status(statuses))
}

pub fn library_summary(db: &VaultDb) -> Result<LibrarySummary, DbError> {
    let mut summary = LibrarySummary::default();
    for course_id in db.list_course_ids()? {
        summary.record_course(course_progress(db, &course_id)?);
    }
    Ok(summary)
}

pub fn refresh_sources(db: &VaultDb) -> Result<(), ProgressPersistenceError> {
    for source in db.local_lesson_sources()? {
        refresh_local_source(db, &source)?;
    }
    Ok(())
}

pub fn refresh_course_sources(
    db: &VaultDb,
    course_id: &str,
) -> Result<(), ProgressPersistenceError> {
    for source in db.local_lesson_sources_for_course(course_id)? {
        refresh_local_source(db, &source)?;
    }
    Ok(())
}

fn refresh_local_source(
    db: &VaultDb,
    source: &crate::db::LocalLessonSource,
) -> Result<(), ProgressPersistenceError> {
    let path = source.root_path.join(&source.relative_path);
    if !path.exists() {
        db.mark_source_missing(&source.lesson_id)?;
    }
    Ok(())
}

/// Before resuming local media playback, mark the lesson broken when its file is gone.
/// Does not mark the lesson done; preserves resume position in progress.
pub fn guard_local_media_for_resume(
    db: &VaultDb,
    lesson_id: &str,
) -> Result<crate::LessonStatus, ProgressPersistenceError> {
    use crate::LessonStatus;

    let progress = db.progress_for_lesson(lesson_id)?;
    if progress.status == LessonStatus::Broken {
        return Ok(LessonStatus::Broken);
    }
    let Some(source) = db.local_lesson_source_for_lesson(lesson_id)? else {
        return Ok(progress.status);
    };
    let path = source.root_path.join(&source.relative_path);
    if path.exists() {
        return Ok(progress.status);
    }
    db.mark_source_missing(lesson_id)?;
    Ok(LessonStatus::Broken)
}
