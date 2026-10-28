import {
  invokeErrorMessage,
  parseCommandErrorPayload,
} from "../api/invokeError";

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
    if (parsed?.code === "import_cancelled") {
      return null;
    }
    return invokeErrorMessage(raw, fallback);
  }
  return fallback;
}
