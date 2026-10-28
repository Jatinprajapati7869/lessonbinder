import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { SettingsCommandPort } from "./src/api/settingsCommands";
import type { VaultSettings } from "./src/api/settingsTypes";
import { SettingsCommandsProvider } from "./src/settings/SettingsCommandsContext";
import { SettingsScreen } from "./src/screens/SettingsScreen";
import { backupRestoreCommandStubs } from "./src/test/settingsCommandStubs";

const defaults: VaultSettings = {
  tesseract_language: "eng",
  transcription_model_path: "",
  anki_desktop_binary_path: "anki",
};

function renderSettings(commands: SettingsCommandPort) {
  render(
    <SettingsCommandsProvider commands={commands}>
      <SettingsScreen />
    </SettingsCommandsProvider>
  );
}

describe("SettingsScreen", () => {
  it("shows disabled transcription when model path is empty", async () => {
    const commands: SettingsCommandPort = {
      getVaultSettings: vi.fn().mockResolvedValue(defaults),
      saveVaultSettings: vi.fn(),
      getAnkiProfilePath: vi
        .fn()
        .mockResolvedValue("/home/user/.local/share/dev.lessonbinder.app/anki-profile"),
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
      });
      expect(screen.getByText("Settings saved.")).toBeInTheDocument();
    });
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
      expect(restoreVault).toHaveBeenCalledWith("/backups/lessonbinder-backup-1");
    });
  });
});
