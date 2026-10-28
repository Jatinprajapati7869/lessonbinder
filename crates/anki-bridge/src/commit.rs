//! Commit vault card drafts into the AuraVault Anki collection.

use std::path::Path;

use anki::collection::Collection;
use anki::prelude::*;
use sha2::{Digest, Sha256};
use vault_core::anki_link::{anki_card_link_for_selection, insert_anki_card_link};
use vault_core::card::{hash_front_text, lesson_exists, CardDraft};
use vault_core::db::VaultDb;

use crate::clip::{add_clip_note, ClipDraft};

#[derive(Debug, thiserror::Error)]
pub enum CommitCardError {
    #[error(transparent)]
    Anki(#[from] anki::error::AnkiError),
    #[error(transparent)]
    VaultDb(#[from] vault_core::db::DbError),
    #[error(transparent)]
    VaultLink(#[from] vault_core::anki_link::AnkiLinkError),
    #[error("lesson not found in vault")]
    LessonNotFound,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Copy PNG into Anki media (content-addressed name), create clip note, store vault link.
/// Re-committing the same lesson/timestamp/front hash returns the existing note id.
#[allow(clippy::too_many_arguments)]
pub fn commit_card_draft(
    col: &mut Collection,
    media_folder: &Path,
    db: &VaultDb,
    lesson_id: &str,
    timestamp_ms: i64,
    draft: &CardDraft,
    png: &[u8],
    audio_mp3: Option<&[u8]>,
) -> Result<i64, CommitCardError> {
    if !lesson_exists(db, lesson_id)? {
        return Err(CommitCardError::LessonNotFound);
    }

    let front_hash = hash_front_text(&draft.front);
    if let Some(existing) = anki_card_link_for_selection(db, lesson_id, timestamp_ms, &front_hash)?
    {
        return Ok(existing.anki_note_id);
    }

    let screenshot = compose_screenshot_field(media_folder, png, audio_mp3)?;
    let clip = ClipDraft {
        course: draft.course.clone(),
        lesson: draft.lesson_title.clone(),
        front: draft.front.clone(),
        back: draft.back.clone(),
        source: draft.source_label.clone(),
        timestamp: draft.timestamp.clone(),
        screenshot,
    };
    let note_id = add_clip_note(col, clip)?;
    insert_anki_card_link(
        db,
        lesson_id,
        timestamp_ms,
        &front_hash,
        &draft.front,
        note_id.0,
    )?;
    Ok(note_id.0)
}

pub fn screenshot_filename_for_bytes(png: &[u8]) -> String {
    format!("{:x}.png", Sha256::digest(png))
}

pub fn audio_filename_for_bytes(mp3: &[u8]) -> String {
    format!("{:x}.mp3", Sha256::digest(mp3))
}

fn compose_screenshot_field(
    media_folder: &Path,
    png: &[u8],
    audio_mp3: Option<&[u8]>,
) -> Result<String, CommitCardError> {
    let mut parts = Vec::new();
    if !png.is_empty() {
        let name = store_screenshot(media_folder, png)?;
        if !name.is_empty() {
            parts.push(name);
        }
    }
    if let Some(mp3) = audio_mp3 {
        if !mp3.is_empty() {
            let name = store_audio_clip(media_folder, mp3)?;
            parts.push(format!("[sound:{name}]"));
        }
    }
    Ok(parts.join("\n"))
}

fn store_screenshot(media_folder: &Path, png: &[u8]) -> Result<String, CommitCardError> {
    if png.is_empty() {
        return Ok(String::new());
    }
    let filename = screenshot_filename_for_bytes(png);
    let path = media_folder.join(&filename);
    if !path.is_file() {
        std::fs::write(&path, png)?;
    }
    Ok(filename)
}

fn store_audio_clip(media_folder: &Path, mp3: &[u8]) -> Result<String, CommitCardError> {
    let filename = audio_filename_for_bytes(mp3);
    let path = media_folder.join(&filename);
    if !path.is_file() {
        std::fs::write(&path, mp3)?;
    }
    Ok(filename)
}
