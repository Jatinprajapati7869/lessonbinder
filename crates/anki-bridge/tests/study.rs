use anki::card::CardQueue;
use anki::prelude::CardId;
use anki::services::CardsService;
use anki::services::NotesService;
use anki_bridge::{
    add_clip_note, answer_card, due_summary, next_review_card, open_profile, start_review_session,
    AnswerCardError, ClipDraft, ReviewAnswerButtons, ReviewEase,
};
use anki_proto::cards::CardId as ProtoCardId;
use anki_proto::notes::NoteId as ProtoNoteId;

fn add_basic_card_in_deck(col: &mut anki::collection::Collection, deck_name: &str) -> CardId {
    let deck = col.get_or_create_normal_deck(deck_name).unwrap();
    let nt = col
        .get_notetype_by_name("Basic")
        .unwrap()
        .expect("Basic notetype");
    let mut note = nt.new_note();
    note.set_field(0, "outside").unwrap();
    note.set_field(1, "back").unwrap();
    col.add_note(&mut note, deck.id).unwrap();
    let card_ids = NotesService::cards_of_note(col, ProtoNoteId { nid: note.id.0 }).unwrap();
    CardId(card_ids.cids[0])
}

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
fn due_summary_empty() {
    let dir = tempfile::tempdir().unwrap();
    let mut col = open_profile(dir.path()).unwrap();

    let summary = due_summary(&mut col).unwrap();
    assert_eq!(summary.new, 0);
    assert_eq!(summary.learning, 0);
    assert_eq!(summary.review, 0);
}

/// Anki's deck tree counts never-studied cards in `new_count` (no review due date required).
#[test]
fn due_summary_after_new_note_increases_new_count_not_learning_or_review() {
    let dir = tempfile::tempdir().unwrap();
    let mut col = open_profile(dir.path()).unwrap();

    add_clip_note(&mut col, sample_draft("Linear Algebra")).unwrap();

    let summary = due_summary(&mut col).unwrap();
    assert_eq!(summary.new, 1);
    assert_eq!(summary.learning, 0);
    assert_eq!(summary.review, 0);
}

#[test]
fn next_review_card_empty_collection() {
    let dir = tempfile::tempdir().unwrap();
    let mut col = open_profile(dir.path()).unwrap();

    assert_eq!(next_review_card(col.collection_mut()).unwrap(), None);
}

#[test]
fn next_review_card_returns_lessonbinder_new_note() {
    let dir = tempfile::tempdir().unwrap();
    let mut col = open_profile(dir.path()).unwrap();

    let note_id = add_clip_note(col.collection_mut(), sample_draft("Linear Algebra")).unwrap();

    let review = next_review_card(col.collection_mut())
        .unwrap()
        .expect("due new card");
    assert_eq!(review.note_id, note_id);
    assert_eq!(
        review.front_fields,
        vec!["What is X?".to_string(), "clip-abc.png".to_string()]
    );
    assert_eq!(
        review.buttons,
        ReviewAnswerButtons {
            again: true,
            hard: true,
            good: true,
            easy: true,
        }
    );
}

#[test]
fn answer_card_rejects_wrong_queue_head() {
    let dir = tempfile::tempdir().unwrap();
    let mut col = open_profile(dir.path()).unwrap();
    add_clip_note(col.collection_mut(), sample_draft("Linear Algebra")).unwrap();

    let first = next_review_card(col.collection_mut())
        .unwrap()
        .expect("card");
    let err = answer_card(
        col.collection_mut(),
        CardId(first.card_id.0 + 999_999),
        ReviewEase::Good,
    )
    .unwrap_err();
    assert!(matches!(err, AnswerCardError::CardNotAtQueueHead));
}

