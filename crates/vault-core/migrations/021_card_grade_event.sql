CREATE TABLE card_grade_event (
    graded_at TEXT NOT NULL CHECK (graded_at GLOB '[0-9]*')
);

CREATE INDEX card_grade_event_graded_at ON card_grade_event (graded_at);
