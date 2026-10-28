import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { LibraryCommandPort } from "./src/api/libraryCommands";
import { emptyStudyHeatmap52Weeks } from "./src/api/libraryStudyHeatmapTypes";
import type { CourseRow } from "./src/api/libraryTypes";
import { LibraryCommandsProvider } from "./src/library/LibraryCommandsContext";
import { LibraryScreen } from "./src/screens/LibraryScreen";

const activeCourse: CourseRow = {
  course_id: "course-active",
  title: "Active Course",
  source_kind: "folder",
  status: "not_started",
  remaining_ms: 0,
  unknown_duration_count: 0,
  is_archived: false,
};

const archivedCourse: CourseRow = {
  course_id: "course-archived",
  title: "Archived Course",
  source_kind: "folder",
  status: "done",
  remaining_ms: 0,
  unknown_duration_count: 0,
  is_archived: true,
};

function renderLibrary(listCourses: LibraryCommandPort["listCourses"]) {
  const commands: LibraryCommandPort = {
    listCourses,
    importFolder: vi.fn(),
    validateImportFolderPath: vi.fn().mockResolvedValue(undefined),
    cancelImportFolder: vi.fn(),
    importPlaylist: vi.fn(),
    createPlaylistFromPaste: vi.fn(),
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
  };

  render(
    <LibraryCommandsProvider commands={commands}>
      <LibraryScreen onOpenCourse={vi.fn()} onOpenLesson={vi.fn()} />
    </LibraryCommandsProvider>
  );

  return commands;
}

describe("Library archive filter", () => {
  it("defaults to Active and reloads Archived courses when selected", async () => {
    const user = userEvent.setup();
    const listCourses = vi
      .fn()
      .mockImplementation((visibility: "active" | "archived" = "active") => {
        if (visibility === "archived") {
          return Promise.resolve([archivedCourse]);
        }
        return Promise.resolve([activeCourse]);
      });

    renderLibrary(listCourses);

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Active Course" })
      ).toBeInTheDocument();
    });
    expect(listCourses).toHaveBeenCalledWith("active");

    await user.click(screen.getByRole("radio", { name: "Archived" }));

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Archived Course" })
      ).toBeInTheDocument();
      expect(
        screen.queryByRole("button", { name: "Active Course" })
      ).not.toBeInTheDocument();
    });
    expect(listCourses).toHaveBeenLastCalledWith("archived");
  });
});
