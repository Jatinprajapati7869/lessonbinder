use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};

use tempfile::tempdir;
use vault_core::import::{import_folder, rescan_with_cancel, ImportError, RescanError};
use vault_core::lesson_note::{create_lesson_note, list_lesson_notes};
use vault_core::order::{learning_order, reorder_lessons};
use vault_core::VaultDb;

#[test]
fn cancel_rescan_leaves_lesson_order_and_notes_unchanged() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("a.mp4"), b"").unwrap();
    fs::write(root.join("b.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let lesson_a = report.lesson_ids[0].clone();
    let lesson_b = report.lesson_ids[1].clone();

    reorder_lessons(
        &db,
        &report.course_id,
        &[lesson_b.clone(), lesson_a.clone()],
    )
    .unwrap();
    create_lesson_note(&db, &lesson_a, "Hold through cancel", Some(1_000)).unwrap();

    let order_before = learning_order(&db, &report.course_id).unwrap();
    let note_before = list_lesson_notes(&db, &lesson_a).unwrap()[0].text.clone();

    fs::write(root.join("extra.mp4"), b"").unwrap();

    let entry_checks = AtomicUsize::new(0);
    let should_cancel = || entry_checks.fetch_add(1, Ordering::Relaxed) > 0;

    let err = rescan_with_cancel(&db, &report.course_id, &should_cancel, None).unwrap_err();
    assert!(matches!(err, RescanError::Import(ImportError::Cancelled)));

    assert_eq!(
        learning_order(&db, &report.course_id).unwrap(),
        order_before
    );
    assert_eq!(
        list_lesson_notes(&db, &lesson_a).unwrap()[0].text,
        note_before
    );
    assert_eq!(
        db.lesson_ids_in_learning_order(&report.course_id)
            .unwrap()
            .len(),
        2
    );
}
