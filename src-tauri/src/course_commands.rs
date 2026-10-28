use std::sync::atomic::Ordering;

use serde::Serialize;
use tauri::{AppHandle, State};
use tauri_plugin_dialog::{DialogExt, FilePath};
use vault_core::chapters::{
    list_chapters as load_lesson_chapters, refresh_lesson_chapters, ChapterError,
};
use vault_core::course::course_detail;
use vault_core::course::LessonKind;
use vault_core::course_archive::{archive_course, unarchive_course};
use vault_core::cues::list_player_cues;
use vault_core::extract::ExtractorEnv;
use vault_core::import::{rescan_with_cancel, FolderScanProgress};
use vault_core::lesson_note::{self as lesson_notes, LessonNoteError};
use vault_core::lesson_note_export::{course_notes_markdown, lesson_notes_markdown};
use vault_core::lesson_resource::list_lesson_resources;
use vault_core::list_lesson_card_links;
use vault_core::order::{accept_proposal, reject_proposal, reorder_lessons};
use vault_core::playlist::set_watched;
use vault_core::record_playback as merge_lesson_playback;
use vault_core::set_document_position;
use vault_core::thumbnail::{capture_thumbnail, ThumbnailCapturer, ThumbnailError};
use vault_core::transcription::{run_lesson_transcription, transcription_engine_from_settings};
use vault_core::unlink_card;
use vault_core::vault_settings::load_vault_settings;
use vault_core::CueError;
use vault_core::LessonStatus;
use vault_core::WatchedRange;

use lessonbinder_tauri::command_error::{from_rescan_error, from_thumbnail_error};
use lessonbinder_tauri::command_log::log_command_error;

