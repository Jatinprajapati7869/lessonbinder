use anki_bridge::{
    commit_card_draft, get_clip_note, open_profile, CommitCardError, LessonbinderProfilePaths,
};
use serde::Serialize;
use tauri::State;
use vault_core::anki_link::anki_card_link_for_selection;
use vault_core::audio_clip::{
    encode_lesson_audio_clip, ensure_local_clip_source, select_audio_clip_range, AudioClipError,
};
use vault_core::card::{
    build_card_draft, card_draft_from_audio_clip, card_draft_from_edit,
    card_draft_from_lesson_note, card_draft_from_subtitle_cue, hash_front_text, CardDraft,
    CardError,
};
use vault_core::clip_encoder::{ClipEncoderError, FfmpegClipEncoder};
use vault_core::ocr::{OcrEngine, OcrError, TesseractOcrEngine};

use serde_json::json;

use crate::app_state::AppState;
use crate::command_failure::unexpected_command_failure;
use crate::vault_paths::anki_profile_dir;

#[derive(Debug, Clone, Serialize)]
pub struct CaptureOcrStatusDto {
    pub available: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct CardDraftDto {
    pub course: String,
    pub lesson_title: String,
    pub front: String,
    pub back: String,
    pub source_label: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommittedCardDto {
    pub duplicate: bool,
    pub note_id: i64,
    pub course: String,
    pub lesson_title: String,
    pub front: String,
    pub back: String,
    pub source_label: String,
    pub timestamp: String,
}

#[tauri::command]
pub fn capture_ocr_status() -> CaptureOcrStatusDto {
    let engine = TesseractOcrEngine::discover();
    CaptureOcrStatusDto {
        available: engine.is_available(),
    }
}

#[tauri::command]
pub fn build_capture_draft(
    state: State<AppState>,
    lesson_id: String,
    timestamp_ms: i64,
    png: Vec<u8>,
    edited_front: Option<String>,
    edited_back: Option<String>,
) -> Result<CardDraftDto, String> {
    state.with_vault(|vault| {
        let engine = TesseractOcrEngine::discover();
        let draft = build_card_draft(
            vault,
            &engine as &dyn OcrEngine,
            &lesson_id,
            timestamp_ms,
            &png,
            edited_front.as_deref(),
            edited_back.as_deref(),
        )
        .map_err(|err| map_card_err(&state, "build_capture_draft", &lesson_id, err))?;
        Ok(CardDraftDto::from(draft))
    })
}

#[tauri::command]
pub fn build_subtitle_cue_draft(
    state: State<AppState>,
    lesson_id: String,
    timestamp_ms: i64,
    cue_text: String,
) -> Result<CardDraftDto, String> {
    state.with_vault(|vault| {
        let draft = card_draft_from_subtitle_cue(vault, &lesson_id, timestamp_ms, &cue_text)
            .map_err(|err| map_card_err(&state, "build_subtitle_cue_draft", &lesson_id, err))?;
        Ok(CardDraftDto::from(draft))
    })
}

#[tauri::command]
pub fn build_lesson_note_draft(
    state: State<AppState>,
    lesson_id: String,
    note_text: String,
    timestamp_ms: Option<i64>,
) -> Result<CardDraftDto, String> {
    state.with_vault(|vault| {
        let draft = card_draft_from_lesson_note(vault, &lesson_id, &note_text, timestamp_ms)
            .map_err(|err| map_card_err(&state, "build_lesson_note_draft", &lesson_id, err))?;
        Ok(CardDraftDto::from(draft))
    })
}

#[tauri::command]
pub fn commit_lesson_note_card(
    state: State<AppState>,
    lesson_id: String,
    timestamp_ms: Option<i64>,
    front: String,
    back: String,
) -> Result<CommittedCardDto, String> {
    state.with_vault(|vault| {
        let commit_ms = timestamp_ms.unwrap_or(0);
        let draft = card_draft_from_edit(vault, &lesson_id, commit_ms, &front, &back)
            .map_err(|err| map_card_err(&state, "commit_lesson_note_card", &lesson_id, err))?;

        let front_hash = hash_front_text(&draft.front);
        let existing = anki_card_link_for_selection(vault, &lesson_id, commit_ms, &front_hash)
            .map_err(|e| e.to_string())?;

        let profile_dir = anki_profile_dir(&state.app_data_dir);
        let mut col = open_profile(&profile_dir).map_err(|e| e.to_string())?;
        let paths = LessonbinderProfilePaths::in_dir(&profile_dir);

        let note_id = commit_card_draft(
            &mut col,
            &paths.media_folder,
            vault,
            &lesson_id,
            commit_ms,
            &draft,
            &[],
            None,
        )
        .map_err(|e| map_commit_err(&state, "commit_lesson_note_card", &lesson_id, e))?;

        let note = get_clip_note(&mut col, note_id).map_err(|e| e.to_string())?;

        Ok(CommittedCardDto {
            duplicate: existing.is_some(),
            note_id,
            course: note.course,
            lesson_title: note.lesson,
            front: note.front,
            back: note.back,
            source_label: note.source,
            timestamp: note.timestamp,
        })
    })
}

#[tauri::command]
pub fn commit_subtitle_cue_card(
    state: State<AppState>,
    lesson_id: String,
    timestamp_ms: i64,
    cue_text: String,
) -> Result<CommittedCardDto, String> {
    state.with_vault(|vault| {
        let draft = card_draft_from_subtitle_cue(vault, &lesson_id, timestamp_ms, &cue_text)
            .map_err(|err| map_card_err(&state, "commit_subtitle_cue_card", &lesson_id, err))?;

        let front_hash = hash_front_text(&draft.front);
        let existing = anki_card_link_for_selection(vault, &lesson_id, timestamp_ms, &front_hash)
            .map_err(|e| e.to_string())?;

        let profile_dir = anki_profile_dir(&state.app_data_dir);
        let mut col = open_profile(&profile_dir).map_err(|e| e.to_string())?;
        let paths = LessonbinderProfilePaths::in_dir(&profile_dir);

        let note_id = commit_card_draft(
            &mut col,
            &paths.media_folder,
            vault,
            &lesson_id,
            timestamp_ms,
            &draft,
            &[],
            None,
        )
        .map_err(|e| map_commit_err(&state, "commit_subtitle_cue_card", &lesson_id, e))?;

        let note = get_clip_note(&mut col, note_id).map_err(|e| e.to_string())?;

        Ok(CommittedCardDto {
            duplicate: existing.is_some(),
            note_id,
            course: note.course,
            lesson_title: note.lesson,
            front: note.front,
            back: note.back,
            source_label: note.source,
            timestamp: note.timestamp,
        })
    })
}

#[tauri::command]
pub fn build_audio_clip_draft(
    state: State<AppState>,
    lesson_id: String,
    playhead_ms: i64,
) -> Result<CardDraftDto, String> {
    state.with_vault(|vault| {
        ensure_local_clip_source(vault, &lesson_id)
            .map_err(|_| "clip_encoder_unavailable".to_string())?;
        let selection = select_audio_clip_range(vault, &lesson_id, playhead_ms)
            .map_err(|err| map_audio_clip_err(&state, "build_audio_clip_draft", &lesson_id, err))?;
        let draft = card_draft_from_audio_clip(
            vault,
            &lesson_id,
            playhead_ms,
            selection.active_cue_text.as_deref(),
        )
        .map_err(|err| map_card_err(&state, "build_audio_clip_draft", &lesson_id, err))?;
        Ok(CardDraftDto::from(draft))
    })
}

#[tauri::command]
pub fn commit_audio_clip_card(
    state: State<AppState>,
    lesson_id: String,
    playhead_ms: i64,
    front: String,
    back: String,
) -> Result<CommittedCardDto, String> {
    state.with_vault(|vault| {
        ensure_local_clip_source(vault, &lesson_id)
            .map_err(|_| "clip_encoder_unavailable".to_string())?;
        let selection = select_audio_clip_range(vault, &lesson_id, playhead_ms)
            .map_err(|err| map_audio_clip_err(&state, "commit_audio_clip_card", &lesson_id, err))?;

        let draft = card_draft_from_edit(vault, &lesson_id, playhead_ms, &front, &back)
            .map_err(|err| map_card_err(&state, "commit_audio_clip_card", &lesson_id, err))?;

        let front_hash = hash_front_text(&draft.front);
        let existing = anki_card_link_for_selection(vault, &lesson_id, playhead_ms, &front_hash)
            .map_err(|e| e.to_string())?;

        let encoder = FfmpegClipEncoder::discover();
        let mp3 = encode_lesson_audio_clip(
            vault,
            &encoder,
            &lesson_id,
            selection.start_ms,
            selection.end_ms,
        )
        .map_err(|err| map_clip_encoder_err(&state, "commit_audio_clip_card", &lesson_id, err))?;

        let profile_dir = anki_profile_dir(&state.app_data_dir);
        let mut col = open_profile(&profile_dir).map_err(|e| e.to_string())?;
        let paths = LessonbinderProfilePaths::in_dir(&profile_dir);

        let note_id = commit_card_draft(
            &mut col,
            &paths.media_folder,
            vault,
            &lesson_id,
            playhead_ms,
            &draft,
            &[],
            Some(&mp3),
        )
        .map_err(|e| map_commit_err(&state, "commit_audio_clip_card", &lesson_id, e))?;

        let note = get_clip_note(&mut col, note_id).map_err(|e| e.to_string())?;

        Ok(CommittedCardDto {
            duplicate: existing.is_some(),
            note_id,
            course: note.course,
            lesson_title: note.lesson,
            front: note.front,
            back: note.back,
            source_label: note.source,
            timestamp: note.timestamp,
        })
    })
}

#[tauri::command]
pub fn commit_capture_card(
    state: State<AppState>,
    lesson_id: String,
    timestamp_ms: i64,
    png: Vec<u8>,
    front: String,
    back: String,
) -> Result<CommittedCardDto, String> {
    state.with_vault(|vault| {
        let draft = card_draft_from_edit(vault, &lesson_id, timestamp_ms, &front, &back)
            .map_err(|err| map_card_err(&state, "commit_capture_card", &lesson_id, err))?;

        let front_hash = hash_front_text(&draft.front);
        let existing = anki_card_link_for_selection(vault, &lesson_id, timestamp_ms, &front_hash)
            .map_err(|e| e.to_string())?;

        let profile_dir = anki_profile_dir(&state.app_data_dir);
        let mut col = open_profile(&profile_dir).map_err(|e| e.to_string())?;
        let paths = LessonbinderProfilePaths::in_dir(&profile_dir);

        let note_id = commit_card_draft(
            &mut col,
            &paths.media_folder,
            vault,
            &lesson_id,
            timestamp_ms,
            &draft,
            &png,
            None,
        )
        .map_err(|e| map_commit_err(&state, "commit_capture_card", &lesson_id, e))?;

        let note = get_clip_note(&mut col, note_id).map_err(|e| e.to_string())?;

        Ok(CommittedCardDto {
            duplicate: existing.is_some(),
            note_id,
            course: note.course,
            lesson_title: note.lesson,
            front: note.front,
            back: note.back,
            source_label: note.source,
            timestamp: note.timestamp,
        })
    })
}

impl From<CardDraft> for CardDraftDto {
    fn from(draft: CardDraft) -> Self {
        Self {
            course: draft.course,
            lesson_title: draft.lesson_title,
            front: draft.front,
            back: draft.back,
            source_label: draft.source_label,
            timestamp: draft.timestamp,
        }
    }
}

fn map_card_err(state: &AppState, command: &str, lesson_id: &str, err: CardError) -> String {
    match err {
        CardError::Ocr(OcrError::OcrUnavailable) => {
            "Tesseract is not installed. Install Tesseract on your system to capture text from video frames. Images are not sent anywhere else.".into()
        }
        other => unexpected_command_failure(
            state,
            command,
            other.to_string(),
            Some(json!({ "lesson_id": lesson_id })),
        ),
    }
}

fn map_commit_err(
    state: &AppState,
    command: &str,
    lesson_id: &str,
    err: CommitCardError,
) -> String {
    unexpected_command_failure(
        state,
        command,
        err.to_string(),
        Some(json!({ "lesson_id": lesson_id })),
    )
}

fn map_audio_clip_err(
    state: &AppState,
    command: &str,
    lesson_id: &str,
    err: AudioClipError,
) -> String {
    match err {
        AudioClipError::Encoder(ClipEncoderError::ClipEncoderUnavailable) => {
            "clip_encoder_unavailable".into()
        }
        other => unexpected_command_failure(
            state,
            command,
            other.to_string(),
            Some(json!({ "lesson_id": lesson_id })),
        ),
    }
}

fn map_clip_encoder_err(
    state: &AppState,
    command: &str,
    lesson_id: &str,
    err: ClipEncoderError,
) -> String {
    if err == ClipEncoderError::ClipEncoderUnavailable {
        return "clip_encoder_unavailable".into();
    }
    unexpected_command_failure(
        state,
        command,
        err.to_string(),
        Some(json!({ "lesson_id": lesson_id })),
    )
}
