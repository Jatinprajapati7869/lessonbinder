ALTER TABLE vault_settings
ADD COLUMN playback_speed REAL NOT NULL DEFAULT 1 CHECK (
    playback_speed IN (0.75, 1, 1.25, 1.5, 2)
);
