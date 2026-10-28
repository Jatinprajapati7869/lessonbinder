//! Local backup of `vault.db` and the AuraVault Anki profile (no cloud).

use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub const VAULT_DB_FILE: &str = "vault.db";
pub const ANKI_PROFILE_DIR: &str = "anki-profile";
const COLLECTION_FILE: &str = "collection.anki2";
const MEDIA_DB_FILE: &str = "collection.media.db";
const MEDIA_FOLDER: &str = "collection.media";

#[derive(Debug, thiserror::Error)]
pub enum BackupError {
    #[error("refuse to write backup into the live Anki profile directory")]
    RefusesLiveProfileDestination,
    #[error("vault.db is missing from app data")]
    VaultDbMissing,
    #[error("backup folder is missing vault.db or Anki collection database")]
    IncompleteBackupFolder,
    #[error("io error: {0}")]
    Io(String),
}

const ASIDE_DIR_PREFIX: &str = "auravault-aside-";

pub fn vault_db_path(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join(VAULT_DB_FILE)
}

pub fn anki_profile_dir(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join(ANKI_PROFILE_DIR)
}

/// Copy `vault.db` and the Anki profile tree into a new timestamped folder under `destination_parent`.
pub fn backup_to(app_data_dir: &Path, destination_parent: &Path) -> Result<PathBuf, BackupError> {
    let live_profile = anki_profile_dir(app_data_dir);
    if destination_targets_live_profile(destination_parent, &live_profile)? {
        return Err(BackupError::RefusesLiveProfileDestination);
    }

    let vault_src = vault_db_path(app_data_dir);
    if !vault_src.is_file() {
        return Err(BackupError::VaultDbMissing);
    }

    fs::create_dir_all(destination_parent).map_err(io_err)?;
    let backup_dir = destination_parent.join(backup_folder_name());
    fs::create_dir_all(&backup_dir).map_err(io_err)?;

    fs::copy(&vault_src, backup_dir.join(VAULT_DB_FILE)).map_err(io_err)?;

    let profile_dst = backup_dir.join(ANKI_PROFILE_DIR);
    copy_profile_tree(&live_profile, &profile_dst)?;

    Ok(backup_dir)
}

/// Replace live `vault.db` and Anki profile from `backup_folder` after copying the current copies aside.
pub fn restore_from(app_data_dir: &Path, backup_folder: &Path) -> Result<PathBuf, BackupError> {
    validate_backup_folder(backup_folder)?;

    let aside_dir = app_data_dir.join(aside_dir_name());
    fs::create_dir_all(&aside_dir).map_err(io_err)?;

    let live_vault = vault_db_path(app_data_dir);
    if live_vault.is_file() {
        fs::copy(&live_vault, aside_dir.join(VAULT_DB_FILE)).map_err(io_err)?;
    }

    let live_profile = anki_profile_dir(app_data_dir);
    if live_profile.exists() {
        copy_profile_tree(&live_profile, &aside_dir.join(ANKI_PROFILE_DIR))?;
    }

    fs::copy(backup_folder.join(VAULT_DB_FILE), &live_vault).map_err(io_err)?;

    if live_profile.exists() {
        fs::remove_dir_all(&live_profile).map_err(io_err)?;
    }
    copy_profile_tree(&backup_folder.join(ANKI_PROFILE_DIR), &live_profile)?;

    Ok(aside_dir)
}

fn validate_backup_folder(backup_folder: &Path) -> Result<(), BackupError> {
    let vault = backup_folder.join(VAULT_DB_FILE);
    let collection = backup_folder.join(ANKI_PROFILE_DIR).join(COLLECTION_FILE);
    if vault.is_file() && collection.is_file() {
        Ok(())
    } else {
        Err(BackupError::IncompleteBackupFolder)
    }
}

fn aside_dir_name() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{ASIDE_DIR_PREFIX}{secs}")
}

fn backup_folder_name() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("auravault-backup-{secs}")
}

fn destination_targets_live_profile(
    destination_parent: &Path,
    live_profile: &Path,
) -> Result<bool, BackupError> {
    let destination = normalize_path(destination_parent);
    let profile = normalize_path(live_profile);
    Ok(destination == profile || destination.starts_with(&profile))
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(part) => normalized.push(part),
            Component::ParentDir => {
                normalized.pop();
            }
            Component::RootDir | Component::Prefix(_) => normalized.push(component.as_os_str()),
        }
    }
    normalized
}

fn copy_profile_tree(src_profile: &Path, dst_profile: &Path) -> Result<(), BackupError> {
    fs::create_dir_all(dst_profile).map_err(io_err)?;
    copy_if_exists(
        &src_profile.join(COLLECTION_FILE),
        &dst_profile.join(COLLECTION_FILE),
    )?;
    copy_if_exists(
        &src_profile.join(MEDIA_DB_FILE),
        &dst_profile.join(MEDIA_DB_FILE),
    )?;
    let media_src = src_profile.join(MEDIA_FOLDER);
    if media_src.is_dir() {
        copy_dir_recursive(&media_src, &dst_profile.join(MEDIA_FOLDER))?;
    }
    Ok(())
}

fn copy_if_exists(src: &Path, dst: &Path) -> Result<(), BackupError> {
    if src.is_file() {
        fs::copy(src, dst).map_err(io_err)?;
    }
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), BackupError> {
    fs::create_dir_all(dst).map_err(io_err)?;
    for entry in fs::read_dir(src).map_err(io_err)? {
        let entry = entry.map_err(io_err)?;
        let path = entry.path();
        let target = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir_recursive(&path, &target)?;
        } else if path.is_file() {
            fs::copy(&path, &target).map_err(io_err)?;
        }
    }
    Ok(())
}

fn io_err(err: io::Error) -> BackupError {
    BackupError::Io(err.to_string())
}
