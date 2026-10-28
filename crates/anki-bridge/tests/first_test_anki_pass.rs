//! First-test Anki / study checklist against `/tmp/lessonbinder-first-test` and an isolated profile.

use std::path::Path;

use anki::prelude::NoteId;
use anki_bridge::{
    commit_card_draft, due_summary, get_note, next_review_card, open_profile, LessonbinderProfilePaths,
};
use vault_core::anki_link::{list_lesson_card_links, unlink_card};
use vault_core::card::{build_card_draft, card_draft_from_subtitle_cue, CardError};
use vault_core::import::import_folder;
use vault_core::ocr::{OcrError, TesseractOcrEngine};
use vault_core::VaultDb;

const FIXTURE_ROOT: &str = "/tmp/lessonbinder-first-test";

#[test]
fn first_test_anki_pass_walkthrough() {
    assert!(
        Path::new(FIXTURE_ROOT).is_dir(),
        "missing {FIXTURE_ROOT}; seed docs/FIRST-TEST.md § Fixture pass"
    );

    let profile = tempfile::tempdir().unwrap();
    let mut col = open_profile(profile.path()).unwrap();

    assert_eq!(next_review_card(col.collection_mut()).unwrap(), None);
    let empty_due = due_summary(&mut col).unwrap();
    assert_eq!(empty_due.new, 0);
    assert_eq!(empty_due.learning, 0);
    assert_eq!(empty_due.review, 0);

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, Path::new(FIXTURE_ROOT)).unwrap();
    let lesson_id = report.lesson_ids[0].clone();

    let engine = TesseractOcrEngine::without_tesseract();
    assert!(!engine.is_available());
    let ocr_err = build_card_draft(
        &db,
        &engine,
        &lesson_id,
        1_000,
        b"\x89PNG\r\n\0local-only",
        None,
        None,
    )
    .unwrap_err();
    assert!(matches!(ocr_err, CardError::Ocr(OcrError::OcrUnavailable)));
    assert_eq!(
        due_summary(&mut col).unwrap(),
        empty_due,
        "failed OCR must not create Anki notes"
    );

    let draft = card_draft_from_subtitle_cue(&db, &lesson_id, 0, "First cue").expect("cue draft");
    let paths = LessonbinderProfilePaths::in_dir(profile.path());
    let note_id = commit_card_draft(
        &mut col,
        &paths.media_folder,
        &db,
        &lesson_id,
        0,
        &draft,
        &[],
        None,
    )
    .expect("commit cue card");

    let after_commit = due_summary(&mut col).unwrap();
    assert_eq!(after_commit.new, 1);
    assert_eq!(after_commit.learning, 0);
    assert_eq!(after_commit.review, 0);

    unlink_card(&db, &lesson_id, note_id).expect("unlink vault link");
    assert!(list_lesson_card_links(&db, &lesson_id).unwrap().is_empty());

    let stored = get_note(&mut col, NoteId(note_id)).expect("note still in collection");
    assert_eq!(stored.front, "First cue");
}
