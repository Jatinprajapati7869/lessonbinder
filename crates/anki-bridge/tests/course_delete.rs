use anki::prelude::NoteId;
use anki_bridge::{commit_card_draft, get_note, open_profile, LessonbinderProfilePaths};
use tempfile::tempdir;
use vault_core::anki_link::{anki_card_link, LinkStatus};
use vault_core::card::build_card_draft;
use vault_core::ocr::FakeOcrEngine;
use vault_core::roadmap::delete_course;
use vault_core::VaultDb;

#[test]
fn breaks_links_not_notes() {
    let profile = tempdir().unwrap();
    let mut col = open_profile(profile.path()).unwrap();
    let paths = LessonbinderProfilePaths::in_dir(profile.path());

    let db = VaultDb::open_in_memory().unwrap();
    let course_id = db.insert_course("Course", "/courses/delete-test").unwrap();
    let lesson_id = db
        .insert_local_file_lesson(&course_id, "clip.mp4", "clip", 0)
        .unwrap();

    let png = b"\x89PNG\r\n\0course-delete-note";
    let engine = FakeOcrEngine::fixed("Front after delete", 1.0);
    let draft = build_card_draft(&db, &engine, &lesson_id, 1_000, png, None, None).unwrap();

    let note_id = commit_card_draft(
        &mut col,
        &paths.media_folder,
        &db,
        &lesson_id,
        1_000,
        &draft,
        png,
        None,
    )
    .unwrap();

    let link_id = vault_core::anki_link::anki_card_link_for_selection(
        &db,
        &lesson_id,
        1_000,
        &vault_core::card::hash_front_text(&draft.front),
    )
    .unwrap()
    .expect("link row")
    .id;

    delete_course(&db, &course_id).unwrap();

    let link = anki_card_link(&db, &link_id).unwrap();
    assert_eq!(link.link_status, LinkStatus::Broken);
    assert_eq!(link.anki_note_id, note_id);
    assert_eq!(link.lesson_id, None);

    let stored = get_note(&mut col, NoteId(note_id)).expect("note still in collection");
    assert_eq!(stored.front, "Front after delete");
}
