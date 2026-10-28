use vault_core::anki_link::{anki_card_link, insert_anki_card_link, LinkStatus};
use vault_core::card::lesson_exists;
use vault_core::course_archive::{archive_course, unarchive_course, CourseArchiveError};
use vault_core::library::{list_courses, CourseLibraryVisibility};
use vault_core::read_model::course_progress;
use vault_core::roadmap::{add_roadmap_step, create_roadmap, step_status, RoadmapError};
use vault_core::{DerivedStatus, VaultDb};

#[test]
fn archive_hides_course_from_list_unarchive_restores() {
    let db = VaultDb::open_in_memory().unwrap();
    let visible = db.insert_course("Visible", "/courses/visible").unwrap();
    let hidden = db.insert_course("Hidden", "/courses/hidden").unwrap();

    assert_eq!(
        list_courses(&db, None, CourseLibraryVisibility::Active)
            .unwrap()
            .len(),
        2,
    );

    archive_course(&db, &hidden).unwrap();
    let listed = list_courses(&db, None, CourseLibraryVisibility::Active).unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].course_id, visible);
    let archived = list_courses(&db, None, CourseLibraryVisibility::Archived).unwrap();
    assert_eq!(archived.len(), 1);
    assert_eq!(archived[0].course_id, hidden);

    unarchive_course(&db, &hidden).unwrap();
    assert_eq!(
        list_courses(&db, None, CourseLibraryVisibility::Active)
            .unwrap()
            .len(),
        2,
    );

    let err = archive_course(&db, "missing-id").unwrap_err();
    assert!(matches!(err, CourseArchiveError::CourseNotFound(id) if id == "missing-id"));
}

#[test]
fn archived_course_rejected_for_new_roadmap_step() {
    let db = VaultDb::open_in_memory().unwrap();
    let course_id = db
        .insert_course("Course", "/courses/archive-roadmap")
        .unwrap();
    let roadmap_id = create_roadmap(&db, "Path").unwrap();

    archive_course(&db, &course_id).unwrap();
    let err = add_roadmap_step(&db, &roadmap_id, &course_id, None).unwrap_err();
    assert!(matches!(
        err,
        RoadmapError::CourseNotFound(id) if id == course_id
    ));
}

#[test]
fn archive_keeps_lessons_progress_roadmap_status_and_anki_links() {
    let db = VaultDb::open_in_memory().unwrap();
    let course_id = db
        .insert_course("Course", "/courses/archive-preserve")
        .unwrap();
    let lesson_id = db
        .insert_local_file_lesson(&course_id, "clip.mp4", "clip", 0)
        .unwrap();
    db.set_resume_position(&lesson_id, 4_000).unwrap();
    let link_id = insert_anki_card_link(&db, &lesson_id, 1_500, "front-hash", "", 77_007).unwrap();

    let roadmap_id = create_roadmap(&db, "Path").unwrap();
    let step_id = add_roadmap_step(&db, &roadmap_id, &course_id, None).unwrap();

    archive_course(&db, &course_id).unwrap();

    assert!(!list_courses(&db, None, CourseLibraryVisibility::Active)
        .unwrap()
        .iter()
        .any(|c| c.course_id == course_id));
    assert!(lesson_exists(&db, &lesson_id).unwrap());
    assert_eq!(
        course_progress(&db, &course_id).unwrap(),
        DerivedStatus::InProgress,
    );
    assert_eq!(
        step_status(&db, &step_id).unwrap(),
        DerivedStatus::InProgress
    );

    let link = anki_card_link(&db, &link_id).unwrap();
    assert_eq!(link.link_status, LinkStatus::Active);
    assert_eq!(link.lesson_id.as_deref(), Some(lesson_id.as_str()));
    assert_eq!(link.anki_note_id, 77_007);
}
