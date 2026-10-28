use anki_bridge::{
    answer_card_by_id, next_review_card, next_review_card_for_course_note_ids,
    note_study_back_text, open_profile, read_media, AnswerCardError, AuraVaultProfilePaths,
    ReviewAnswerButtons, ReviewCard, ReviewEase,
};
use serde::{Deserialize, Serialize};
use tauri::State;
use vault_core::anki_link::anki_note_ids_for_course;

use crate::app_state::AppState;
use crate::vault_paths::anki_profile_dir;

#[derive(Debug, Clone, Serialize)]
pub struct ReviewAnswerButtonsDto {
    pub again: bool,
    pub hard: bool,
    pub good: bool,
    pub easy: bool,
}

impl From<ReviewAnswerButtons> for ReviewAnswerButtonsDto {
    fn from(value: ReviewAnswerButtons) -> Self {
        Self {
            again: value.again,
            hard: value.hard,
            good: value.good,
            easy: value.easy,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ReviewCardDto {
    pub card_id: i64,
    pub note_id: i64,
    pub front_text: String,
    pub screenshot_filename: Option<String>,
    pub buttons: ReviewAnswerButtonsDto,
}

fn review_card_dto(card: ReviewCard) -> ReviewCardDto {
    let front_text = card.front_fields.first().cloned().unwrap_or_default();
    let screenshot_filename = card
        .front_fields
        .get(1)
        .filter(|name| !name.is_empty())
        .map(|s| s.to_string());
    ReviewCardDto {
        card_id: card.card_id.0,
        note_id: card.note_id.0,
        front_text,
        screenshot_filename,
        buttons: ReviewAnswerButtonsDto::from(card.buttons),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct NoteStudyBackDto {
    pub back: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewEaseDto {
    Again,
    Hard,
    Good,
    Easy,
}

impl From<ReviewEaseDto> for ReviewEase {
    fn from(value: ReviewEaseDto) -> Self {
        match value {
            ReviewEaseDto::Again => ReviewEase::Again,
            ReviewEaseDto::Hard => ReviewEase::Hard,
            ReviewEaseDto::Good => ReviewEase::Good,
            ReviewEaseDto::Easy => ReviewEase::Easy,
        }
    }
}

fn profile_paths(state: &AppState) -> AuraVaultProfilePaths {
    AuraVaultProfilePaths::in_dir(anki_profile_dir(&state.app_data_dir))
}

#[tauri::command]
pub fn review_next_card(state: State<AppState>) -> Result<Option<ReviewCardDto>, String> {
    let profile_dir = anki_profile_dir(&state.app_data_dir);
    let mut guarded = open_profile(&profile_dir).map_err(|e| e.to_string())?;
    let col = guarded.collection_mut();
    let next = next_review_card(col).map_err(|e| e.to_string())?;
    Ok(next.map(review_card_dto))
}

#[tauri::command]
pub fn review_next_card_for_course(
    state: State<AppState>,
    course_id: String,
) -> Result<Option<ReviewCardDto>, String> {
    state.with_vault(|vault| {
        let note_ids = anki_note_ids_for_course(vault, &course_id).map_err(|e| e.to_string())?;
        let profile_dir = anki_profile_dir(&state.app_data_dir);
        let mut guarded = open_profile(&profile_dir).map_err(|e| e.to_string())?;
        let col = guarded.collection_mut();
        let next =
            next_review_card_for_course_note_ids(col, &note_ids).map_err(|e| e.to_string())?;
        Ok(next.map(review_card_dto))
    })
}

#[tauri::command]
pub fn review_note_back(state: State<AppState>, note_id: i64) -> Result<NoteStudyBackDto, String> {
    let profile_dir = anki_profile_dir(&state.app_data_dir);
    let mut guarded = open_profile(&profile_dir).map_err(|e| e.to_string())?;
    let col = guarded.collection_mut();
    let back = note_study_back_text(col, note_id).map_err(|e| e.to_string())?;
    Ok(NoteStudyBackDto { back })
}

#[tauri::command]
pub fn review_answer_card(
    state: State<AppState>,
    card_id: i64,
    ease: ReviewEaseDto,
) -> Result<(), String> {
    let profile_dir = anki_profile_dir(&state.app_data_dir);
    let mut guarded = open_profile(&profile_dir).map_err(|e| e.to_string())?;
    let col = guarded.collection_mut();
    answer_card_by_id(col, card_id, ReviewEase::from(ease)).map_err(map_answer_error)
}

fn map_answer_error(err: AnswerCardError) -> String {
    err.to_string()
}

#[tauri::command]
pub fn review_read_media(state: State<AppState>, filename: String) -> Result<Vec<u8>, String> {
    let paths = profile_paths(&state);
    read_media(&paths.media_folder, &filename).map_err(|e| e.to_string())
}
