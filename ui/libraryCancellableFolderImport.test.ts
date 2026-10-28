import { describe, expect, it, vi } from "vitest";

import { RISKY_IMPORT_PATH_MESSAGE } from "./src/library/libraryImportError";
import { importCancellableFolderAtPath } from "./src/library/libraryCancellableFolderImport";

describe("importCancellableFolderAtPath", () => {
  it("rejects risky roots before import_course_folder runs", async () => {
    const importFolder = vi.fn();
    const validateImportFolderPath = vi.fn().mockRejectedValue(
      new Error(
        JSON.stringify({
          code: "risky_import_path",
          message: "risky_import_path",
        })
      )
    );

    const message = await importCancellableFolderAtPath("/", {
      validateImportFolderPath,
      importFolder,
    });

    expect(validateImportFolderPath).toHaveBeenCalledWith("/");
    expect(importFolder).not.toHaveBeenCalled();
    expect(message).toBe(RISKY_IMPORT_PATH_MESSAGE);
  });

  it("is the single validate-then-import path for picker and drop", async () => {
    const importFolder = vi.fn().mockResolvedValue(undefined);
    const validateImportFolderPath = vi.fn().mockResolvedValue(undefined);

    const message = await importCancellableFolderAtPath("/tmp/course", {
      validateImportFolderPath,
      importFolder,
    });

    expect(message).toBeNull();
    expect(validateImportFolderPath).toHaveBeenCalledWith("/tmp/course");
    expect(importFolder).toHaveBeenCalledWith("/tmp/course");
  });
});
