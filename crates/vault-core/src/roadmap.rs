//! Roadmap steps, ordering, derived progress, and flowchart graph data (R-01–R-04).

use std::collections::{HashMap, HashSet};

use crate::db::DbError;
use crate::db::VaultDb;
use crate::order::InvalidProposalError;
use crate::read_model::course_progress;
use crate::rollup::derive_roadmap_status;
use crate::time::{course_remaining_time, roadmap_remaining_time};
use crate::DerivedStatus;

#[derive(Debug, thiserror::Error)]
pub enum RoadmapError {
    #[error(transparent)]
    Db(#[from] DbError),
    #[error("roadmap not found")]
    RoadmapNotFound,
    #[error("roadmap step not found")]
    StepNotFound,
    #[error("course not found: {0}")]
    CourseNotFound(String),
    #[error("invalid step order: {0}")]
    InvalidStepOrder(InvalidProposalError),
    #[error("Prerequisite edge would create a cycle.")]
    PrerequisiteCycle,
    #[error("prerequisite edge not found")]
    PrerequisiteEdgeNotFound,
    #[error("prerequisite steps must belong to the same roadmap")]
    PrerequisiteRoadmapMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoadmapEdgeKind {
    StepOrder,
    Prerequisite,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoadmapGraphNode {
    pub step_id: String,
    pub course_title: String,
    pub status: DerivedStatus,
    pub remaining_ms: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoadmapGraphEdge {
    pub from_step_id: String,
    pub to_step_id: String,
    pub kind: RoadmapEdgeKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoadmapGraph {
    pub nodes: Vec<RoadmapGraphNode>,
    pub edges: Vec<RoadmapGraphEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoadmapStepView {
    pub id: String,
    pub course_id: Option<String>,
    pub course_title: String,
    pub note: Option<String>,
    pub status: DerivedStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoadmapListEntry {
    pub roadmap_id: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoadmapDetail {
    pub roadmap_id: String,
    pub title: String,
    pub status: DerivedStatus,
    pub remaining_ms: i64,
    pub unknown_duration_count: u32,
    pub steps: Vec<RoadmapStepView>,
    pub graph: RoadmapGraph,
}

pub fn list_roadmaps(db: &VaultDb) -> Result<Vec<RoadmapListEntry>, RoadmapError> {
    Ok(db
        .list_roadmaps()
        .map_err(RoadmapError::from)?
        .into_iter()
        .map(|row| RoadmapListEntry {
            roadmap_id: row.id,
            title: row.title,
        })
        .collect())
}

pub fn create_roadmap(db: &VaultDb, title: &str) -> Result<String, RoadmapError> {
    db.insert_roadmap(title).map_err(RoadmapError::from)
}

pub fn roadmap_detail(db: &VaultDb, roadmap_id: &str) -> Result<RoadmapDetail, RoadmapError> {
    let title = db
        .roadmap_title(roadmap_id)?
        .ok_or(RoadmapError::RoadmapNotFound)?;
    let status = roadmap_progress(db, roadmap_id)?;
    let remaining = roadmap_remaining_time(db, roadmap_id)?;
    let steps = roadmap_steps(db, roadmap_id)?;
    let graph = roadmap_graph(db, roadmap_id)?;
    Ok(RoadmapDetail {
        roadmap_id: roadmap_id.to_string(),
        title,
        status,
        remaining_ms: remaining.remaining_ms,
        unknown_duration_count: remaining.unknown_lessons,
        steps,
        graph,
    })
}

pub fn update_roadmap_step_note(
    db: &VaultDb,
    step_id: &str,
    note: Option<&str>,
) -> Result<(), RoadmapError> {
    if !db.update_roadmap_step_note(step_id, note)? {
        return Err(RoadmapError::StepNotFound);
    }
    Ok(())
}

pub fn add_roadmap_step(
    db: &VaultDb,
    roadmap_id: &str,
    course_id: &str,
    note: Option<&str>,
) -> Result<String, RoadmapError> {
    if !db.course_exists(course_id)? || db.course_is_archived(course_id)? {
        return Err(RoadmapError::CourseNotFound(course_id.to_string()));
    }
    db.insert_roadmap_step(roadmap_id, course_id, note)
        .map_err(RoadmapError::from)
}

pub fn reorder_roadmap_steps(
    db: &VaultDb,
    roadmap_id: &str,
    step_ids: &[String],
) -> Result<(), RoadmapError> {
    validate_step_order(db, roadmap_id, step_ids)?;
    db.apply_roadmap_step_order(roadmap_id, step_ids)
        .map_err(RoadmapError::from)
}

pub fn remove_roadmap_step(db: &VaultDb, step_id: &str) -> Result<(), RoadmapError> {
    if !db.delete_roadmap_step(step_id)? {
        return Err(RoadmapError::StepNotFound);
    }
    Ok(())
}

/// Point a roadmap step at a library course (e.g. after the previous course was deleted).
pub fn relink_roadmap_step(
    db: &VaultDb,
    step_id: &str,
    course_id: &str,
) -> Result<(), RoadmapError> {
    if db.roadmap_id_for_step(step_id)?.is_none() {
        return Err(RoadmapError::StepNotFound);
    }
    if !db.course_exists(course_id)? || db.course_is_archived(course_id)? {
        return Err(RoadmapError::CourseNotFound(course_id.to_string()));
    }
    if !db.update_roadmap_step_course_id(step_id, course_id)? {
        return Err(RoadmapError::StepNotFound);
    }
    Ok(())
}

/// Prerequisite: `from_step_id` should be completed before `to_step_id`.
pub fn add_prerequisite_edge(
    db: &VaultDb,
    from_step_id: &str,
    to_step_id: &str,
) -> Result<(), RoadmapError> {
    let from_roadmap = db.roadmap_id_for_step(from_step_id)?;
    let to_roadmap = db.roadmap_id_for_step(to_step_id)?;
    let (Some(from_roadmap), Some(to_roadmap)) = (from_roadmap, to_roadmap) else {
        return Err(RoadmapError::StepNotFound);
    };
    if from_roadmap != to_roadmap {
        return Err(RoadmapError::PrerequisiteRoadmapMismatch);
    }
    if would_create_cycle(db, &from_roadmap, from_step_id, to_step_id)? {
        return Err(RoadmapError::PrerequisiteCycle);
    }
    db.insert_prerequisite_edge(from_step_id, to_step_id)
        .map_err(RoadmapError::from)
}

pub fn remove_prerequisite_edge(
    db: &VaultDb,
    from_step_id: &str,
    to_step_id: &str,
) -> Result<(), RoadmapError> {
    let from_roadmap = db.roadmap_id_for_step(from_step_id)?;
    let to_roadmap = db.roadmap_id_for_step(to_step_id)?;
    match (from_roadmap, to_roadmap) {
        (None, _) | (_, None) => Err(RoadmapError::StepNotFound),
        (Some(a), Some(b)) if a != b => Err(RoadmapError::PrerequisiteRoadmapMismatch),
        (Some(_), Some(_)) => {
            if !db.delete_prerequisite_edge(from_step_id, to_step_id)? {
                return Err(RoadmapError::PrerequisiteEdgeNotFound);
            }
            Ok(())
        }
    }
}

pub fn roadmap_graph(db: &VaultDb, roadmap_id: &str) -> Result<RoadmapGraph, RoadmapError> {
    if !db.roadmap_exists(roadmap_id)? {
        return Err(RoadmapError::RoadmapNotFound);
    }
    let rows = db.roadmap_graph_step_rows(roadmap_id)?;
    let step_ids = rows.iter().map(|r| r.step_id.clone()).collect::<Vec<_>>();

    let mut nodes = Vec::with_capacity(rows.len());
    for row in rows {
        let status = step_status_for_course(db, row.course_id.as_deref())?;
        let remaining_ms = node_remaining_ms(db, row.course_id.as_deref())?;
        nodes.push(RoadmapGraphNode {
            step_id: row.step_id,
            course_title: row.course_title,
            status,
            remaining_ms,
        });
    }

    let mut edges = step_order_edges(&step_ids);
    for (from, to) in db.prerequisite_edges_for_roadmap(roadmap_id)? {
        edges.push(RoadmapGraphEdge {
            from_step_id: from,
            to_step_id: to,
            kind: RoadmapEdgeKind::Prerequisite,
        });
    }

    Ok(RoadmapGraph { nodes, edges })
}

pub fn roadmap_steps(db: &VaultDb, roadmap_id: &str) -> Result<Vec<RoadmapStepView>, RoadmapError> {
    let rows = db.roadmap_steps_ordered(roadmap_id)?;
    rows.into_iter()
        .map(|row| {
            let status = step_status_for_course(db, row.course_id.as_deref())?;
            Ok(RoadmapStepView {
                id: row.id,
                course_id: row.course_id,
                course_title: row.course_title,
                note: row.note,
                status,
            })
        })
        .collect()
}

pub fn step_status(db: &VaultDb, step_id: &str) -> Result<DerivedStatus, RoadmapError> {
    match db.roadmap_step_course_id(step_id)? {
        None => Err(RoadmapError::StepNotFound),
        Some(course_id) => step_status_for_course(db, course_id.as_deref()),
    }
}

pub fn roadmap_progress(db: &VaultDb, roadmap_id: &str) -> Result<DerivedStatus, RoadmapError> {
    let steps = roadmap_steps(db, roadmap_id)?;
    Ok(derive_roadmap_status(steps.into_iter().map(|s| s.status)))
}

/// Remove a course and leave roadmap steps with a null `course_id` (broken step).
pub fn delete_course(db: &VaultDb, course_id: &str) -> Result<(), RoadmapError> {
    if !db.delete_course(course_id)? {
        return Err(RoadmapError::CourseNotFound(course_id.to_string()));
    }
    Ok(())
}

fn step_status_for_course(
    db: &VaultDb,
    course_id: Option<&str>,
) -> Result<DerivedStatus, RoadmapError> {
    match course_id {
        None => Ok(DerivedStatus::Broken),
        Some(id) => {
            if !db.course_exists(id)? {
                return Ok(DerivedStatus::Broken);
            }
            course_progress(db, id).map_err(RoadmapError::from)
        }
    }
}

fn validate_step_order(
    db: &VaultDb,
    roadmap_id: &str,
    step_ids: &[String],
) -> Result<(), RoadmapError> {
    let current = db.roadmap_step_ids_in_order(roadmap_id)?;
    let current_set: HashSet<&str> = current.iter().map(String::as_str).collect();

    if step_ids.len() != step_ids.iter().collect::<HashSet<_>>().len() {
        return Err(RoadmapError::InvalidStepOrder(
            InvalidProposalError::DuplicateLesson,
        ));
    }
    if step_ids.len() != current_set.len() {
        return Err(RoadmapError::InvalidStepOrder(
            InvalidProposalError::LessonSetMismatch,
        ));
    }
    for id in step_ids {
        if !current_set.contains(id.as_str()) {
            return Err(RoadmapError::InvalidStepOrder(
                InvalidProposalError::LessonSetMismatch,
            ));
        }
    }
    Ok(())
}

fn node_remaining_ms(db: &VaultDb, course_id: Option<&str>) -> Result<Option<i64>, RoadmapError> {
    match course_id {
        None => Ok(None),
        Some(id) => {
            let estimate = course_remaining_time(db, id).map_err(RoadmapError::from)?;
            if estimate.unknown_lessons > 0 {
                Ok(None)
            } else {
                Ok(Some(estimate.remaining_ms))
            }
        }
    }
}

fn step_order_edges(step_ids: &[String]) -> Vec<RoadmapGraphEdge> {
    let mut edges = Vec::new();
    for pair in step_ids.windows(2) {
        edges.push(RoadmapGraphEdge {
            from_step_id: pair[0].clone(),
            to_step_id: pair[1].clone(),
            kind: RoadmapEdgeKind::StepOrder,
        });
    }
    edges
}

fn would_create_cycle(
    db: &VaultDb,
    roadmap_id: &str,
    from_step_id: &str,
    to_step_id: &str,
) -> Result<bool, RoadmapError> {
    let step_ids = db.roadmap_step_ids_in_order(roadmap_id)?;
    let mut edges = db.prerequisite_edges_for_roadmap(roadmap_id)?;
    edges.push((from_step_id.to_string(), to_step_id.to_string()));
    Ok(directed_graph_has_cycle(&step_ids, &edges))
}

fn directed_graph_has_cycle(nodes: &[String], edges: &[(String, String)]) -> bool {
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for node in nodes {
        adj.entry(node.clone()).or_default();
    }
    for (from, to) in edges {
        adj.entry(from.clone()).or_default().push(to.clone());
    }

    let mut visiting = HashSet::new();
    let mut done = HashSet::new();

    fn dfs(
        node: &str,
        adj: &HashMap<String, Vec<String>>,
        visiting: &mut HashSet<String>,
        done: &mut HashSet<String>,
    ) -> bool {
        if done.contains(node) {
            return false;
        }
        if visiting.contains(node) {
            return true;
        }
        visiting.insert(node.to_string());
        for next in adj.get(node).into_iter().flat_map(|v| v.iter()) {
            if dfs(next, adj, visiting, done) {
                return true;
            }
        }
        visiting.remove(node);
        done.insert(node.to_string());
        false
    }

    for node in nodes {
        if !done.contains(node) && dfs(node, &adj, &mut visiting, &mut done) {
            return true;
        }
    }
    false
}
