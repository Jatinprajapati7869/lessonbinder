use std::path::Path;
use std::sync::atomic::Ordering;

use serde::Deserialize;
use serde::Serialize;
use tauri::State;
use vault_core::continue_learning::library_continue_row;
use vault_core::import::import_folder_with_cancel;
use vault_core::library::{list_courses, CourseLibraryVisibility, CourseSourceKind};
use vault_core::playlist::{import_playlist_definition, import_playlist_from_pasted_lines};
use vault_core::search::{search, SearchMatchKind};
use vault_core::DerivedStatus;
use vault_core::VaultDb;

use lessonbinder_tauri::command_error::from_import_error;

use crate::app_state::AppState;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CourseSourceKindDto {
    Folder,
    Playlist,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivedStatusDto {
    NotStarted,
    InProgress,
    Done,
    Broken,
}

#[derive(Debug, Clone, Serialize)]
pub struct CourseRowDto {
    pub course_id: String,
    pub title: String,
    pub source_kind: CourseSourceKindDto,
    pub status: DerivedStatusDto,
    pub remaining_ms: i64,
    pub unknown_duration_count: u32,
    pub is_archived: bool,
    pub thumbnail_path: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LibraryVisibilityDto {
    Active,
    Archived,
}

impl LibraryVisibilityDto {
    fn to_core(self) -> CourseLibraryVisibility {
        match self {
            Self::Active => CourseLibraryVisibility::Active,
            Self::Archived => CourseLibraryVisibility::Archived,
        }
    }
}

pub fn list_library_course_rows(
    db: &VaultDb,
    visibility: CourseLibraryVisibility,
) -> Result<Vec<CourseRowDto>, vault_core::db::DbError> {
    let is_archived = visibility == CourseLibraryVisibility::Archived;
    let courses = list_courses(db, None, visibility)?;
    Ok(courses
        .into_iter()
        .map(|entry| CourseRowDto::from_entry(entry, is_archived))
        .collect())
}

impl CourseRowDto {
    fn from_entry(entry: vault_core::library::CourseListEntry, is_archived: bool) -> Self {
        Self {
            course_id: entry.course_id,
            title: entry.title,
            source_kind: match entry.source_kind {
                CourseSourceKind::Folder => CourseSourceKindDto::Folder,
                CourseSourceKind::Playlist => CourseSourceKindDto::Playlist,
            },
            status: match entry.status {
                DerivedStatus::NotStarted => DerivedStatusDto::NotStarted,
                DerivedStatus::InProgress => DerivedStatusDto::InProgress,
                DerivedStatus::Done => DerivedStatusDto::Done,
                DerivedStatus::Broken => DerivedStatusDto::Broken,
            },
            remaining_ms: entry.remaining.remaining_ms,
            unknown_duration_count: entry.remaining.unknown_lessons,
            is_archived,
            thumbnail_path: entry.thumbnail_path,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchMatchKindDto {
    LessonTitle,
    RelativePath,
    SubtitleCue,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchHitDto {
    pub lesson_id: String,
    pub lesson_title: String,
    pub course_id: String,
    pub course_title: String,
    pub match_kind: SearchMatchKindDto,
    pub snippet: String,
    pub cue_start_ms: Option<i64>,
}

#[tauri::command]
pub fn search_library(state: State<AppState>, query: String) -> Result<Vec<SearchHitDto>, String> {
    state.with_vault(|vault| {
        search(vault, &query)
            .map_err(|e| e.to_string())
            .map(|hits| hits.into_iter().map(SearchHitDto::from).collect())
    })
}

impl From<vault_core::search::SearchHit> for SearchHitDto {
    fn from(hit: vault_core::search::SearchHit) -> Self {
        Self {
            lesson_id: hit.lesson_id,
            lesson_title: hit.lesson_title,
            course_id: hit.course_id,
            course_title: hit.course_title,
            match_kind: match hit.match_kind {
                SearchMatchKind::LessonTitle => SearchMatchKindDto::LessonTitle,
                SearchMatchKind::RelativePath => SearchMatchKindDto::RelativePath,
                SearchMatchKind::SubtitleCue => SearchMatchKindDto::SubtitleCue,
            },
            snippet: hit.snippet,
            cue_start_ms: hit.cue_start_ms,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LibraryContinueRowDto {
    pub course_id: String,
    pub course_title: String,
    pub lesson_id: String,
    pub lesson_title: String,
    pub resume_position_ms: i64,
    pub watched_percent: Option<i64>,
}

#[tauri::command]
pub fn get_library_continue_row(
    state: State<AppState>,
) -> Result<Option<LibraryContinueRowDto>, String> {
    state.with_vault(|vault| {
        library_continue_row(vault)
            .map_err(|e| e.to_string())
            .map(|row| row.map(LibraryContinueRowDto::from))
    })
}

impl From<vault_core::continue_learning::LibraryContinueRow> for LibraryContinueRowDto {
    fn from(row: vault_core::continue_learning::LibraryContinueRow) -> Self {
        Self {
            course_id: row.course_id,
            course_title: row.course_title,
            lesson_id: row.lesson_id,
            lesson_title: row.lesson_title,
            resume_position_ms: row.resume_position_ms,
            watched_percent: row.watched_percent,
        }
    }
}

#[tauri::command]
pub fn list_library_courses(
    state: State<AppState>,
    visibility: Option<LibraryVisibilityDto>,
) -> Result<Vec<CourseRowDto>, String> {
    let visibility = visibility
        .map(LibraryVisibilityDto::to_core)
        .unwrap_or(CourseLibraryVisibility::Active);
    state.with_vault(|vault| list_library_course_rows(vault, visibility).map_err(|e| e.to_string()))
}

#[tauri::command]
pub fn import_course_folder(state: State<AppState>, path: String) -> Result<(), String> {
    state.import_folder_cancel.store(false, Ordering::Relaxed);
    state.with_vault(|vault| {
        let cancel = &state.import_folder_cancel;
        import_folder_with_cancel(vault, Path::new(&path), &|| cancel.load(Ordering::Relaxed))
            .map_err(|e| from_import_error(e).to_invoke_string())?;
        Ok(())
    })
}

#[tauri::command]
pub fn cancel_import_course_folder(state: State<AppState>) {
    state.import_folder_cancel.store(true, Ordering::Relaxed);
}

#[tauri::command]
pub fn import_playlist_json(state: State<AppState>, path: String) -> Result<(), String> {
    state.with_vault(|vault| {
        import_playlist_definition(vault, Path::new(&path)).map_err(|e| e.to_string())?;
        Ok(())
    })
}

#[tauri::command]
pub fn import_playlist_paste(
    state: State<AppState>,
    course_title: String,
    text: String,
) -> Result<(), String> {
    state.with_vault(|vault| {
        let definition_path = format!("lessonbinder://pasted/{}", uuid::Uuid::new_v4());
        import_playlist_from_pasted_lines(vault, &definition_path, &course_title, &text)
            .map_err(|e| e.to_string())?;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use tempfile::tempdir;
    use vault_core::import::import_folder;
    use vault_core::playlist::import_playlist_definition;
    use vault_core::VaultDb;

    use super::list_library_course_rows;
    use vault_core::library::CourseLibraryVisibility;

    fn playlist_fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../crates/vault-core/tests/fixtures/playlists")
            .join(name)
    }

    #[test]
    fn list_library_courses_after_imports() {
        let db = VaultDb::open_in_memory().unwrap();
        let folder = tempdir().unwrap();
        fs::write(folder.path().join("a.mp4"), b"").unwrap();
        import_folder(&db, folder.path()).unwrap();
        import_playlist_definition(&db, &playlist_fixture("two_items.json")).unwrap();

        let rows = list_library_course_rows(&db, CourseLibraryVisibility::Active).unwrap();
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().any(|r| r.unknown_duration_count == 1));
        assert!(rows.iter().any(|r| r.remaining_ms == 155_000));
    }
}
