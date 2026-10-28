import { invoke } from "@tauri-apps/api/core";

import type { VaultSettings } from "./settingsTypes";

export type SettingsCommandPort = {
  getVaultSettings(): Promise<VaultSettings>;
  saveVaultSettings(settings: VaultSettings): Promise<void>;
  getAnkiProfilePath(): Promise<string>;
  pickBackupFolder(): Promise<string | null>;
  pickRestoreFolder(): Promise<string | null>;
  backupVault(destinationParent: string): Promise<string>;
  restoreVault(backupFolder: string): Promise<void>;
};

export const tauriSettingsCommands: SettingsCommandPort = {
  getVaultSettings: () => invoke<VaultSettings>("get_vault_settings"),
  saveVaultSettings: (settings) =>
    invoke("save_vault_settings_entry", { settings }),
  getAnkiProfilePath: () => invoke<string>("get_anki_profile_path"),
  pickBackupFolder: async () => {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const selected = await open({ directory: true, multiple: false });
    if (selected === null || Array.isArray(selected)) {
      return null;
    }
    return selected;
  },
  pickRestoreFolder: async () => {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const selected = await open({ directory: true, multiple: false });
    if (selected === null || Array.isArray(selected)) {
      return null;
    }
    return selected;
  },
  backupVault: (destinationParent) =>
    invoke<string>("backup_vault", { destinationParent }),
  restoreVault: (backupFolder) => invoke("restore_vault", { backupFolder }),
};
