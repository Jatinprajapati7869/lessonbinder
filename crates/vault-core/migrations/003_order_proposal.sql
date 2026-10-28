CREATE TABLE order_proposal (
    id TEXT PRIMARY KEY NOT NULL,
    course_id TEXT NOT NULL REFERENCES course (id) ON DELETE CASCADE,
    created_at TEXT NOT NULL
);

CREATE TABLE order_proposal_entry (
    proposal_id TEXT NOT NULL REFERENCES order_proposal (id) ON DELETE CASCADE,
    lesson_id TEXT NOT NULL REFERENCES lesson (id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    PRIMARY KEY (proposal_id, lesson_id),
    UNIQUE (proposal_id, position)
);

CREATE UNIQUE INDEX idx_order_proposal_one_per_course ON order_proposal (course_id);
