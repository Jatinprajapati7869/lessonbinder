import {
  libraryImportErrorMessage,
  RISKY_IMPORT_PATH_MESSAGE,
} from "./libraryImportError";

export type CancellableFolderImportDeps = {
  validateImportFolderPath: (path: string) => Promise<void>;
  importFolder: (path: string) => Promise<void>;
};

/**
 * Validate then invoke the backend cancellable folder import (`import_course_folder`).
 * Used by Add folder and drag-and-drop so neither path runs its own scan loop.
 */
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
