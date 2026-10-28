ALTER TABLE vault_settings
ADD COLUMN ui_theme TEXT NOT NULL DEFAULT 'light' CHECK (ui_theme IN ('light', 'dark'));
