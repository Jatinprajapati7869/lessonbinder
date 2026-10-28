//! Library search over lesson titles, media paths, and subtitle cue text.

use crate::cues::{list_cues, CueError};
use crate::db::{DbError, LessonSearchRow, VaultDb};

fn hit(
    row: &LessonSearchRow,
    match_kind: SearchMatchKind,
    snippet: String,
    cue_start_ms: Option<i64>,
) -> SearchHit {
    SearchHit {
        lesson_id: row.lesson_id.clone(),
        lesson_title: row.display_title.clone(),
        course_id: row.course_id.clone(),
        course_title: row.course_title.clone(),
        match_kind,
        snippet,
        cue_start_ms,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchMatchKind {
    LessonTitle,
    RelativePath,
    SubtitleCue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchHit {
    pub lesson_id: String,
    pub lesson_title: String,
    pub course_id: String,
    pub course_title: String,
    pub match_kind: SearchMatchKind,
    pub snippet: String,
    pub cue_start_ms: Option<i64>,
}

#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error(transparent)]
    Db(#[from] DbError),
    #[error(transparent)]
    Cue(#[from] CueError),
}

pub fn search(db: &VaultDb, query: &str) -> Result<Vec<SearchHit>, SearchError> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let like_pattern = sql_like_pattern(trimmed);
    let mut hits = Vec::new();

    for row in db.lessons_matching_title_or_path(&like_pattern)? {
        if contains_insensitive(&row.display_title, trimmed) {
            hits.push(hit(
                &row,
                SearchMatchKind::LessonTitle,
                snippet_around(&row.display_title, trimmed),
                None,
            ));
        }
        if let Some(path) = &row.relative_path {
            if contains_insensitive(path, trimmed) {
                hits.push(hit(
                    &row,
                    SearchMatchKind::RelativePath,
                    snippet_around(path, trimmed),
                    None,
                ));
            }
        }
    }

    for (row, start_ms, text) in db.search_transcription_cue_rows(&like_pattern)? {
        hits.push(hit(
            &row,
            SearchMatchKind::SubtitleCue,
            snippet_around(&text, trimmed),
            Some(start_ms),
        ));
    }

    for lesson_id in db.lesson_ids_with_subtitle_sidecar()? {
        let row = db.lesson_search_row(&lesson_id)?;
        let cues = list_cues(db, &lesson_id)?;
        for cue in cues {
            if contains_insensitive(&cue.text, trimmed) {
                hits.push(hit(
                    &row,
                    SearchMatchKind::SubtitleCue,
                    snippet_around(&cue.text, trimmed),
                    Some(cue.start_ms),
                ));
            }
        }
    }

    Ok(hits)
}

fn sql_like_pattern(query: &str) -> String {
    let mut out = String::from("%");
    for ch in query.chars() {
        match ch {
            '%' | '_' | '\\' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out.push('%');
    out.to_ascii_lowercase()
}

fn contains_insensitive(haystack: &str, needle: &str) -> bool {
    haystack
        .to_ascii_lowercase()
        .contains(&needle.to_ascii_lowercase())
}

fn snippet_around(text: &str, query: &str) -> String {
    const MAX_LEN: usize = 80;
    let lower_text = text.to_ascii_lowercase();
    let lower_query = query.to_ascii_lowercase();
    let start = lower_text.find(&lower_query).unwrap_or(0);
    let end = (start + query.len()).min(text.len());
    let pad = (MAX_LEN - query.len()) / 2;
    let from = start.saturating_sub(pad);
    let to = (end + pad).min(text.len());
    let slice = text.get(from..to).unwrap_or(text);
    let mut snippet = String::new();
    if from > 0 {
        snippet.push('…');
    }
    snippet.push_str(slice.trim());
    if to < text.len() {
        snippet.push('…');
    }
    if snippet.len() > MAX_LEN {
        snippet.truncate(MAX_LEN);
        snippet.push('…');
    }
    snippet
}
