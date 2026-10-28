use std::fs;

use tempfile::tempdir;
use vault_core::import::import_folder;
use vault_core::read_model::course_progress;
use vault_core::roadmap::{
    add_roadmap_step, create_roadmap, delete_course, relink_roadmap_step, step_status, RoadmapError,
};
use vault_core::{DerivedStatus, VaultDb};

fn single_lesson_course(db: &VaultDb) -> String {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("lesson.mp4"), b"").unwrap();
    import_folder(db, dir.path()).unwrap().course_id
}

#[test]
fn relink_broken_step_restores_derived_status_from_new_course() {
    let db = VaultDb::open_in_memory().unwrap();
    let old_course = single_lesson_course(&db);
    let new_course = {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("other.mp4"), b"").unwrap();
        import_folder(&db, dir.path()).unwrap().course_id
    };

    let roadmap_id = create_roadmap(&db, "Path").unwrap();
    let step_id = add_roadmap_step(&db, &roadmap_id, &old_course, None).unwrap();
    delete_course(&db, &old_course).unwrap();
    assert_eq!(step_status(&db, &step_id).unwrap(), DerivedStatus::Broken);

    let lesson_id = db.lesson_ids_in_learning_order(&new_course).unwrap()[0].clone();
    db.set_resume_position(&lesson_id, 2_000).unwrap();
    assert_eq!(
        course_progress(&db, &new_course).unwrap(),
        DerivedStatus::InProgress,
    );

    relink_roadmap_step(&db, &step_id, &new_course).unwrap();
    assert_eq!(
        step_status(&db, &step_id).unwrap(),
        DerivedStatus::InProgress
    );
}

#[test]
fn relink_to_missing_course_fails() {
    let db = VaultDb::open_in_memory().unwrap();
    let course_id = single_lesson_course(&db);
    let roadmap_id = create_roadmap(&db, "Path").unwrap();
    let step_id = add_roadmap_step(&db, &roadmap_id, &course_id, None).unwrap();
    delete_course(&db, &course_id).unwrap();

    let err = relink_roadmap_step(&db, &step_id, "missing-course-id").unwrap_err();
    assert!(matches!(
        err,
        RoadmapError::CourseNotFound(id) if id == "missing-course-id"
    ));
    assert_eq!(step_status(&db, &step_id).unwrap(), DerivedStatus::Broken);
}
