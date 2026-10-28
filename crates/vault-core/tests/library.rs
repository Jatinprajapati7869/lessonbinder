use std::fs;
use std::path::PathBuf;

use tempfile::tempdir;
use vault_core::import::import_folder;
use vault_core::library::{list_courses, CourseLibraryVisibility, CourseSourceKind};
use vault_core::playlist::{import_playlist_definition, set_watched};
use vault_core::{DerivedStatus, VaultDb};

fn playlist_fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/playlists")
        .join(name)
}

#[test]
fn list_mixed_sources() {
    let db = VaultDb::open_in_memory().unwrap();

    let folder = tempdir().unwrap();
    fs::write(folder.path().join("a.mp4"), b"").unwrap();
    let folder_report = import_folder(&db, folder.path()).unwrap();

    let playlist_report =
        import_playlist_definition(&db, &playlist_fixture("two_items.json")).unwrap();

    let courses = list_courses(&db, None, CourseLibraryVisibility::Active).unwrap();
    assert_eq!(courses.len(), 2);

    let folder_entry = courses
        .iter()
        .find(|c| c.course_id == folder_report.course_id)
        .unwrap();
    assert_eq!(folder_entry.source_kind, CourseSourceKind::Folder);
    assert_eq!(folder_entry.status, DerivedStatus::NotStarted);
    assert_eq!(folder_entry.remaining.remaining_ms, 0);
    assert_eq!(folder_entry.remaining.unknown_lessons, 1);

    let playlist_entry = courses
        .iter()
        .find(|c| c.course_id == playlist_report.course_id)
        .unwrap();
    assert_eq!(playlist_entry.source_kind, CourseSourceKind::Playlist);
    assert_eq!(playlist_entry.title, "Fixture Playlist");
    assert_eq!(playlist_entry.remaining.remaining_ms, 155_000);
    assert_eq!(playlist_entry.remaining.unknown_lessons, 0);
}

#[test]
fn filter_by_status() {
    let db = VaultDb::open_in_memory().unwrap();

    let folder = tempdir().unwrap();
    fs::write(folder.path().join("a.mp4"), b"").unwrap();
    let folder_id = import_folder(&db, folder.path()).unwrap().course_id;

    let playlist_id = import_playlist_definition(&db, &playlist_fixture("two_items.json"))
        .unwrap()
        .course_id;
    let alpha_item = db
        .playlist_item_id_for_external_id(&playlist_id, "item-alpha")
        .unwrap();
    set_watched(&db, &alpha_item, 60_000, true).unwrap();

    let not_started = list_courses(
        &db,
        Some(DerivedStatus::NotStarted),
        CourseLibraryVisibility::Active,
    )
    .unwrap();
    assert_eq!(not_started.len(), 1);
    assert_eq!(not_started[0].course_id, folder_id);

    let in_progress = list_courses(
        &db,
        Some(DerivedStatus::InProgress),
        CourseLibraryVisibility::Active,
    )
    .unwrap();
    assert_eq!(in_progress.len(), 1);
    assert_eq!(in_progress[0].course_id, playlist_id);
    assert_eq!(in_progress[0].source_kind, CourseSourceKind::Playlist);

    assert!(list_courses(
        &db,
        Some(DerivedStatus::Done),
        CourseLibraryVisibility::Active
    )
    .unwrap()
    .is_empty());
}
