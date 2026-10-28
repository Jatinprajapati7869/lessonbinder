use serde::Serialize;
use tauri::{AppHandle, State};
use tauri_plugin_dialog::{DialogExt, FilePath};
use vault_core::roadmap::{
    add_prerequisite_edge, add_roadmap_step, create_roadmap, list_roadmaps, relink_roadmap_step,
    remove_prerequisite_edge, reorder_roadmap_steps, roadmap_detail, update_roadmap_step_note,
    RoadmapEdgeKind, RoadmapError,
};

use crate::app_state::AppState;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivedStatusDto {
    NotStarted,
    InProgress,
    Done,
    Broken,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RoadmapEdgeKindDto {
    StepOrder,
    Prerequisite,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoadmapListEntryDto {
    pub roadmap_id: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoadmapGraphNodeDto {
    pub step_id: String,
    pub course_title: String,
    pub status: DerivedStatusDto,
    pub remaining_ms: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoadmapGraphEdgeDto {
    pub from_step_id: String,
    pub to_step_id: String,
    pub kind: RoadmapEdgeKindDto,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoadmapGraphDto {
    pub nodes: Vec<RoadmapGraphNodeDto>,
    pub edges: Vec<RoadmapGraphEdgeDto>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoadmapStepDto {
    pub step_id: String,
    pub course_id: Option<String>,
    pub course_title: String,
    pub note: Option<String>,
    pub status: DerivedStatusDto,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoadmapDetailDto {
    pub roadmap_id: String,
    pub title: String,
    pub status: DerivedStatusDto,
    pub remaining_ms: i64,
    pub unknown_duration_count: u32,
    pub steps: Vec<RoadmapStepDto>,
    pub graph: RoadmapGraphDto,
}

#[tauri::command]
pub fn list_roadmap_entries(state: State<AppState>) -> Result<Vec<RoadmapListEntryDto>, String> {
    state.with_vault(|vault| {
        let entries = list_roadmaps(vault).map_err(map_roadmap_err)?;
        Ok(entries
            .into_iter()
            .map(|e| RoadmapListEntryDto {
                roadmap_id: e.roadmap_id,
                title: e.title,
            })
            .collect())
    })
}

#[tauri::command]
pub fn create_roadmap_entry(state: State<AppState>, title: String) -> Result<String, String> {
    state.with_vault(|vault| create_roadmap(vault, &title).map_err(map_roadmap_err))
}

#[tauri::command]
pub fn get_roadmap_detail(
    state: State<AppState>,
    roadmap_id: String,
) -> Result<RoadmapDetailDto, String> {
    state.with_vault(|vault| {
        let detail = roadmap_detail(vault, &roadmap_id).map_err(map_roadmap_err)?;
        Ok(RoadmapDetailDto::from(detail))
    })
}

#[tauri::command]
pub fn add_roadmap_step_entry(
    state: State<AppState>,
    roadmap_id: String,
    course_id: String,
) -> Result<String, String> {
    state.with_vault(|vault| {
        add_roadmap_step(vault, &roadmap_id, &course_id, None).map_err(map_roadmap_err)
    })
}

#[tauri::command]
pub fn reorder_roadmap_step_entries(
    state: State<AppState>,
    roadmap_id: String,
    step_ids: Vec<String>,
) -> Result<(), String> {
    state.with_vault(|vault| {
        reorder_roadmap_steps(vault, &roadmap_id, &step_ids).map_err(map_roadmap_err)
    })
}

#[tauri::command]
pub fn update_roadmap_step_note_entry(
    state: State<AppState>,
    step_id: String,
    note: Option<String>,
) -> Result<(), String> {
    state.with_vault(|vault| {
        let note_ref = note.as_deref();
        update_roadmap_step_note(vault, &step_id, note_ref).map_err(map_roadmap_err)
    })
}

#[tauri::command]
pub fn add_prerequisite_edge_entry(
    state: State<AppState>,
    from_step_id: String,
    to_step_id: String,
) -> Result<(), String> {
    state.with_vault(|vault| {
        add_prerequisite_edge(vault, &from_step_id, &to_step_id).map_err(map_roadmap_err)
    })
}

#[tauri::command]
pub fn remove_prerequisite_edge_entry(
    state: State<AppState>,
    from_step_id: String,
    to_step_id: String,
) -> Result<(), String> {
    state.with_vault(|vault| {
        remove_prerequisite_edge(vault, &from_step_id, &to_step_id).map_err(map_roadmap_err)
    })
}

#[tauri::command]
pub fn relink_roadmap_step_entry(
    state: State<AppState>,
    step_id: String,
    course_id: String,
) -> Result<(), String> {
    state.with_vault(|vault| {
        relink_roadmap_step(vault, &step_id, &course_id).map_err(map_roadmap_err)
    })
}

#[tauri::command]
pub fn export_roadmap_graph_svg(
    app: AppHandle,
    svg: String,
    default_file_name: String,
) -> Result<(), String> {
    let picked = app
        .dialog()
        .file()
        .add_filter("SVG", &["svg"])
        .set_file_name(&default_file_name)
        .blocking_save_file();
    let Some(file_path) = picked else {
        return Ok(());
    };
    let path = match file_path {
        FilePath::Path(path) => path,
        FilePath::Url(url) => return Err(format!("unsupported save location: {url}")),
    };
    std::fs::write(&path, svg.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

impl From<vault_core::roadmap::RoadmapDetail> for RoadmapDetailDto {
    fn from(detail: vault_core::roadmap::RoadmapDetail) -> Self {
        Self {
            roadmap_id: detail.roadmap_id,
            title: detail.title,
            status: derived_status_dto(detail.status),
            remaining_ms: detail.remaining_ms,
            unknown_duration_count: detail.unknown_duration_count,
            steps: detail
                .steps
                .into_iter()
                .map(|s| RoadmapStepDto {
                    step_id: s.id,
                    course_id: s.course_id,
                    course_title: s.course_title,
                    note: s.note,
                    status: derived_status_dto(s.status),
                })
                .collect(),
            graph: RoadmapGraphDto::from(detail.graph),
        }
    }
}

impl From<vault_core::roadmap::RoadmapGraph> for RoadmapGraphDto {
    fn from(graph: vault_core::roadmap::RoadmapGraph) -> Self {
        Self {
            nodes: graph
                .nodes
                .into_iter()
                .map(|n| RoadmapGraphNodeDto {
                    step_id: n.step_id,
                    course_title: n.course_title,
                    status: derived_status_dto(n.status),
                    remaining_ms: n.remaining_ms,
                })
                .collect(),
            edges: graph
                .edges
                .into_iter()
                .map(|e| RoadmapGraphEdgeDto {
                    from_step_id: e.from_step_id,
                    to_step_id: e.to_step_id,
                    kind: match e.kind {
                        RoadmapEdgeKind::StepOrder => RoadmapEdgeKindDto::StepOrder,
                        RoadmapEdgeKind::Prerequisite => RoadmapEdgeKindDto::Prerequisite,
                    },
                })
                .collect(),
        }
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

fn map_roadmap_err(err: RoadmapError) -> String {
    err.to_string()
}
