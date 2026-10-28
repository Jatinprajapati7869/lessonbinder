-- Lessonbinder vault.db schema v1 (domain entities: library → course → source_file → lesson → progress)

CREATE TABLE library (
    id TEXT PRIMARY KEY NOT NULL,
    display_name TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE course (
    id TEXT PRIMARY KEY NOT NULL,
    library_id TEXT NOT NULL REFERENCES library (id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    source_kind TEXT NOT NULL DEFAULT 'folder' CHECK (source_kind IN ('folder', 'playlist')),
    root_path TEXT,
    playlist_url TEXT,
    imported_at TEXT NOT NULL,
    last_scanned_at TEXT NOT NULL,
    UNIQUE (root_path)
);

CREATE TABLE source_file (
    id TEXT PRIMARY KEY NOT NULL,
    course_id TEXT NOT NULL REFERENCES course (id) ON DELETE CASCADE,
    relative_path TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (
        kind IN ('video', 'audio', 'pdf', 'text', 'subtitle')
    ),
    extension TEXT NOT NULL,
    size_bytes INTEGER,
    last_seen_at TEXT NOT NULL,
    missing_since TEXT,
    UNIQUE (course_id, relative_path)
);

CREATE TABLE lesson (
    id TEXT PRIMARY KEY NOT NULL,
    course_id TEXT NOT NULL REFERENCES course (id) ON DELETE CASCADE,
    lesson_stable_key TEXT NOT NULL,
    display_title TEXT NOT NULL,
    sort_index INTEGER NOT NULL,
    source_kind TEXT NOT NULL DEFAULT 'local_file' CHECK (source_kind IN ('local_file', 'playlist_item')),
    source_file_id TEXT REFERENCES source_file (id),
    playlist_item_id TEXT,
    subtitle_file_id TEXT REFERENCES source_file (id),
    UNIQUE (course_id, lesson_stable_key),
    CHECK (
        (
            source_kind = 'local_file'
            AND source_file_id IS NOT NULL
            AND playlist_item_id IS NULL
        )
        OR (
            source_kind = 'playlist_item'
            AND playlist_item_id IS NOT NULL
            AND source_file_id IS NULL
        )
    )
);

CREATE TABLE progress (
    lesson_id TEXT PRIMARY KEY NOT NULL REFERENCES lesson (id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'not_started' CHECK (
        status IN ('not_started', 'in_progress', 'done', 'broken')
    ),
    resume_position_ms INTEGER NOT NULL DEFAULT 0,
    max_position_reached_ms INTEGER NOT NULL DEFAULT 0,
    marked_done_at TEXT,
    last_engaged_at TEXT
);

CREATE TRIGGER lesson_progress_after_insert
AFTER INSERT ON lesson
BEGIN
    INSERT INTO progress (lesson_id) VALUES (NEW.id);
END;
