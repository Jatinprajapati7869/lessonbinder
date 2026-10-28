import { vi } from "vitest";

import type { VaultSettings } from "../api/settingsTypes";

import { backupRestoreCommandStubs } from "./settingsCommandStubs";

export const defaultVaultSettings: VaultSettings = {
  tesseract_language: "eng",
  transcription_model_path: "",
  anki_desktop_binary_path: "anki",
  ui_theme: "light",
  playback_speed: 1,
  player_volume: 1,
  player_muted: false,
};

export const settingsCommandStubs = {
  getVaultSettings: vi.fn().mockResolvedValue(defaultVaultSettings),
  saveVaultSettings: vi.fn().mockResolvedValue(undefined),
  getAnkiProfilePath: vi.fn().mockResolvedValue(""),
  ...backupRestoreCommandStubs,
};
