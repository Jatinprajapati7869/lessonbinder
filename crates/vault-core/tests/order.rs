use std::fs;

use tempfile::tempdir;
use vault_core::import::import_folder;
use vault_core::order::{
    accept_proposal, learning_order, propose_order, reject_proposal, InvalidProposalError,
    OrderError,
};
use vault_core::VaultDb;

fn three_lesson_course(db: &VaultDb) -> (tempfile::TempDir, String, Vec<String>) {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.mp4"), b"").unwrap();
    fs::write(dir.path().join("b.mp4"), b"").unwrap();
    fs::write(dir.path().join("c.mp4"), b"").unwrap();
    let report = import_folder(db, dir.path()).unwrap();
    (dir, report.course_id, report.lesson_ids)
}

#[test]
fn default_learning_order_matches_import_path_order() {
    let db = VaultDb::open_in_memory().unwrap();
    let (_, course_id, lesson_ids) = three_lesson_course(&db);
    assert_eq!(learning_order(&db, &course_id).unwrap(), lesson_ids);
}

#[test]
fn accept_applies() {
    let db = VaultDb::open_in_memory().unwrap();
    let (_, course_id, lesson_ids) = three_lesson_course(&db);
    let proposed = vec![
        lesson_ids[2].clone(),
        lesson_ids[0].clone(),
        lesson_ids[1].clone(),
    ];

    let proposal_id = propose_order(&db, &course_id, &proposed).unwrap();
    assert_eq!(learning_order(&db, &course_id).unwrap(), lesson_ids);

    accept_proposal(&db, &proposal_id).unwrap();
    assert_eq!(learning_order(&db, &course_id).unwrap(), proposed);
}

#[test]
fn reject_unchanged() {
    let db = VaultDb::open_in_memory().unwrap();
    let (_, course_id, lesson_ids) = three_lesson_course(&db);
    let before = learning_order(&db, &course_id).unwrap();
    let proposed = vec![
        lesson_ids[2].clone(),
        lesson_ids[1].clone(),
        lesson_ids[0].clone(),
    ];

    let proposal_id = propose_order(&db, &course_id, &proposed).unwrap();
    reject_proposal(&db, &proposal_id).unwrap();

    assert_eq!(learning_order(&db, &course_id).unwrap(), before);
}

#[test]
fn invalid_proposal_errors() {
    let db = VaultDb::open_in_memory().unwrap();
    let (_, course_id, lesson_ids) = three_lesson_course(&db);

    let omit = vec![lesson_ids[0].clone(), lesson_ids[1].clone()];
    assert!(matches!(
        propose_order(&db, &course_id, &omit).unwrap_err(),
        OrderError::InvalidProposal(InvalidProposalError::LessonSetMismatch)
    ));

    let duplicate = vec![
        lesson_ids[0].clone(),
        lesson_ids[0].clone(),
        lesson_ids[2].clone(),
    ];
    assert!(matches!(
        propose_order(&db, &course_id, &duplicate).unwrap_err(),
        OrderError::InvalidProposal(InvalidProposalError::DuplicateLesson)
    ));

    let other = tempdir().unwrap();
    fs::write(other.path().join("x.mp4"), b"").unwrap();
    let other_id = import_folder(&db, other.path()).unwrap().lesson_ids[0].clone();
    let foreign = vec![lesson_ids[0].clone(), lesson_ids[1].clone(), other_id];
    assert!(matches!(
        propose_order(&db, &course_id, &foreign).unwrap_err(),
        OrderError::InvalidProposal(InvalidProposalError::LessonSetMismatch)
    ));
}

#[test]
fn reorder_lessons_swaps_two_lessons_without_proposal() {
    use vault_core::order::reorder_lessons;

    let db = VaultDb::open_in_memory().unwrap();
    let (_, course_id, lesson_ids) = three_lesson_course(&db);
    let swapped = vec![
        lesson_ids[1].clone(),
        lesson_ids[0].clone(),
        lesson_ids[2].clone(),
    ];

    reorder_lessons(&db, &course_id, &swapped).unwrap();

    assert_eq!(learning_order(&db, &course_id).unwrap(), swapped);
    assert!(db.pending_order_proposal(&course_id).unwrap().is_none());
}

#[test]
fn reorder_lessons_rejects_duplicate_ids() {
    use vault_core::order::reorder_lessons;

    let db = VaultDb::open_in_memory().unwrap();
    let (_, course_id, lesson_ids) = three_lesson_course(&db);
    let duplicate = vec![
        lesson_ids[0].clone(),
        lesson_ids[0].clone(),
        lesson_ids[2].clone(),
    ];

    assert!(matches!(
        reorder_lessons(&db, &course_id, &duplicate).unwrap_err(),
        OrderError::InvalidProposal(InvalidProposalError::DuplicateLesson)
    ));
}

#[test]
fn rescan_after_reorder_keeps_order_and_appends_new_files() {
    use vault_core::import::rescan;
    use vault_core::order::reorder_lessons;

    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("a.mp4"), b"").unwrap();
    fs::write(root.join("b.mp4"), b"").unwrap();
    fs::write(root.join("c.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let ids = report.lesson_ids.clone();
    let custom = vec![ids[2].clone(), ids[0].clone(), ids[1].clone()];
    reorder_lessons(&db, &report.course_id, &custom).unwrap();

    fs::write(root.join("z-new.mp4"), b"").unwrap();
    rescan(&db, &report.course_id).unwrap();

    let order = learning_order(&db, &report.course_id).unwrap();
    assert_eq!(order.len(), 4);
    assert_eq!(order[0..3], custom[..]);
    assert_eq!(
        db.lesson_stable_keys_for_course(&report.course_id).unwrap(),
        vec!["c.mp4", "a.mp4", "b.mp4", "z-new.mp4"]
    );
}

#[test]
fn course_detail_keeps_reordered_lessons_after_rescan() {
    use vault_core::course::course_detail;
    use vault_core::import::rescan;
    use vault_core::order::reorder_lessons;

    let dir = tempdir().unwrap();
    fs::write(dir.path().join("intro.mp4"), b"").unwrap();
    fs::write(dir.path().join("second.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, dir.path()).unwrap();
    let swapped = vec![report.lesson_ids[1].clone(), report.lesson_ids[0].clone()];
    reorder_lessons(&db, &report.course_id, &swapped).unwrap();
    rescan(&db, &report.course_id).unwrap();

    let detail = course_detail(&db, &report.course_id).unwrap();
    assert_eq!(
        detail
            .lessons
            .iter()
            .map(|lesson| lesson.title.as_str())
            .collect::<Vec<_>>(),
        vec!["second", "intro"],
    );
    assert_eq!(
        detail.lesson_groups[0]
            .lessons
            .iter()
            .map(|lesson| lesson.title.as_str())
            .collect::<Vec<_>>(),
        vec!["second", "intro"],
    );
}
