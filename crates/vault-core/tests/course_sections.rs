use std::fs;

use tempfile::tempdir;
use vault_core::course::course_detail;
use vault_core::import::import_folder;
use vault_core::order::{accept_proposal, propose_order};
use vault_core::read_model::course_progress;
use vault_core::DerivedStatus;
use vault_core::VaultDb;

#[test]
fn course_detail_groups_two_sections_in_lesson_order() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::create_dir(root.join("alpha")).unwrap();
    fs::create_dir(root.join("beta")).unwrap();
    fs::write(root.join("alpha").join("a1.mp4"), b"").unwrap();
    fs::write(root.join("alpha").join("a2.mp4"), b"").unwrap();
    fs::write(root.join("beta").join("b1.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let detail = course_detail(&db, &report.course_id).unwrap();

    assert_eq!(detail.lessons.len(), 3);
    assert_eq!(detail.lesson_groups.len(), 2);
    assert_eq!(detail.lesson_groups[0].section.as_deref(), Some("alpha"));
    assert_eq!(
        detail.lesson_groups[0]
            .lessons
            .iter()
            .map(|l| l.title.as_str())
            .collect::<Vec<_>>(),
        vec!["a1", "a2"],
    );
    assert_eq!(detail.lesson_groups[1].section.as_deref(), Some("beta"));
    assert_eq!(detail.lesson_groups[1].lessons.len(), 1);
    assert_eq!(detail.course_status, DerivedStatus::NotStarted);
    assert_eq!(
        course_progress(&db, &report.course_id).unwrap(),
        detail.course_status,
    );
}

#[test]
fn lesson_order_inside_section_follows_accepted_order() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::create_dir(root.join("alpha")).unwrap();
    fs::write(root.join("alpha").join("a1.mp4"), b"").unwrap();
    fs::write(root.join("alpha").join("a2.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let reversed = vec![report.lesson_ids[1].clone(), report.lesson_ids[0].clone()];
    let proposal_id = propose_order(&db, &report.course_id, &reversed).unwrap();
    accept_proposal(&db, &proposal_id).unwrap();

    let detail = course_detail(&db, &report.course_id).unwrap();
    assert_eq!(detail.lesson_groups.len(), 1);
    assert_eq!(
        detail.lesson_groups[0]
            .lessons
            .iter()
            .map(|l| l.lesson_id.as_str())
            .collect::<Vec<_>>(),
        vec![reversed[0].as_str(), reversed[1].as_str()],
    );
}