use crate::app_state::AppState;
use crate::folder_scan_progress::emit_folder_scan_progress;
use crate::vault_paths::thumbnails_dir;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LessonKindDto {
    LocalFile,
    PlaylistItem,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LessonStatusDto {
    NotStarted,
    InProgress,
    Done,
    Broken,
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
pub struct LessonRowDto {
    pub lesson_id: String,
    pub position: u32,
    pub title: String,
    pub kind: LessonKindDto,
    pub status: LessonStatusDto,
    pub duration_ms: Option<i64>,
    pub resume_position_ms: i64,
    pub max_position_reached_ms: i64,
    pub document_position: i64,
    pub playlist_item_id: Option<String>,
    pub playlist_url: Option<String>,
    pub local_media_path: Option<String>,
    pub thumbnail_path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderProposalDto {
    pub proposal_id: String,
    pub current_titles: Vec<String>,
    pub proposed_titles: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LessonSectionGroupDto {
    pub section: Option<String>,
    pub lessons: Vec<LessonRowDto>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CourseDetailDto {
    pub course_id: String,
    pub title: String,
    pub course_status: DerivedStatusDto,
    pub lessons: Vec<LessonRowDto>,
    pub lesson_groups: Vec<LessonSectionGroupDto>,
    pub proposal: Option<OrderProposalDto>,
}

#[tauri::command]
pub fn get_course_detail(
    state: State<AppState>,
    course_id: String,
) -> Result<CourseDetailDto, String> {
    state.with_vault(|vault| {
        let detail = course_detail(vault, &course_id).map_err(|e| e.to_string())?;
        Ok(CourseDetailDto::from(detail))
    })
}

#[tauri::command]
pub fn mark_lesson_complete(state: State<AppState>, lesson_id: String) -> Result<(), String> {
    state.with_vault(|vault| {
        vault.mark_complete(&lesson_id).map_err(|e| e.to_string())?;
        Ok(())
    })
}

#[tauri::command]
pub fn mark_lesson_incomplete(state: State<AppState>, lesson_id: String) -> Result<(), String> {
    state.with_vault(|vault| {
        vault
            .mark_incomplete(&lesson_id)
            .map_err(|e| e.to_string())?;
        Ok(())
    })
}

#[tauri::command]
pub fn mark_lesson_broken(state: State<AppState>, lesson_id: String) -> Result<(), String> {
    state.with_vault(|vault| {
        vault
            .mark_source_missing(&lesson_id)
            .map_err(|e| e.to_string())?;
        Ok(())
    })
}

#[tauri::command(async)]
pub fn rescan_course(
    app: AppHandle,
    state: State<AppState>,
    course_id: String,
) -> Result<(), String> {
    let _scan_guard = state.try_begin_folder_scan()?;
    state.with_vault(|vault| {
        let cancel = &state.import_folder_cancel;
        let on_progress = |progress: FolderScanProgress| {
            emit_folder_scan_progress(&app, progress);
        };
        rescan_with_cancel(
            vault,
            &course_id,
            &|| cancel.load(Ordering::Relaxed),
            Some(&on_progress),
        )
        .map_err(|err| map_rescan_failure(&state, &course_id, err))?;
        Ok(())
    })
}

#[tauri::command]
pub fn archive_course_entry(state: State<AppState>, course_id: String) -> Result<(), String> {
    state.with_vault(|vault| {
        archive_course(vault, &course_id).map_err(|e| e.to_string())?;
        Ok(())
    })
}

#[tauri::command]
pub fn unarchive_course_entry(state: State<AppState>, course_id: String) -> Result<(), String> {
    state.with_vault(|vault| {
        unarchive_course(vault, &course_id).map_err(|e| e.to_string())?;
        Ok(())
    })
}

fn map_rescan_failure(
    state: &AppState,
    course_id: &str,
    err: vault_core::import::RescanError,
) -> String {
    let coded = from_rescan_error(err);
    let context = serde_json::json!({ "course_id": course_id });
    let _ = log_command_error(
        &state.app_data_dir,
        "rescan_course",
        &coded.code,
        Some(context),
    );
    coded.to_invoke_string()
}

#[derive(Debug, Clone, Serialize)]
pub struct LessonNoteDto {
    pub note_id: String,
    pub lesson_id: String,
    pub text: String,
    pub timestamp_ms: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[tauri::command]
pub fn list_lesson_notes(
    state: State<AppState>,
    lesson_id: String,
) -> Result<Vec<LessonNoteDto>, String> {
    state.with_vault(|vault| {
        lesson_notes::list_lesson_notes(vault, &lesson_id)
            .map_err(map_lesson_note_err)
            .map(|notes| notes.into_iter().map(LessonNoteDto::from).collect())
    })
}

#[tauri::command]
pub fn create_lesson_note_entry(
    state: State<AppState>,
    lesson_id: String,
    text: String,
    timestamp_ms: Option<i64>,
) -> Result<LessonNoteDto, String> {
    state.with_vault(|vault| {
        lesson_notes::create_lesson_note(vault, &lesson_id, &text, timestamp_ms)
            .map_err(map_lesson_note_err)
            .map(LessonNoteDto::from)
    })
}

#[tauri::command]
pub fn update_lesson_note_entry(
    state: State<AppState>,
    note_id: String,
    text: String,
    timestamp_ms: Option<i64>,
) -> Result<LessonNoteDto, String> {
    state.with_vault(|vault| {
        lesson_notes::update_lesson_note(vault, &note_id, &text, timestamp_ms)
            .map_err(map_lesson_note_err)
            .map(LessonNoteDto::from)
    })
}

#[tauri::command]
pub fn delete_lesson_note_entry(state: State<AppState>, note_id: String) -> Result<(), String> {
    state.with_vault(|vault| {
        lesson_notes::delete_lesson_note(vault, &note_id).map_err(map_lesson_note_err)?;
        Ok(())
    })
}

#[tauri::command]
pub fn export_lesson_notes_markdown(
    app: AppHandle,
    state: State<AppState>,
    lesson_id: String,
    default_file_name: String,
) -> Result<(), String> {
    let markdown = state.with_vault(|vault| {
        lesson_notes_markdown(vault, &lesson_id).map_err(map_lesson_note_err)
    })?;
    let picked = app
        .dialog()
        .file()
        .add_filter("Markdown", &["md"])
        .set_file_name(&default_file_name)
        .blocking_save_file();
    let Some(file_path) = picked else {
        return Ok(());
    };
    let path = match file_path {
        FilePath::Path(path) => path,
        FilePath::Url(url) => return Err(format!("unsupported save location: {url}")),
    };
    std::fs::write(&path, markdown.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn export_course_notes_markdown(
    app: AppHandle,
    state: State<AppState>,
    course_id: String,
    default_file_name: String,
) -> Result<(), String> {
    let markdown = state
        .with_vault(|vault| course_notes_markdown(vault, &course_id).map_err(|e| e.to_string()))?;
    let picked = app
        .dialog()
        .file()
        .add_filter("Markdown", &["md"])
        .set_file_name(&default_file_name)
        .blocking_save_file();
    let Some(file_path) = picked else {
        return Ok(());
    };
    let path = match file_path {
        FilePath::Path(path) => path,
        FilePath::Url(url) => return Err(format!("unsupported save location: {url}")),
    };
    std::fs::write(&path, markdown.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

fn map_lesson_note_err(err: LessonNoteError) -> String {
    err.to_string()
}

impl From<vault_core::lesson_note::LessonNote> for LessonNoteDto {
    fn from(note: vault_core::lesson_note::LessonNote) -> Self {
        Self {
            note_id: note.id,
            lesson_id: note.lesson_id,
            text: note.text,
            timestamp_ms: note.timestamp_ms,
            created_at: note.created_at,
            updated_at: note.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UpNextTargetDto {
    pub lesson_id: String,
    pub title: String,
}

#[tauri::command]
pub fn up_next(
    state: State<AppState>,
    lesson_id: String,
) -> Result<Option<UpNextTargetDto>, String> {
    state.with_vault(|vault| {
        vault_core::up_next(vault, &lesson_id)
            .map_err(|e| e.to_string())
            .map(|target| {
                target.map(|next| UpNextTargetDto {
                    lesson_id: next.lesson_id,
                    title: next.title,
                })
            })
    })
}

#[derive(Debug, Clone, Serialize)]
pub struct SubtitleCueDto {
    pub start_ms: i64,
    pub end_ms: i64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MediaChapterDto {
    pub start_ms: i64,
    pub title: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LessonChaptersDto {
    pub chapters: Vec<MediaChapterDto>,
    pub reason: Option<String>,
}

#[tauri::command]
pub fn list_chapters(
    state: State<AppState>,
    lesson_id: String,
) -> Result<LessonChaptersDto, String> {
    state.with_vault(|vault| {
        refresh_lesson_chapters(vault, &lesson_id, &ExtractorEnv::discover())
            .map_err(map_chapter_error)?;
        let listed = load_lesson_chapters(vault, &lesson_id).map_err(map_chapter_error)?;
        Ok(LessonChaptersDto {
            chapters: listed
                .chapters
                .into_iter()
                .map(|chapter| MediaChapterDto {
                    start_ms: chapter.start_ms,
                    title: chapter.title,
                })
                .collect(),
            reason: listed.reason,
        })
    })
}

#[tauri::command]
pub fn list_lesson_cues(
    state: State<AppState>,
    lesson_id: String,
) -> Result<Vec<SubtitleCueDto>, String> {
    state.with_vault(|vault| {
        let cues = list_player_cues(vault, &lesson_id).map_err(map_cue_error)?;
        Ok(cues
            .into_iter()
            .map(|cue| SubtitleCueDto {
                start_ms: cue.start_ms,
                end_ms: cue.end_ms,
                text: cue.text,
            })
            .collect())
    })
}

fn map_chapter_error(err: ChapterError) -> String {
    err.to_string()
}

fn map_cue_error(err: CueError) -> String {
    err.to_string()
}

#[derive(Debug, Clone, Serialize)]
pub struct LessonResourceDto {
    pub resource_id: String,
    pub filename: String,
    pub kind: String,
    pub absolute_path: String,
}

#[derive(serde::Serialize)]
pub struct LessonCardLinkDto {
    pub anki_note_id: i64,
    pub timestamp_ms: i64,
    pub front_preview: String,
}

#[tauri::command]
pub fn unlink_lesson_card_entry(
    state: State<AppState>,
    lesson_id: String,
    anki_note_id: i64,
) -> Result<(), String> {
    state.with_vault(|vault| {
        unlink_card(vault, &lesson_id, anki_note_id).map_err(|e| e.to_string())?;
        Ok(())
    })
}

#[tauri::command]
pub fn list_lesson_card_links_entry(
    state: State<AppState>,
    lesson_id: String,
) -> Result<Vec<LessonCardLinkDto>, String> {
    state.with_vault(|vault| {
        let links = list_lesson_card_links(vault, &lesson_id).map_err(|e| e.to_string())?;
        Ok(links
            .into_iter()
            .map(|link| LessonCardLinkDto {
                anki_note_id: link.anki_note_id,
                timestamp_ms: link.timestamp_ms,
                front_preview: link.front_preview,
            })
            .collect())
    })
}

#[tauri::command]
pub fn list_lesson_resources_entry(
    state: State<AppState>,
    lesson_id: String,
) -> Result<Vec<LessonResourceDto>, String> {
    state.with_vault(|vault| {
        let resources = list_lesson_resources(vault, &lesson_id).map_err(|e| e.to_string())?;
        Ok(resources
            .into_iter()
            .map(|resource| LessonResourceDto {
                resource_id: resource.resource_id,
                filename: resource.filename,
                kind: resource.kind,
                absolute_path: resource.absolute_path,
            })
            .collect())
    })
}

struct ProvidedJpegCapturer {
    jpeg: Vec<u8>,
}

impl ThumbnailCapturer for ProvidedJpegCapturer {
    fn capture_jpeg(
        &self,
        _db: &vault_core::VaultDb,
        _lesson_id: &str,
        _position_ms: i64,
    ) -> Result<Vec<u8>, ThumbnailError> {
        Ok(self.jpeg.clone())
    }
}

#[tauri::command]
pub fn capture_lesson_thumbnail(
    state: State<AppState>,
    lesson_id: String,
    position_ms: i64,
    jpeg: Vec<u8>,
) -> Result<(), String> {
    let dir = thumbnails_dir(&state.app_data_dir);
    state.with_vault(|vault| {
        capture_thumbnail(
            vault,
            &dir,
            &lesson_id,
            position_ms,
            &ProvidedJpegCapturer { jpeg },
        )
        .map_err(|err| from_thumbnail_error(err).to_invoke_string())?;
        Ok(())
    })
}

#[tauri::command]
pub fn get_transcription_model_path(state: State<AppState>) -> Result<String, String> {
    state.with_vault(|vault| {
        load_vault_settings(vault)
            .map(|settings| settings.transcription_model_path)
            .map_err(|e| e.to_string())
    })
}

#[tauri::command]
pub fn transcribe_lesson(state: State<AppState>, lesson_id: String) -> Result<(), String> {
    state.with_vault(|vault| {
        let settings = load_vault_settings(vault).map_err(|e| e.to_string())?;
        let engine = transcription_engine_from_settings(&settings, None);
        run_lesson_transcription(vault, &lesson_id, &engine).map_err(|e| e.to_string())
    })
}

#[derive(Debug, Clone, Serialize)]
pub struct WatchedRangeDto {
    pub start_ms: i64,
    pub end_ms: i64,
}

#[tauri::command]
pub fn list_lesson_watched_ranges(
    state: State<AppState>,
    lesson_id: String,
) -> Result<Vec<WatchedRangeDto>, String> {
    state.with_vault(|vault| {
        let ranges = vault
            .watched_ranges_for_lesson(&lesson_id)
            .map_err(|e| e.to_string())?;
        Ok(ranges.into_iter().map(WatchedRangeDto::from).collect())
    })
}

#[tauri::command]
pub fn record_lesson_playback(
    state: State<AppState>,
    lesson_id: String,
    start_ms: i64,
    end_ms: i64,
) -> Result<Vec<WatchedRangeDto>, String> {
    state.with_vault(|vault| {
        merge_lesson_playback(vault, &lesson_id, start_ms, end_ms).map_err(|e| e.to_string())?;
        let ranges = vault
            .watched_ranges_for_lesson(&lesson_id)
            .map_err(|e| e.to_string())?;
        Ok(ranges.into_iter().map(WatchedRangeDto::from).collect())
    })
}

impl From<WatchedRange> for WatchedRangeDto {
    fn from(value: WatchedRange) -> Self {
        Self {
            start_ms: value.start_ms,
            end_ms: value.end_ms,
        }
    }
}

#[tauri::command]
pub fn save_document_position(
    state: State<AppState>,
    lesson_id: String,
    position: i64,
) -> Result<(), String> {
    state.with_vault(|vault| {
        set_document_position(vault, &lesson_id, position).map_err(|e| e.to_string())?;
        Ok(())
    })
}

#[tauri::command]
pub fn save_lesson_resume(
    state: State<AppState>,
    lesson_id: String,
    position_ms: i64,
) -> Result<(), String> {
    state.with_vault(|vault| {
        vault
            .record_playback_position(&lesson_id, position_ms)
            .map_err(|e| e.to_string())?;
        Ok(())
    })
}

#[tauri::command]
pub fn accept_order_proposal(state: State<AppState>, proposal_id: String) -> Result<(), String> {
    state.with_vault(|vault| {
        accept_proposal(vault, &proposal_id).map_err(|e| e.to_string())?;
        Ok(())
    })
}

#[tauri::command]
pub fn reject_order_proposal(state: State<AppState>, proposal_id: String) -> Result<(), String> {
    state.with_vault(|vault| {
        reject_proposal(vault, &proposal_id).map_err(|e| e.to_string())?;
        Ok(())
    })
}

#[tauri::command]
pub fn reorder_course_lessons(
    state: State<AppState>,
    course_id: String,
    lesson_ids: Vec<String>,
) -> Result<(), String> {
    state.with_vault(|vault| {
        reorder_lessons(vault, &course_id, &lesson_ids).map_err(|e| e.to_string())?;
        Ok(())
    })
}

#[tauri::command]
pub fn set_playlist_item_watched(
    state: State<AppState>,
    playlist_item_id: String,
    watched: bool,
) -> Result<(), String> {
    state.with_vault(|vault| {
        if watched {
            set_watched(vault, &playlist_item_id, 0, true).map_err(|e| e.to_string())?;
        } else {
            let lesson_id = vault
                .lesson_id_for_playlist_item(&playlist_item_id)
                .map_err(|e| e.to_string())?
                .ok_or_else(|| "playlist item not found".to_string())?;
            vault
                .mark_incomplete(&lesson_id)
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    })
}

impl From<vault_core::course::CourseDetail> for CourseDetailDto {
    fn from(detail: vault_core::course::CourseDetail) -> Self {
        Self {
            course_id: detail.course_id,
            title: detail.title,
            course_status: derived_status_dto(detail.course_status),
            lessons: detail.lessons.into_iter().map(LessonRowDto::from).collect(),
            lesson_groups: detail
                .lesson_groups
                .into_iter()
                .map(|group| LessonSectionGroupDto {
                    section: group.section,
                    lessons: group.lessons.into_iter().map(LessonRowDto::from).collect(),
                })
                .collect(),
            proposal: detail.proposal.map(|p| OrderProposalDto {
                proposal_id: p.proposal_id,
                current_titles: p.current_titles,
                proposed_titles: p.proposed_titles,
            }),
        }
    }
}

impl From<vault_core::course::LessonView> for LessonRowDto {
    fn from(lesson: vault_core::course::LessonView) -> Self {
        Self {
            lesson_id: lesson.lesson_id,
            position: lesson.position,
            title: lesson.title,
            kind: match lesson.kind {
                LessonKind::LocalFile => LessonKindDto::LocalFile,
                LessonKind::PlaylistItem => LessonKindDto::PlaylistItem,
            },
            status: lesson_status_dto(lesson.status),
            duration_ms: lesson.duration_ms,
            resume_position_ms: lesson.resume_position_ms,
            max_position_reached_ms: lesson.max_position_reached_ms,
            document_position: lesson.document_position,
            playlist_item_id: lesson.playlist_item_id,
            playlist_url: lesson.playlist_url,
            local_media_path: lesson.local_media_path,
            thumbnail_path: lesson.thumbnail_path,
        }
    }
}

fn lesson_status_dto(status: LessonStatus) -> LessonStatusDto {
    match status {
        LessonStatus::NotStarted => LessonStatusDto::NotStarted,
        LessonStatus::InProgress => LessonStatusDto::InProgress,
        LessonStatus::Done => LessonStatusDto::Done,
        LessonStatus::Broken => LessonStatusDto::Broken,
    }
}

fn derived_status_dto(status: vault_core::DerivedStatus) -> DerivedStatusDto {
    match status {
        vault_core::DerivedStatus::NotStarted => DerivedStatusDto::NotStarted,
        vault_core::DerivedStatus::InProgress => DerivedStatusDto::InProgress,
        vault_core::DerivedStatus::Done => DerivedStatusDto::Done,
        vault_core::DerivedStatus::Broken => DerivedStatusDto::Broken,
    }
}
