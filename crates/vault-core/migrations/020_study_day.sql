CREATE TABLE study_day (
    local_date TEXT PRIMARY KEY NOT NULL CHECK (local_date GLOB '[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]')
);
