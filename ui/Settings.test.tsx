import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { SettingsCommandPort } from "./src/api/settingsCommands";
import type { VaultSettings } from "./src/api/settingsTypes";
import { SettingsCommandsProvider } from "./src/settings/SettingsCommandsContext";
import { SettingsScreen } from "./src/screens/SettingsScreen";
import { backupRestoreCommandStubs } from "./src/test/settingsCommandStubs";
import {
  defaultVaultSettings,
  settingsCommandStubs,
} from "./src/test/settingsPlaybackStubs";
import {
  applyUiTheme,
  readUiThemeFromDocument,
} from "./src/theme/applyUiTheme";
import {
  formatShortcutKeysForSettings,
  IMPLEMENTED_PLAYER_AND_REVIEW_SHORTCUTS,
} from "./src/keyboard/implementedShortcuts";

const defaults: VaultSettings = {
  tesseract_language: "eng",
  transcription_model_path: "",
  anki_desktop_binary_path: "anki",
  ui_theme: "light",
  playback_speed: 1,
  player_volume: 1,
  player_muted: false,
};

function renderSettings(commands: SettingsCommandPort) {
  render(
    <SettingsCommandsProvider commands={commands}>
      <SettingsScreen />
    </SettingsCommandsProvider>
  );
}

describe("SettingsScreen", () => {
  it("theme radios save dark then light and apply each to the document", async () => {
    const user = userEvent.setup();
    applyUiTheme("light");
    const saveVaultSettings = vi.fn().mockResolvedValue(undefined);
    const commands: SettingsCommandPort = {
      ...settingsCommandStubs,
      getVaultSettings: vi.fn().mockResolvedValue(defaultVaultSettings),
      saveVaultSettings,
      getAnkiProfilePath: vi.fn().mockResolvedValue("/data/anki-profile"),
    };

    renderSettings(commands);

    await waitFor(() => {
      expect(screen.getByRole("radio", { name: "Light" })).toBeChecked();
    });
    expect(
      screen.getAllByRole("radio", { name: /^(Light|Dark)$/ })
    ).toHaveLength(2);

    await user.click(screen.getByRole("radio", { name: "Dark" }));
    await user.click(screen.getByRole("button", { name: "Save" }));

    await waitFor(() => {
      expect(saveVaultSettings).toHaveBeenCalledWith({
        ...defaultVaultSettings,
        ui_theme: "dark",
      });
      expect(readUiThemeFromDocument()).toBe("dark");
    });

    await user.click(screen.getByRole("radio", { name: "Light" }));
    await user.click(screen.getByRole("button", { name: "Save" }));

    await waitFor(() => {
      expect(saveVaultSettings).toHaveBeenLastCalledWith({
        ...defaultVaultSettings,
        ui_theme: "light",
      });
      expect(readUiThemeFromDocument()).toBe("light");
    });
  });

  it("shows disabled transcription when model path is empty", async () => {
    const commands: SettingsCommandPort = {
      getVaultSettings: vi.fn().mockResolvedValue(defaults),
      saveVaultSettings: vi.fn(),
      getAnkiProfilePath: vi
        .fn()
        .mockResolvedValue(
          "/home/user/.local/share/dev.lessonbinder.app/anki-profile"
        ),
      ...backupRestoreCommandStubs,
    };

    renderSettings(commands);

    await waitFor(() => {
      expect(screen.getByTestId("transcription-status")).toHaveTextContent(
        "Transcription is disabled."
      );
    });
    expect(screen.getByTestId("anki-profile-path")).toHaveTextContent(
      "/home/user/.local/share/dev.lessonbinder.app/anki-profile"
    );
  });

  it("save sends edited settings to the command port", async () => {
    const user = userEvent.setup();
    const saveVaultSettings = vi.fn().mockResolvedValue(undefined);
    const commands: SettingsCommandPort = {
      getVaultSettings: vi.fn().mockResolvedValue(defaults),
      saveVaultSettings,
      getAnkiProfilePath: vi.fn().mockResolvedValue("/data/anki-profile"),
      ...backupRestoreCommandStubs,
    };

    renderSettings(commands);

    await waitFor(() => {
      expect(screen.getByLabelText("Tesseract language")).toBeInTheDocument();
    });

    await user.clear(screen.getByLabelText("Tesseract language"));
    await user.type(screen.getByLabelText("Tesseract language"), "deu");
    await user.type(
      screen.getByLabelText("Transcription model path"),
      "/models/whisper.bin"
    );
    await user.clear(screen.getByLabelText("Anki desktop binary"));
    await user.type(
      screen.getByLabelText("Anki desktop binary"),
      "/usr/bin/anki"
    );

    expect(screen.getByTestId("transcription-status")).toHaveTextContent(
      "Transcription is enabled."
    );

    await user.click(screen.getByRole("button", { name: "Save" }));

    await waitFor(() => {
      expect(saveVaultSettings).toHaveBeenCalledWith({
        tesseract_language: "deu",
        transcription_model_path: "/models/whisper.bin",
        anki_desktop_binary_path: "/usr/bin/anki",
        ui_theme: "light",
        playback_speed: 1,
        player_volume: 1,
        player_muted: false,
      });
      expect(screen.getByText("Settings saved.")).toBeInTheDocument();
    });
  });

  it("renders keyboard shortcut rows from the implemented list", async () => {
    const commands: SettingsCommandPort = {
      getVaultSettings: vi.fn().mockResolvedValue(defaults),
      saveVaultSettings: vi.fn(),
      getAnkiProfilePath: vi.fn().mockResolvedValue("/data/anki-profile"),
      ...backupRestoreCommandStubs,
    };

    renderSettings(commands);

    await waitFor(() => {
      expect(
        screen.getByTestId("settings-keyboard-shortcuts")
      ).toBeInTheDocument();
    });

    IMPLEMENTED_PLAYER_AND_REVIEW_SHORTCUTS.forEach((row, index) => {
      const line = screen.getByTestId(`settings-shortcut-row-${index}`);
      expect(line).toHaveTextContent(formatShortcutKeysForSettings(row.keys));
      expect(line).toHaveTextContent(row.action);
    });
    expect(screen.getAllByTestId(/^settings-shortcut-row-/)).toHaveLength(
      IMPLEMENTED_PLAYER_AND_REVIEW_SHORTCUTS.length
    );
  });

  it("restore requires explicit confirm before calling command", async () => {
    const user = userEvent.setup();
    const restoreVault = vi.fn().mockResolvedValue(undefined);
    const commands: SettingsCommandPort = {
      getVaultSettings: vi.fn().mockResolvedValue(defaults),
      saveVaultSettings: vi.fn(),
      getAnkiProfilePath: vi.fn().mockResolvedValue("/data/anki-profile"),
      pickBackupFolder: vi.fn(),
      backupVault: vi.fn(),
      pickRestoreFolder: vi
        .fn()
        .mockResolvedValue("/backups/lessonbinder-backup-1"),
      restoreVault,
    };

    renderSettings(commands);

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Restore" })
      ).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Restore" }));
    expect(restoreVault).not.toHaveBeenCalled();
    expect(
      screen.getByRole("button", { name: "Restore backup" })
    ).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Restore backup" }));

    await waitFor(() => {
      expect(restoreVault).toHaveBeenCalledWith(
        "/backups/lessonbinder-backup-1"
      );
    });
  });
});
