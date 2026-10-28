use std::fs;

use tempfile::tempdir;
use vault_core::import::{import_folder, rescan, RescanError};
use vault_core::order::{accept_proposal, learning_order, propose_order};
use vault_core::playlist::import_playlist_definition;
use vault_core::{LessonStatus, VaultDb};

fn fixture_playlist(name: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/playlists")
        .join(name)
}

#[test]
fn rescan_adds_new_eligible_file() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("one.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let kept_id = report.lesson_ids[0].clone();

    fs::write(root.join("two.mp4"), b"").unwrap();
    let rescanned = rescan(&db, &report.course_id).unwrap();

    assert_eq!(db.lesson_count_for_course(&report.course_id).unwrap(), 2);
    assert_eq!(
        db.lesson_stable_keys_for_course(&report.course_id).unwrap(),
        vec!["one.mp4", "two.mp4"]
    );
    assert_eq!(rescanned.lesson_ids[0], kept_id);
}

#[test]
fn rescan_marks_missing_file_broken_and_keeps_lesson_id() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("stay.mp4"), b"").unwrap();
    fs::write(root.join("gone.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let gone_id = db
        .lesson_id_for_stable_key(&report.course_id, "gone.mp4")
        .unwrap();

    fs::remove_file(root.join("gone.mp4")).unwrap();
    rescan(&db, &report.course_id).unwrap();

    assert_eq!(
        db.lesson_id_for_stable_key(&report.course_id, "gone.mp4")
            .unwrap(),
        gone_id
    );
    assert_eq!(
        db.progress_for_lesson(&gone_id).unwrap().status,
        LessonStatus::Broken
    );
    assert_eq!(
        db.progress_for_lesson(
            &db.lesson_id_for_stable_key(&report.course_id, "stay.mp4")
                .unwrap(),
        )
        .unwrap()
        .status,
        LessonStatus::NotStarted
    );
}

#[test]
fn rescan_keeps_progress_for_remaining_lessons() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("clip.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let lesson_id = report.lesson_ids[0].clone();
    db.set_resume_position(&lesson_id, 9_500).unwrap();

    rescan(&db, &report.course_id).unwrap();

    let progress = db.progress_for_lesson(&lesson_id).unwrap();
    assert_eq!(progress.resume_position_ms, 9_500);
    assert_eq!(progress.status, LessonStatus::InProgress);
}

#[test]
fn rescan_applies_lessonbinderignore_to_new_files() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("lecture.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();

    fs::write(root.join(".lessonbinderignore"), "secret/*.mp4\n").unwrap();
    fs::create_dir(root.join("secret")).unwrap();
    fs::write(root.join("secret/new.mp4"), b"").unwrap();

    rescan(&db, &report.course_id).unwrap();

    assert_eq!(db.lesson_count_for_course(&report.course_id).unwrap(), 1);
    assert_eq!(
        db.lesson_stable_keys_for_course(&report.course_id).unwrap(),
        vec!["lecture.mp4"]
    );
}

#[test]
fn rescan_appends_new_lessons_without_reordering_accepted_order() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("a.mp4"), b"").unwrap();
    fs::write(root.join("b.mp4"), b"").unwrap();
    fs::write(root.join("c.mp4"), b"").unwrap();

    let db = VaultDb::open_in_memory().unwrap();
    let report = import_folder(&db, root).unwrap();
    let ids = report.lesson_ids.clone();
    let custom = vec![ids[2].clone(), ids[0].clone(), ids[1].clone()];
    let proposal_id = propose_order(&db, &report.course_id, &custom).unwrap();
    accept_proposal(&db, &proposal_id).unwrap();

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
fn rescan_rejects_playlist_courses() {
    let db = VaultDb::open_in_memory().unwrap();
    let report = import_playlist_definition(&db, &fixture_playlist("two_items.json")).unwrap();

    let err = rescan(&db, &report.course_id).unwrap_err();
    assert_eq!(err, RescanError::NotSupported);
}

#[test]
#[cfg(unix)]
fn rescan_after_rename_keeps_order_notes_and_study_days() {
    use vault_core::lesson_note::{create_lesson_note, list_lesson_notes};
    use vault_core::order::{learning_order, reorder_lessons};
    use vault_core::study_day::{has_study_day, record_study_day};

    let dir = tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("a.mp4"), b"lesson-a-bytes").unwrap();
    fs::write(root.join("b.mp4"), b"lesson-b-bytes").unwrap();

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
    create_lesson_note(&db, &lesson_a, "Survives rescan", Some(2_000)).unwrap();
    record_study_day(&db, "2026-10-28").unwrap();

    fs::rename(root.join("a.mp4"), root.join("moved-a.mp4")).unwrap();

    rescan(&db, &report.course_id).unwrap();

    assert_eq!(
        db.lesson_id_for_stable_key(&report.course_id, "moved-a.mp4")
            .unwrap(),
        lesson_a
    );
    assert_eq!(
        learning_order(&db, &report.course_id).unwrap(),
        vec![lesson_b.clone(), lesson_a.clone()]
    );
    assert_eq!(
        list_lesson_notes(&db, &lesson_a).unwrap()[0].text,
        "Survives rescan"
    );
    assert!(has_study_day(&db, "2026-10-28").unwrap());
    assert_ne!(
        db.progress_for_lesson(&lesson_a).unwrap().status,
        LessonStatus::Broken
    );
}
