//! ffprobe chapter metadata for local media lessons.

use std::path::Path;
use std::process::Command;

use crate::db::DbError;
use crate::db::VaultDb;
use crate::extract::ExtractorEnv;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaChapter {
    pub start_ms: i64,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonChapters {
    pub chapters: Vec<MediaChapter>,
    /// Set when `chapters` is empty (missing ffprobe, no embedded chapters, etc.).
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChapterIndexOutcome {
    Chapters(Vec<MediaChapter>),
    Empty { reason: String },
}

#[derive(Debug, thiserror::Error)]
pub enum ChapterError {
    #[error(transparent)]
    Db(#[from] DbError),
    #[error("lesson not found")]
    LessonNotFound,
}

impl PartialEq for ChapterError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::LessonNotFound, Self::LessonNotFound) => true,
            (Self::Db(left), Self::Db(right)) => left.to_string() == right.to_string(),
            _ => false,
        }
    }
}

/// Read stored chapters for a lesson (does not run ffprobe).
pub fn list_chapters(db: &VaultDb, lesson_id: &str) -> Result<LessonChapters, ChapterError> {
    if !crate::card::lesson_exists(db, lesson_id)? {
        return Err(ChapterError::LessonNotFound);
    }
    db.list_lesson_chapters(lesson_id)
        .map_err(ChapterError::from)
}

/// Probe local media and persist chapter rows (or empty list + reason).
pub fn index_lesson_chapters(
    db: &VaultDb,
    lesson_id: &str,
    source_path: &Path,
    extension: &str,
    env: &ExtractorEnv,
) -> Result<(), DbError> {
    let ext = extension.to_ascii_lowercase();
    let outcome = if matches!(ext.as_str(), "mp4" | "mkv" | "webm" | "mp3" | "m4a") {
        extract_media_chapters(source_path, env)
    } else {
        ChapterIndexOutcome::Empty {
            reason: "not a media lesson".into(),
        }
    };
    db.save_lesson_chapters(lesson_id, &outcome)
}

/// Refresh chapters from the lesson's on-disk media file when present.
pub fn refresh_lesson_chapters(
    db: &VaultDb,
    lesson_id: &str,
    env: &ExtractorEnv,
) -> Result<(), ChapterError> {
    let Some(path) = db.local_media_path_for_lesson(lesson_id)? else {
        let outcome = ChapterIndexOutcome::Empty {
            reason: "no local media file".into(),
        };
        db.save_lesson_chapters(lesson_id, &outcome)?;
        return Ok(());
    };
    if !path.is_file() {
        let outcome = ChapterIndexOutcome::Empty {
            reason: "no local media file".into(),
        };
        db.save_lesson_chapters(lesson_id, &outcome)?;
        return Ok(());
    }
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default();
    index_lesson_chapters(db, lesson_id, &path, ext, env).map_err(ChapterError::from)
}

pub fn extract_media_chapters(path: &Path, env: &ExtractorEnv) -> ChapterIndexOutcome {
    let Some(ffprobe) = env.ffprobe_executable() else {
        return ChapterIndexOutcome::Empty {
            reason: "ffprobe not available".into(),
        };
    };
    let output = Command::new(ffprobe)
        .args(["-v", "error", "-print_format", "json", "-show_chapters"])
        .arg(path)
        .output();
    match output {
        Ok(out) if out.status.success() => {
            let json = String::from_utf8_lossy(&out.stdout);
            parse_ffprobe_chapters_json(&json)
        }
        Ok(out) => ChapterIndexOutcome::Empty {
            reason: String::from_utf8_lossy(&out.stderr).trim().to_string(),
        },
        Err(e) => ChapterIndexOutcome::Empty {
            reason: e.to_string(),
        },
    }
}

/// Parse ffprobe `-show_chapters` JSON. Skips chapters without a non-empty `tags.title`.
pub fn parse_ffprobe_chapters_json(json: &str) -> ChapterIndexOutcome {
    let root: FfprobeChapterRoot = match serde_json::from_str(json) {
        Ok(value) => value,
        Err(err) => {
            return ChapterIndexOutcome::Empty {
                reason: err.to_string(),
            };
        }
    };
    let mut chapters = Vec::new();
    for entry in root.chapters {
        let Some(title) = entry
            .tags
            .and_then(|tags| tags.title)
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
        else {
            continue;
        };
        let Some(start_ms) = parse_start_time_ms(entry.start_time.as_deref()) else {
            continue;
        };
        chapters.push(MediaChapter { start_ms, title });
    }
    if chapters.is_empty() {
        ChapterIndexOutcome::Empty {
            reason: "no chapters in file".into(),
        }
    } else {
        ChapterIndexOutcome::Chapters(chapters)
    }
}

fn parse_start_time_ms(raw: Option<&str>) -> Option<i64> {
    let seconds: f64 = raw?.trim().parse().ok()?;
    if seconds < 0.0 {
        return None;
    }
    Some((seconds * 1_000.0).round() as i64)
}

#[derive(Debug, serde::Deserialize)]
struct FfprobeChapterRoot {
    #[serde(default)]
    chapters: Vec<FfprobeChapterEntry>,
}

#[derive(Debug, serde::Deserialize)]
struct FfprobeChapterEntry {
    start_time: Option<String>,
    tags: Option<FfprobeChapterTags>,
}

#[derive(Debug, serde::Deserialize)]
struct FfprobeChapterTags {
    title: Option<String>,
}
