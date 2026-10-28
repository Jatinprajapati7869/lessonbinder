import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import type { LibraryCommandPort } from "./src/api/libraryCommands";
import { emptyStudyHeatmap52Weeks } from "./src/api/libraryStudyHeatmapTypes";
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

const lessonNoteHit: LibrarySearchHit = {
  lesson_id: "lesson-2",
  lesson_title: "Dot product",
  course_id: "course-1",
  course_title: "Linear Algebra",
  match_kind: "lesson_note",
  snippet: "… mnemonic for projection …",
  cue_start_ms: 4_200,
};

const untimedLessonNoteHit: LibrarySearchHit = {
  lesson_id: "lesson-3",
  lesson_title: "Cross product",
  course_id: "course-1",
  course_title: "Linear Algebra",
  match_kind: "lesson_note",
  snippet: "… reminder without timestamp …",
  cue_start_ms: null,
};

function baseCommands(
  overrides: Partial<LibraryCommandPort> = {}
): LibraryCommandPort {
  return {
    listCourses: vi.fn().mockResolvedValue([sampleCourse]),
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

  it("keeps Search library as the search field accessible name with the outline icon", async () => {
    render(
      <LibraryCommandsProvider commands={baseCommands()}>
        <LibraryScreen onOpenCourse={vi.fn()} onOpenLesson={vi.fn()} />
      </LibraryCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("searchbox", { name: "Search library" })
      ).toBeInTheDocument();
    });
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

  it("opens the lesson at the note timestamp when a timed note result is chosen", async () => {
    const commands = baseCommands({
      searchLibrary: vi.fn().mockResolvedValue([lessonNoteHit]),
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

    await user.type(screen.getByRole("searchbox"), "mnemonic");
    await vi.advanceTimersByTimeAsync(LIBRARY_SEARCH_DEBOUNCE_MS);

    await waitFor(() => {
      expect(commands.searchLibrary).toHaveBeenCalledWith("mnemonic");
    });

    await user.click(
      screen.getByRole("button", {
        name: /Dot product/,
      })
    );

    expect(onOpenLesson).toHaveBeenCalledWith({
      courseId: "course-1",
      lessonId: "lesson-2",
      seekMs: 4_200,
    });
  });

  it("opens the lesson without a search seek when the note has no timestamp", async () => {
    const commands = baseCommands({
      searchLibrary: vi.fn().mockResolvedValue([untimedLessonNoteHit]),
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

    await user.type(screen.getByRole("searchbox"), "reminder");
    await vi.advanceTimersByTimeAsync(LIBRARY_SEARCH_DEBOUNCE_MS);

    await waitFor(() => {
      expect(commands.searchLibrary).toHaveBeenCalledWith("reminder");
    });

    await user.click(
      screen.getByRole("button", {
        name: /Cross product/,
      })
    );

    expect(onOpenLesson).toHaveBeenCalledWith({
      courseId: "course-1",
      lessonId: "lesson-3",
    });
    expect(onOpenLesson.mock.calls[0][0]).not.toHaveProperty("seekMs");
  });
});
