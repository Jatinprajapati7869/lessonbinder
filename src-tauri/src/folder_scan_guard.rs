use std::sync::atomic::{AtomicBool, Ordering};

use lessonbinder_tauri::command_error::CommandError;

use crate::app_state::AppState;

pub const FOLDER_SCAN_BUSY_CODE: &str = "folder_scan_in_progress";

pub const FOLDER_SCAN_BUSY_MESSAGE: &str = "A folder scan is already running.";

pub fn folder_scan_busy_invoke_string() -> String {
    CommandError {
        code: FOLDER_SCAN_BUSY_CODE.to_string(),
        message: FOLDER_SCAN_BUSY_MESSAGE.to_string(),
    }
    .to_invoke_string()
}

#[derive(Debug)]
pub struct FolderScanGuard<'a> {
    active: &'a AtomicBool,
}

impl Drop for FolderScanGuard<'_> {
    fn drop(&mut self) {
        self.active.store(false, Ordering::Release);
    }
}

impl AppState {
    pub fn try_begin_folder_scan(&self) -> Result<FolderScanGuard<'_>, String> {
        if self
            .folder_scan_active
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            return Err(folder_scan_busy_invoke_string());
        }
        self.import_folder_cancel.store(false, Ordering::Relaxed);
        Ok(FolderScanGuard {
            active: &self.folder_scan_active,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use tempfile::tempdir;

    use super::*;
    use crate::app_state::AppState;

    #[test]
    fn rejects_second_scan_while_first_is_active() {
        let dir = tempdir().unwrap();
        let state = AppState::new(dir.path().to_path_buf());
        let first = state.try_begin_folder_scan().unwrap();
        let err = state
            .try_begin_folder_scan()
            .expect_err("second scan should be rejected");
        assert!(err.contains(FOLDER_SCAN_BUSY_MESSAGE));
        assert!(err.contains(FOLDER_SCAN_BUSY_CODE));
        drop(first);
        state.try_begin_folder_scan().unwrap();
    }

    #[test]
    fn first_scan_keeps_running_when_second_is_rejected() {
        let dir = tempdir().unwrap();
        let state = AppState::new(dir.path().to_path_buf());
        let _first = state.try_begin_folder_scan().unwrap();
        assert!(state.try_begin_folder_scan().is_err());
        assert!(state.folder_scan_active.load(Ordering::Acquire));
        thread::sleep(Duration::from_millis(10));
        assert!(state.folder_scan_active.load(Ordering::Acquire));
    }
}
