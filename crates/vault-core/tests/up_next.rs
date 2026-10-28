use vault_core::up_next::{up_next, UpNextError};
use vault_core::VaultDb;

fn three_lesson_course(db: &VaultDb) -> Vec<String> {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.mp4"), b"").unwrap();
    std::fs::write(dir.path().join("b.mp4"), b"").unwrap();
    std::fs::write(dir.path().join("c.mp4"), b"").unwrap();
    vault_core::import::import_folder(db, dir.path())
        .unwrap()
        .lesson_ids
}

#[test]
fn skips_done_lesson_in_accepted_order() {
    let db = VaultDb::open_in_memory().unwrap();
    let lesson_ids = three_lesson_course(&db);

    db.mark_complete(&lesson_ids[1]).unwrap();

    let next = up_next(&db, &lesson_ids[0]).unwrap().expect("third lesson");
    assert_eq!(next.lesson_id, lesson_ids[2]);

    assert!(up_next(&db, &lesson_ids[2]).unwrap().is_none());
}

#[test]
fn unknown_lesson_errors() {
    let db = VaultDb::open_in_memory().unwrap();
    let err = up_next(&db, "missing-lesson").unwrap_err();
    assert_eq!(err, UpNextError::LessonNotFound);
}
