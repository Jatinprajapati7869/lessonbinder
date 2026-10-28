mod app_state;
mod backup_commands;
mod card_commands;
mod command_failure;
mod commands;
mod course_commands;
mod folder_scan_guard;
mod folder_scan_progress;
mod library_commands;
mod mpv_commands;
mod report_commands;
mod roadmap_commands;
mod settings_commands;
mod study_commands;
mod vault_paths;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
            app.manage(commands::init_app_state(app_data_dir));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::vault_bootstrap,
            commands::retry_open_vault,
            commands::vault_db_info,
            commands::study_bar_state,
            commands::launch_anki_profile,
            study_commands::review_next_card,
            study_commands::review_next_card_for_course,
            study_commands::review_note_back,
            study_commands::review_answer_card,
            study_commands::review_read_media,
            library_commands::list_library_courses,
            library_commands::get_library_continue_row,
            library_commands::get_library_study_streak_days,
            library_commands::get_library_study_heatmap_52_weeks,
            report_commands::get_weekly_report,
            library_commands::search_library,
            library_commands::get_search_lesson_note_body,
            library_commands::path_is_directory,
            library_commands::validate_import_folder_path,
            library_commands::import_course_folder,
            library_commands::cancel_import_course_folder,
            library_commands::open_course_folder_in_file_manager,
            library_commands::import_playlist_json,
            library_commands::import_playlist_paste,
            settings_commands::get_vault_settings,
            settings_commands::save_vault_settings_entry,
            settings_commands::get_anki_profile_path,
            backup_commands::backup_vault,
            backup_commands::restore_vault,
            course_commands::get_course_detail,
            course_commands::list_lesson_notes,
            course_commands::create_lesson_note_entry,
            course_commands::update_lesson_note_entry,
            course_commands::delete_lesson_note_entry,
            course_commands::export_lesson_notes_markdown,
            course_commands::export_course_notes_markdown,
            course_commands::list_chapters,
            course_commands::list_lesson_cues,
            course_commands::list_lesson_resources_entry,
            course_commands::list_lesson_card_links_entry,
            course_commands::unlink_lesson_card_entry,
            course_commands::capture_lesson_thumbnail,
            course_commands::get_transcription_model_path,
            course_commands::transcribe_lesson,
            course_commands::list_lesson_watched_ranges,
            course_commands::record_lesson_playback,
            course_commands::mark_lesson_complete,
            course_commands::mark_lesson_incomplete,
            course_commands::save_document_position,
            course_commands::save_lesson_resume,
            course_commands::accept_order_proposal,
            course_commands::reject_order_proposal,
            course_commands::up_next,
            course_commands::reorder_course_lessons,
            course_commands::set_playlist_item_watched,
            course_commands::mark_lesson_broken,
            course_commands::guard_lesson_media_for_resume,
            course_commands::open_lesson_file_directory_in_file_manager,
            course_commands::rescan_course,
            course_commands::archive_course_entry,
            course_commands::unarchive_course_entry,
            roadmap_commands::list_roadmap_entries,
            roadmap_commands::create_roadmap_entry,
            roadmap_commands::get_roadmap_detail,
            roadmap_commands::add_roadmap_step_entry,
            roadmap_commands::reorder_roadmap_step_entries,
            roadmap_commands::update_roadmap_step_note_entry,
            roadmap_commands::add_prerequisite_edge_entry,
            roadmap_commands::remove_prerequisite_edge_entry,
            roadmap_commands::relink_roadmap_step_entry,
            roadmap_commands::export_roadmap_graph_svg,
            card_commands::capture_ocr_status,
            card_commands::build_capture_draft,
            card_commands::build_subtitle_cue_draft,
            card_commands::build_lesson_note_draft,
            card_commands::commit_subtitle_cue_card,
            card_commands::commit_lesson_note_card,
            card_commands::build_audio_clip_draft,
            card_commands::commit_audio_clip_card,
            card_commands::commit_capture_card,
            mpv_commands::start_mpv_lesson_handoff,
            mpv_commands::update_mpv_lesson_embed_rect,
            mpv_commands::stop_mpv_lesson_handoff,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Lessonbinder");
}
