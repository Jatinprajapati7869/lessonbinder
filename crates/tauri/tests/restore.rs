use std::fs;

use auravault_tauri::backup::{
    anki_profile_dir, backup_to, restore_from, vault_db_path, BackupError,
};
use tempfile::tempdir;
use vault_core::anki_link::{anki_card_link, insert_anki_card_link};
use vault_core::VaultDb;

const COLLECTION_MARKER: &str = "collection-backup-v1";
const KNOWN_LESSON_TITLE: &str = "Known lesson";
const KNOWN_NOTE_ID: i64 = 42_001;

fn seed_live_app_data(app_data: &std::path::Path) -> (String, String) {
    fs::create_dir_all(anki_profile_dir(app_data)).unwrap();
    let db = VaultDb::open(vault_db_path(app_data)).unwrap();
    let course_id = db
        .insert_course("Restore course", "/courses/restore")
        .unwrap();
    let lesson_id = db
        .insert_local_file_lesson(&course_id, "clip.mp4", KNOWN_LESSON_TITLE, 0)
        .unwrap();
    let link_id =
        insert_anki_card_link(&db, &lesson_id, 1_000, "front-hash", "", KNOWN_NOTE_ID).unwrap();
    drop(db);

    fs::write(
        anki_profile_dir(app_data).join("collection.anki2"),
        COLLECTION_MARKER,
    )
    .unwrap();
    (lesson_id, link_id)
}

#[test]
fn restore_from_replaces_live_data_and_preserves_lesson_and_note_id() {
    let app_data = tempdir().unwrap();
    let backups_parent = tempdir().unwrap();
    let (lesson_id, link_id) = seed_live_app_data(app_data.path());

    let backup_dir = backup_to(app_data.path(), backups_parent.path()).unwrap();

    fs::write(vault_db_path(app_data.path()), b"corrupt-vault").unwrap();
    fs::write(
        anki_profile_dir(app_data.path()).join("collection.anki2"),
        "collection-wiped",
    )
    .unwrap();

    let aside = restore_from(app_data.path(), &backup_dir).unwrap();
    assert!(aside.is_dir());

    let db = VaultDb::open(vault_db_path(app_data.path())).unwrap();
    assert_eq!(
        db.lesson_display_title(&lesson_id).unwrap(),
        KNOWN_LESSON_TITLE
    );
    let link = anki_card_link(&db, &link_id).unwrap();
    assert_eq!(link.anki_note_id, KNOWN_NOTE_ID);

    assert_eq!(
        fs::read_to_string(anki_profile_dir(app_data.path()).join("collection.anki2")).unwrap(),
        COLLECTION_MARKER,
    );
}

#[test]
fn restore_from_refuses_backup_missing_either_database() {
    let app_data = tempdir().unwrap();
    let backup_dir = tempdir().unwrap();
    fs::write(backup_dir.path().join("vault.db"), b"db").unwrap();

    let err = restore_from(app_data.path(), backup_dir.path()).unwrap_err();
    assert!(matches!(err, BackupError::IncompleteBackupFolder));
}
