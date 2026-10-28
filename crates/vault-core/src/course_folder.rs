//! Open a folder course root in the system file manager (path resolution only; no I/O side effects here).

use std::fs;
use std::path::{Path, PathBuf};

use crate::db::DbError;
use crate::import::{canonical_import_folder_path, ImportError};
use crate::VaultDb;

#[derive(Debug, thiserror::Error)]
pub enum CourseFolderOpenError {
    #[error("course_not_found")]
    CourseNotFound,
    #[error("not_folder_course")]
    NotFolderCourse,
    #[error("course_root_missing")]
    CourseRootMissing,
    #[error("course_root_stale")]
    CourseRootStale,
    #[error("lesson_not_found")]
    LessonNotFound,
    #[error("lesson_media_missing")]
    LessonMediaMissing,
    #[error("path_outside_course_root")]
    PathOutsideCourseRoot,
    #[error(transparent)]
    Import(#[from] ImportError),
    #[error(transparent)]
    Db(#[from] DbError),
}

/// Resolve the stored course root for opening in a file manager.
///
/// Uses only the path recorded for `course_id` (never a caller-supplied path). Rejects playlist
/// courses, missing roots, roots that no longer canonicalize to the stored value, and paths that
/// fail the same risky-import checks as **Add folder**.
pub fn course_folder_path_for_open(
    db: &VaultDb,
    course_id: &str,
) -> Result<PathBuf, CourseFolderOpenError> {
    let source_kind = db.course_source_kind(course_id).map_err(|err| {
        if matches!(err, DbError::Sqlite(rusqlite::Error::QueryReturnedNoRows)) {
            CourseFolderOpenError::CourseNotFound
        } else {
            CourseFolderOpenError::Db(err)
        }
    })?;
    if source_kind != "folder" {
        return Err(CourseFolderOpenError::NotFolderCourse);
    }
    let Some(stored) = db.course_root_path(course_id)? else {
        return Err(CourseFolderOpenError::CourseRootMissing);
    };
    let canonical = canonical_import_folder_path(Path::new(&stored))?;
    if canonical.to_string_lossy() != stored {
        return Err(CourseFolderOpenError::CourseRootStale);
    }
    Ok(canonical)
}

fn canonical_media_path_under_course_root(
    course_root: &Path,
    media_path: &Path,
) -> Result<PathBuf, CourseFolderOpenError> {
    let canonical_root = fs::canonicalize(course_root).map_err(ImportError::from)?;
    let canonical_media = fs::canonicalize(media_path).map_err(ImportError::from)?;
    if !canonical_media.starts_with(&canonical_root) {
        return Err(CourseFolderOpenError::PathOutsideCourseRoot);
    }
    Ok(canonical_media)
}

/// Directory containing the lesson's local media file, for opening in a file manager.
///
/// Resolves paths from `lesson_id` only. The media file must lie under the course root after
/// canonicalization (symlinks or `..` segments that escape the root are rejected).
pub fn lesson_file_directory_for_open(
    db: &VaultDb,
    lesson_id: &str,
) -> Result<PathBuf, CourseFolderOpenError> {
    let Some(course_id) = db.course_id_for_lesson(lesson_id)? else {
        return Err(CourseFolderOpenError::LessonNotFound);
    };
    let course_root = course_folder_path_for_open(db, &course_id)?;
    let Some(media_path) = db.local_media_path_for_lesson(lesson_id)? else {
        return Err(CourseFolderOpenError::LessonMediaMissing);
    };
    let canonical_media = canonical_media_path_under_course_root(&course_root, &media_path)?;
    canonical_media
        .parent()
        .map(Path::to_path_buf)
        .ok_or(CourseFolderOpenError::LessonMediaMissing)
}
