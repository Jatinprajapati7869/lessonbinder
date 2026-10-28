//! Lessonbinder domain logic and `vault.db` access. No Anki dependency.

pub mod anki_link;
pub mod audio_clip;
pub mod card;
pub mod chapters;
pub mod clip_encoder;
pub mod continue_learning;
pub mod course;
pub mod course_archive;
pub mod course_delete;
pub mod cues;
pub mod db;
pub mod extract;
pub mod import;
pub mod lesson_note;
pub mod lesson_note_export;
pub mod lesson_resource;
pub mod lessonbinderignore;
pub mod library;
pub mod ocr;
pub mod order;
pub mod playlist;
pub mod progress;
mod progress_persist;
pub mod read_model;
pub mod roadmap;
pub mod rollup;
pub mod search;
pub mod sections;
pub mod study_day;
pub mod thumbnail;
pub mod time;
pub mod transcription;
pub mod up_next;
pub mod vault_settings;
pub mod watched;
pub mod weekly_report;
pub mod whisper_cpp;

pub use anki_link::anki_card_link_for_selection;
pub use anki_link::{
    anki_card_link, anki_note_ids_for_course, insert_anki_card_link, list_lesson_card_links,
    unlink_card, AnkiCardLinkRecord, AnkiLinkError, LessonCardLinkEntry, LinkStatus,
};
pub use audio_clip::{
    audio_clip_range_ms, encode_lesson_audio_clip, ensure_local_clip_source,
    select_audio_clip_range, AudioClipError, AudioClipRange, PLAYHEAD_FALLBACK_CLIP_MS,
};
pub use card::{
    build_card_draft, card_draft_from_audio_clip, card_draft_from_edit,
    card_draft_from_lesson_note, card_draft_from_subtitle_cue, format_timestamp_ms,
    hash_front_text, lesson_exists, CardDraft, CardError,
};
pub use chapters::{
    extract_media_chapters, list_chapters, parse_ffprobe_chapters_json, refresh_lesson_chapters,
    ChapterError, ChapterIndexOutcome, LessonChapters, MediaChapter,
};
pub use clip_encoder::{
    validate_clip_range, ClipEncoder, ClipEncoderError, FakeClipEncoder, FfmpegClipEncoder,
    MAX_CLIP_DURATION_MS,
};
pub use continue_learning::{
    continue_target, library_continue_row, ContinueTarget, LibraryContinueRow,
};
pub use course::{course_detail, CourseDetail, LessonKind, LessonView, OrderProposalView};
pub use course_archive::{archive_course, unarchive_course, CourseArchiveError};
pub use cues::{list_cues, list_player_cues, parse_subtitle_file, CueError, SubtitleCue};
pub use db::LessonStatus;
pub use db::VaultDb;
pub use extract::{
    extract_first_heading, extract_media_duration, extract_pdf_page_count, extract_subtitle_cues,
    index_lesson_metadata, ExtractorEnv, ExtractorOutcome,
};
pub use import::{
    import_folder, import_folder_with_cancel, rescan, rescan_with_cancel,
    validate_import_folder_path, FolderScanProgress, ImportError, ImportReport, RescanError,
};
pub use lesson_note::{
    create_lesson_note, delete_lesson_note, list_lesson_notes, update_lesson_note, LessonNote,
    LessonNoteError,
};
pub use lesson_note_export::{
    course_notes_markdown, format_lesson_notes_markdown, lesson_notes_markdown,
};
pub use lesson_resource::{list_lesson_resources, LessonResourceView};
pub use library::{list_courses, CourseLibraryVisibility, CourseListEntry, CourseSourceKind};
pub use ocr::{FakeOcrEngine, OcrEngine, OcrError, OcrResult, TesseractOcrEngine};
pub use order::{
    accept_proposal, learning_order, propose_order, reject_proposal, reorder_lessons,
    InvalidProposalError, OrderError,
};
pub use playlist::{
    import_playlist_definition, import_playlist_from_pasted_lines, parse_playlist_definition_json,
    parse_playlist_lines, set_watched, ParsedPlaylistLine, PlaylistDefinition, PlaylistError,
    PlaylistItemDefinition,
};
pub use progress::{
    apply_mark_complete, apply_mark_incomplete, apply_playback_starts, apply_set_resume,
    apply_source_missing, apply_source_restored, LessonProgressState, ProgressError,
};
pub use progress_persist::record_playback;
pub use progress_persist::set_document_position;
pub use progress_persist::ProgressPersistenceError;
pub use read_model::{
    course_progress, guard_local_media_for_resume, library_summary, refresh_course_sources,
    refresh_sources, LibrarySummary,
};
pub use roadmap::{
    add_prerequisite_edge, add_roadmap_step, create_roadmap, delete_course, list_roadmaps,
    relink_roadmap_step, remove_prerequisite_edge, remove_roadmap_step, reorder_roadmap_steps,
    roadmap_detail, roadmap_graph, roadmap_progress, roadmap_steps, step_status,
    update_roadmap_step_note, RoadmapEdgeKind, RoadmapError, RoadmapGraph, RoadmapGraphEdge,
    RoadmapGraphNode, RoadmapStepView,
};
pub use rollup::{derive_course_status, derive_roadmap_status};
pub use search::{lesson_note_body, search, SearchHit, SearchMatchKind};
pub use sections::{
    folder_lesson_section, group_lessons_by_section, list_sections, LessonSectionGroup,
    SectionError,
};
pub use thumbnail::{
    capture_thumbnail, persist_lesson_thumbnail, ThumbnailCapturer, ThumbnailError,
};
pub use time::{course_remaining_time, roadmap_remaining_time, RemainingStudyTime};
pub use transcription::{
    list_transcription_cues, run_lesson_transcription, transcription_disabled,
    transcription_engine_from_settings, DisabledTranscriptionEngine, FakeTranscriptionEngine,
    SettingsTranscriptionEngine, TranscriptionEngine, TranscriptionError,
};
pub use up_next::{up_next, UpNextError, UpNextTarget};
pub use vault_settings::{
    load_vault_settings, save_vault_settings, PlaybackSpeed, SettingsError, UiTheme, VaultSettings,
    DEFAULT_PLAYER_VOLUME, PLAYBACK_SPEED_CHOICES,
};
pub use watched::{watched_ms, watched_percent, PlaybackRangeError, WatchedRange};
pub use whisper_cpp::{
    parse_whisper_json_output, resolve_whisper_cpp_binary, resolve_whisper_cpp_binary_in,
    whisper_cli_argv, WhisperCppError, WHISPER_CPP_BINARY,
};

/// Computed course / roadmap status (same values as [`LessonStatus`], not stored on lessons).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedStatus {
    NotStarted,
    InProgress,
    Done,
    Broken,
}
