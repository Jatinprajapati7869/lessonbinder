use std::path::Path;

use lessonbinder_tauri::backup::{backup_to, restore_from};
use lessonbinder_tauri::command_error::from_backup_error;
use lessonbinder_tauri::command_log::log_command_error;
use tauri::State;

use crate::app_state::AppState;

fn map_backup_failure(
    state: &AppState,
    command: &str,
    err: lessonbinder_tauri::backup::BackupError,
) -> String {
    let coded = from_backup_error(err);
    let _ = log_command_error(&state.app_data_dir, command, &coded.code, None);
    coded.to_invoke_string()
}

#[tauri::command]
pub fn backup_vault(state: State<AppState>, destination_parent: String) -> Result<String, String> {
    let backup_dir = backup_to(&state.app_data_dir, Path::new(&destination_parent))
        .map_err(|err| map_backup_failure(&state, "backup_vault", err))?;
    Ok(backup_dir.display().to_string())
}

#[tauri::command]
pub fn restore_vault(state: State<AppState>, backup_folder: String) -> Result<(), String> {
    state.close_vault()?;
    let result = restore_from(&state.app_data_dir, Path::new(&backup_folder));
    match result {
        Ok(_aside) => state.reopen_vault()?,
        Err(error) => {
            let _ = state.reopen_vault();
            return Err(map_backup_failure(&state, "restore_vault", error));
        }
    }
    Ok(())
}
