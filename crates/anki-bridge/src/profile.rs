//! Lessonbinder-owned Anki profile paths and collection open helpers.

use std::fs::{File, OpenOptions};
use std::io;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

use anki::collection::Collection;
use anki::collection::CollectionBuilder;
use anki::prelude::*;
use anki::storage::SchemaVersion;
use anki::sync::version::SyncVersion;
use anki_io::create_dir_all;
use fs2::FileExt;

/// Schema version (`col.ver`) for collections created by current pinned `anki` (see `docs/ANKI_PIN.md`).
pub const COLLECTION_SCHEMA_VERSION: u8 = 18;

const PROFILE_LOCK_FILE: &str = ".lessonbinder-collection.lock";

/// Standard layout under `{app_data}/anki-profile/`.
#[derive(Debug, Clone)]
pub struct LessonbinderProfilePaths {
    pub profile_dir: PathBuf,
    pub collection_path: PathBuf,
    pub media_folder: PathBuf,
    pub media_db: PathBuf,
}

impl LessonbinderProfilePaths {
    pub fn in_dir(profile_dir: impl Into<PathBuf>) -> Self {
        let profile_dir = profile_dir.into();
        let collection_path = profile_dir.join("collection.anki2");
        let media_folder = profile_dir.join("collection.media");
        let media_db = profile_dir.join("collection.media.db");
        Self {
            profile_dir,
            collection_path,
            media_folder,
            media_db,
        }
    }
}

/// Collection handle that holds an exclusive profile lock for the lifetime of the value.
pub struct GuardedProfile {
    col: Collection,
    #[allow(dead_code)]
    lock: File,
}

impl Deref for GuardedProfile {
    type Target = Collection;

    fn deref(&self) -> &Collection {
        &self.col
    }
}

impl DerefMut for GuardedProfile {
    fn deref_mut(&mut self) -> &mut Collection {
        &mut self.col
    }
}

impl GuardedProfile {
    pub fn collection_mut(&mut self) -> &mut Collection {
        &mut self.col
    }
}

/// Create profile directories and open (or create) the collection with desktop media paths.
///
/// Holds an exclusive lock on `{profile_dir}/.lessonbinder-collection.lock` until the returned
/// [`GuardedProfile`] is dropped, so only one in-app collection session runs at a time.
pub fn open_profile(profile_dir: impl AsRef<Path>) -> Result<GuardedProfile> {
    let paths = LessonbinderProfilePaths::in_dir(profile_dir.as_ref());
    create_dir_all(&paths.profile_dir)?;
    create_dir_all(&paths.media_folder)?;

    let lock_path = paths.profile_dir.join(PROFILE_LOCK_FILE);
    let lock = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&lock_path)
        .map_err(anki::error::AnkiError::from)?;
    if let Err(e) = lock.try_lock_exclusive() {
        if e.kind() == io::ErrorKind::WouldBlock {
            invalid_input!(
                "Lessonbinder Anki profile is in use (close Anki or wait for another operation to finish)"
            );
        }
        return Err(anki::error::AnkiError::from(e));
    }

    let col = CollectionBuilder::new(&paths.collection_path)
        .set_media_paths(&paths.media_folder, &paths.media_db)
        .build()?;
    // Ensure desktop media database exists (Anki creates it on first media access).
    col.media()?;
    Ok(GuardedProfile { col, lock })
}

/// Map Anki's public [`SchemaVersion`] enum to the on-disk `col.ver` integer.
pub fn schema_version_as_u8(ver: SchemaVersion) -> u8 {
    match ver {
        SchemaVersion::V11 => 11,
        SchemaVersion::V18 => 18,
    }
}

/// Read the stored collection schema version using Anki's public `Collection` API only (no SQL in this crate).
///
/// Anki upgrades collections to the schema required by the current sync protocol on open. After a successful
/// open, [`SyncVersion::latest`] reports that schema via [`SyncVersion::collection_schema`].
pub fn read_stored_schema_version(col: &Collection) -> Result<u8> {
    col.changes_since_open()?;
    Ok(schema_version_as_u8(
        SyncVersion::latest().collection_schema(),
    ))
}
