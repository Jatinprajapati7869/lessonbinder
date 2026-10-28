//! Card draft from lesson capture + OCR (no Anki writes).

use crate::db::DbError;
use crate::db::VaultDb;
use crate::ocr::{OcrEngine, OcrError};

#[derive(Debug, Clone)]
pub(crate) struct LessonCardContextRow {
    pub course_title: String,
    pub lesson_title: String,
    pub source_label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardDraft {
    pub course: String,
    pub lesson_title: String,
    pub front: String,
    pub back: String,
    pub source_label: String,
    /// Wall-clock position in the lesson, formatted `HH:MM:SS`.
    pub timestamp: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CardError {
    #[error(transparent)]
    Db(#[from] DbError),
    #[error("lesson not found")]
    LessonNotFound,
    #[error("card front must not be empty")]
    EmptyFront,
    #[error(transparent)]
    Ocr(#[from] OcrError),
}

impl PartialEq for CardError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::LessonNotFound, Self::LessonNotFound) => true,
            (Self::EmptyFront, Self::EmptyFront) => true,
            (Self::Ocr(left), Self::Ocr(right)) => left == right,
            _ => false,
        }
    }
}

pub fn lesson_exists(db: &VaultDb, lesson_id: &str) -> Result<bool, DbError> {
    Ok(db.lesson_card_context(lesson_id)?.is_some())
}

/// Build a draft for an audio clip at `playhead_ms` (cue text when inside a cue).
pub fn card_draft_from_audio_clip(
    db: &VaultDb,
    lesson_id: &str,
    playhead_ms: i64,
    cue_text: Option<&str>,
) -> Result<CardDraft, CardError> {
    let context = db
        .lesson_card_context(lesson_id)?
        .ok_or(CardError::LessonNotFound)?;

    let timestamp_ms = playhead_ms.max(0);
    let timestamp = format_timestamp_ms(timestamp_ms);
    let front = match cue_text.map(str::trim).filter(|s| !s.is_empty()) {
        Some(text) => text.to_string(),
        None => format!("Audio at {timestamp}"),
    };

    let back = format_subtitle_cue_back(&context.lesson_title, &context.course_title, &timestamp);

    Ok(CardDraft {
        course: context.course_title,
        lesson_title: context.lesson_title,
        front,
        back,
        source_label: "audio_clip".into(),
        timestamp,
    })
}

/// Build a draft from a subtitle cue (no OCR or frame capture).
pub fn card_draft_from_subtitle_cue(
    db: &VaultDb,
    lesson_id: &str,
    timestamp_ms: i64,
    cue_text: &str,
) -> Result<CardDraft, CardError> {
    let context = db
        .lesson_card_context(lesson_id)?
        .ok_or(CardError::LessonNotFound)?;

    let front = cue_text.trim().to_string();
    if front.is_empty() {
        return Err(CardError::EmptyFront);
    }

    let timestamp = format_timestamp_ms(timestamp_ms);
    let back = format_subtitle_cue_back(&context.lesson_title, &context.course_title, &timestamp);

    Ok(CardDraft {
        course: context.course_title,
        lesson_title: context.lesson_title,
        front,
        back,
        source_label: "subtitle".into(),
        timestamp,
    })
}

/// Build a draft from a lesson note (no OCR or Anki writes).
pub fn card_draft_from_lesson_note(
    db: &VaultDb,
    lesson_id: &str,
    note_text: &str,
    timestamp_ms: Option<i64>,
) -> Result<CardDraft, CardError> {
    let context = db
        .lesson_card_context(lesson_id)?
        .ok_or(CardError::LessonNotFound)?;

    let front = note_text.trim().to_string();
    if front.is_empty() {
        return Err(CardError::EmptyFront);
    }

    let timestamp = format_timestamp_ms(timestamp_ms.unwrap_or(0));
    let back = format_card_context_back(
        &context.lesson_title,
        &context.course_title,
        timestamp_ms.map(|_| timestamp.as_str()),
    );

    Ok(CardDraft {
        course: context.course_title,
        lesson_title: context.lesson_title,
        front,
        back,
        source_label: "lesson_note".into(),
        timestamp,
    })
}

fn format_subtitle_cue_back(lesson_title: &str, course: &str, timestamp: &str) -> String {
    format_card_context_back(lesson_title, course, Some(timestamp))
}

fn format_card_context_back(lesson_title: &str, course: &str, timestamp: Option<&str>) -> String {
    match timestamp {
        Some(ts) => format!("{lesson_title}\n{course}\n{ts}"),
        None => format!("{lesson_title}\n{course}"),
    }
}

/// Build a draft from user-edited fields (no OCR).
pub fn card_draft_from_edit(
    db: &VaultDb,
    lesson_id: &str,
    timestamp_ms: i64,
    front: &str,
    back: &str,
) -> Result<CardDraft, CardError> {
    let context = db
        .lesson_card_context(lesson_id)?
        .ok_or(CardError::LessonNotFound)?;

    let front = front.trim().to_string();
    if front.is_empty() {
        return Err(CardError::EmptyFront);
    }

    Ok(CardDraft {
        course: context.course_title,
        lesson_title: context.lesson_title,
        front,
        back: back.to_string(),
        source_label: context.source_label,
        timestamp: format_timestamp_ms(timestamp_ms),
    })
}

/// Build a draft: OCR supplies default front; back defaults to empty unless overridden.
pub fn build_card_draft(
    db: &VaultDb,
    engine: &dyn OcrEngine,
    lesson_id: &str,
    timestamp_ms: i64,
    image: &[u8],
    edited_front: Option<&str>,
    edited_back: Option<&str>,
) -> Result<CardDraft, CardError> {
    let context = db
        .lesson_card_context(lesson_id)?
        .ok_or(CardError::LessonNotFound)?;

    let ocr = engine.recognize(image)?;
    let front = match edited_front {
        Some(value) => value.trim().to_string(),
        None => ocr.text.trim().to_string(),
    };
    let back = edited_back
        .map(|value| value.to_string())
        .unwrap_or_default();

    if front.is_empty() {
        return Err(CardError::EmptyFront);
    }

    Ok(CardDraft {
        course: context.course_title,
        lesson_title: context.lesson_title,
        front,
        back,
        source_label: context.source_label,
        timestamp: format_timestamp_ms(timestamp_ms),
    })
}

/// Stable hash for deduplicating Anki card links for the same OCR/front text.
pub fn hash_front_text(front: &str) -> String {
    sha256_hex(front.as_bytes())
}

pub fn format_timestamp_ms(timestamp_ms: i64) -> String {
    let total_secs = timestamp_ms.max(0) / 1_000;
    let hours = total_secs / 3_600;
    let minutes = (total_secs % 3_600) / 60;
    let seconds = total_secs % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}
