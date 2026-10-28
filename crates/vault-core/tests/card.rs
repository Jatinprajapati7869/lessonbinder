use vault_core::card::{
    build_card_draft, card_draft_from_lesson_note, card_draft_from_subtitle_cue, CardError,
};
use vault_core::ocr::{FakeOcrEngine, OcrEngine, OcrError};
use vault_core::VaultDb;

fn lesson(db: &VaultDb) -> String {
    let course_id = db
        .insert_course("Linear Algebra", "/courses/linear")
        .unwrap();
    db.insert_local_file_lesson(&course_id, "lecture.mp4", "Intro clip", 0)
        .unwrap()
}

#[test]
fn subtitle_cue_draft_shape() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = lesson(&db);

    let draft =
        card_draft_from_subtitle_cue(&db, &lesson_id, 83_000, "Definition of a vector space")
            .unwrap();

    assert_eq!(draft.front, "Definition of a vector space");
    assert_eq!(draft.source_label, "subtitle");
    assert_eq!(draft.timestamp, "00:01:23");
    assert_eq!(draft.course, "Linear Algebra");
    assert_eq!(draft.lesson_title, "Intro clip");
    assert_eq!(draft.back, "Intro clip\nLinear Algebra\n00:01:23");
}

#[test]
fn lesson_note_draft_includes_timestamp_on_back_when_linked() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = lesson(&db);

    let draft =
        card_draft_from_lesson_note(&db, &lesson_id, "Important idea", Some(42_000)).unwrap();

    assert_eq!(draft.front, "Important idea");
    assert_eq!(draft.source_label, "lesson_note");
    assert_eq!(draft.timestamp, "00:00:42");
    assert_eq!(draft.back, "Intro clip\nLinear Algebra\n00:00:42");
}

#[test]
fn lesson_note_draft_omits_timestamp_line_when_unlinked() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = lesson(&db);

    let draft = card_draft_from_lesson_note(&db, &lesson_id, "Standalone note", None).unwrap();

    assert_eq!(draft.back, "Intro clip\nLinear Algebra");
    assert_eq!(draft.timestamp, "00:00:00");
}

#[test]
fn lesson_note_rejects_empty_text() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = lesson(&db);

    let err = card_draft_from_lesson_note(&db, &lesson_id, "  ", None).unwrap_err();
    assert_eq!(err, CardError::EmptyFront);
}

#[test]
fn subtitle_cue_rejects_empty_text() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = lesson(&db);

    let err = card_draft_from_subtitle_cue(&db, &lesson_id, 0, "   ").unwrap_err();
    assert_eq!(err, CardError::EmptyFront);
}

#[test]
fn draft_from_ocr() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = lesson(&db);
    let engine = FakeOcrEngine::fixed("OCR line", 0.91);

    let draft = build_card_draft(
        &db,
        &engine,
        &lesson_id,
        83_000,
        &[0x89, 0x50, 0x4e, 0x47],
        None,
        None,
    )
    .unwrap();

    assert_eq!(draft.course, "Linear Algebra");
    assert_eq!(draft.lesson_title, "Intro clip");
    assert_eq!(draft.front, "OCR line");
    assert_eq!(draft.back, "");
    assert_eq!(draft.source_label, "lecture.mp4");
    assert_eq!(draft.timestamp, "00:01:23");
}

#[test]
fn draft_uses_user_edits() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = lesson(&db);
    let engine = FakeOcrEngine::fixed("ignored", 1.0);

    let draft = build_card_draft(
        &db,
        &engine,
        &lesson_id,
        1_000,
        &[],
        Some("User front"),
        Some("User back"),
    )
    .unwrap();

    assert_eq!(draft.front, "User front");
    assert_eq!(draft.back, "User back");
}

#[test]
fn rejects_empty_front() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = lesson(&db);
    let engine = FakeOcrEngine::fixed("   ", 1.0);

    let err = build_card_draft(&db, &engine, &lesson_id, 0, &[], None, None).unwrap_err();
    assert_eq!(err, CardError::EmptyFront);

    let err = build_card_draft(&db, &engine, &lesson_id, 0, &[], Some(""), None).unwrap_err();
    assert_eq!(err, CardError::EmptyFront);
}

#[test]
fn ocr_errors_propagate() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = lesson(&db);
    let engine = UnavailableEngine;

    let err = build_card_draft(&db, &engine, &lesson_id, 0, &[], None, None).unwrap_err();
    assert!(matches!(err, CardError::Ocr(OcrError::OcrUnavailable)));
}

struct UnavailableEngine;

impl OcrEngine for UnavailableEngine {
    fn recognize(&self, _image: &[u8]) -> Result<vault_core::ocr::OcrResult, OcrError> {
        Err(OcrError::OcrUnavailable)
    }
}
