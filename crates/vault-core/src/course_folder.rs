//! Open a folder course root in the system file manager (path resolution only; no I/O side effects here).

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
