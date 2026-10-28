use std::fs;
use std::path::{Path, PathBuf};

use vault_core::import::import_folder;
use vault_core::playlist::{import_playlist_definition, set_watched};
use vault_core::read_model::course_progress;
use vault_core::{DerivedStatus, LessonStatus, VaultDb};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/playlists")
        .join(name)
}

fn import_fixture_at_path(
    db: &VaultDb,
    source: &str,
    dest: &Path,
) -> vault_core::import::ImportReport {
    fs::copy(fixture(source), dest).unwrap();
    import_playlist_definition(db, dest).unwrap()
}

#[test]
fn import_from_fixture() {
    let db = VaultDb::open_in_memory().unwrap();
    let path = fixture("three_items.json");
    let report = import_playlist_definition(&db, &path).unwrap();

    assert_eq!(db.lesson_count_for_course(&report.course_id).unwrap(), 3);
    assert_eq!(
        db.course_source_kind(&report.course_id).unwrap(),
        "playlist"
    );
    assert_eq!(
        db.lesson_stable_keys_for_course(&report.course_id).unwrap(),
        vec!["item-alpha", "item-beta", "item-gamma"]
    );

    let gamma_id = db
        .lesson_id_for_stable_key(&report.course_id, "item-gamma")
        .unwrap();
    let progress = db.progress_for_lesson(&gamma_id).unwrap();
    assert_eq!(progress.status, LessonStatus::NotStarted);
}

#[test]
fn refresh_marks_missing_broken() {
    let db = VaultDb::open_in_memory().unwrap();
    let definition_path = tempfile::NamedTempFile::new().unwrap().into_temp_path();

    let first = import_fixture_at_path(&db, "three_items.json", &definition_path);
    let alpha_id = db
        .lesson_id_for_stable_key(&first.course_id, "item-alpha")
        .unwrap();
    db.set_resume_position(&alpha_id, 4_500).unwrap();

    let second = import_fixture_at_path(&db, "two_items.json", &definition_path);
    assert_eq!(first.course_id, second.course_id);
    assert_eq!(db.lesson_count_for_course(&first.course_id).unwrap(), 3);

    let gamma_id = db
        .lesson_id_for_stable_key(&first.course_id, "item-gamma")
        .unwrap();
    assert_eq!(
        db.progress_for_lesson(&gamma_id).unwrap().status,
        LessonStatus::Broken
    );
    assert_eq!(db.resume_position(&alpha_id).unwrap(), 4_500);

    assert_eq!(
        db.progress_for_lesson(
            &db.lesson_id_for_stable_key(&first.course_id, "item-beta")
                .unwrap()
        )
        .unwrap()
        .status,
        LessonStatus::NotStarted
    );
    assert_eq!(
        course_progress(&db, &first.course_id).unwrap(),
        DerivedStatus::InProgress
    );

    import_fixture_at_path(&db, "with_delta.json", &definition_path);
    assert_eq!(db.lesson_count_for_course(&first.course_id).unwrap(), 4);
    assert!(db
        .lesson_id_for_stable_key(&first.course_id, "item-delta")
        .is_ok());
    assert_eq!(
        db.progress_for_lesson(&gamma_id).unwrap().status,
        LessonStatus::Broken
    );
}

#[test]
fn watched_updates_progress() {
    let db = VaultDb::open_in_memory().unwrap();
    let report = import_playlist_definition(&db, &fixture("two_items.json")).unwrap();
    let item_id = db
        .playlist_item_id_for_external_id(&report.course_id, "item-alpha")
        .unwrap();

    set_watched(&db, &item_id, 12_000, false).unwrap();

    let lesson_id = db
        .lesson_id_for_stable_key(&report.course_id, "item-alpha")
        .unwrap();
    assert_eq!(db.resume_position(&lesson_id).unwrap(), 12_000);
    assert_eq!(
        db.progress_for_lesson(&lesson_id).unwrap().status,
        LessonStatus::InProgress
    );
    assert_eq!(
        db.course_source_kind(&report.course_id).unwrap(),
        "playlist"
    );
}

#[test]
fn rollup_in_progress_partial_done() {
    let db = VaultDb::open_in_memory().unwrap();
    let report = import_playlist_definition(&db, &fixture("two_items.json")).unwrap();
    let alpha_item = db
        .playlist_item_id_for_external_id(&report.course_id, "item-alpha")
        .unwrap();

    set_watched(&db, &alpha_item, 60_000, true).unwrap();

    assert_eq!(
        course_progress(&db, &report.course_id).unwrap(),
        DerivedStatus::InProgress
    );
    let beta_id = db
        .lesson_id_for_stable_key(&report.course_id, "item-beta")
        .unwrap();
    assert_eq!(
        db.progress_for_lesson(&beta_id).unwrap().status,
        LessonStatus::NotStarted
    );
}

#[test]
fn folder_import_unchanged_by_playlist_module() {
    let db = VaultDb::open_in_memory().unwrap();
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("only.mp4"), b"").unwrap();
    let report = import_folder(&db, dir.path()).unwrap();
    assert_eq!(db.course_source_kind(&report.course_id).unwrap(), "folder");
}
