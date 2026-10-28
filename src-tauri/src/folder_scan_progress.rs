use tauri::{AppHandle, Emitter};
use vault_core::import::FolderScanProgress;

pub const FOLDER_SCAN_PROGRESS_EVENT: &str = "folder_scan_progress";

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub struct FolderScanProgressEvent {
    pub seen: u32,
    pub planned: u32,
}

pub fn emit_folder_scan_progress(app: &AppHandle, progress: FolderScanProgress) {
    let _ = app.emit(
        FOLDER_SCAN_PROGRESS_EVENT,
        FolderScanProgressEvent {
            seen: progress.seen,
            planned: progress.planned,
        },
    );
}
