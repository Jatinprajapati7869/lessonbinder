use vault_core::anki_link::{
    anki_card_link, insert_anki_card_link, list_lesson_card_links, unlink_card, AnkiLinkError,
    LessonCardLinkEntry, LinkStatus,
};
use vault_core::VaultDb;

fn lesson_id(db: &VaultDb) -> String {
    let course_id = db
        .insert_course("Course", "/courses/anki-link-test")
        .unwrap();
    db.insert_local_file_lesson(&course_id, "clip.mp4", "clip", 0)
        .unwrap()
}

#[test]
fn insert_roundtrip() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = lesson_id(&db);

    let link_id = insert_anki_card_link(
        &db,
        &lesson_id,
        12_500,
        "front-hash-1",
        "What is X?",
        42_001,
    )
    .unwrap();
    let link = anki_card_link(&db, &link_id).unwrap();

    assert_eq!(link.lesson_id.as_deref(), Some(lesson_id.as_str()));
    assert_eq!(link.timestamp_ms, 12_500);
    assert_eq!(link.front_hash, "front-hash-1");
    assert_eq!(link.front_preview, "What is X?");
    assert_eq!(link.anki_note_id, 42_001);
    assert_eq!(link.link_status, LinkStatus::Active);
}

#[test]
fn list_lesson_card_links_orders_by_timestamp_ms() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = lesson_id(&db);

    insert_anki_card_link(&db, &lesson_id, 30_000, "hash-b", "Second capture", 2).unwrap();
    insert_anki_card_link(&db, &lesson_id, 10_000, "hash-a", "First capture", 1).unwrap();

    let links = list_lesson_card_links(&db, &lesson_id).unwrap();
    assert_eq!(
        links,
        vec![
            LessonCardLinkEntry {
                anki_note_id: 1,
                timestamp_ms: 10_000,
                front_preview: "First capture".into(),
            },
            LessonCardLinkEntry {
                anki_note_id: 2,
                timestamp_ms: 30_000,
                front_preview: "Second capture".into(),
            },
        ]
    );
}

#[test]
fn unlink_card_removes_vault_link_row() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = lesson_id(&db);

    insert_anki_card_link(&db, &lesson_id, 5_000, "hash-a", "Front A", 101).unwrap();

    unlink_card(&db, &lesson_id, 101).unwrap();
    assert!(list_lesson_card_links(&db, &lesson_id).unwrap().is_empty());

    let err = unlink_card(&db, &lesson_id, 101).unwrap_err();
    assert!(matches!(err, AnkiLinkError::NotFound));
}

#[test]
fn duplicate_selection_rejected() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_id = lesson_id(&db);

    insert_anki_card_link(&db, &lesson_id, 1_000, "same-hash", "", 1).unwrap();
    let err = insert_anki_card_link(&db, &lesson_id, 1_000, "same-hash", "", 2).unwrap_err();
    assert!(matches!(err, AnkiLinkError::DuplicateSelection));

    insert_anki_card_link(&db, &lesson_id, 1_000, "other-hash", "", 3).unwrap();
    insert_anki_card_link(&db, &lesson_id, 2_000, "same-hash", "", 4).unwrap();
}
