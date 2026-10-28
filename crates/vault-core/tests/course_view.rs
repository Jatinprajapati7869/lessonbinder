use std::fs;

use tempfile::tempdir;
use vault_core::course::course_detail;
use vault_core::import::import_folder;
use vault_core::order::propose_order;
use vault_core::DerivedStatus;
use vault_core::VaultDb;

#[test]
fn course_detail_includes_pending_proposal() {
    let db = VaultDb::open_in_memory().unwrap();
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("a.mp4"), b"").unwrap();
    fs::write(dir.path().join("b.mp4"), b"").unwrap();
    let report = import_folder(&db, dir.path()).unwrap();
    let proposed = vec![report.lesson_ids[1].clone(), report.lesson_ids[0].clone()];
    propose_order(&db, &report.course_id, &proposed).unwrap();

    let detail = course_detail(&db, &report.course_id).unwrap();
    assert_eq!(detail.lessons.len(), 2);
    assert_eq!(detail.course_status, DerivedStatus::NotStarted);
    let proposal = detail.proposal.expect("proposal");
    assert_eq!(proposal.current_titles.len(), 2);
    assert_eq!(proposal.proposed_titles.len(), 2);
    assert_ne!(proposal.current_titles, proposal.proposed_titles);
}
