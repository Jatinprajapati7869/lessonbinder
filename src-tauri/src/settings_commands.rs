use serde::{Deserialize, Serialize};
use tauri::State;
use vault_core::vault_settings::{load_vault_settings, save_vault_settings, VaultSettings};

use crate::app_state::AppState;
use crate::vault_paths::anki_profile_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultSettingsDto {
    pub tesseract_language: String,
    pub transcription_model_path: String,
    pub anki_desktop_binary_path: String,
}

impl From<VaultSettings> for VaultSettingsDto {
    fn from(value: VaultSettings) -> Self {
        Self {
            tesseract_language: value.tesseract_language,
            transcription_model_path: value.transcription_model_path,
            anki_desktop_binary_path: value.anki_desktop_binary_path,
        }
    }
}

impl From<VaultSettingsDto> for VaultSettings {
    fn from(value: VaultSettingsDto) -> Self {
        Self {
            tesseract_language: value.tesseract_language,
            transcription_model_path: value.transcription_model_path,
            anki_desktop_binary_path: value.anki_desktop_binary_path,
        }
    }
}

#[tauri::command]
pub fn get_vault_settings(state: State<AppState>) -> Result<VaultSettingsDto, String> {
    state.with_vault(|vault| {
        load_vault_settings(vault)
            .map(VaultSettingsDto::from)
            .map_err(|e| e.to_string())
    })
}

#[tauri::command]
pub fn save_vault_settings_entry(
    state: State<AppState>,
    settings: VaultSettingsDto,
) -> Result<(), String> {
    state.with_vault(|vault| {
        save_vault_settings(vault, &VaultSettings::from(settings)).map_err(|e| e.to_string())?;
        Ok(())
    })
}

#[tauri::command]
pub fn get_anki_profile_path(state: State<AppState>) -> String {
    anki_profile_dir(&state.app_data_dir).display().to_string()
}
