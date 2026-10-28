use std::fs;

use tempfile::tempdir;
use vault_core::import::import_folder;
use vault_core::read_model::course_progress;
use vault_core::roadmap::{
    add_roadmap_step, create_roadmap, delete_course, remove_roadmap_step, reorder_roadmap_steps,
    roadmap_progress, roadmap_steps, step_status,
};
use vault_core::{DerivedStatus, VaultDb};

fn single_lesson_course(db: &VaultDb) -> String {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("lesson.mp4"), b"").unwrap();
    import_folder(db, dir.path()).unwrap().course_id
}

#[test]
fn step_status_from_course() {
    let db = VaultDb::open_in_memory().unwrap();
    let course_id = single_lesson_course(&db);
    let roadmap_id = create_roadmap(&db, "Path").unwrap();
    let step_id = add_roadmap_step(&db, &roadmap_id, &course_id, Some("note")).unwrap();

    assert_eq!(
        step_status(&db, &step_id).unwrap(),
        DerivedStatus::NotStarted
    );
    assert_eq!(
        course_progress(&db, &course_id).unwrap(),
        DerivedStatus::NotStarted
    );

    let lesson_id = db.lesson_ids_in_learning_order(&course_id).unwrap()[0].clone();
    db.set_resume_position(&lesson_id, 1_000).unwrap();

    assert_eq!(
        course_progress(&db, &course_id).unwrap(),
        DerivedStatus::InProgress
    );
    assert_eq!(
        step_status(&db, &step_id).unwrap(),
        DerivedStatus::InProgress
    );
}

#[test]
fn rollup_matches_course_rules() {
    let db = VaultDb::open_in_memory().unwrap();
    let course_a = single_lesson_course(&db);
    let course_b = single_lesson_course(&db);
    let roadmap_id = create_roadmap(&db, "Rollup").unwrap();
    add_roadmap_step(&db, &roadmap_id, &course_a, None).unwrap();
    add_roadmap_step(&db, &roadmap_id, &course_b, None).unwrap();

    assert_eq!(
        roadmap_progress(&db, &roadmap_id).unwrap(),
        DerivedStatus::NotStarted
    );

    let lesson_a = db.lesson_ids_in_learning_order(&course_a).unwrap()[0].clone();
    db.mark_complete(&lesson_a).unwrap();
    assert_eq!(
        roadmap_progress(&db, &roadmap_id).unwrap(),
        DerivedStatus::InProgress
    );

    let lesson_b = db.lesson_ids_in_learning_order(&course_b).unwrap()[0].clone();
    db.mark_complete(&lesson_b).unwrap();
    assert_eq!(
        roadmap_progress(&db, &roadmap_id).unwrap(),
        DerivedStatus::Done
    );

    let course_c = single_lesson_course(&db);
    let roadmap2 = create_roadmap(&db, "Broken rollup").unwrap();
    add_roadmap_step(&db, &roadmap2, &course_c, None).unwrap();
    let lesson_c = db.lesson_ids_in_learning_order(&course_c).unwrap()[0].clone();
    db.mark_source_missing(&lesson_c).unwrap();
    assert_eq!(
        course_progress(&db, &course_c).unwrap(),
        DerivedStatus::Broken
    );
    assert_eq!(
        roadmap_progress(&db, &roadmap2).unwrap(),
        DerivedStatus::Broken
    );
}

#[test]
fn reorder_steps() {
    let db = VaultDb::open_in_memory().unwrap();
    let c1 = single_lesson_course(&db);
    let c2 = single_lesson_course(&db);
    let c3 = single_lesson_course(&db);
    let roadmap_id = create_roadmap(&db, "Order").unwrap();
    let s1 = add_roadmap_step(&db, &roadmap_id, &c1, None).unwrap();
    let s2 = add_roadmap_step(&db, &roadmap_id, &c2, None).unwrap();
    let s3 = add_roadmap_step(&db, &roadmap_id, &c3, None).unwrap();

    reorder_roadmap_steps(&db, &roadmap_id, &[s3.clone(), s1.clone(), s2.clone()]).unwrap();
    let ids = roadmap_steps(&db, &roadmap_id)
        .unwrap()
        .into_iter()
        .map(|s| s.id)
        .collect::<Vec<_>>();
    assert_eq!(ids, vec![s3, s1, s2]);
}

#[test]
fn remove_step() {
    let db = VaultDb::open_in_memory().unwrap();
    let c1 = single_lesson_course(&db);
    let c2 = single_lesson_course(&db);
    let roadmap_id = create_roadmap(&db, "Remove").unwrap();
    let s1 = add_roadmap_step(&db, &roadmap_id, &c1, None).unwrap();
    let s2 = add_roadmap_step(&db, &roadmap_id, &c2, None).unwrap();

    remove_roadmap_step(&db, &s1).unwrap();
    let steps = roadmap_steps(&db, &roadmap_id).unwrap();
    assert_eq!(steps.len(), 1);
    assert_eq!(steps[0].id, s2);
}

#[test]
fn missing_course_step_is_broken() {
    let db = VaultDb::open_in_memory().unwrap();
    let course_id = single_lesson_course(&db);
    let roadmap_id = create_roadmap(&db, "Missing").unwrap();
    let step_id = add_roadmap_step(&db, &roadmap_id, &course_id, Some("orphan")).unwrap();

    delete_course(&db, &course_id).unwrap();
    assert_eq!(step_status(&db, &step_id).unwrap(), DerivedStatus::Broken);
    assert_eq!(
        roadmap_progress(&db, &roadmap_id).unwrap(),
        DerivedStatus::Broken
    );
}
