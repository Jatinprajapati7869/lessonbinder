use serde::Serialize;
use tauri::State;
use vault_core::study_day::machine_local_date_string;
use vault_core::weekly_report::current_weekly_report;

use crate::app_state::AppState;

#[derive(Debug, Clone, Serialize)]
pub struct WeeklyReportDto {
    pub week_monday: String,
    pub week_sunday: String,
    pub lessons_marked_done: u32,
    pub cards_graded: u32,
    pub watched_minutes: Option<u32>,
}

#[tauri::command]
pub fn get_weekly_report(state: State<AppState>) -> Result<WeeklyReportDto, String> {
    state.with_vault(|vault| {
        let report = current_weekly_report(vault, &machine_local_date_string())
            .map_err(|e| e.to_string())?;
        Ok(WeeklyReportDto {
            week_monday: report.week_monday,
            week_sunday: report.week_sunday,
            lessons_marked_done: report.lessons_marked_done,
            cards_graded: report.cards_graded,
            watched_minutes: report.watched_minutes,
        })
    })
}
