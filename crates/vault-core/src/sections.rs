//! Folder course sections (first path component under the course root).

use std::collections::HashSet;

use crate::db::DbError;
use crate::db::VaultDb;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonSectionGroup<L> {
    pub section: Option<String>,
    pub lessons: Vec<L>,
}

/// Contiguous runs of lessons with the same section (in learning order).
pub fn group_lessons_by_section<L, F>(lessons: &[L], section_of: F) -> Vec<LessonSectionGroup<L>>
where
    L: Clone,
    F: Fn(&L) -> Option<String>,
{
    let mut groups: Vec<LessonSectionGroup<L>> = Vec::new();
    for lesson in lessons {
        let section = section_of(lesson);
        if let Some(last) = groups.last_mut() {
            if last.section == section {
                last.lessons.push(lesson.clone());
                continue;
            }
        }
        groups.push(LessonSectionGroup {
            section,
            lessons: vec![lesson.clone()],
        });
    }
    groups
}

#[derive(Debug, thiserror::Error)]
pub enum SectionError {
    #[error(transparent)]
    Db(#[from] DbError),
}

/// First path component under the course root, or `None` for lessons at the root.
pub fn folder_lesson_section(relative_path: &str) -> Option<String> {
    let normalized = relative_path.replace('\\', "/");
    let parts: Vec<&str> = normalized
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    if parts.len() <= 1 {
        None
    } else {
        Some(parts[0].to_string())
    }
}

/// Section names in first-appearance order (by lesson `sort_index`). Playlist courses return empty.
pub fn list_sections(db: &VaultDb, course_id: &str) -> Result<Vec<String>, SectionError> {
    if db.course_source_kind(course_id)? != "folder" {
        return Ok(vec![]);
    }
    let keys = db.lesson_stable_keys_for_course(course_id)?;
    let mut sections = Vec::new();
    let mut seen = HashSet::new();
    for key in keys {
        if let Some(section) = folder_lesson_section(&key) {
            if seen.insert(section.clone()) {
                sections.push(section);
            }
        }
    }
    Ok(sections)
}
