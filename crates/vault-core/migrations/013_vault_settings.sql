CREATE TABLE vault_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    tesseract_language TEXT NOT NULL DEFAULT 'eng',
    transcription_model_path TEXT NOT NULL DEFAULT '',
    anki_desktop_binary_path TEXT NOT NULL DEFAULT 'anki'
);

INSERT INTO vault_settings (id) VALUES (1);
