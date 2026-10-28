use vault_core::anki_link::{anki_card_link, insert_anki_card_link, LinkStatus};
use vault_core::card::lesson_exists;
use vault_core::roadmap::delete_course;
use vault_core::VaultDb;

fn course_with_link(db: &VaultDb) -> (String, String, String) {
    let course_id = db.insert_course("Course", "/courses/delete-test").unwrap();
    let lesson_id = db
        .insert_local_file_lesson(&course_id, "clip.mp4", "clip", 0)
        .unwrap();
    let link_id = insert_anki_card_link(db, &lesson_id, 9_000, "front-hash", "", 99_001).unwrap();
    (course_id, lesson_id, link_id)
}

#[test]
fn breaks_links_not_notes() {
    let db = VaultDb::open_in_memory().unwrap();
    let (course_id, lesson_id, link_id) = course_with_link(&db);

    delete_course(&db, &course_id).unwrap();

    let link = anki_card_link(&db, &link_id).unwrap();
    assert_eq!(link.link_status, LinkStatus::Broken);
    assert_eq!(link.anki_note_id, 99_001);
    assert_eq!(link.lesson_id, None);
    assert!(!lesson_exists(&db, &lesson_id).unwrap());
}
