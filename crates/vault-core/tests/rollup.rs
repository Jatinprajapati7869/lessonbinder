use vault_core::rollup::derive_course_status;
use vault_core::DerivedStatus;
use vault_core::LessonStatus;

#[test]
fn course_status_cases() {
    assert_eq!(
        derive_course_status([]),
        DerivedStatus::NotStarted,
        "empty course is not_started"
    );

    assert_eq!(
        derive_course_status([LessonStatus::NotStarted, LessonStatus::NotStarted,]),
        DerivedStatus::NotStarted
    );

    assert_eq!(
        derive_course_status([LessonStatus::Done, LessonStatus::Done]),
        DerivedStatus::Done
    );

    assert_eq!(
        derive_course_status([LessonStatus::NotStarted, LessonStatus::InProgress,]),
        DerivedStatus::InProgress
    );

    assert_eq!(
        derive_course_status([LessonStatus::Broken, LessonStatus::NotStarted,]),
        DerivedStatus::Broken
    );

    assert_eq!(
        derive_course_status([LessonStatus::Broken, LessonStatus::InProgress,]),
        DerivedStatus::InProgress
    );

    assert_eq!(
        derive_course_status([LessonStatus::Broken, LessonStatus::Done,]),
        DerivedStatus::InProgress
    );
}
