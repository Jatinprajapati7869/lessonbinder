//! Learning order and order proposals (O-01–O-04).

use std::collections::HashSet;

use crate::db::DbError;
use crate::db::VaultDb;

#[derive(Debug, thiserror::Error)]
pub enum OrderError {
    #[error(transparent)]
    Db(#[from] DbError),
    #[error("order proposal not found")]
    ProposalNotFound,
    #[error("invalid order proposal: {0}")]
    InvalidProposal(InvalidProposalError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum InvalidProposalError {
    #[error("proposal must list each lesson exactly once")]
    LessonSetMismatch,
    #[error("proposal lists duplicate lessons")]
    DuplicateLesson,
}

/// Current persisted learning order for a course (lesson ids by `sort_index`).
pub fn learning_order(db: &VaultDb, course_id: &str) -> Result<Vec<String>, DbError> {
    db.lesson_ids_in_learning_order(course_id)
}

/// Store a proposed order without changing live `sort_index` values.
pub fn propose_order(
    db: &VaultDb,
    course_id: &str,
    lesson_ids: &[String],
) -> Result<String, OrderError> {
    validate_proposal(db, course_id, lesson_ids)?;
    db.replace_order_proposal(course_id, lesson_ids)
        .map_err(OrderError::from)
}

pub fn accept_proposal(db: &VaultDb, proposal_id: &str) -> Result<(), OrderError> {
    let (course_id, lesson_ids) = db
        .order_proposal_lesson_ids(proposal_id)?
        .ok_or(OrderError::ProposalNotFound)?;
    db.apply_learning_order(&course_id, &lesson_ids)?;
    db.delete_order_proposal(proposal_id)?;
    Ok(())
}

pub fn reject_proposal(db: &VaultDb, proposal_id: &str) -> Result<(), OrderError> {
    if !db.delete_order_proposal(proposal_id)? {
        return Err(OrderError::ProposalNotFound);
    }
    Ok(())
}

/// Apply a new lesson order immediately (`sort_index` only; no order proposal).
pub fn reorder_lessons(
    db: &VaultDb,
    course_id: &str,
    lesson_ids: &[String],
) -> Result<(), OrderError> {
    validate_proposal(db, course_id, lesson_ids)?;
    db.apply_learning_order(course_id, lesson_ids)?;
    Ok(())
}

fn validate_proposal(
    db: &VaultDb,
    course_id: &str,
    lesson_ids: &[String],
) -> Result<(), OrderError> {
    let course_lessons = db.lesson_ids_in_learning_order(course_id)?;
    let course_set: HashSet<&str> = course_lessons.iter().map(String::as_str).collect();

    if lesson_ids.len() != lesson_ids.iter().collect::<HashSet<_>>().len() {
        return Err(OrderError::InvalidProposal(
            InvalidProposalError::DuplicateLesson,
        ));
    }
    if lesson_ids.len() != course_set.len() {
        return Err(OrderError::InvalidProposal(
            InvalidProposalError::LessonSetMismatch,
        ));
    }
    for id in lesson_ids {
        if !course_set.contains(id.as_str()) {
            return Err(OrderError::InvalidProposal(
                InvalidProposalError::LessonSetMismatch,
            ));
        }
    }
    Ok(())
}
