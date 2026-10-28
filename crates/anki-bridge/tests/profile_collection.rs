use std::path::Path;

use anki_bridge::{
    anki_error_user_message, open_profile, read_stored_schema_version, LessonbinderProfilePaths,
};
use tempfile::tempdir;

/// `SCHEMA_MAX_VERSION` in pinned `anki-main` (`rslib/src/storage/upgrades/mod.rs`).
const LATEST_COLLECTION_SCHEMA: u8 = 18;

#[test]
fn opens_new_profile_and_creates_collection_files() {
    let dir = tempdir().unwrap();
    let paths = LessonbinderProfilePaths::in_dir(dir.path());

    let col = open_profile(dir.path()).unwrap();
    let ver = read_stored_schema_version(&col).unwrap();
    assert_eq!(ver, LATEST_COLLECTION_SCHEMA);

    assert!(paths.collection_path.is_file());
    assert!(paths.media_folder.is_dir());
    assert!(paths.media_db.is_file());
}

#[test]
fn reopens_existing_profile_and_reads_schema_version() {
    let dir = tempdir().unwrap();
    let paths = LessonbinderProfilePaths::in_dir(dir.path());

    {
        let col = open_profile(dir.path()).unwrap();
        assert_eq!(
            read_stored_schema_version(&col).unwrap(),
            LATEST_COLLECTION_SCHEMA
        );
    }

    assert!(Path::new(&paths.collection_path).is_file());

    let col = open_profile(dir.path()).unwrap();
    assert_eq!(
        read_stored_schema_version(&col).unwrap(),
        LATEST_COLLECTION_SCHEMA
    );
}

#[test]
fn open_profile_while_lock_held_returns_in_use_message() {
    let dir = tempdir().unwrap();
    let _first = open_profile(dir.path()).unwrap();
    match open_profile(dir.path()) {
        Err(e) => {
            let err = anki_error_user_message(e);
            assert!(
                err.contains("Lessonbinder Anki profile is in use"),
                "unexpected error: {err}"
            );
        }
        Ok(_) => panic!("expected profile-in-use error while lock is held"),
    }
}
