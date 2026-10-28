//! App data paths and vault.db open (testable without the webview).

use std::path::{Path, PathBuf};

use vault_core::db::DbError;
use vault_core::VaultDb;

pub const VAULT_DB_FILE: &str = "vault.db";
pub const ANKI_PROFILE_DIR: &str = "anki-profile";
pub const THUMBNAILS_DIR: &str = "thumbnails";

pub fn vault_db_path(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join(VAULT_DB_FILE)
}

pub fn anki_profile_dir(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join(ANKI_PROFILE_DIR)
}

pub fn thumbnails_dir(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join(THUMBNAILS_DIR)
}

/// Create `app_data_dir` if needed and open (or create) `vault.db`.
pub fn open_or_create_vault_db(app_data_dir: &Path) -> Result<VaultDb, DbError> {
    std::fs::create_dir_all(app_data_dir).map_err(|e| DbError::InvalidData(e.to_string()))?;
    VaultDb::open(vault_db_path(app_data_dir))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_vault_db_in_app_data_dir() {
        let dir = tempfile::tempdir().unwrap();
        let db = open_or_create_vault_db(dir.path()).unwrap();
        assert_eq!(db.applied_migration_count(), 24);
        assert!(vault_db_path(dir.path()).is_file());
    }
}
