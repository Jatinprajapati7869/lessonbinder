-- Keep anki_card_link rows when lessons are removed (SPEC A-03: link_status broken).

PRAGMA foreign_keys=OFF;

CREATE TABLE anki_card_link_new (
    id TEXT PRIMARY KEY NOT NULL,
    lesson_id TEXT REFERENCES lesson (id) ON DELETE SET NULL,
    timestamp_ms INTEGER NOT NULL,
    front_hash TEXT NOT NULL,
    anki_note_id INTEGER NOT NULL,
    link_status TEXT NOT NULL DEFAULT 'active' CHECK (link_status IN ('active', 'broken')),
    created_at TEXT NOT NULL,
    UNIQUE (lesson_id, timestamp_ms, front_hash)
);

INSERT INTO anki_card_link_new (
    id, lesson_id, timestamp_ms, front_hash, anki_note_id, link_status, created_at
)
SELECT id, lesson_id, timestamp_ms, front_hash, anki_note_id, link_status, created_at
FROM anki_card_link;

DROP TABLE anki_card_link;

ALTER TABLE anki_card_link_new RENAME TO anki_card_link;

PRAGMA foreign_keys=ON;
