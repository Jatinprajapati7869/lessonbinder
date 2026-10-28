CREATE TABLE watched_range (
    lesson_id TEXT NOT NULL REFERENCES lesson (id) ON DELETE CASCADE,
    start_ms INTEGER NOT NULL,
    end_ms INTEGER NOT NULL,
    CHECK (start_ms >= 0),
    CHECK (end_ms > start_ms),
    PRIMARY KEY (lesson_id, start_ms)
);

CREATE INDEX watched_range_lesson_id ON watched_range (lesson_id);
