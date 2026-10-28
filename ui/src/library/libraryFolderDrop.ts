import {
  libraryImportErrorMessage,
  LIBRARY_DROP_FILE_MESSAGE,
  LIBRARY_DROP_ONE_FOLDER_MESSAGE,
  RISKY_IMPORT_PATH_MESSAGE,
} from "./libraryImportError";

export {
  LIBRARY_DROP_FILE_MESSAGE,
  LIBRARY_DROP_ONE_FOLDER_MESSAGE,
} from "./libraryImportError";

export type LibraryFolderDropDecision =
  | { action: "ignore" }
  | { action: "error"; message: string }
  | { action: "import"; path: string };

export async function resolveLibraryFolderDrop(
  paths: readonly string[],
  isDirectory: (path: string) => Promise<boolean>
): Promise<LibraryFolderDropDecision> {
  if (paths.length === 0) {
    return { action: "ignore" };
  }
  if (paths.length !== 1) {
    return { action: "error", message: LIBRARY_DROP_ONE_FOLDER_MESSAGE };
  }
  const path = paths[0]!;
  if (!(await isDirectory(path))) {
    return { action: "error", message: LIBRARY_DROP_FILE_MESSAGE };
  }
  return { action: "import", path };
}

export async function applyLibraryFolderDrop(
  paths: readonly string[],
  deps: {
    isDirectory: (path: string) => Promise<boolean>;
    validateImportFolderPath: (path: string) => Promise<void>;
    importFolder: (path: string) => Promise<void>;
  }
): Promise<string | null> {
  const decision = await resolveLibraryFolderDrop(paths, deps.isDirectory);
  if (decision.action === "ignore") {
    return null;
  }
  if (decision.action === "error") {
    return decision.message;
  }
  try {
    await deps.validateImportFolderPath(decision.path);
  } catch (raw) {
    return (
      libraryImportErrorMessage(raw, RISKY_IMPORT_PATH_MESSAGE) ??
      RISKY_IMPORT_PATH_MESSAGE
    );
  }
  await deps.importFolder(decision.path);
  return null;
}

export async function libraryFolderDropIntoLibrary(
  paths: readonly string[],
  deps: {
    pathIsDirectory: (path: string) => Promise<boolean>;
    validateImportFolderPath: (path: string) => Promise<void>;
    importFolderAtPath: (path: string) => Promise<void>;
  }
): Promise<string | null> {
  return applyLibraryFolderDrop(paths, {
    isDirectory: deps.pathIsDirectory,
    validateImportFolderPath: deps.validateImportFolderPath,
    importFolder: deps.importFolderAtPath,
  });
}
