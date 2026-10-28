use anki::prelude::NoteId;
use anki_bridge::{commit_card_draft, get_note, open_profile, screenshot_filename_for_bytes};
use tempfile::tempdir;
use vault_core::anki_link::{anki_card_link_for_selection, list_lesson_card_links, unlink_card};
use vault_core::card::{build_card_draft, hash_front_text};
use vault_core::ocr::FakeOcrEngine;
use vault_core::VaultDb;

fn fixture_lesson(db: &VaultDb) -> String {
    let course_id = db
        .insert_course("Linear Algebra", "/courses/linear")
        .unwrap();
    db.insert_local_file_lesson(&course_id, "lecture.mp4", "Intro clip", 0)
        .unwrap()
}

#[test]
fn commit_card_creates_note_and_link() {
    let profile = tempdir().unwrap();
    let mut col = open_profile(profile.path()).unwrap();
    let paths = anki_bridge::AuraVaultProfilePaths::in_dir(profile.path());

    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = fixture_lesson(&db);
    let png = b"\x89PNG\r\n\0fake-screenshot-bytes";
    let engine = FakeOcrEngine::fixed("What is a matrix?", 0.95);
    let draft = build_card_draft(
        &db,
        &engine,
        &lesson_id,
        83_000,
        png,
        None,
        Some("Definition on back"),
    )
    .unwrap();

    let note_id = commit_card_draft(
        &mut col,
        &paths.media_folder,
        &db,
        &lesson_id,
        83_000,
        &draft,
        png,
        None,
    )
    .unwrap();
    assert!(note_id > 0);

    let screenshot = screenshot_filename_for_bytes(png);
    assert!(paths.media_folder.join(&screenshot).is_file());

    let stored = get_note(&mut col, NoteId(note_id)).unwrap();
    assert_eq!(stored.course, "Linear Algebra");
    assert_eq!(stored.lesson, "Intro clip");
    assert_eq!(stored.front, "What is a matrix?");
    assert_eq!(stored.back, "Definition on back");
    assert_eq!(stored.screenshot, screenshot);

    let front_hash = hash_front_text(&draft.front);
    let link = anki_card_link_for_selection(&db, &lesson_id, 83_000, &front_hash)
        .unwrap()
        .expect("link row");
    assert_eq!(link.anki_note_id, note_id);
}

#[test]
fn duplicate_commit_same_note_id() {
    let profile = tempdir().unwrap();
    let mut col = open_profile(profile.path()).unwrap();
    let paths = anki_bridge::AuraVaultProfilePaths::in_dir(profile.path());

    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = fixture_lesson(&db);
    let png = b"\x89PNG\r\n\0duplicate-test";
    let engine = FakeOcrEngine::fixed("Same front", 1.0);
    let draft = build_card_draft(&db, &engine, &lesson_id, 5_000, png, None, None).unwrap();

    let first = commit_card_draft(
        &mut col,
        &paths.media_folder,
        &db,
        &lesson_id,
        5_000,
        &draft,
        png,
        None,
    )
    .unwrap();
    let second = commit_card_draft(
        &mut col,
        &paths.media_folder,
        &db,
        &lesson_id,
        5_000,
        &draft,
        png,
        None,
    )
    .unwrap();

    assert_eq!(first, second);
}

#[test]
fn unlink_card_keeps_note_in_temp_profile() {
    let profile = tempdir().unwrap();
    let mut col = open_profile(profile.path()).unwrap();
    let paths = anki_bridge::AuraVaultProfilePaths::in_dir(profile.path());

    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = fixture_lesson(&db);
    let png = b"\x89PNG\r\n\0unlink-test";
    let engine = FakeOcrEngine::fixed("Kept after unlink", 1.0);
    let draft = build_card_draft(&db, &engine, &lesson_id, 12_000, png, None, None).unwrap();

    let note_id = commit_card_draft(
        &mut col,
        &paths.media_folder,
        &db,
        &lesson_id,
        12_000,
        &draft,
        png,
        None,
    )
    .unwrap();

    unlink_card(&db, &lesson_id, note_id).unwrap();
    assert!(list_lesson_card_links(&db, &lesson_id).unwrap().is_empty());

    let stored = get_note(&mut col, NoteId(note_id)).unwrap();
    assert_eq!(stored.front, "Kept after unlink");
}
