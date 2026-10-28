import { describe, expect, it, vi } from "vitest";

import { RISKY_IMPORT_PATH_MESSAGE } from "./src/library/libraryImportError";
import {
  applyLibraryFolderDrop,
  LIBRARY_DROP_FILE_MESSAGE,
  LIBRARY_DROP_ONE_FOLDER_MESSAGE,
  resolveLibraryFolderDrop,
} from "./src/library/libraryFolderDrop";

describe("resolveLibraryFolderDrop", () => {
  it("rejects a single file path", async () => {
    const decision = await resolveLibraryFolderDrop(
      ["/tmp/course/lesson.mp4"],
      async () => false
    );
    expect(decision).toEqual({
      action: "error",
      message: LIBRARY_DROP_FILE_MESSAGE,
    });
  });

  it("accepts a single directory path", async () => {
    const decision = await resolveLibraryFolderDrop(
      ["/tmp/my-course"],
      async () => true
    );
    expect(decision).toEqual({ action: "import", path: "/tmp/my-course" });
  });

  it("rejects multiple dropped paths", async () => {
    const decision = await resolveLibraryFolderDrop(
      ["/tmp/a", "/tmp/b"],
      async () => true
    );
    expect(decision).toEqual({
      action: "error",
      message: LIBRARY_DROP_ONE_FOLDER_MESSAGE,
    });
  });
});

describe("applyLibraryFolderDrop", () => {
  it("does not import when the drop is a file", async () => {
    const importFolder = vi.fn();
    const validateImportFolderPath = vi.fn();
    const message = await applyLibraryFolderDrop(["/tmp/notes.md"], {
      isDirectory: async () => false,
      validateImportFolderPath,
      importFolder,
    });
    expect(message).toBe(LIBRARY_DROP_FILE_MESSAGE);
    expect(validateImportFolderPath).not.toHaveBeenCalled();
    expect(importFolder).not.toHaveBeenCalled();
  });

  it("runs risky-import validation before import", async () => {
    const importFolder = vi.fn();
    const validateImportFolderPath = vi.fn().mockRejectedValue(
      new Error(
        JSON.stringify({
          code: "risky_import_path",
          message: "risky_import_path",
        })
      )
    );
    const message = await applyLibraryFolderDrop(["/"], {
      isDirectory: async () => true,
      validateImportFolderPath,
      importFolder,
    });
    expect(validateImportFolderPath).toHaveBeenCalledWith("/");
    expect(importFolder).not.toHaveBeenCalled();
    expect(message).toBe(RISKY_IMPORT_PATH_MESSAGE);
  });

  it("imports a dropped directory like Add folder", async () => {
    const importFolder = vi.fn().mockResolvedValue(undefined);
    const validateImportFolderPath = vi.fn().mockResolvedValue(undefined);
    const message = await applyLibraryFolderDrop(["/tmp/course-dir"], {
      isDirectory: async () => true,
      validateImportFolderPath,
      importFolder,
    });
    expect(message).toBeNull();
    expect(validateImportFolderPath).toHaveBeenCalledWith("/tmp/course-dir");
    expect(importFolder).toHaveBeenCalledWith("/tmp/course-dir");
  });
});
