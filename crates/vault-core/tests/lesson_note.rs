use vault_core::lesson_note::{
    create_lesson_note, delete_lesson_note, list_lesson_notes, update_lesson_note, LessonNoteError,
};
use vault_core::VaultDb;

fn lesson_fixture(db: &VaultDb) -> (String, String) {
    let course_id = db.insert_course("Notes", "/courses/notes").unwrap();
    let lesson_id = db
        .insert_local_file_lesson(&course_id, "lecture.mp4", "Lecture", 0)
        .unwrap();
    (course_id, lesson_id)
}

#[test]
fn create_list_and_update_round_trip() {
    let db = VaultDb::open_in_memory().unwrap();
    let (_, lesson_id) = lesson_fixture(&db);

    let created = create_lesson_note(&db, &lesson_id, "First thought", Some(12_000)).unwrap();
    assert_eq!(created.lesson_id, lesson_id);
    assert_eq!(created.text, "First thought");
    assert_eq!(created.timestamp_ms, Some(12_000));
    assert!(!created.id.is_empty());

    let listed = list_lesson_notes(&db, &lesson_id).unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, created.id);

    let updated = update_lesson_note(&db, &created.id, "Revised thought", None).unwrap();
    assert_eq!(updated.text, "Revised thought");
    assert_eq!(updated.timestamp_ms, None);

    let after = list_lesson_notes(&db, &lesson_id).unwrap();
    assert_eq!(after[0].text, "Revised thought");
}

#[test]
fn delete_removes_note() {
    let db = VaultDb::open_in_memory().unwrap();
    let (_, lesson_id) = lesson_fixture(&db);
    let note = create_lesson_note(&db, &lesson_id, "Temporary", None).unwrap();

    delete_lesson_note(&db, &note.id).unwrap();
    assert!(list_lesson_notes(&db, &lesson_id).unwrap().is_empty());

    let err = delete_lesson_note(&db, &note.id).unwrap_err();
    assert!(matches!(err, LessonNoteError::NotFound));
}

#[test]
fn list_orders_by_timestamp_then_creation_time() {
    let db = VaultDb::open_in_memory().unwrap();
    let (_, lesson_id) = lesson_fixture(&db);

    let early_untimed = create_lesson_note(&db, &lesson_id, "untimed a", None).unwrap();
    let late_untimed = create_lesson_note(&db, &lesson_id, "untimed b", None).unwrap();
    let at_five = create_lesson_note(&db, &lesson_id, "at 5s", Some(5_000)).unwrap();
    let at_one = create_lesson_note(&db, &lesson_id, "at 1s", Some(1_000)).unwrap();

    let ids = list_lesson_notes(&db, &lesson_id)
        .unwrap()
        .into_iter()
        .map(|n| n.id)
        .collect::<Vec<_>>();

    assert_eq!(
        ids,
        vec![at_one.id, at_five.id, early_untimed.id, late_untimed.id]
    );
}

#[test]
fn delete_lesson_cascades_notes() {
    let db = VaultDb::open_in_memory().unwrap();
    let (_, lesson_id) = lesson_fixture(&db);
    create_lesson_note(&db, &lesson_id, "Keep until lesson goes", Some(0)).unwrap();
    create_lesson_note(&db, &lesson_id, "Second", None).unwrap();
    assert_eq!(list_lesson_notes(&db, &lesson_id).unwrap().len(), 2);

    db.delete_lesson(&lesson_id).unwrap();
    assert!(list_lesson_notes(&db, &lesson_id).unwrap().is_empty());
}

#[test]
fn create_rejects_unknown_lesson() {
    let db = VaultDb::open_in_memory().unwrap();
    let err = create_lesson_note(&db, "missing-lesson", "nope", None).unwrap_err();
    assert!(matches!(err, LessonNoteError::LessonNotFound));
}
