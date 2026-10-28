use std::fs;
use std::sync::Mutex;

use tempfile::tempdir;
use vault_core::import::{import_folder_with_cancel, FolderScanProgress};
use vault_core::VaultDb;

#[test]
fn import_reports_seen_lesson_files_then_n_of_m_while_writing() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("a.mp4"), b"").unwrap();
    fs::write(root.join("b.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let progress = Mutex::new(Vec::<FolderScanProgress>::new());
    let on_progress = |update: FolderScanProgress| {
        progress.lock().unwrap().push(update);
    };

    import_folder_with_cancel(&db, root, &|| false, Some(&on_progress)).unwrap();

    let snapshots = progress.lock().unwrap();
    assert!(
        snapshots.iter().any(|p| p.planned == 0 && p.seen >= 1),
        "expected scan-phase lesson file counts, got {snapshots:?}"
    );
    assert!(
        snapshots
            .iter()
            .any(|p| p.planned == 2 && p.seen == 2 && p.seen == p.planned),
        "expected planned total after scan, got {snapshots:?}"
    );
    assert_eq!(
        snapshots.last().copied(),
        Some(FolderScanProgress {
            seen: 2,
            planned: 2
        })
    );
}
