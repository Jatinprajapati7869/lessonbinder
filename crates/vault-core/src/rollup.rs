//! Derived course / roadmap status from lesson statuses.

use crate::DerivedStatus;
use crate::LessonStatus;

/// Course rollup per domain Decision 13 / SPEC D-01–D-05.
pub fn derive_course_status(lessons: impl IntoIterator<Item = LessonStatus>) -> DerivedStatus {
    let statuses: Vec<LessonStatus> = lessons.into_iter().collect();
    if statuses.is_empty() || statuses.iter().all(|s| *s == LessonStatus::NotStarted) {
        return DerivedStatus::NotStarted;
    }
    if statuses.iter().all(|s| *s == LessonStatus::Done) {
        return DerivedStatus::Done;
    }
    let any_broken = statuses.contains(&LessonStatus::Broken);
    let any_active = statuses
        .iter()
        .any(|s| *s == LessonStatus::InProgress || *s == LessonStatus::Done);
    if any_broken && !any_active {
        return DerivedStatus::Broken;
    }
    DerivedStatus::InProgress
}

/// Roadmap rollup: same rule as courses, over each step's derived course status.
pub fn derive_roadmap_status(
    step_statuses: impl IntoIterator<Item = DerivedStatus>,
) -> DerivedStatus {
    derive_course_status(step_statuses.into_iter().map(derived_as_lesson_status))
}

fn derived_as_lesson_status(status: DerivedStatus) -> LessonStatus {
    match status {
        DerivedStatus::NotStarted => LessonStatus::NotStarted,
        DerivedStatus::InProgress => LessonStatus::InProgress,
        DerivedStatus::Done => LessonStatus::Done,
        DerivedStatus::Broken => LessonStatus::Broken,
    }
}
