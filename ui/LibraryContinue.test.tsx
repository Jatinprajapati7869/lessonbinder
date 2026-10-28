import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { LibraryCommandPort } from "./src/api/libraryCommands";
import type { LibraryContinueRow } from "./src/api/libraryContinueTypes";
import type { CourseRow } from "./src/api/libraryTypes";
import { LibraryCommandsProvider } from "./src/library/LibraryCommandsContext";
import { LibraryScreen } from "./src/screens/LibraryScreen";

const sampleCourse: CourseRow = {
  course_id: "course-1",
  title: "Linear Algebra",
  source_kind: "folder",
  status: "in_progress",
  remaining_ms: 60_000,
  unknown_duration_count: 0,
  is_archived: false,
};

const continueRow: LibraryContinueRow = {
  course_id: "course-1",
  course_title: "Linear Algebra",
  lesson_id: "lesson-1",
  lesson_title: "Eigenvalues",
  resume_position_ms: 4_500,
  watched_percent: 37,
};

function baseCommands(
  overrides: Partial<LibraryCommandPort> = {}
): LibraryCommandPort {
  return {
    listCourses: vi.fn().mockResolvedValue([sampleCourse]),
    importFolder: vi.fn(),
    cancelImportFolder: vi.fn(),
    importPlaylist: vi.fn(),
    createPlaylistFromPaste: vi.fn(),
    pickFolder: vi.fn(),
    pickPlaylistJson: vi.fn(),
    searchLibrary: vi.fn().mockResolvedValue([]),
    getLibraryContinueRow: vi.fn().mockResolvedValue(null),
    mediaUrlForPath: vi.fn().mockResolvedValue("asset://file"),
    ...overrides,
  };
}

describe("Library continue row", () => {
  it("shows course, lesson, and watched percent and opens at resume position", async () => {
    const user = userEvent.setup();
    const onOpenLesson = vi.fn();
    const commands = baseCommands({
      getLibraryContinueRow: vi.fn().mockResolvedValue(continueRow),
    });

    render(
      <LibraryCommandsProvider commands={commands}>
        <LibraryScreen onOpenCourse={vi.fn()} onOpenLesson={onOpenLesson} />
      </LibraryCommandsProvider>
    );

    const continueButton = await screen.findByRole("button", {
      name: "Continue Eigenvalues",
    });
    expect(continueButton).toHaveTextContent("Continue");
    expect(continueButton).toHaveTextContent("Linear Algebra");
    expect(continueButton).toHaveTextContent("Eigenvalues");
    expect(continueButton).toHaveTextContent("37%");

    await user.click(
      screen.getByRole("button", { name: "Continue Eigenvalues" })
    );

    expect(onOpenLesson).toHaveBeenCalledWith({
      courseId: "course-1",
      lessonId: "lesson-1",
      seekMs: 4_500,
    });
  });

  it("hides the row when continue target is none", async () => {
    render(
      <LibraryCommandsProvider commands={baseCommands()}>
        <LibraryScreen onOpenCourse={vi.fn()} onOpenLesson={vi.fn()} />
      </LibraryCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByText("Linear Algebra")).toBeInTheDocument();
    });

    expect(screen.queryByText("Continue")).not.toBeInTheDocument();
  });
});
