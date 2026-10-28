CREATE TABLE roadmap (
    id TEXT PRIMARY KEY NOT NULL,
    library_id TEXT NOT NULL REFERENCES library (id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE roadmap_step (
    id TEXT PRIMARY KEY NOT NULL,
    roadmap_id TEXT NOT NULL REFERENCES roadmap (id) ON DELETE CASCADE,
    course_id TEXT REFERENCES course (id) ON DELETE SET NULL,
    note TEXT,
    sort_index INTEGER NOT NULL
);

CREATE INDEX idx_roadmap_step_roadmap_sort ON roadmap_step (roadmap_id, sort_index);
