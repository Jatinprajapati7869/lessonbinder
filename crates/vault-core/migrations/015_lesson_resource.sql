CREATE TABLE lesson_resource (
    id TEXT PRIMARY KEY NOT NULL,
    lesson_id TEXT NOT NULL REFERENCES lesson (id) ON DELETE CASCADE,
    source_file_id TEXT NOT NULL REFERENCES source_file (id) ON DELETE CASCADE,
    kind TEXT NOT NULL CHECK (kind IN ('pdf', 'image', 'text')),
    UNIQUE (lesson_id, source_file_id)
);

CREATE INDEX lesson_resource_lesson_id ON lesson_resource (lesson_id);
