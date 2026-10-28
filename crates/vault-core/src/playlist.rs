//! Playlist-backed course import from a local JSON definition (no network).

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::db::VaultDb;
use crate::import::ImportReport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedPlaylistLine {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaylistItemDefinition {
    pub external_id: String,
    pub title: String,
    pub url: String,
    pub duration_ms: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaylistDefinition {
    pub title: String,
    pub playlist_url: String,
    pub items: Vec<PlaylistItemDefinition>,
}

#[derive(Debug, thiserror::Error)]
pub enum PlaylistError {
    #[error(transparent)]
    Db(#[from] crate::db::DbError),
    #[error(transparent)]
    Progress(#[from] crate::progress_persist::ProgressPersistenceError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    InvalidDefinition(String),
    #[error("line {line}: each non-empty line must be title<TAB>url")]
    InvalidLine { line: usize },
    #[error("playlist item not found")]
    PlaylistItemNotFound,
}

#[derive(Debug, Deserialize)]
struct PlaylistDefinitionFile {
    title: String,
    playlist_url: String,
    items: Vec<PlaylistDefinitionItemFile>,
}

#[derive(Debug, Deserialize)]
struct PlaylistDefinitionItemFile {
    external_id: String,
    title: String,
    url: String,
    duration_ms: Option<i64>,
}

const PASTED_PLAYLIST_URL: &str = "auravault://pasted-playlist";

/// Parse `title<TAB>url` lines; blank lines are skipped.
pub fn parse_playlist_lines(text: &str) -> Result<Vec<ParsedPlaylistLine>, PlaylistError> {
    let mut lines = Vec::new();
    for (index, raw) in text.lines().enumerate() {
        if raw.trim().is_empty() {
            continue;
        }
        let Some((title, url)) = raw.split_once('\t') else {
            return Err(PlaylistError::InvalidLine { line: index + 1 });
        };
        lines.push(ParsedPlaylistLine {
            title: title.to_string(),
            url: url.to_string(),
        });
    }
    Ok(lines)
}

pub fn playlist_definition_from_pasted_lines(
    course_title: &str,
    parsed: &[ParsedPlaylistLine],
) -> Result<PlaylistDefinition, PlaylistError> {
    if course_title.trim().is_empty() {
        return Err(PlaylistError::InvalidDefinition(
            "course title must not be empty".into(),
        ));
    }
    if parsed.is_empty() {
        return Err(PlaylistError::InvalidDefinition(
            "playlist must contain at least one line".into(),
        ));
    }
    let mut items = Vec::with_capacity(parsed.len());
    for (index, line) in parsed.iter().enumerate() {
        items.push(PlaylistItemDefinition {
            external_id: format!("paste-{index}"),
            title: line.title.clone(),
            url: line.url.clone(),
            duration_ms: None,
        });
    }
    Ok(PlaylistDefinition {
        title: course_title.to_string(),
        playlist_url: PASTED_PLAYLIST_URL.to_string(),
        items,
    })
}

/// Import or refresh a playlist course from pasted lines (no network).
pub fn import_playlist_from_pasted_lines(
    db: &VaultDb,
    definition_path: &str,
    course_title: &str,
    text: &str,
) -> Result<ImportReport, PlaylistError> {
    let parsed = parse_playlist_lines(text)?;
    let definition = playlist_definition_from_pasted_lines(course_title, &parsed)?;
    upsert_playlist_definition(db, definition_path, &definition)
}

fn upsert_playlist_definition(
    db: &VaultDb,
    definition_path: &str,
    definition: &PlaylistDefinition,
) -> Result<ImportReport, PlaylistError> {
    let report = db
        .upsert_playlist_course(definition_path, definition)
        .map_err(PlaylistError::from)?;
    let active: HashSet<&str> = definition
        .items
        .iter()
        .map(|item| item.external_id.as_str())
        .collect();
    for lesson_id in db
        .playlist_lesson_ids_missing_from_definition(&report.course_id, &active)
        .map_err(PlaylistError::from)?
    {
        db.mark_source_missing(&lesson_id)?;
    }
    Ok(report)
}

/// Import or refresh a playlist course from a JSON file on disk (idempotent on definition path).
pub fn import_playlist_definition(
    db: &VaultDb,
    path: &Path,
) -> Result<ImportReport, PlaylistError> {
    let definition = load_playlist_definition(path)?;
    let definition_path = fs::canonicalize(path)?.to_string_lossy().to_string();
    upsert_playlist_definition(db, &definition_path, &definition)
}

/// Update playlist lesson progress (no local media file required).
pub fn set_watched(
    db: &VaultDb,
    playlist_item_id: &str,
    position_ms: i64,
    completed: bool,
) -> Result<(), PlaylistError> {
    let lesson_id = db
        .lesson_id_for_playlist_item(playlist_item_id)?
        .ok_or(PlaylistError::PlaylistItemNotFound)?;
    db.set_resume_position(&lesson_id, position_ms)?;
    if completed {
        db.mark_complete(&lesson_id)?;
    }
    Ok(())
}

pub fn parse_playlist_definition_json(bytes: &[u8]) -> Result<PlaylistDefinition, PlaylistError> {
    let file: PlaylistDefinitionFile = serde_json::from_slice(bytes)?;
    let mut seen = HashSet::new();
    let mut items = Vec::with_capacity(file.items.len());
    for item in file.items {
        if item.external_id.is_empty() {
            return Err(PlaylistError::InvalidDefinition(
                "playlist item external_id must not be empty".into(),
            ));
        }
        if !seen.insert(item.external_id.clone()) {
            return Err(PlaylistError::InvalidDefinition(format!(
                "duplicate external_id: {}",
                item.external_id
            )));
        }
        items.push(PlaylistItemDefinition {
            external_id: item.external_id,
            title: item.title,
            url: item.url,
            duration_ms: item.duration_ms,
        });
    }
    if file.title.is_empty() {
        return Err(PlaylistError::InvalidDefinition(
            "title must not be empty".into(),
        ));
    }
    if file.playlist_url.is_empty() {
        return Err(PlaylistError::InvalidDefinition(
            "playlist_url must not be empty".into(),
        ));
    }
    Ok(PlaylistDefinition {
        title: file.title,
        playlist_url: file.playlist_url,
        items,
    })
}

fn load_playlist_definition(path: &Path) -> Result<PlaylistDefinition, PlaylistError> {
    let bytes = fs::read(path)?;
    parse_playlist_definition_json(&bytes)
}
