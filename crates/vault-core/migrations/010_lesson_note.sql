CREATE TABLE lesson_note (
    id TEXT PRIMARY KEY NOT NULL,
    lesson_id TEXT NOT NULL REFERENCES lesson (id) ON DELETE CASCADE,
    body TEXT NOT NULL,
    timestamp_ms INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX lesson_note_lesson_list_order ON lesson_note (
    lesson_id,
    timestamp_ms,
    created_at
);
