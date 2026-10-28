import {
  invokeErrorMessage,
  parseCommandErrorPayload,
} from "../api/invokeError";

export const LIBRARY_DROP_ONE_FOLDER_MESSAGE =
  "Drop one folder to import a course.";

export const LIBRARY_DROP_FILE_MESSAGE = "Drop a folder, not a single file.";

export const RISKY_IMPORT_PATH_MESSAGE =
  "That folder is too broad; pick the course directory itself.";

export function libraryImportErrorMessage(
  raw: unknown,
  fallback: string
): string | null {
  if (raw instanceof Error) {
    const parsed = parseCommandErrorPayload(raw.message);
    if (parsed?.code === "risky_import_path") {
      return RISKY_IMPORT_PATH_MESSAGE;
    }
    if (parsed?.code === "import_invalid_path") {
      return LIBRARY_DROP_FILE_MESSAGE;
    }
    if (parsed?.code === "import_cancelled") {
      return null;
    }
    return invokeErrorMessage(raw, fallback);
  }
  return fallback;
}
