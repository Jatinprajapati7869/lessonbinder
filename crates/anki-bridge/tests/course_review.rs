use std::collections::HashSet;

use anki::prelude::NoteId;
use anki_bridge::{
    add_clip_note, next_review_card, next_review_card_for_course, open_profile, ClipDraft,
};
use vault_core::anki_link::{anki_note_ids_for_course, insert_anki_card_link};
use vault_core::VaultDb;

fn sample_draft(course: &str, front: &str) -> ClipDraft {
    ClipDraft {
        course: course.into(),
        lesson: "Lesson 1".into(),
        front: front.into(),
        back: "back".into(),
        source: "file:///lesson.mp4".into(),
        timestamp: "00:00:01".into(),
        screenshot: String::new(),
    }
}

fn two_linked_courses(
    db: &VaultDb,
    col: &mut anki::collection::Collection,
) -> (String, String, NoteId, NoteId) {
    let dir_a = tempfile::tempdir().unwrap();
    let dir_b = tempfile::tempdir().unwrap();
    let course_a = db
        .insert_course("Alpha", dir_a.path().to_str().unwrap())
        .unwrap();
    let course_b = db
        .insert_course("Beta", dir_b.path().to_str().unwrap())
        .unwrap();
    std::fs::write(dir_a.path().join("a.mp4"), b"").unwrap();
    std::fs::write(dir_b.path().join("b.mp4"), b"").unwrap();
    let lesson_a = db
        .insert_local_file_lesson(&course_a, "a.mp4", "Alpha lesson", 0)
        .unwrap();
    let lesson_b = db
        .insert_local_file_lesson(&course_b, "b.mp4", "Beta lesson", 0)
        .unwrap();

    let note_a = add_clip_note(col, sample_draft("Alpha", "Alpha front")).unwrap();
    let note_b = add_clip_note(col, sample_draft("Beta", "Beta front")).unwrap();

    insert_anki_card_link(db, &lesson_a, 0, "hash-alpha", "Alpha front", note_a.0).unwrap();
    insert_anki_card_link(db, &lesson_b, 0, "hash-beta", "Beta front", note_b.0).unwrap();

    (course_a, course_b, note_a, note_b)
}

fn allowed_notes(db: &VaultDb, course_id: &str) -> HashSet<NoteId> {
    anki_note_ids_for_course(db, course_id)
        .unwrap()
        .into_iter()
        .map(NoteId)
        .collect()
}

#[test]
fn each_course_gets_its_own_due_card_while_global_queue_keeps_both() {
    let db = VaultDb::open_in_memory().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let mut col = open_profile(dir.path()).unwrap();
    let (course_a, course_b, note_a, note_b) = two_linked_courses(&db, col.collection_mut());

    let from_a = next_review_card_for_course(col.collection_mut(), &allowed_notes(&db, &course_a))
        .unwrap()
        .expect("alpha due");
    assert_eq!(from_a.note_id, note_a);
    assert_eq!(from_a.front_fields[0], "Alpha front");

    let from_b = next_review_card_for_course(col.collection_mut(), &allowed_notes(&db, &course_b))
        .unwrap()
        .expect("beta due");
    assert_eq!(from_b.note_id, note_b);
    assert_eq!(from_b.front_fields[0], "Beta front");

    let global = next_review_card(col.collection_mut())
        .unwrap()
        .expect("global due");
    assert!(
        global.note_id == note_a || global.note_id == note_b,
        "global reviewer still sees Lessonbinder dues from any course"
    );
}

#[test]
fn course_filter_skips_other_course_due_ahead_in_queue() {
    let db = VaultDb::open_in_memory().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let mut col = open_profile(dir.path()).unwrap();
    let (course_a, course_b, note_a, note_b) = two_linked_courses(&db, col.collection_mut());

    let global_first = next_review_card(col.collection_mut())
        .unwrap()
        .expect("due")
        .note_id;

    let beta_only =
        next_review_card_for_course(col.collection_mut(), &allowed_notes(&db, &course_b))
            .unwrap()
            .expect("beta card");
    assert_eq!(beta_only.note_id, note_b);

    if global_first == note_a {
        assert_ne!(beta_only.note_id, note_a);
    } else {
        assert_eq!(global_first, note_b);
    }

    let _ = course_a;
}
