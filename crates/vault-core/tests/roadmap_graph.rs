use vault_core::roadmap::{
    add_prerequisite_edge, add_roadmap_step, create_roadmap, remove_prerequisite_edge,
    remove_roadmap_step, roadmap_graph, RoadmapEdgeKind, RoadmapError,
};
use vault_core::{DerivedStatus, VaultDb};

fn course(db: &VaultDb, title: &str) -> String {
    let root = format!("/courses/{title}");
    db.insert_course(title, &root).unwrap()
}

#[test]
fn graph_nodes_and_edges() {
    let db = VaultDb::open_in_memory().unwrap();
    let course_a = course(&db, "Linear Algebra");
    let course_b = course(&db, "Probability");
    let roadmap_id = create_roadmap(&db, "ML path").unwrap();
    let step_a = add_roadmap_step(&db, &roadmap_id, &course_a, None).unwrap();
    let step_b = add_roadmap_step(&db, &roadmap_id, &course_b, None).unwrap();

    add_prerequisite_edge(&db, &step_a, &step_b).unwrap();

    let graph = roadmap_graph(&db, &roadmap_id).unwrap();
    assert_eq!(graph.nodes.len(), 2);

    let node_a = graph.nodes.iter().find(|n| n.step_id == step_a).unwrap();
    assert_eq!(node_a.course_title, "Linear Algebra");
    assert_eq!(node_a.status, DerivedStatus::NotStarted);
    assert_eq!(node_a.remaining_ms, Some(0));

    assert_eq!(graph.edges.len(), 2);
    assert!(graph.edges.iter().any(|e| {
        e.from_step_id == step_a && e.to_step_id == step_b && e.kind == RoadmapEdgeKind::StepOrder
    }));
    assert!(graph.edges.iter().any(|e| {
        e.from_step_id == step_a
            && e.to_step_id == step_b
            && e.kind == RoadmapEdgeKind::Prerequisite
    }));
}

#[test]
fn rejects_prerequisite_cycle() {
    let db = VaultDb::open_in_memory().unwrap();
    let roadmap_id = create_roadmap(&db, "Cycle").unwrap();
    let s0 = add_roadmap_step(&db, &roadmap_id, &course(&db, "A"), None).unwrap();
    let s1 = add_roadmap_step(&db, &roadmap_id, &course(&db, "B"), None).unwrap();
    let s2 = add_roadmap_step(&db, &roadmap_id, &course(&db, "C"), None).unwrap();

    add_prerequisite_edge(&db, &s0, &s1).unwrap();
    add_prerequisite_edge(&db, &s1, &s2).unwrap();
    let err = add_prerequisite_edge(&db, &s2, &s0).unwrap_err();
    assert!(matches!(err, RoadmapError::PrerequisiteCycle));
    assert_eq!(err.to_string(), "Prerequisite edge would create a cycle.");
}

#[test]
fn remove_prerequisite_edge_updates_graph() {
    let db = VaultDb::open_in_memory().unwrap();
    let roadmap_id = create_roadmap(&db, "Edges").unwrap();
    let s0 = add_roadmap_step(&db, &roadmap_id, &course(&db, "A"), None).unwrap();
    let s1 = add_roadmap_step(&db, &roadmap_id, &course(&db, "B"), None).unwrap();
    add_prerequisite_edge(&db, &s0, &s1).unwrap();

    let before = roadmap_graph(&db, &roadmap_id).unwrap();
    assert_eq!(
        before
            .edges
            .iter()
            .filter(|e| e.kind == RoadmapEdgeKind::Prerequisite)
            .count(),
        1
    );

    remove_prerequisite_edge(&db, &s0, &s1).unwrap();
    let after = roadmap_graph(&db, &roadmap_id).unwrap();
    assert!(after
        .edges
        .iter()
        .all(|e| e.kind != RoadmapEdgeKind::Prerequisite));
}

#[test]
fn remove_step_updates_edges() {
    let db = VaultDb::open_in_memory().unwrap();
    let roadmap_id = create_roadmap(&db, "Trim").unwrap();
    let s0 = add_roadmap_step(&db, &roadmap_id, &course(&db, "A"), None).unwrap();
    let s1 = add_roadmap_step(&db, &roadmap_id, &course(&db, "B"), None).unwrap();
    let s2 = add_roadmap_step(&db, &roadmap_id, &course(&db, "C"), None).unwrap();
    add_prerequisite_edge(&db, &s0, &s2).unwrap();

    remove_roadmap_step(&db, &s1).unwrap();
    let graph = roadmap_graph(&db, &roadmap_id).unwrap();
    assert_eq!(graph.nodes.len(), 2);
    assert!(graph
        .nodes
        .iter()
        .all(|n| n.step_id == s0 || n.step_id == s2));
    assert!(graph
        .edges
        .iter()
        .all(|e| e.from_step_id != s1 && e.to_step_id != s1));
    assert_eq!(graph.edges.len(), 2);
}
