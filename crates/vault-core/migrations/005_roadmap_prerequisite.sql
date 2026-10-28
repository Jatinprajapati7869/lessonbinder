CREATE TABLE roadmap_prerequisite_edge (
    from_step_id TEXT NOT NULL REFERENCES roadmap_step (id) ON DELETE CASCADE,
    to_step_id TEXT NOT NULL REFERENCES roadmap_step (id) ON DELETE CASCADE,
    PRIMARY KEY (from_step_id, to_step_id),
    CHECK (from_step_id != to_step_id)
);
