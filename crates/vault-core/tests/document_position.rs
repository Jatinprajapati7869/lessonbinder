use std::fs;

use tempfile::tempdir;
use vault_core::course::course_detail;
use vault_core::import::import_folder;
use vault_core::set_document_position;
use vault_core::{LessonStatus, VaultDb};

fn import_single_file(db: &VaultDb, root: &std::path::Path, name: &str) -> (String, String) {
    fs::write(root.join(name), b"sample content for offset tests").unwrap();
    let report = import_folder(db, root).unwrap();
    (report.course_id, report.lesson_ids[0].clone())
}

#[test]
fn markdown_position_round_trip() {
    let data = tempdir().unwrap();
    let db_path = data.path().join("vault.db");
    let course = tempdir().unwrap();
    let lesson_id = {
        let db = VaultDb::open(&db_path).unwrap();
        let (_, lesson_id) = import_single_file(&db, course.path(), "notes.md");
        set_document_position(&db, &lesson_id, 1_200).unwrap();
        lesson_id
    };

    let db = VaultDb::open(&db_path).unwrap();
    assert_eq!(db.document_position(&lesson_id).unwrap(), 1_200);
    assert_eq!(
        db.progress_for_lesson(&lesson_id).unwrap().status,
        LessonStatus::InProgress
    );
}

#[test]
fn pdf_position_round_trip() {
    let data = tempdir().unwrap();
    let db_path = data.path().join("vault.db");
    let course = tempdir().unwrap();
    let lesson_id = {
        let db = VaultDb::open(&db_path).unwrap();
        import_single_file(&db, course.path(), "slides.pdf").1
    };

    {
        let db = VaultDb::open(&db_path).unwrap();
        set_document_position(&db, &lesson_id, 3).unwrap();
    }

    let db = VaultDb::open(&db_path).unwrap();
    assert_eq!(db.document_position(&lesson_id).unwrap(), 3);
}

#[test]
fn document_position_does_not_mark_done() {
    let db = VaultDb::open_in_memory().unwrap();
    let course = tempdir().unwrap();
    let (_, lesson_id) = import_single_file(&db, course.path(), "readme.txt");

    set_document_position(&db, &lesson_id, 50).unwrap();
    assert_ne!(
        db.progress_for_lesson(&lesson_id).unwrap().status,
        LessonStatus::Done
    );
}

#[test]
fn document_position_survives_mark_incomplete() {
    let db = VaultDb::open_in_memory().unwrap();
    let course = tempdir().unwrap();
    let (_, lesson_id) = import_single_file(&db, course.path(), "chapter.md");

    set_document_position(&db, &lesson_id, 800).unwrap();
    db.mark_complete(&lesson_id).unwrap();
    db.mark_incomplete(&lesson_id).unwrap();

    assert_eq!(db.document_position(&lesson_id).unwrap(), 800);
    assert_eq!(
        db.progress_for_lesson(&lesson_id).unwrap().status,
        LessonStatus::InProgress
    );
}

#[test]
fn pdf_lesson_resumes_page_in_course_detail() {
    let db = VaultDb::open_in_memory().unwrap();
    let course = tempdir().unwrap();
    let (course_id, lesson_id) = import_single_file(&db, course.path(), "book.pdf");

    set_document_position(&db, &lesson_id, 7).unwrap();

    let detail = course_detail(&db, &course_id).unwrap();
    let lesson = detail
        .lessons
        .iter()
        .find(|l| l.lesson_id == lesson_id)
        .unwrap();
    assert_eq!(lesson.document_position, 7);
}
