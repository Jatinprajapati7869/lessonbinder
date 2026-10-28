//! Markdown export for vault lesson notes (local file only).

use crate::db::{DbError, VaultDb};
use crate::lesson_note::{list_lesson_notes, LessonNote, LessonNoteError};
use crate::order::learning_order;

pub fn lesson_notes_markdown(db: &VaultDb, lesson_id: &str) -> Result<String, LessonNoteError> {
    let notes = list_lesson_notes(db, lesson_id)?;
    Ok(format_lesson_notes_markdown(&notes))
}

pub fn course_notes_markdown(db: &VaultDb, course_id: &str) -> Result<String, DbError> {
    let lesson_ids = learning_order(db, course_id)?;
    let mut out = String::new();
    for lesson_id in lesson_ids {
        let notes = list_lesson_notes(db, &lesson_id).map_err(note_err_to_db)?;
        if notes.is_empty() {
            continue;
        }
        if !out.is_empty() {
            out.push('\n');
        }
        let title = db.lesson_display_title(&lesson_id)?;
        out.push_str("## ");
        out.push_str(title.trim());
        out.push_str("\n\n");
        out.push_str(format_lesson_notes_markdown(&notes).trim_end());
        out.push('\n');
    }
    Ok(out)
}

fn note_err_to_db(err: LessonNoteError) -> DbError {
    match err {
        LessonNoteError::Db(db) => db,
        LessonNoteError::LessonNotFound | LessonNoteError::NotFound => {
            DbError::InvalidData("lesson note export failed".into())
        }
    }
}

pub fn format_lesson_notes_markdown(notes: &[LessonNote]) -> String {
    let mut out = String::new();
    for (index, note) in notes.iter().enumerate() {
        if index > 0 {
            out.push('\n');
            out.push('\n');
        }
        out.push_str(note.text.trim());
        if let Some(timestamp_ms) = note.timestamp_ms {
            out.push_str(" (");
            out.push_str(&format_note_timestamp_mm_ss(timestamp_ms));
            out.push(')');
        }
    }
    if !out.is_empty() {
        out.push('\n');
    }
    out
}

fn format_note_timestamp_mm_ss(timestamp_ms: i64) -> String {
    let total_seconds = (timestamp_ms.max(0) / 1_000) as u64;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{minutes}:{seconds:02}")
}
