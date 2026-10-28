import { describe, expect, it } from "vitest";

import {
  FOLDER_SCAN_BUSY_MESSAGE,
  IMPORT_CANCELLED_MESSAGE,
  LIBRARY_DROP_FILE_MESSAGE,
  libraryImportErrorMessage,
  NO_LESSON_FILES_MESSAGE,
  RISKY_IMPORT_PATH_MESSAGE,
} from "./src/library/libraryImportError";

describe("libraryImportErrorMessage", () => {
  it("maps risky_import_path to library copy", () => {
    const message = libraryImportErrorMessage(
      new Error(
        JSON.stringify({
          code: "risky_import_path",
          message: "risky_import_path",
        })
      ),
      "Import folder failed."
    );
    expect(message).toBe(RISKY_IMPORT_PATH_MESSAGE);
  });

  it("maps import_invalid_path to drop-a-folder copy", () => {
    const message = libraryImportErrorMessage(
      new Error(
        JSON.stringify({
          code: "import_invalid_path",
          message: "import root must be a directory",
        })
      ),
      "Import folder failed."
    );
    expect(message).toBe(LIBRARY_DROP_FILE_MESSAGE);
  });

  it("maps import_cancelled to one library sentence", () => {
    const message = libraryImportErrorMessage(
      new Error(
        JSON.stringify({
          code: "import_cancelled",
          message: "cancelled",
        })
      ),
      "Import folder failed."
    );
    expect(message).toBe(IMPORT_CANCELLED_MESSAGE);
    expect(message).not.toMatch(/deleted|removed/i);
    expect(message).not.toContain("\n");
    expect(message).not.toMatch(/Import folder failed/);
  });

  it("maps folder_scan_in_progress to one sentence", () => {
    const message = libraryImportErrorMessage(
      new Error(
        JSON.stringify({
          code: "folder_scan_in_progress",
          message: "A folder scan is already running.",
        })
      ),
      "Import folder failed."
    );
    expect(message).toBe(FOLDER_SCAN_BUSY_MESSAGE);
  });

  it("maps no_lesson_files to one sentence", () => {
    const message = libraryImportErrorMessage(
      new Error(
        JSON.stringify({
          code: "no_lesson_files",
          message: "no_lesson_files",
        })
      ),
      "Import folder failed."
    );
    expect(message).toBe(NO_LESSON_FILES_MESSAGE);
  });
});
