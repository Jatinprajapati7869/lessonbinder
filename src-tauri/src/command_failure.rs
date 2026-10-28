use lessonbinder_tauri::command_error::CommandError;
use lessonbinder_tauri::command_log::{log_unexpected_command_error, UNEXPECTED_COMMAND_ERROR_CODE};
use serde_json::Value;

use crate::app_state::AppState;

pub fn unexpected_command_failure(
    state: &AppState,
    command: &str,
    message: String,
    context: Option<Value>,
) -> String {
    let _ = log_unexpected_command_error(&state.app_data_dir, command, context);
    CommandError {
        code: UNEXPECTED_COMMAND_ERROR_CODE.to_string(),
        message,
    }
    .to_invoke_string()
}
