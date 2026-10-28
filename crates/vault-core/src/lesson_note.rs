//! Vault-only lesson notes (text + optional playback timestamp). Not stored in Anki.

use crate::card::lesson_exists;
use crate::db::{DbError, VaultDb};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonNote {
    pub id: String,
    pub lesson_id: String,
    pub text: String,
    pub timestamp_ms: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, thiserror::Error)]
pub enum LessonNoteError {
    #[error(transparent)]
    Db(#[from] DbError),
    #[error("lesson not found")]
    LessonNotFound,
    #[error("lesson note not found")]
    NotFound,
}

pub fn create_lesson_note(
    db: &VaultDb,
    lesson_id: &str,
    text: &str,
    timestamp_ms: Option<i64>,
) -> Result<LessonNote, LessonNoteError> {
    if !lesson_exists(db, lesson_id)? {
        return Err(LessonNoteError::LessonNotFound);
    }
    db.insert_lesson_note(lesson_id, text, timestamp_ms)
        .map(row_to_note)
        .map_err(LessonNoteError::from)
}

pub fn update_lesson_note(
    db: &VaultDb,
    note_id: &str,
    text: &str,
    timestamp_ms: Option<i64>,
) -> Result<LessonNote, LessonNoteError> {
    db.update_lesson_note(note_id, text, timestamp_ms)
        .map(row_to_note)
        .map_err(map_update_error)
}

pub fn delete_lesson_note(db: &VaultDb, note_id: &str) -> Result<(), LessonNoteError> {
    db.delete_lesson_note(note_id).map_err(map_delete_error)
}

pub fn list_lesson_notes(
    db: &VaultDb,
    lesson_id: &str,
) -> Result<Vec<LessonNote>, LessonNoteError> {
    db.list_lesson_notes(lesson_id)
        .map(|rows| rows.into_iter().map(row_to_note).collect())
        .map_err(LessonNoteError::from)
}

fn row_to_note(row: crate::db::LessonNoteRow) -> LessonNote {
    LessonNote {
        id: row.id,
        lesson_id: row.lesson_id,
        text: row.body,
        timestamp_ms: row.timestamp_ms,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}

fn map_update_error(error: DbError) -> LessonNoteError {
    if matches_not_found(&error) {
        LessonNoteError::NotFound
    } else {
        LessonNoteError::Db(error)
    }
}

fn map_delete_error(error: DbError) -> LessonNoteError {
    if matches_not_found(&error) {
        LessonNoteError::NotFound
    } else {
        LessonNoteError::Db(error)
    }
}

fn matches_not_found(error: &DbError) -> bool {
    matches!(
        error,
        DbError::InvalidData(message) if message == "lesson note not found"
    )
}
