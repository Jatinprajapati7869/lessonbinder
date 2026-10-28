use anki::services::CardsService;
use anki::services::NotesService;
use anki_bridge::open_profile;
use anki_bridge::{
    add_clip_note, ensure_lessonbinder_clip_notetype, get_note, validate_lessonbinder_deck_name,
    ClipDraft, LESSONBINDER_CLIP_FIELDS, LESSONBINDER_CLIP_NOTETYPE, LESSONBINDER_DECK_PREFIX,
};
use tempfile::tempdir;

fn sample_draft(course: &str) -> ClipDraft {
    ClipDraft {
        course: course.into(),
        lesson: "Lesson 1".into(),
        front: "What is X?".into(),
        back: "Y".into(),
        source: "file:///course/lesson1.mp4".into(),
        timestamp: "00:01:23".into(),
        screenshot: "clip-abc.png".into(),
    }
}

#[test]
fn ensure_notetype_defines_lessonbinder_clip_fields() {
    let dir = tempdir().unwrap();
    let mut col = open_profile(dir.path()).unwrap();

    let nt = ensure_lessonbinder_clip_notetype(&mut col).unwrap();
    assert_eq!(nt.name, LESSONBINDER_CLIP_NOTETYPE);
    assert_eq!(
        nt.fields
            .iter()
            .map(|f| f.name.as_str())
            .collect::<Vec<_>>(),
        LESSONBINDER_CLIP_FIELDS
    );
}

#[test]
fn creates_note_in_prefixed_deck() {
    let dir = tempdir().unwrap();
    let mut col = open_profile(dir.path()).unwrap();

    let draft = sample_draft("Linear Algebra");
    let nid = add_clip_note(&mut col, draft).unwrap();
    assert!(nid.0 > 0);

    let expected_deck = format!("{}Linear Algebra", LESSONBINDER_DECK_PREFIX);
    validate_lessonbinder_deck_name(&expected_deck).unwrap();

    let stored = get_note(&mut col, nid).unwrap();
    assert_eq!(stored.course, "Linear Algebra");
    assert_eq!(stored.lesson, "Lesson 1");
    assert_eq!(stored.front, "What is X?");
    assert_eq!(stored.back, "Y");
    assert_eq!(stored.source, "file:///course/lesson1.mp4");
    assert_eq!(stored.timestamp, "00:01:23");
    assert_eq!(stored.screenshot, "clip-abc.png");

    let deck_id = col
        .get_deck_id(&expected_deck)
        .unwrap()
        .expect("deck should exist");
    let card_ids = NotesService::cards_of_note(
        col.collection_mut(),
        anki_proto::notes::NoteId { nid: nid.0 },
    )
    .unwrap();
    assert!(!card_ids.cids.is_empty());
    let card = CardsService::get_card(
        col.collection_mut(),
        anki_proto::cards::CardId {
            cid: card_ids.cids[0],
        },
    )
    .unwrap();
    assert_eq!(card.deck_id, deck_id.0);
}

#[test]
fn rejects_deck_outside_prefix() {
    assert!(validate_lessonbinder_deck_name("Default").is_err());
    assert!(validate_lessonbinder_deck_name("Lessonbinder::").is_err());
    assert!(validate_lessonbinder_deck_name("Lessonbinder::Course::Nested").is_err());
}
