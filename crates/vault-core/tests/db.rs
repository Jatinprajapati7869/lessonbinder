use vault_core::db::VaultDb;
use vault_core::LessonStatus;

#[test]
fn migrate_fresh() {
    let db = VaultDb::open_in_memory().expect("migrate fresh db");
    assert_eq!(db.applied_migration_count(), 18);

    let course_id = db
        .insert_course("Linear Algebra", "/courses/linear-algebra")
        .expect("insert course");
    let lesson_a = db
        .insert_local_file_lesson(&course_id, "videos/01-intro.mp4", "Intro", 0)
        .expect("lesson a");
    let lesson_b = db
        .insert_local_file_lesson(&course_id, "videos/02-matrices.mp4", "Matrices", 1)
        .expect("lesson b");

    assert_ne!(lesson_a, lesson_b);
    assert_eq!(db.lesson_count_for_course(&course_id).unwrap(), 2);
}

#[test]
fn course_unique_source_path() {
    let db = VaultDb::open_in_memory().unwrap();
    db.insert_course("Course A", "/courses/shared-root")
        .expect("first course");
    let err = db
        .insert_course("Course B", "/courses/shared-root")
        .expect_err("duplicate root_path");
    assert!(err.to_string().contains("UNIQUE"), "{}", err);
}

#[test]
fn lesson_defaults_not_started() {
    let db = VaultDb::open_in_memory().unwrap();
    let course_id = db.insert_course("Rust", "/courses/rust").unwrap();
    let lesson_id = db
        .insert_local_file_lesson(&course_id, "01-hello.mp4", "Hello", 0)
        .unwrap();

    let progress = db.progress_for_lesson(&lesson_id).unwrap();
    assert_eq!(progress.status, LessonStatus::NotStarted);
    assert_eq!(progress.resume_position_ms, 0);
    assert_eq!(progress.max_position_reached_ms, 0);
}
