//! Vault-side records linking lessons to Anki note ids (no Anki I/O).

use crate::db::DbError;
use crate::db::VaultDb;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkStatus {
    Active,
    Broken,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonCardLinkEntry {
    pub anki_note_id: i64,
    pub timestamp_ms: i64,
    pub front_preview: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnkiCardLinkRecord {
    pub id: String,
    /// `None` after the lesson row was removed (course delete); link row is retained as broken.
    pub lesson_id: Option<String>,
    pub timestamp_ms: i64,
    pub front_hash: String,
    pub front_preview: String,
    pub anki_note_id: i64,
    pub link_status: LinkStatus,
}

#[derive(Debug, thiserror::Error)]
pub enum AnkiLinkError {
    #[error(transparent)]
    Db(#[from] DbError),
    #[error("anki card link not found")]
    NotFound,
    #[error("duplicate card link for lesson, timestamp, and front hash")]
    DuplicateSelection,
}

pub fn insert_anki_card_link(
    db: &VaultDb,
    lesson_id: &str,
    timestamp_ms: i64,
    front_hash: &str,
    front_preview: &str,
    anki_note_id: i64,
) -> Result<String, AnkiLinkError> {
    db.insert_anki_card_link(
        lesson_id,
        timestamp_ms,
        front_hash,
        front_preview,
        anki_note_id,
    )
    .map_err(map_db_error)
}

pub fn list_lesson_card_links(
    db: &VaultDb,
    lesson_id: &str,
) -> Result<Vec<LessonCardLinkEntry>, AnkiLinkError> {
    db.list_lesson_card_links(lesson_id)
        .map_err(AnkiLinkError::from)
}

/// Remove the vault link row for this lesson and note. Does not touch the Anki collection.
pub fn unlink_card(db: &VaultDb, lesson_id: &str, anki_note_id: i64) -> Result<(), AnkiLinkError> {
    if db.delete_anki_card_link(lesson_id, anki_note_id)? {
        Ok(())
    } else {
        Err(AnkiLinkError::NotFound)
    }
}

pub fn anki_card_link_for_selection(
    db: &VaultDb,
    lesson_id: &str,
    timestamp_ms: i64,
    front_hash: &str,
) -> Result<Option<AnkiCardLinkRecord>, AnkiLinkError> {
    db.anki_card_link_for_selection(lesson_id, timestamp_ms, front_hash)
        .map_err(AnkiLinkError::from)
}

pub fn anki_card_link(db: &VaultDb, link_id: &str) -> Result<AnkiCardLinkRecord, AnkiLinkError> {
    db.anki_card_link_by_id(link_id)
        .map_err(map_db_error)?
        .ok_or(AnkiLinkError::NotFound)
}

/// Anki note ids linked from lessons in `course_id` (active vault links only).
pub fn anki_note_ids_for_course(
    db: &VaultDb,
    course_id: &str,
) -> Result<std::collections::HashSet<i64>, AnkiLinkError> {
    db.anki_note_ids_for_course(course_id)
        .map_err(AnkiLinkError::from)
}

fn map_db_error(error: DbError) -> AnkiLinkError {
    if is_unique_violation(&error) {
        AnkiLinkError::DuplicateSelection
    } else {
        AnkiLinkError::Db(error)
    }
}

fn is_unique_violation(error: &DbError) -> bool {
    match error {
        DbError::Sqlite(rusqlite::Error::SqliteFailure(code, _)) => {
            code.extended_code == rusqlite::ffi::SQLITE_CONSTRAINT_UNIQUE
                || code.code == rusqlite::ErrorCode::ConstraintViolation
        }
        _ => false,
    }
}

pub(crate) fn parse_link_status(value: &str) -> Result<LinkStatus, DbError> {
    match value {
        "active" => Ok(LinkStatus::Active),
        "broken" => Ok(LinkStatus::Broken),
        other => Err(DbError::InvalidData(format!(
            "unknown link_status: {other}"
        ))),
    }
}
