//! Lesson-attached files (PDF, images, text) shown beside the primary media lesson.

use std::path::PathBuf;

use crate::db::{DbError, VaultDb};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonResourceView {
    pub resource_id: String,
    pub filename: String,
    pub kind: String,
    pub absolute_path: String,
}

pub fn list_lesson_resources(
    db: &VaultDb,
    lesson_id: &str,
) -> Result<Vec<LessonResourceView>, DbError> {
    let rows = db.lesson_resource_rows_for_lesson(lesson_id)?;
    Ok(rows
        .into_iter()
        .filter_map(|row| {
            let root = row.course_root_path?;
            let absolute = PathBuf::from(&root)
                .join(&row.relative_path)
                .to_string_lossy()
                .into_owned();
            let filename = PathBuf::from(&row.relative_path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(&row.relative_path)
                .to_string();
            Some(LessonResourceView {
                resource_id: row.resource_id,
                filename,
                kind: row.kind,
                absolute_path: absolute,
            })
        })
        .collect())
}
