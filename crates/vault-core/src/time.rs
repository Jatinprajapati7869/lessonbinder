//! Remaining study time estimates for courses and roadmaps.

use crate::db::DbError;
use crate::db::LessonStatus;
use crate::db::VaultDb;
use crate::read_model::course_progress;
use crate::roadmap::{roadmap_progress, RoadmapError};
use crate::DerivedStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RemainingStudyTime {
    pub remaining_ms: i64,
    pub unknown_lessons: u32,
}

pub fn course_remaining_time(db: &VaultDb, course_id: &str) -> Result<RemainingStudyTime, DbError> {
    if course_progress(db, course_id)? == DerivedStatus::Done {
        return Ok(RemainingStudyTime {
            remaining_ms: 0,
            unknown_lessons: 0,
        });
    }
    let lessons = db.lesson_remaining_inputs_for_course(course_id)?;
    Ok(accumulate_remaining(&lessons))
}

pub fn roadmap_remaining_time(
    db: &VaultDb,
    roadmap_id: &str,
) -> Result<RemainingStudyTime, RoadmapError> {
    if roadmap_progress(db, roadmap_id)? == DerivedStatus::Done {
        return Ok(RemainingStudyTime {
            remaining_ms: 0,
            unknown_lessons: 0,
        });
    }
    let course_ids = db.roadmap_step_course_ids_ordered(roadmap_id)?;
    let mut total = RemainingStudyTime::default();
    for course_id in course_ids.into_iter().flatten() {
        let part = course_remaining_time(db, &course_id).map_err(RoadmapError::from)?;
        total.remaining_ms += part.remaining_ms;
        total.unknown_lessons += part.unknown_lessons;
    }
    Ok(total)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LessonRemainingInput {
    pub status: LessonStatus,
    pub duration_ms: Option<i64>,
}

fn accumulate_remaining(lessons: &[LessonRemainingInput]) -> RemainingStudyTime {
    let mut remaining_ms = 0i64;
    let mut unknown_lessons = 0u32;
    for lesson in lessons {
        if lesson.status == LessonStatus::Done {
            continue;
        }
        match lesson.duration_ms {
            Some(ms) => remaining_ms += ms,
            None => unknown_lessons += 1,
        }
    }
    RemainingStudyTime {
        remaining_ms,
        unknown_lessons,
    }
}
