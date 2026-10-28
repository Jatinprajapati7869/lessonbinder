//! Persist lesson progress transitions to `vault.db`.

use crate::db::DbError;
use crate::db::LessonStatus;
use crate::db::VaultDb;
use crate::progress::apply_mark_complete;
use crate::progress::apply_mark_incomplete;
use crate::progress::apply_playback_starts;
use crate::progress::apply_set_document_position;
use crate::progress::apply_set_resume;
use crate::progress::apply_source_missing;
use crate::progress::LessonProgressState;
use crate::progress::ProgressError;
use crate::study_day::StudyDayError;
use crate::watched::union_watched_ranges;
use crate::watched::PlaybackRangeError;
use crate::watched::WatchedRange;

#[derive(Debug, thiserror::Error)]
pub enum ProgressPersistenceError {
    #[error(transparent)]
    Db(#[from] DbError),
    #[error("illegal progress transition from {from:?} via {action}")]
    IllegalTransition {
        from: LessonStatus,
        action: &'static str,
    },
    #[error("invalid playback range")]
    InvalidPlaybackRange,
    #[error(transparent)]
    StudyDay(#[from] StudyDayError),
}

impl From<ProgressError> for ProgressPersistenceError {
    fn from(value: ProgressError) -> Self {
        match value {
            ProgressError::IllegalTransition { from, action } => {
                Self::IllegalTransition { from, action }
            }
        }
    }
}

impl From<PlaybackRangeError> for ProgressPersistenceError {
    fn from(value: PlaybackRangeError) -> Self {
        match value {
            PlaybackRangeError::InvalidRange => Self::InvalidPlaybackRange,
        }
    }
}

/// Union `start_ms..end_ms` into stored watched ranges for the lesson. Does not change `done`.
pub fn set_document_position(
    db: &VaultDb,
    lesson_id: &str,
    position: i64,
) -> Result<(), ProgressPersistenceError> {
    db.set_document_position(lesson_id, position)
}

pub fn record_playback(
    db: &VaultDb,
    lesson_id: &str,
    start_ms: i64,
    end_ms: i64,
) -> Result<(), ProgressPersistenceError> {
    let existing = db.watched_ranges_for_lesson(lesson_id)?;
    let merged = union_watched_ranges(&existing, WatchedRange { start_ms, end_ms })?;
    db.replace_watched_ranges(lesson_id, &merged)?;
    Ok(())
}

impl VaultDb {
    pub fn resume_position(&self, lesson_id: &str) -> Result<i64, DbError> {
        Ok(self.progress_for_lesson(lesson_id)?.resume_position_ms)
    }

    pub fn set_resume_position(
        &self,
        lesson_id: &str,
        position_ms: i64,
    ) -> Result<(), ProgressPersistenceError> {
        let mut state = self.load_progress_state(lesson_id)?;
        apply_set_resume(&mut state, position_ms)?;
        self.save_lesson_progress_state(lesson_id, &state)?;
        self.set_progress_last_engaged_at(lesson_id, &progress_timestamp())?;
        Ok(())
    }

    pub fn document_position(&self, lesson_id: &str) -> Result<i64, DbError> {
        Ok(self.progress_for_lesson(lesson_id)?.document_position)
    }

    pub fn set_document_position(
        &self,
        lesson_id: &str,
        position: i64,
    ) -> Result<(), ProgressPersistenceError> {
        let mut state = self.load_progress_state(lesson_id)?;
        apply_set_document_position(&mut state, position)?;
        self.save_lesson_progress_state(lesson_id, &state)?;
        Ok(())
    }

    /// Playback tick: resume position, max reached, and in-progress when appropriate.
    pub fn record_playback_position(
        &self,
        lesson_id: &str,
        position_ms: i64,
    ) -> Result<(), ProgressPersistenceError> {
        let mut state = self.load_progress_state(lesson_id)?;
        apply_playback_starts(&mut state)?;
        apply_set_resume(&mut state, position_ms)?;
        if position_ms > state.max_position_reached_ms {
            state.max_position_reached_ms = position_ms;
        }
        self.save_lesson_progress_state(lesson_id, &state)?;
        self.set_progress_last_engaged_at(lesson_id, &progress_timestamp())?;
        Ok(())
    }

    pub fn mark_complete(&self, lesson_id: &str) -> Result<(), ProgressPersistenceError> {
        self.persist_mark_complete(lesson_id)?;
        crate::study_day::record_study_day_today(self)?;
        Ok(())
    }

    /// Marks complete and records a study day for `local_date` (tests and explicit dates).
    pub fn mark_complete_with_local_study_day(
        &self,
        lesson_id: &str,
        local_date: &str,
    ) -> Result<(), ProgressPersistenceError> {
        self.persist_mark_complete(lesson_id)?;
        crate::study_day::record_study_day(self, local_date)?;
        Ok(())
    }

    fn persist_mark_complete(&self, lesson_id: &str) -> Result<(), ProgressPersistenceError> {
        let mut state = self.load_progress_state(lesson_id)?;
        apply_mark_complete(&mut state)?;
        self.save_lesson_progress_state(lesson_id, &state)?;
        self.set_marked_done_at(lesson_id, Some(progress_timestamp()))?;
        Ok(())
    }

    pub fn mark_incomplete(&self, lesson_id: &str) -> Result<(), ProgressPersistenceError> {
        let mut state = self.load_progress_state(lesson_id)?;
        apply_mark_incomplete(&mut state)?;
        self.save_lesson_progress_state(lesson_id, &state)?;
        self.set_marked_done_at(lesson_id, None)?;
        Ok(())
    }

    pub fn mark_source_missing(&self, lesson_id: &str) -> Result<(), ProgressPersistenceError> {
        let mut state = self.load_progress_state(lesson_id)?;
        apply_source_missing(&mut state)?;
        self.save_lesson_progress_state(lesson_id, &state)?;
        self.set_marked_done_at(lesson_id, None)?;
        Ok(())
    }

    fn load_progress_state(&self, lesson_id: &str) -> Result<LessonProgressState, DbError> {
        let row = self.progress_for_lesson(lesson_id)?;
        Ok(LessonProgressState {
            status: row.status,
            resume_position_ms: row.resume_position_ms,
            max_position_reached_ms: row.max_position_reached_ms,
            document_position: row.document_position,
        })
    }
}

fn progress_timestamp() -> String {
    format!(
        "{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    )
}
