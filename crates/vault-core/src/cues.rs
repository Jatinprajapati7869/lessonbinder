//! Subtitle cue lists for lessons (SRT / VTT sidecars).

use std::path::{Path, PathBuf};

use crate::db::DbError;
use crate::db::VaultDb;

pub(crate) struct LessonSubtitleFile {
    pub course_root: PathBuf,
    pub relative_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubtitleCue {
    pub start_ms: i64,
    pub end_ms: i64,
    pub text: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CueError {
    #[error("lesson not found")]
    LessonNotFound,
    #[error(transparent)]
    Db(#[from] DbError),
    #[error("subtitle parse error: {0}")]
    Parse(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl PartialEq for CueError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::LessonNotFound, Self::LessonNotFound) => true,
            (Self::Parse(left), Self::Parse(right)) => left == right,
            (Self::Db(left), Self::Db(right)) => left.to_string() == right.to_string(),
            (Self::Io(left), Self::Io(right)) => {
                left.kind() == right.kind() && left.to_string() == right.to_string()
            }
            _ => false,
        }
    }
}

/// Ordered cues for `lesson_id` from its attached SRT or VTT file.
///
/// Returns an empty list when the lesson has no subtitle sidecar or the file is missing on disk.
pub fn list_cues(db: &VaultDb, lesson_id: &str) -> Result<Vec<SubtitleCue>, CueError> {
    let Some(location) = db.lesson_subtitle_file(lesson_id)? else {
        return Err(CueError::LessonNotFound);
    };
    let Some(relative_path) = location.relative_path else {
        return Ok(Vec::new());
    };
    let path = location.course_root.join(&relative_path);
    if !path.is_file() {
        return Ok(Vec::new());
    }
    parse_subtitle_file(&path)
}

/// Sidecar subtitles plus generated transcription cues (never replaces on-disk SRT/VTT).
pub fn list_player_cues(db: &VaultDb, lesson_id: &str) -> Result<Vec<SubtitleCue>, CueError> {
    let sidecar = list_cues(db, lesson_id)?;
    let transcription = db.list_lesson_transcription_cues(lesson_id)?;
    let mut merged = sidecar;
    merged.extend(transcription);
    merged.sort_by_key(|cue| cue.start_ms);
    Ok(merged)
}

pub fn parse_subtitle_file(path: &Path) -> Result<Vec<SubtitleCue>, CueError> {
    let text = std::fs::read_to_string(path)?;
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .unwrap_or_default();
    match ext.as_str() {
        "vtt" => parse_vtt(&text),
        _ => parse_srt(&text),
    }
}

fn parse_srt(text: &str) -> Result<Vec<SubtitleCue>, CueError> {
    let mut cues = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0usize;
    while i < lines.len() {
        while i < lines.len() && lines[i].trim().is_empty() {
            i += 1;
        }
        if i >= lines.len() {
            break;
        }
        if lines[i].trim().chars().all(|c| c.is_ascii_digit()) {
            i += 1;
            if i >= lines.len() {
                break;
            }
        }
        let timing = lines[i].trim();
        if !timing.contains("-->") {
            return Err(CueError::Parse("expected subtitle timing line".into()));
        }
        let (start_ms, end_ms) = parse_timing_line(timing)?;
        i += 1;
        let mut body = Vec::new();
        while i < lines.len() && !lines[i].trim().is_empty() {
            body.push(lines[i].trim_end());
            i += 1;
        }
        if body.is_empty() {
            return Err(CueError::Parse("subtitle cue missing text".into()));
        }
        cues.push(SubtitleCue {
            start_ms,
            end_ms,
            text: body.join("\n"),
        });
    }
    if cues.is_empty() && !text.trim().is_empty() {
        return Err(CueError::Parse("no cues found in subtitle file".into()));
    }
    Ok(cues)
}

fn parse_vtt(text: &str) -> Result<Vec<SubtitleCue>, CueError> {
    let trimmed = text.trim_start();
    if !trimmed.starts_with("WEBVTT") {
        return Err(CueError::Parse("VTT file must start with WEBVTT".into()));
    }
    let mut cues = Vec::new();
    let mut i = 0usize;
    let lines: Vec<&str> = text.lines().collect();
    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty()
            || line.starts_with("WEBVTT")
            || line.starts_with("NOTE")
            || line.starts_with("STYLE")
            || line.starts_with("REGION")
        {
            i += 1;
            continue;
        }
        if line.contains("-->") {
            let (start_ms, end_ms) = parse_timing_line(line)?;
            i += 1;
            let mut body = Vec::new();
            while i < lines.len() && !lines[i].trim().is_empty() {
                let text_line = lines[i].trim();
                if !text_line.starts_with("NOTE") {
                    body.push(text_line);
                }
                i += 1;
            }
            if body.is_empty() {
                return Err(CueError::Parse("subtitle cue missing text".into()));
            }
            cues.push(SubtitleCue {
                start_ms,
                end_ms,
                text: body.join("\n"),
            });
            continue;
        }
        i += 1;
    }
    if cues.is_empty() && !text.trim().is_empty() {
        return Err(CueError::Parse("no cues found in subtitle file".into()));
    }
    Ok(cues)
}

fn parse_timing_line(line: &str) -> Result<(i64, i64), CueError> {
    let (left, right) = line
        .split_once("-->")
        .ok_or_else(|| CueError::Parse("invalid timing line".into()))?;
    let start_ms = parse_timestamp(left.trim())
        .ok_or_else(|| CueError::Parse(format!("invalid start time: {}", left.trim())))?;
    let end_part = right.split_whitespace().next().unwrap_or(right).trim();
    let end_ms = parse_timestamp(end_part)
        .ok_or_else(|| CueError::Parse(format!("invalid end time: {}", end_part)))?;
    if end_ms < start_ms {
        return Err(CueError::Parse("cue end before start".into()));
    }
    Ok((start_ms, end_ms))
}

fn parse_timestamp(raw: &str) -> Option<i64> {
    let raw = raw.trim();
    let (hms, millis) = if let Some((h, ms)) = raw.split_once(',') {
        (h, ms.parse::<i64>().ok()?)
    } else if let Some((h, ms)) = raw.split_once('.') {
        (h, ms.parse::<i64>().ok()?)
    } else {
        return None;
    };
    let mut parts = hms.split(':');
    let hours = parts.next()?.parse::<i64>().ok()?;
    let minutes = parts.next()?.parse::<i64>().ok()?;
    let seconds = parts.next()?.parse::<i64>().ok()?;
    Some(hours * 3_600_000 + minutes * 60_000 + seconds * 1_000 + millis)
}
