//! Manual first-test walkthrough against `/tmp/lessonbinder-first-test` (see `docs/FIRST-TEST.md`).

use std::path::Path;

use vault_core::course::course_detail;
use vault_core::import::{import_folder, rescan};
use vault_core::order::{learning_order, reorder_lessons};
use vault_core::{LessonStatus, VaultDb};

const FIXTURE_ROOT: &str = "/tmp/lessonbinder-first-test";

#[test]
fn first_test_fixture_walkthrough() {
    let root = Path::new(FIXTURE_ROOT);
    assert!(
        root.is_dir(),
        "missing fixture dir {FIXTURE_ROOT}; run the seed commands in docs/FIRST-TEST.md"
    );

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).expect("import folder");
    assert_eq!(
        db.lesson_count_for_course(&report.course_id).unwrap(),
        2,
        "expected alpha.mp4 + notes.md; private-beta.mp4 ignored"
    );
    assert_eq!(
        db.lesson_stable_keys_for_course(&report.course_id).unwrap(),
        vec!["alpha.mp4", "notes.md"]
    );

    let ids = report.lesson_ids.clone();
    db.mark_complete(&ids[0]).expect("mark complete");
    assert_eq!(
        db.progress_for_lesson(&ids[0]).unwrap().status,
        LessonStatus::Done
    );

    let swapped = vec![ids[1].clone(), ids[0].clone()];
    reorder_lessons(&db, &report.course_id, &swapped).expect("move down (swap)");
    assert_eq!(learning_order(&db, &report.course_id).unwrap(), swapped);

    rescan(&db, &report.course_id).expect("rescan folder");
    assert_eq!(
        learning_order(&db, &report.course_id).unwrap(),
        swapped,
        "rescan must not revert user order"
    );
    let detail = course_detail(&db, &report.course_id).unwrap();
    assert_eq!(
        detail
            .lessons
            .iter()
            .map(|l| l.title.as_str())
            .collect::<Vec<_>>(),
        vec!["notes", "alpha"]
    );
}
