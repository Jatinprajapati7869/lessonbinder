use std::path::Path;

use anki_bridge::{open_profile, read_stored_schema_version, LessonbinderProfilePaths};
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
