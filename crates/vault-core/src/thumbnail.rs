//! Lesson thumbnail capture (JPEG on disk under app data; no Anki writes).

use std::fs;
use std::path::{Path, PathBuf};

use crate::db::{DbError, VaultDb};

#[derive(Debug, thiserror::Error)]
pub enum ThumbnailError {
    #[error(transparent)]
    Db(#[from] DbError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("thumbnail_requires_local_file")]
    RequiresLocalFile,
    #[error("lesson not found")]
    LessonNotFound,
}

pub trait ThumbnailCapturer {
    fn capture_jpeg(
        &self,
        db: &VaultDb,
        lesson_id: &str,
        position_ms: i64,
    ) -> Result<Vec<u8>, ThumbnailError>;
}

/// Capture JPEG via `capturer`, write under `thumbnails_dir`, store absolute path on the lesson.
pub fn capture_thumbnail(
    db: &VaultDb,
    thumbnails_dir: &Path,
    lesson_id: &str,
    position_ms: i64,
    capturer: &dyn ThumbnailCapturer,
) -> Result<PathBuf, ThumbnailError> {
    ensure_local_file_lesson(db, lesson_id)?;
    let jpeg = capturer.capture_jpeg(db, lesson_id, position_ms)?;
    persist_lesson_thumbnail(db, thumbnails_dir, lesson_id, &jpeg)
}

pub fn persist_lesson_thumbnail(
    db: &VaultDb,
    thumbnails_dir: &Path,
    lesson_id: &str,
    jpeg: &[u8],
) -> Result<PathBuf, ThumbnailError> {
    fs::create_dir_all(thumbnails_dir)?;
    let file_path = thumbnails_dir.join(format!("{lesson_id}.jpg"));
    fs::write(&file_path, jpeg)?;
    let stored = file_path.to_string_lossy().into_owned();
    db.set_lesson_thumbnail_path(lesson_id, &stored)?;
    Ok(file_path)
}

fn ensure_local_file_lesson(db: &VaultDb, lesson_id: &str) -> Result<(), ThumbnailError> {
    let context = db
        .lesson_thumbnail_context(lesson_id)?
        .ok_or(ThumbnailError::LessonNotFound)?;
    if context.source_kind != "local_file" || context.media_relative_path.is_none() {
        return Err(ThumbnailError::RequiresLocalFile);
    }
    Ok(())
}
