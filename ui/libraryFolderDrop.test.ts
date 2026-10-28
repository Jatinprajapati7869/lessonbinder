import { describe, expect, it, vi } from "vitest";

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
    const importFolderAtPath = vi.fn();
    const message = await applyLibraryFolderDrop(["/tmp/notes.md"], {
      isDirectory: async () => false,
      importFolderAtPath,
    });
    expect(message).toBe(LIBRARY_DROP_FILE_MESSAGE);
    expect(importFolderAtPath).not.toHaveBeenCalled();
  });

  it("delegates directory import to the same entry as Add folder", async () => {
    const importFolderAtPath = vi.fn().mockResolvedValue(undefined);
    const message = await applyLibraryFolderDrop(["/tmp/course-dir"], {
      isDirectory: async () => true,
      importFolderAtPath,
    });
    expect(message).toBeNull();
    expect(importFolderAtPath).toHaveBeenCalledWith("/tmp/course-dir");
  });
});
