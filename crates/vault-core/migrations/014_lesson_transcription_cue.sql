CREATE TABLE lesson_transcription_cue (
    id TEXT PRIMARY KEY NOT NULL,
    lesson_id TEXT NOT NULL REFERENCES lesson (id) ON DELETE CASCADE,
    sort_index INTEGER NOT NULL,
    start_ms INTEGER NOT NULL,
    end_ms INTEGER NOT NULL,
    text TEXT NOT NULL,
    source TEXT NOT NULL DEFAULT 'transcription' CHECK (source = 'transcription'),
    UNIQUE (lesson_id, start_ms, source)
);

CREATE INDEX lesson_transcription_cue_lesson_id ON lesson_transcription_cue (lesson_id, sort_index);
