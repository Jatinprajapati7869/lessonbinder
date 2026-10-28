CREATE TABLE anki_card_link (
    id TEXT PRIMARY KEY NOT NULL,
    lesson_id TEXT NOT NULL REFERENCES lesson (id) ON DELETE CASCADE,
    timestamp_ms INTEGER NOT NULL,
    front_hash TEXT NOT NULL,
    anki_note_id INTEGER NOT NULL,
    link_status TEXT NOT NULL DEFAULT 'active' CHECK (link_status IN ('active', 'broken')),
    created_at TEXT NOT NULL,
    UNIQUE (lesson_id, timestamp_ms, front_hash)
);
