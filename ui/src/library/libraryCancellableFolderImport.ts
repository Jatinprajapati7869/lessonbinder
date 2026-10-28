import {
  libraryImportErrorMessage,
  RISKY_IMPORT_PATH_MESSAGE,
} from "./libraryImportError";

export type CancellableFolderImportDeps = {
  validateImportFolderPath: (path: string) => Promise<void>;
  importFolder: (path: string) => Promise<void>;
};

export async function importCancellableFolderAtPath(
  path: string,
  deps: CancellableFolderImportDeps
): Promise<string | null> {
  try {
    await deps.validateImportFolderPath(path);
  } catch (raw) {
    return (
      libraryImportErrorMessage(raw, RISKY_IMPORT_PATH_MESSAGE) ??
      RISKY_IMPORT_PATH_MESSAGE
    );
  }
  await deps.importFolder(path);
  return null;
}
