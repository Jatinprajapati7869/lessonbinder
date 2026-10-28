CREATE TABLE lesson_chapter (
    id TEXT PRIMARY KEY,
    lesson_id TEXT NOT NULL REFERENCES lesson (id) ON DELETE CASCADE,
    sort_index INTEGER NOT NULL,
    start_ms INTEGER NOT NULL,
    title TEXT NOT NULL,
    source TEXT NOT NULL DEFAULT 'ffprobe' CHECK (source = 'ffprobe')
);

CREATE INDEX lesson_chapter_lesson_id ON lesson_chapter (lesson_id, sort_index);

ALTER TABLE lesson ADD COLUMN chapters_extractor TEXT;
ALTER TABLE lesson ADD COLUMN chapters_error TEXT;
