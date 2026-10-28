//! Audio clip range selection and local encode for Anki cards.

use crate::clip_encoder::{validate_clip_range, ClipEncoder, ClipEncoderError};
use crate::cues::{list_player_cues, CueError, SubtitleCue};
use crate::db::VaultDb;

/// When the playhead is not inside a cue, clip the last fifteen seconds before it.
pub const PLAYHEAD_FALLBACK_CLIP_MS: i64 = 15_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioClipRange {
    pub start_ms: i64,
    pub end_ms: i64,
    pub active_cue_text: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum AudioClipError {
    #[error(transparent)]
    Cue(#[from] CueError),
    #[error(transparent)]
    Encoder(#[from] ClipEncoderError),
    #[error("lesson not found")]
    LessonNotFound,
}

impl PartialEq for AudioClipError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::LessonNotFound, Self::LessonNotFound) => true,
            (Self::Cue(left), Self::Cue(right)) => left == right,
            (Self::Encoder(left), Self::Encoder(right)) => left == right,
            _ => false,
        }
    }
}

pub fn audio_clip_range_ms(cues: &[SubtitleCue], playhead_ms: i64) -> (i64, i64) {
    let end_ms = playhead_ms.max(0);
    if let Some(cue) = active_cue_at(cues, playhead_ms) {
        return (cue.start_ms, cue.end_ms);
    }
    let start_ms = (end_ms - PLAYHEAD_FALLBACK_CLIP_MS).max(0);
    (start_ms, end_ms)
}

fn active_cue_at(cues: &[SubtitleCue], playhead_ms: i64) -> Option<&SubtitleCue> {
    cues.iter()
        .find(|cue| playhead_ms >= cue.start_ms && playhead_ms < cue.end_ms)
}

/// Resolve cue list, pick the clip window, and validate duration (≤ 60 s).
pub fn select_audio_clip_range(
    db: &VaultDb,
    lesson_id: &str,
    playhead_ms: i64,
) -> Result<AudioClipRange, AudioClipError> {
    let cues = list_player_cues(db, lesson_id)?;
    let (start_ms, end_ms) = audio_clip_range_ms(&cues, playhead_ms);
    validate_clip_range(start_ms, end_ms)?;
    let active_cue_text = active_cue_at(&cues, playhead_ms).map(|cue| cue.text.clone());
    Ok(AudioClipRange {
        start_ms,
        end_ms,
        active_cue_text,
    })
}

/// Encode audio from the lesson's on-disk media file (never from playlist URLs).
pub fn encode_lesson_audio_clip(
    db: &VaultDb,
    encoder: &dyn ClipEncoder,
    lesson_id: &str,
    start_ms: i64,
    end_ms: i64,
) -> Result<Vec<u8>, ClipEncoderError> {
    let Some(path) = db
        .local_media_path_for_lesson(lesson_id)
        .map_err(|_| ClipEncoderError::ClipEncoderUnavailable)?
    else {
        return Err(ClipEncoderError::ClipEncoderUnavailable);
    };
    if !path.is_file() {
        return Err(ClipEncoderError::ClipEncoderUnavailable);
    }
    encoder.encode_clip(&path, start_ms, end_ms)
}

pub fn ensure_local_clip_source(db: &VaultDb, lesson_id: &str) -> Result<(), ClipEncoderError> {
    let Some(path) = db
        .local_media_path_for_lesson(lesson_id)
        .map_err(|_| ClipEncoderError::ClipEncoderUnavailable)?
    else {
        return Err(ClipEncoderError::ClipEncoderUnavailable);
    };
    if path.is_file() {
        Ok(())
    } else {
        Err(ClipEncoderError::ClipEncoderUnavailable)
    }
}
