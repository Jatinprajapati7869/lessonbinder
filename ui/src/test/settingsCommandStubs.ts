import { vi } from "vitest";

export const backupRestoreCommandStubs = {
  pickBackupFolder: vi.fn().mockResolvedValue(null),
  pickRestoreFolder: vi.fn().mockResolvedValue(null),
  backupVault: vi.fn().mockResolvedValue("/tmp/auravault-backup-1"),
  restoreVault: vi.fn().mockResolvedValue(undefined),
};
