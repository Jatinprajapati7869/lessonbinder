import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { LibraryCommandPort } from "./src/api/libraryCommands";
import { emptyStudyHeatmap52Weeks } from "./src/api/libraryStudyHeatmapTypes";
import type { CourseRow } from "./src/api/libraryTypes";
import { LibraryCommandsProvider } from "./src/library/LibraryCommandsContext";
import { LibraryScreen } from "./src/screens/LibraryScreen";

const sampleCourse: CourseRow = {
  course_id: "course-1",
  title: "Linear Algebra",
  source_kind: "folder",
  status: "not_started",
  remaining_ms: 0,
  unknown_duration_count: 0,
  is_archived: false,
  thumbnail_path: null,
};

function libraryCommands(
  overrides: Partial<LibraryCommandPort> = {}
): LibraryCommandPort {
  return {
    listCourses: vi.fn().mockResolvedValue([sampleCourse]),
    importFolder: vi.fn(),
    validateImportFolderPath: vi.fn().mockResolvedValue(undefined),
    cancelImportFolder: vi.fn(),
    importPlaylist: vi.fn(),
    createPlaylistFromPaste: vi.fn().mockResolvedValue(undefined),
    pickFolder: vi.fn(),
    pickPlaylistJson: vi.fn(),
    searchLibrary: vi.fn().mockResolvedValue([]),
    getLibraryContinueRow: vi.fn().mockResolvedValue(null),
    getLibraryStudyStreakDays: vi.fn().mockResolvedValue(0),
    getLibraryStudyHeatmap52Weeks: vi
      .fn()
      .mockResolvedValue(emptyStudyHeatmap52Weeks()),
    mediaUrlForPath: vi.fn().mockResolvedValue("asset://file"),
    pathIsDirectory: vi.fn().mockResolvedValue(true),
    ...overrides,
  };
}

describe("Library paste playlist", () => {
  it("creates a course from pasted lines through the command port", async () => {
    const user = userEvent.setup();
    const createPlaylistFromPaste = vi.fn().mockResolvedValue(undefined);
    const listCourses = vi
      .fn()
      .mockResolvedValueOnce([sampleCourse])
      .mockResolvedValueOnce([sampleCourse]);

    render(
      <LibraryCommandsProvider
        commands={libraryCommands({ createPlaylistFromPaste, listCourses })}
      >
        <LibraryScreen onOpenCourse={vi.fn()} onOpenLesson={vi.fn()} />
      </LibraryCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Paste playlist" })
      ).toBeEnabled();
    });

    await user.click(screen.getByRole("button", { name: "Paste playlist" }));

    await user.type(screen.getByLabelText("Course title"), "Week one");
    await user.type(
      screen.getByLabelText("Playlist lines"),
      "Intro\thttps://example.com/intro\nNext\thttps://example.com/next"
    );
    await user.click(screen.getByRole("button", { name: "Create" }));

    await waitFor(() => {
      expect(createPlaylistFromPaste).toHaveBeenCalledWith(
        "Week one",
        "Intro\thttps://example.com/intro\nNext\thttps://example.com/next"
      );
    });
  });
});
