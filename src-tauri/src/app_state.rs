use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Mutex;

use vault_core::VaultDb;

use crate::mpv_commands::MpvPlaybackState;
use crate::vault_paths::{open_or_create_vault_db, vault_db_path};

pub struct AppState {
    pub vault: Mutex<Option<VaultDb>>,
    pub vault_open_error: Mutex<String>,
    pub app_data_dir: PathBuf,
    pub import_folder_cancel: AtomicBool,
    pub(crate) folder_scan_active: AtomicBool,
    pub mpv: MpvPlaybackState,
}

impl AppState {
    pub fn new(app_data_dir: PathBuf) -> Self {
        match open_or_create_vault_db(&app_data_dir) {
            Ok(vault) => Self {
                vault: Mutex::new(Some(vault)),
                vault_open_error: Mutex::new(String::new()),
                app_data_dir,
                import_folder_cancel: AtomicBool::new(false),
                folder_scan_active: AtomicBool::new(false),
                mpv: MpvPlaybackState::new(),
            },
            Err(error) => Self {
                vault: Mutex::new(None),
                vault_open_error: Mutex::new(error.to_string()),
                app_data_dir,
                import_folder_cancel: AtomicBool::new(false),
                folder_scan_active: AtomicBool::new(false),
                mpv: MpvPlaybackState::new(),
            },
        }
    }

    pub fn vault_db_path(&self) -> PathBuf {
        vault_db_path(&self.app_data_dir)
    }

    pub fn vault_open_error_message(&self) -> String {
        self.vault_open_error
            .lock()
            .map(|message| message.clone())
            .unwrap_or_else(|_| "vault db lock poisoned".to_string())
    }

    pub fn with_vault<R>(
        &self,
        f: impl FnOnce(&VaultDb) -> Result<R, String>,
    ) -> Result<R, String> {
        let guard = self
            .vault
            .lock()
            .map_err(|_| "vault db lock poisoned".to_string())?;
        match guard.as_ref() {
            Some(db) => f(db),
            None => Err(self.vault_open_error_message()),
        }
    }

    pub fn close_vault(&self) -> Result<(), String> {
        let mut guard = self
            .vault
            .lock()
            .map_err(|_| "vault db lock poisoned".to_string())?;
        *guard = None;
        Ok(())
    }

    pub fn reopen_vault(&self) -> Result<(), String> {
        let mut guard = self
            .vault
            .lock()
            .map_err(|_| "vault db lock poisoned".to_string())?;
        if guard.is_some() {
            return Ok(());
        }
        match open_or_create_vault_db(&self.app_data_dir) {
            Ok(db) => {
                *guard = Some(db);
                if let Ok(mut message) = self.vault_open_error.lock() {
                    message.clear();
                }
                Ok(())
            }
            Err(error) => {
                let message = error.to_string();
                if let Ok(mut stored) = self.vault_open_error.lock() {
                    *stored = message.clone();
                }
                Err(message)
            }
        }
    }

    pub fn retry_open_vault(&self) -> Result<(), String> {
        let mut guard = self
            .vault
            .lock()
            .map_err(|_| "vault db lock poisoned".to_string())?;
        if guard.is_some() {
            return Ok(());
        }
        match open_or_create_vault_db(&self.app_data_dir) {
            Ok(db) => {
                *guard = Some(db);
                if let Ok(mut message) = self.vault_open_error.lock() {
                    message.clear();
                }
                Ok(())
            }
            Err(error) => {
                let message = error.to_string();
                if let Ok(mut stored) = self.vault_open_error.lock() {
                    *stored = message.clone();
                }
                Err(message)
            }
        }
    }
}
