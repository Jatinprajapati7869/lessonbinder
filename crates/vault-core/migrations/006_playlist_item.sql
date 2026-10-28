CREATE TABLE playlist_item (
    id TEXT PRIMARY KEY NOT NULL,
    course_id TEXT NOT NULL REFERENCES course (id) ON DELETE CASCADE,
    external_id TEXT NOT NULL,
    ordinal INTEGER NOT NULL,
    url TEXT NOT NULL,
    title TEXT NOT NULL,
    duration_ms INTEGER,
    local_override_path TEXT,
    UNIQUE (course_id, external_id)
);
