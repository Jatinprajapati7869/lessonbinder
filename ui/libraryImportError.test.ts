import { describe, expect, it } from "vitest";

import {
  libraryImportErrorMessage,
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

  it("returns null for user-cancelled import", () => {
    const message = libraryImportErrorMessage(
      new Error(
        JSON.stringify({
          code: "import_cancelled",
          message: "cancelled",
        })
      ),
      "Import folder failed."
    );
    expect(message).toBeNull();
  });
});