#[test]
fn answer_card_good_then_next_is_other_card() {
    let dir = tempfile::tempdir().unwrap();
    let mut col = open_profile(dir.path()).unwrap();
    let col = col.collection_mut();

    add_clip_note(
        col,
        ClipDraft {
            course: "Linear Algebra".into(),
            lesson: "Lesson 1".into(),
            front: "First card".into(),
            back: "Y".into(),
            source: "file:///a.mp4".into(),
            timestamp: "00:00:01".into(),
            screenshot: "a.png".into(),
        },
    )
    .unwrap();
    add_clip_note(
        col,
        ClipDraft {
            course: "Linear Algebra".into(),
            lesson: "Lesson 2".into(),
            front: "Second card".into(),
            back: "Y".into(),
            source: "file:///b.mp4".into(),
            timestamp: "00:00:02".into(),
            screenshot: "b.png".into(),
        },
    )
    .unwrap();

    let first = next_review_card(col).unwrap().expect("first due");
    answer_card(col, first.card_id, ReviewEase::Good).unwrap();

    let second = next_review_card(col).unwrap().expect("second due");
    assert_ne!(second.card_id, first.card_id);
    assert_eq!(second.front_fields[0], "Second card");
}

#[test]
fn review_session_start_does_not_create_cards() {
    let dir = tempfile::tempdir().unwrap();
    let mut profile = open_profile(dir.path()).unwrap();
    let col = profile.collection_mut();
    add_clip_note(col, sample_draft("Linear Algebra")).unwrap();
    let before = due_summary(col).unwrap();

    let _session = start_review_session(col);

    assert_eq!(due_summary(col).unwrap(), before);
}

#[test]
fn next_card_from_lessonbinder_decks() {
    let dir = tempfile::tempdir().unwrap();
    let mut col = open_profile(dir.path()).unwrap();
    let col = col.collection_mut();

    let foreign_card = add_basic_card_in_deck(col, "Other");
    add_clip_note(col, sample_draft("Linear Algebra")).unwrap();

    let review = next_review_card(col)
        .unwrap()
        .expect("Lessonbinder due card while foreign deck also has a new card");
    assert_eq!(review.front_fields[0], "What is X?");

    let foreign_stored = CardsService::get_card(
        col,
        ProtoCardId {
            cid: foreign_card.0,
        },
    )
    .unwrap();
    assert_eq!(
        foreign_stored.queue,
        CardQueue::New as i32,
        "skip must not answer or bury foreign due cards"
    );
}

#[test]
fn answer_card_grades_lessonbinder_card_when_foreign_due_first() {
    let dir = tempfile::tempdir().unwrap();
    let mut col = open_profile(dir.path()).unwrap();
    let col = col.collection_mut();

    add_basic_card_in_deck(col, "Other");
    add_clip_note(col, sample_draft("Linear Algebra")).unwrap();

    let due = next_review_card(col).unwrap().expect("Lessonbinder card");
    answer_card(col, due.card_id, ReviewEase::Good).unwrap();
}

#[test]
fn review_session_rejects_card_outside_lessonbinder_prefix() {
    let dir = tempfile::tempdir().unwrap();
    let mut profile = open_profile(dir.path()).unwrap();
    let col = profile.collection_mut();
    let other_card = add_basic_card_in_deck(col, "Other");

    let mut session = start_review_session(col);
    let err = session
        .answer(col, other_card, ReviewEase::Good)
        .unwrap_err();
    assert!(matches!(err, AnswerCardError::CardNotInLessonbinderDecks));
    assert!(session.answers().is_empty());
}

#[test]
fn review_session_close_does_not_bury_or_suspend() {
    let dir = tempfile::tempdir().unwrap();
    let mut profile = open_profile(dir.path()).unwrap();
    let col = profile.collection_mut();
    add_clip_note(col, sample_draft("Linear Algebra")).unwrap();

    let card = next_review_card(col).unwrap().expect("due");
    let mut session = start_review_session(col);
    session.answer(col, card.card_id, ReviewEase::Good).unwrap();

    let answers = session.close();
    assert_eq!(answers.len(), 1);

    let stored = CardsService::get_card(
        col,
        ProtoCardId {
            cid: card.card_id.0,
        },
    )
    .unwrap();
    assert!(
        stored.queue != CardQueue::Suspended as i32
            && stored.queue != CardQueue::SchedBuried as i32
            && stored.queue != CardQueue::UserBuried as i32,
        "close must not bury or suspend"
    );
}
