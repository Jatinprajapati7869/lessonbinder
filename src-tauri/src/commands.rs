use anki_bridge::due_summary;
use anki_bridge::open_profile;
use anki_bridge::DueSummary;
use auravault_tauri::study::{anki_launch_command, resolve_anki_binary, LaunchAnkiError};
use serde::Serialize;
use std::process::Command;
use tauri::State;

use crate::app_state::AppState;
use crate::vault_paths::anki_profile_dir;

#[derive(Debug, Clone, Serialize)]
pub struct DueSummaryDto {
    pub new: u32,
    pub learning: u32,
    pub review: u32,
}

impl From<DueSummary> for DueSummaryDto {
    fn from(value: DueSummary) -> Self {
        Self {
            new: value.new,
            learning: value.learning,
            review: value.review,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AnkiLaunchStatusDto {
    Available,
    Unavailable { message: String },
}

#[derive(Debug, Clone, Serialize)]
pub struct StudyBarDto {
    pub due: DueSummaryDto,
    pub anki: AnkiLaunchStatusDto,
}

#[derive(Debug, Clone, Serialize)]
pub struct VaultDbInfoDto {
    pub path: String,
    pub migration_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct VaultBootstrapDto {
    pub ready: bool,
    pub path: String,
    pub error: Option<String>,
}

#[tauri::command]
pub fn vault_bootstrap(state: State<AppState>) -> VaultBootstrapDto {
    let ready = state
        .vault
        .lock()
        .map(|guard| guard.is_some())
        .unwrap_or(false);
    VaultBootstrapDto {
        ready,
        path: state.vault_db_path().display().to_string(),
        error: if ready {
            None
        } else {
            Some(state.vault_open_error_message())
        },
    }
}

#[tauri::command]
pub fn retry_open_vault(state: State<AppState>) -> Result<VaultBootstrapDto, String> {
    state.retry_open_vault()?;
    Ok(vault_bootstrap(state))
}

#[tauri::command]
pub fn vault_db_info(state: State<AppState>) -> Result<VaultDbInfoDto, String> {
    let path = state.vault_db_path().display().to_string();
    state.with_vault(|vault| {
        Ok(VaultDbInfoDto {
            path,
            migration_count: vault.applied_migration_count(),
        })
    })
}

#[tauri::command]
pub fn study_bar_state(state: State<AppState>) -> Result<StudyBarDto, String> {
    let profile_dir = anki_profile_dir(&state.app_data_dir);
    let mut col = open_profile(&profile_dir).map_err(|e| e.to_string())?;
    let due = due_summary(&mut col).map_err(|e| e.to_string())?;
    let anki = match resolve_anki_binary() {
        Ok(_) => AnkiLaunchStatusDto::Available,
        Err(LaunchAnkiError::AnkiNotFound) => AnkiLaunchStatusDto::Unavailable {
            message:
                "Anki is not installed. Install Anki or check Settings for the executable path."
                    .into(),
        },
        Err(LaunchAnkiError::ProfileDirHasNoParent) => AnkiLaunchStatusDto::Unavailable {
            message: "anki profile path is invalid".into(),
        },
    };
    Ok(StudyBarDto {
        due: DueSummaryDto::from(due),
        anki,
    })
}

#[tauri::command]
pub fn launch_anki_profile(state: State<AppState>) -> Result<(), String> {
    let profile_dir = anki_profile_dir(&state.app_data_dir);
    let (program, args) = anki_launch_command(&profile_dir).map_err(|e| e.to_string())?;
    Command::new(program)
        .args(args)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn init_app_state(app_data_dir: std::path::PathBuf) -> AppState {
    AppState::new(app_data_dir)
}
