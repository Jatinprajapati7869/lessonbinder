//! User preferences stored in `vault.db` (not the Anki collection).

use std::path::Path;

use crate::db::DbError;
use crate::db::VaultDb;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UiTheme {
    #[default]
    Light,
    Dark,
}

pub const PLAYBACK_SPEED_CHOICES: [f64; 5] = [0.75, 1.0, 1.25, 1.5, 2.0];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaybackSpeed(u8);

impl Default for PlaybackSpeed {
    fn default() -> Self {
        Self(1) // index of 1.0
    }
}

impl PlaybackSpeed {
    pub fn value(self) -> f64 {
        PLAYBACK_SPEED_CHOICES[usize::from(self.0)]
    }

    pub fn parse(raw: f64) -> Self {
        for (index, choice) in PLAYBACK_SPEED_CHOICES.iter().enumerate() {
            if (raw - choice).abs() < f64::EPSILON {
                return Self(u8::try_from(index).unwrap_or(1));
            }
        }
        PlaybackSpeed::default()
    }
}

impl UiTheme {
    pub fn as_str(self) -> &'static str {
        match self {
            UiTheme::Light => "light",
            UiTheme::Dark => "dark",
        }
    }

    pub fn parse(value: &str) -> Self {
        match value {
            "dark" => UiTheme::Dark,
            _ => UiTheme::Light,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VaultSettings {
    pub tesseract_language: String,
    pub transcription_model_path: String,
    pub anki_desktop_binary_path: String,
    pub ui_theme: UiTheme,
    pub playback_speed: PlaybackSpeed,
    pub player_volume: f64,
    pub player_muted: bool,
}

pub const DEFAULT_PLAYER_VOLUME: f64 = 1.0;

pub fn normalize_player_volume(raw: f64) -> f64 {
    if raw.is_finite() {
        raw.clamp(0.0, 1.0)
    } else {
        DEFAULT_PLAYER_VOLUME
    }
}

impl Default for VaultSettings {
    fn default() -> Self {
        Self {
            tesseract_language: "eng".to_string(),
            transcription_model_path: String::new(),
            anki_desktop_binary_path: "anki".to_string(),
            ui_theme: UiTheme::default(),
            playback_speed: PlaybackSpeed::default(),
            player_volume: DEFAULT_PLAYER_VOLUME,
            player_muted: false,
        }
    }
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
