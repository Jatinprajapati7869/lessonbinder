import { useCallback, useEffect, useState, type FormEvent } from "react";

import type { VaultSettings } from "../api/settingsTypes";
import { invokeErrorMessage } from "../api/invokeError";
import { useSettingsCommands } from "../settings/useSettingsCommands";
import { applyUiTheme } from "../theme/applyUiTheme";

const defaultSettings: VaultSettings = {
  tesseract_language: "eng",
  transcription_model_path: "",
  anki_desktop_binary_path: "anki",
  ui_theme: "light",
  playback_speed: 1,
  player_volume: 1,
  player_muted: false,
};

export function SettingsScreen() {
  const commands = useSettingsCommands();
  const [settings, setSettings] = useState<VaultSettings>(defaultSettings);
  const [ankiProfilePath, setAnkiProfilePath] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [saved, setSaved] = useState(false);
  const [busy, setBusy] = useState(false);
  const [backupMessage, setBackupMessage] = useState<string | null>(null);
  const [restoreConfirmOpen, setRestoreConfirmOpen] = useState(false);
  const [restoreFolder, setRestoreFolder] = useState<string | null>(null);

  const load = useCallback(async () => {
    setError(null);
    try {
      const [loaded, profilePath] = await Promise.all([
        commands.getVaultSettings(),
        commands.getAnkiProfilePath(),
      ]);
      setSettings(loaded);
      setAnkiProfilePath(profilePath);
      applyUiTheme(loaded.ui_theme);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not load settings.");
    }
  }, [commands]);

  useEffect(() => {
    void load();
  }, [load]);

  async function handleSave(event: FormEvent) {
    event.preventDefault();
    setBusy(true);
    setError(null);
    setSaved(false);
    try {
      await commands.saveVaultSettings(settings);
      applyUiTheme(settings.ui_theme);
      setSaved(true);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not save settings.");
    } finally {
      setBusy(false);
    }
  }

  const transcriptionDisabled = settings.transcription_model_path.trim() === "";

  async function handleBackup() {
    setBusy(true);
    setError(null);
    setBackupMessage(null);
    try {
      const parent = await commands.pickBackupFolder();
      if (!parent) {
        return;
      }
      const backupPath = await commands.backupVault(parent);
      setBackupMessage(`Backup saved to ${backupPath}`);
    } catch (e) {
      setError(invokeErrorMessage(e, "Backup failed."));
    } finally {
      setBusy(false);
    }
  }

  async function handlePickRestore() {
    setError(null);
    const folder = await commands.pickRestoreFolder();
    if (!folder) {
      return;
    }
    setRestoreFolder(folder);
    setRestoreConfirmOpen(true);
  }

  async function handleConfirmRestore() {
    if (!restoreFolder) {
      return;
    }
    setBusy(true);
    setError(null);
    try {
      await commands.restoreVault(restoreFolder);
      setRestoreConfirmOpen(false);
      setRestoreFolder(null);
      setBackupMessage("Restore completed.");
      await load();
    } catch (e) {
      setError(invokeErrorMessage(e, "Restore failed."));
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="settings-screen">
      <form
        className="settings-form"
        onSubmit={(event) => void handleSave(event)}
      >
        <div className="settings-field">
          <label
            className="settings-field__label"
            htmlFor="settings-tesseract-lang"
          >
            Tesseract language
          </label>
          <input
            id="settings-tesseract-lang"
            className="settings-field__input"
            value={settings.tesseract_language}
            onChange={(event) =>
              setSettings((prev) => ({
                ...prev,
                tesseract_language: event.target.value,
              }))
            }
          />
        </div>

        <div className="settings-field">
          <label
            className="settings-field__label"
            htmlFor="settings-transcription-model"
          >
            Transcription model path
          </label>
          <input
            id="settings-transcription-model"
            className="settings-field__input"
            value={settings.transcription_model_path}
            onChange={(event) =>
              setSettings((prev) => ({
                ...prev,
                transcription_model_path: event.target.value,
              }))
            }
          />
          <p
            className="settings-field__hint"
            data-testid="transcription-status"
            role="status"
          >
            {transcriptionDisabled
              ? "Transcription is disabled."
              : "Transcription is enabled."}
          </p>
        </div>

        <div className="settings-field">
          <label
            className="settings-field__label"
            htmlFor="settings-anki-binary"
          >
            Anki desktop binary
          </label>
          <input
            id="settings-anki-binary"
            className="settings-field__input"
            value={settings.anki_desktop_binary_path}
            onChange={(event) =>
              setSettings((prev) => ({
                ...prev,
                anki_desktop_binary_path: event.target.value,
              }))
            }
          />
        </div>

        <fieldset className="settings-field">
          <legend className="settings-field__label">Appearance</legend>
          <label className="settings-theme-option">
            <input
              type="radio"
              name="settings-ui-theme"
              value="light"
              checked={settings.ui_theme === "light"}
              onChange={() =>
                setSettings((prev) => ({ ...prev, ui_theme: "light" }))
              }
            />
            Light
          </label>
          <label className="settings-theme-option">
            <input
              type="radio"
              name="settings-ui-theme"
              value="dark"
              checked={settings.ui_theme === "dark"}
              onChange={() =>
                setSettings((prev) => ({ ...prev, ui_theme: "dark" }))
              }
            />
            Dark
          </label>
        </fieldset>

        <div className="settings-field">
          <label
            className="settings-field__label"
            htmlFor="settings-playback-speed"
          >
            Playback speed
          </label>
          <select
            id="settings-playback-speed"
            className="settings-field__input"
            value={String(settings.playback_speed)}
            onChange={(event) =>
              setSettings((prev) => ({
                ...prev,
                playback_speed: Number(event.target.value),
              }))
            }
          >
            <option value="0.75">0.75×</option>
            <option value="1">1×</option>
            <option value="1.25">1.25×</option>
            <option value="1.5">1.5×</option>
            <option value="2">2×</option>
          </select>
        </div>

        <div className="settings-field settings-field--readonly">
          <span className="settings-field__label">Anki profile directory</span>
          <p className="settings-readonly-path" data-testid="anki-profile-path">
            {ankiProfilePath || "—"}
          </p>
        </div>

        <div
          className="settings-backup"
          role="group"
          aria-label="Backup and restore"
        >
          <div className="settings-backup__actions">
            <button
              type="button"
              className="library-button-secondary"
              disabled={busy}
              onClick={() => void handleBackup()}
            >
              Backup
            </button>
            <button
              type="button"
              className="library-button-secondary"
              disabled={busy}
              onClick={() => void handlePickRestore()}
            >
              Restore
            </button>
          </div>
          {backupMessage ? (
            <p className="settings-saved" role="status">
              {backupMessage}
            </p>
          ) : null}
        </div>

        {restoreConfirmOpen ? (
          <div
            className="settings-restore-confirm"
            role="alertdialog"
            aria-labelledby="settings-restore-confirm-title"
          >
            <p
              id="settings-restore-confirm-title"
              className="settings-restore-confirm__line"
            >
              Replace your library and Anki profile from this backup? Your
              current data is copied aside first.
            </p>
            <div className="settings-restore-confirm__actions">
              <button
                type="button"
                className="library-button-secondary"
                disabled={busy}
                onClick={() => {
                  setRestoreConfirmOpen(false);
                  setRestoreFolder(null);
                }}
              >
                Cancel
              </button>
              <button
                type="button"
                className="study-button"
                disabled={busy}
                onClick={() => void handleConfirmRestore()}
              >
                Restore backup
              </button>
            </div>
          </div>
        ) : null}

        {error ? (
          <p className="library-error" role="alert">
            {error}
          </p>
        ) : null}
        {saved ? (
          <p className="settings-saved" role="status">
            Settings saved.
          </p>
        ) : null}

        <button type="submit" className="study-button" disabled={busy}>
          Save
        </button>
      </form>
    </div>
  );
}
