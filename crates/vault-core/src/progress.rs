//! Lesson progress state machine (pure; no I/O).

use crate::LessonStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonProgressState {
    pub status: LessonStatus,
    pub resume_position_ms: i64,
    pub max_position_reached_ms: i64,
    /// PDF page (1-based) or text character offset, depending on lesson media kind.
    pub document_position: i64,
}

impl Default for LessonProgressState {
    fn default() -> Self {
        Self {
            status: LessonStatus::NotStarted,
            resume_position_ms: 0,
            max_position_reached_ms: 0,
            document_position: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgressError {
    IllegalTransition {
        from: LessonStatus,
        action: &'static str,
    },
}

/// Playback started (or continued) on a non-broken lesson.
pub fn apply_playback_starts(state: &mut LessonProgressState) -> Result<(), ProgressError> {
    match state.status {
        LessonStatus::Broken => Err(illegal(state.status, "playback_starts")),
        LessonStatus::Done => Ok(()),
        LessonStatus::NotStarted | LessonStatus::InProgress => {
            state.status = LessonStatus::InProgress;
            Ok(())
        }
    }
}

/// Persist resume position; may move `not_started` → `in_progress` when position > 0.
pub fn apply_set_resume(
    state: &mut LessonProgressState,
    position_ms: i64,
) -> Result<(), ProgressError> {
    if position_ms < 0 {
        return Err(illegal(state.status, "set_resume_negative"));
    }
    state.resume_position_ms = position_ms;
    if state.status == LessonStatus::NotStarted && position_ms > 0 {
        state.status = LessonStatus::InProgress;
    }
    Ok(())
}

/// Persist document read position; may move `not_started` → `in_progress` when position > 0.
pub fn apply_set_document_position(
    state: &mut LessonProgressState,
    position: i64,
) -> Result<(), ProgressError> {
    if position < 0 {
        return Err(illegal(state.status, "set_document_position_negative"));
    }
    state.document_position = position;
    if state.status == LessonStatus::NotStarted && position > 0 {
        state.status = LessonStatus::InProgress;
    }
    Ok(())
}

pub fn apply_mark_complete(state: &mut LessonProgressState) -> Result<(), ProgressError> {
    if state.status == LessonStatus::Broken {
        return Err(illegal(state.status, "mark_complete"));
    }
    state.status = LessonStatus::Done;
    Ok(())
}

pub fn apply_mark_incomplete(state: &mut LessonProgressState) -> Result<(), ProgressError> {
    if state.status != LessonStatus::Done {
        return Err(illegal(state.status, "mark_incomplete"));
    }
    state.status = if state.resume_position_ms > 0 || state.document_position > 0 {
        LessonStatus::InProgress
    } else {
        LessonStatus::NotStarted
    };
    Ok(())
}

pub fn apply_source_missing(state: &mut LessonProgressState) -> Result<(), ProgressError> {
    state.status = LessonStatus::Broken;
    Ok(())
}

/// After source is available again; prior status is not restored.
pub fn apply_source_restored(state: &mut LessonProgressState) -> Result<(), ProgressError> {
    if state.status != LessonStatus::Broken {
        return Err(illegal(state.status, "source_restored"));
    }
    state.status = LessonStatus::NotStarted;
    Ok(())
}

fn illegal(from: LessonStatus, action: &'static str) -> ProgressError {
    ProgressError::IllegalTransition { from, action }
}
