use std::fs;
use std::path::Path;

use auravault_tauri::backup::{anki_profile_dir, backup_to, vault_db_path, BackupError};
use tempfile::tempdir;

fn write_live_layout(app_data: &Path) -> (String, String) {
    fs::create_dir_all(anki_profile_dir(app_data)).unwrap();
    fs::create_dir_all(anki_profile_dir(app_data).join("collection.media")).unwrap();

    let vault_marker = "vault-live-marker";
    let collection_marker = "collection-live-marker";
    fs::write(vault_db_path(app_data), vault_marker).unwrap();
    fs::write(
        anki_profile_dir(app_data).join("collection.anki2"),
        collection_marker,
    )
    .unwrap();
    fs::write(
        anki_profile_dir(app_data).join("collection.media.db"),
        "media-db-live",
    )
    .unwrap();
    fs::write(
        anki_profile_dir(app_data)
            .join("collection.media")
            .join("clip.jpg"),
        b"media-bytes",
    )
    .unwrap();
    (vault_marker.into(), collection_marker.into())
}

#[test]
fn backup_to_timestamped_folder_contains_both_databases_and_leaves_live_unchanged() {
    let app_data = tempdir().unwrap();
    let backups_parent = tempdir().unwrap();
    let (vault_marker, collection_marker) = write_live_layout(app_data.path());

    let backup_dir = backup_to(app_data.path(), backups_parent.path()).unwrap();
    assert!(backup_dir.is_dir());
    assert!(backup_dir.starts_with(backups_parent.path()));
    assert_ne!(backup_dir, anki_profile_dir(app_data.path()));

    assert_eq!(
        fs::read_to_string(backup_dir.join("vault.db")).unwrap(),
        vault_marker,
    );
    let profile_backup = backup_dir.join("anki-profile");
    assert_eq!(
        fs::read_to_string(profile_backup.join("collection.anki2")).unwrap(),
        collection_marker,
    );
    assert!(profile_backup.join("collection.media.db").is_file());
    assert!(profile_backup
        .join("collection.media")
        .join("clip.jpg")
        .is_file());

    assert_eq!(
        fs::read_to_string(vault_db_path(app_data.path())).unwrap(),
        vault_marker,
    );
    assert_eq!(
        fs::read_to_string(anki_profile_dir(app_data.path()).join("collection.anki2")).unwrap(),
        collection_marker,
    );
}

#[test]
fn backup_to_refuses_live_anki_profile_directory() {
    let app_data = tempdir().unwrap();
    write_live_layout(app_data.path());

    let err = backup_to(app_data.path(), &anki_profile_dir(app_data.path())).unwrap_err();
    assert!(matches!(err, BackupError::RefusesLiveProfileDestination));
}
