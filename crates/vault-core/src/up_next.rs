//! Next playable lesson in the same course's accepted learning order.

use crate::db::DbError;
use crate::db::LessonStatus;
use crate::db::VaultDb;
use crate::order::learning_order;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpNextTarget {
    pub lesson_id: String,
    pub title: String,
}

#[derive(Debug, thiserror::Error)]
pub enum UpNextError {
    #[error(transparent)]
    Db(#[from] DbError),
    #[error("lesson not found")]
    LessonNotFound,
}

impl PartialEq for UpNextError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::LessonNotFound, Self::LessonNotFound) => true,
            (Self::Db(left), Self::Db(right)) => left.to_string() == right.to_string(),
            _ => false,
        }
    }
}

/// Next lesson after `lesson_id` in this course's accepted order that is not done or broken.
pub fn up_next(db: &VaultDb, lesson_id: &str) -> Result<Option<UpNextTarget>, UpNextError> {
    let Some(course_id) = db.course_id_for_lesson(lesson_id)? else {
        return Err(UpNextError::LessonNotFound);
    };
    let order = learning_order(db, &course_id)?;
    let Some(current_index) = order.iter().position(|id| id == lesson_id) else {
        return Err(UpNextError::LessonNotFound);
    };

    for next_id in order.iter().skip(current_index + 1) {
        let progress = db.progress_for_lesson(next_id)?;
        if progress.status == LessonStatus::Done || progress.status == LessonStatus::Broken {
            continue;
        }
        let title = db.lesson_display_title(next_id)?;
        return Ok(Some(UpNextTarget {
            lesson_id: next_id.clone(),
            title,
        }));
    }

    Ok(None)
}
