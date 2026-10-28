//! Deterministic metadata extractors (no LLM).

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::db::DbError;
use crate::db::VaultDb;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtractorOutcome<T> {
    Known { extractor: String, value: T },
    Unknown { extractor: String, reason: String },
}

impl<T> ExtractorOutcome<T> {
    pub fn known(extractor: &str, value: T) -> Self {
        Self::Known {
            extractor: extractor.into(),
            value,
        }
    }

    pub fn unknown(extractor: &str, reason: impl Into<String>) -> Self {
        Self::Unknown {
            extractor: extractor.into(),
            reason: reason.into(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExtractorEnv {
    ffprobe: Option<PathBuf>,
    pdfinfo: Option<PathBuf>,
    disable_external_tools: bool,
}

impl ExtractorEnv {
    pub fn discover() -> Self {
        Self {
            ffprobe: which("ffprobe"),
            pdfinfo: which("pdfinfo"),
            disable_external_tools: false,
        }
    }

    pub fn without_tools() -> Self {
        Self {
            ffprobe: None,
            pdfinfo: None,
            disable_external_tools: true,
        }
    }

    pub(crate) fn ffprobe_executable(&self) -> Option<PathBuf> {
        self.ffprobe_path()
    }

    fn ffprobe_path(&self) -> Option<PathBuf> {
        if self.disable_external_tools {
            return None;
        }
        self.ffprobe.clone().or_else(|| which("ffprobe"))
    }

    fn pdfinfo_path(&self) -> Option<PathBuf> {
        if self.disable_external_tools {
            return None;
        }
        self.pdfinfo.clone().or_else(|| which("pdfinfo"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubtitleExtract {
    pub cue_count: ExtractorOutcome<i64>,
    pub first_timestamp_ms: ExtractorOutcome<i64>,
    pub last_timestamp_ms: ExtractorOutcome<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonMetadataView {
    pub duration_ms: ExtractorOutcome<i64>,
    pub page_count: ExtractorOutcome<i64>,
    pub heading: ExtractorOutcome<String>,
    pub subtitle_cue_count: ExtractorOutcome<i64>,
    pub subtitle_first_ms: ExtractorOutcome<i64>,
    pub subtitle_last_ms: ExtractorOutcome<i64>,
}

/// Run extractors for one lesson source file and persist results on the lesson row.
pub fn index_lesson_metadata(
    db: &VaultDb,
    lesson_id: &str,
    source_path: &Path,
    extension: &str,
    subtitle_path: Option<&Path>,
    env: &ExtractorEnv,
) -> Result<(), DbError> {
    let ext = extension.to_ascii_lowercase();
    let duration_ms = if matches!(ext.as_str(), "mp4" | "mkv" | "webm" | "mp3" | "m4a") {
        extract_media_duration(source_path, env)
    } else {
        ExtractorOutcome::unknown("ffprobe", "not a media lesson")
    };
    let page_count = if ext == "pdf" {
        extract_pdf_page_count(source_path, env).map_err(|e| DbError::InvalidData(e.to_string()))?
    } else {
        ExtractorOutcome::unknown("pdfinfo", "not a pdf lesson")
    };
    let heading = if matches!(ext.as_str(), "md" | "txt") {
        extract_first_heading(source_path).map_err(|e| DbError::InvalidData(e.to_string()))?
    } else {
        ExtractorOutcome::unknown("markdown_heading", "not a text lesson")
    };

    crate::chapters::index_lesson_chapters(db, lesson_id, source_path, extension, env)?;

    let subtitle = if let Some(path) = subtitle_path {
        extract_subtitle_cues(path).map_err(|e| DbError::InvalidData(e.to_string()))?
    } else {
        SubtitleExtract {
            cue_count: ExtractorOutcome::unknown("srt", "no subtitle file"),
            first_timestamp_ms: ExtractorOutcome::unknown("srt", "no subtitle file"),
            last_timestamp_ms: ExtractorOutcome::unknown("srt", "no subtitle file"),
        }
    };

    db.save_lesson_metadata(lesson_id, &duration_ms, &page_count, &heading, &subtitle)
}

pub fn extract_media_duration(path: &Path, env: &ExtractorEnv) -> ExtractorOutcome<i64> {
    let ffprobe = env.ffprobe_path();
    let Some(ffprobe) = ffprobe else {
        return ExtractorOutcome::unknown("ffprobe", "ffprobe not available");
    };
    let output = Command::new(ffprobe)
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
        ])
        .arg(path)
        .output();
    match output {
        Ok(out) if out.status.success() => {
            let text = String::from_utf8_lossy(&out.stdout).into_owned();
            match text.trim().parse::<f64>() {
                Ok(seconds) if seconds >= 0.0 => {
                    ExtractorOutcome::known("ffprobe", (seconds * 1_000.0).round() as i64)
                }
                _ => ExtractorOutcome::unknown("ffprobe", "could not parse duration"),
            }
        }
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr);
            ExtractorOutcome::unknown("ffprobe", err.trim())
        }
        Err(e) => ExtractorOutcome::unknown("ffprobe", e.to_string()),
    }
}

pub fn extract_pdf_page_count(
    path: &Path,
    env: &ExtractorEnv,
) -> Result<ExtractorOutcome<i64>, std::io::Error> {
    let pdfinfo = env.pdfinfo_path();
    let Some(pdfinfo) = pdfinfo else {
        return Ok(ExtractorOutcome::unknown(
            "pdfinfo",
            "pdfinfo not available",
        ));
    };
    let output = Command::new(pdfinfo).arg(path).output()?;
    if !output.status.success() {
        return Ok(ExtractorOutcome::unknown(
            "pdfinfo",
            String::from_utf8_lossy(&output.stderr).trim(),
        ));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("Pages:") {
            if let Ok(pages) = rest.trim().parse::<i64>() {
                return Ok(ExtractorOutcome::known("pdfinfo", pages));
            }
        }
    }
    Ok(ExtractorOutcome::unknown(
        "pdfinfo",
        "could not read page count",
    ))
}

pub fn extract_first_heading(path: &Path) -> Result<ExtractorOutcome<String>, std::io::Error> {
    let text = std::fs::read_to_string(path)?;
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix('#') {
            let heading = rest.trim_start_matches('#').trim();
            if !heading.is_empty() {
                return Ok(ExtractorOutcome::known(
                    "markdown_heading",
                    heading.to_string(),
                ));
            }
        }
    }
    Ok(ExtractorOutcome::unknown(
        "markdown_heading",
        "no heading found",
    ))
}

pub fn extract_subtitle_cues(path: &Path) -> Result<SubtitleExtract, std::io::Error> {
    let text = std::fs::read_to_string(path)?;
    let mut cue_count = 0i64;
    let mut first_ms: Option<i64> = None;
    let mut last_ms: Option<i64> = None;

    for line in text.lines() {
        if let Some((start, end)) = parse_cue_line(line) {
            cue_count += 1;
            first_ms = Some(first_ms.unwrap_or(start).min(start));
            last_ms = Some(last_ms.unwrap_or(end).max(end));
        }
    }

    if cue_count == 0 {
        return Ok(SubtitleExtract {
            cue_count: ExtractorOutcome::unknown("srt", "no cues found"),
            first_timestamp_ms: ExtractorOutcome::unknown("srt", "no cues found"),
            last_timestamp_ms: ExtractorOutcome::unknown("srt", "no cues found"),
        });
    }

    Ok(SubtitleExtract {
        cue_count: ExtractorOutcome::known("srt", cue_count),
        first_timestamp_ms: ExtractorOutcome::known("srt", first_ms.unwrap_or(0)),
        last_timestamp_ms: ExtractorOutcome::known("srt", last_ms.unwrap_or(0)),
    })
}

fn parse_cue_line(line: &str) -> Option<(i64, i64)> {
    let (left, right) = line.split_once("-->")?;
    let start = parse_timestamp(left.trim())?;
    let end = parse_timestamp(right.trim())?;
    Some((start, end))
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

fn which(cmd: &str) -> Option<PathBuf> {
    let path_var = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path_var) {
        let candidate = dir.join(cmd);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}
