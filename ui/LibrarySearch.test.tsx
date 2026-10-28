import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import type { LibraryCommandPort } from "./src/api/libraryCommands";
import type { LibrarySearchHit } from "./src/api/librarySearchTypes";
import type { CourseRow } from "./src/api/libraryTypes";
import { LIBRARY_SEARCH_DEBOUNCE_MS } from "./src/library/LibrarySearchResults";
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
};

const subtitleHit: LibrarySearchHit = {
  lesson_id: "lesson-1",
  lesson_title: "Eigenvalues",
  course_id: "course-1",
  course_title: "Linear Algebra",
  match_kind: "subtitle_cue",
  snippet: "… eigenvalue decomposition …",
  cue_start_ms: 12_500,
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

describe("Library search", () => {
  beforeEach(() => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("opens the lesson at the cue start time when a subtitle result is chosen", async () => {
    const commands = baseCommands({
      searchLibrary: vi.fn().mockResolvedValue([subtitleHit]),
    });
    const onOpenLesson = vi.fn();
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });

    render(
      <LibraryCommandsProvider commands={commands}>
        <LibraryScreen onOpenCourse={vi.fn()} onOpenLesson={onOpenLesson} />
      </LibraryCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Linear Algebra" })
      ).toBeInTheDocument();
    });

    await user.type(screen.getByRole("searchbox"), "eigen");
    await vi.advanceTimersByTimeAsync(LIBRARY_SEARCH_DEBOUNCE_MS);

    await waitFor(() => {
      expect(commands.searchLibrary).toHaveBeenCalledWith("eigen");
    });

    await user.click(
      screen.getByRole("button", {
        name: /Eigenvalues/,
      })
    );

    expect(onOpenLesson).toHaveBeenCalledWith({
      courseId: "course-1",
      lessonId: "lesson-1",
      seekMs: 12_500,
    });
  });
});
