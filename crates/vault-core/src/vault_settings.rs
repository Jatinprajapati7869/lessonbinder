//! User preferences stored in `vault.db` (not the Anki collection).

use std::path::Path;

use crate::db::DbError;
use crate::db::VaultDb;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VaultSettings {
    pub tesseract_language: String,
    pub transcription_model_path: String,
    pub anki_desktop_binary_path: String,
}

impl VaultSettings {
    pub fn transcription_enabled(&self) -> bool {
        !self.transcription_model_path.is_empty()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    #[error(transparent)]
    Db(#[from] DbError),
    #[error("transcription model path is not a file: {0}")]
    TranscriptionModelNotFound(String),
}

pub fn load_vault_settings(db: &VaultDb) -> Result<VaultSettings, SettingsError> {
    db.load_vault_settings().map_err(SettingsError::from)
}

pub fn save_vault_settings(db: &VaultDb, settings: &VaultSettings) -> Result<(), SettingsError> {
    validate_transcription_model_path(&settings.transcription_model_path)?;
    db.save_vault_settings(settings)
        .map_err(SettingsError::from)
}

fn validate_transcription_model_path(path: &str) -> Result<(), SettingsError> {
    if path.is_empty() {
        return Ok(());
    }
    if !Path::new(path).is_file() {
        return Err(SettingsError::TranscriptionModelNotFound(path.to_string()));
    }
    Ok(())
}
