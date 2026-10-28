ALTER TABLE vault_settings
ADD COLUMN player_volume REAL NOT NULL DEFAULT 1 CHECK (
    player_volume >= 0 AND player_volume <= 1
);

ALTER TABLE vault_settings
ADD COLUMN player_muted INTEGER NOT NULL DEFAULT 0 CHECK (player_muted IN (0, 1));
