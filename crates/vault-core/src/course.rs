//! Course screen read model (lessons in learning order, optional order proposal).

use std::collections::HashMap;
use std::path::PathBuf;

use crate::db::DbError;
use crate::db::LessonDetailRow;
use crate::db::LessonStatus;
use crate::db::VaultDb;
use crate::order::learning_order;
use crate::read_model::course_progress;
use crate::sections::{folder_lesson_section, group_lessons_by_section, LessonSectionGroup};
use crate::DerivedStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LessonKind {
    LocalFile,
    PlaylistItem,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonView {
    pub lesson_id: String,
    pub position: u32,
    pub title: String,
    pub kind: LessonKind,
    pub status: LessonStatus,
    pub duration_ms: Option<i64>,
    pub resume_position_ms: i64,
    pub max_position_reached_ms: i64,
    pub document_position: i64,
    pub playlist_item_id: Option<String>,
    pub playlist_url: Option<String>,
    pub local_media_path: Option<String>,
    pub thumbnail_path: Option<String>,
    pub section: Option<String>,
}

pub type LessonSectionGroupView = LessonSectionGroup<LessonView>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderProposalView {
    pub proposal_id: String,
    pub current_titles: Vec<String>,
    pub proposed_titles: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseDetail {
    pub course_id: String,
    pub title: String,
    pub course_status: DerivedStatus,
    pub lessons: Vec<LessonView>,
    pub lesson_groups: Vec<LessonSectionGroupView>,
    pub proposal: Option<OrderProposalView>,
}

pub fn course_detail(db: &VaultDb, course_id: &str) -> Result<CourseDetail, DbError> {
    let title = db.course_title(course_id)?;
    let course_status = course_progress(db, course_id)?;
    let rows = db.lesson_details_for_course(course_id)?;
    let by_id: HashMap<String, LessonDetailRow> = rows
        .into_iter()
        .map(|row| (row.lesson_id.clone(), row))
        .collect();
    let ordered_ids = learning_order(db, course_id)?;
    let lessons: Vec<LessonView> = ordered_ids
        .iter()
        .enumerate()
        .filter_map(|(idx, id)| by_id.get(id).map(|row| lesson_view_from_row(row, idx + 1)))
        .collect();
    let lesson_groups = group_lessons_by_section(&lessons, |lesson| lesson.section.clone());

    let title_by_id: HashMap<String, String> = lessons
        .iter()
        .map(|l| (l.lesson_id.clone(), l.title.clone()))
        .collect();

    let proposal = db
        .pending_order_proposal(course_id)?
        .map(|(proposal_id, proposed_ids)| {
            let current_titles = ordered_ids
                .iter()
                .filter_map(|id| title_by_id.get(id).cloned())
                .collect();
            let proposed_titles = proposed_ids
                .iter()
                .filter_map(|id| title_by_id.get(id).cloned())
                .collect();
            OrderProposalView {
                proposal_id,
                current_titles,
                proposed_titles,
            }
        });

    Ok(CourseDetail {
        course_id: course_id.to_string(),
        title,
        course_status,
        lessons,
        lesson_groups,
        proposal,
    })
}

fn lesson_view_from_row(row: &LessonDetailRow, position: usize) -> LessonView {
    let kind = if row.lesson_source_kind == "playlist_item" {
        LessonKind::PlaylistItem
    } else {
        LessonKind::LocalFile
    };
    let local_media_path = match (&row.course_root_path, &row.media_relative_path) {
        (Some(root), Some(rel)) => {
            Some(PathBuf::from(root).join(rel).to_string_lossy().into_owned())
        }
        _ => None,
    };
    let section = row
        .media_relative_path
        .as_deref()
        .and_then(folder_lesson_section);
    LessonView {
        lesson_id: row.lesson_id.clone(),
        position: position as u32,
        title: row.display_title.clone(),
        kind,
        status: row.status,
        duration_ms: row.duration_ms,
        resume_position_ms: row.resume_position_ms,
        max_position_reached_ms: row.max_position_reached_ms,
        document_position: row.document_position,
        playlist_item_id: row.playlist_item_id.clone(),
        playlist_url: row.playlist_url.clone(),
        local_media_path,
        thumbnail_path: row.thumbnail_path.clone(),
        section,
    }
}
