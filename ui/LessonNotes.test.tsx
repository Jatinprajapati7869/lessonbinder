import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { CourseCommandPort } from "./src/api/courseCommands";
import type { LessonNote } from "./src/api/courseTypes";
import { CourseCommandsProvider } from "./src/course/CourseCommandsContext";
import { LessonNotesColumn } from "./src/lesson/LessonNotesColumn";

function noteCommands(
  overrides: Partial<CourseCommandPort> = {}
): CourseCommandPort {
  return {
    getCourseDetail: vi.fn(),
    markLessonComplete: vi.fn(),
    markLessonIncomplete: vi.fn(),
    saveLessonResume: vi.fn(),
    acceptOrderProposal: vi.fn(),
    rejectOrderProposal: vi.fn(),
    reorderLessons: vi.fn(),
    setPlaylistItemWatched: vi.fn(),
    markLessonBroken: vi.fn(),
    rescanCourse: vi.fn(),
    mediaUrlForPath: vi.fn(),
    listLessonCues: vi.fn().mockResolvedValue([]),
    listLessonResources: vi.fn().mockResolvedValue([]),
    captureLessonThumbnail: vi.fn().mockResolvedValue(undefined),
    getTranscriptionModelPath: vi.fn().mockResolvedValue(""),
    transcribeLesson: vi.fn(),
    listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
    recordPlayback: vi.fn().mockResolvedValue([]),
    listLessonCardLinks: vi.fn().mockResolvedValue([]),
    unlinkLessonCard: vi.fn().mockResolvedValue(undefined),
    listLessonNotes: vi.fn().mockResolvedValue([]),
    createLessonNote: vi.fn(),
    updateLessonNote: vi.fn(),
    deleteLessonNote: vi.fn(),
    saveDocumentPosition: vi.fn().mockResolvedValue(undefined),
    ...overrides,
  };
}

describe("LessonNotesColumn", () => {
  it("add note calls command layer and lists the saved note", async () => {
    const user = userEvent.setup();
    const saved: LessonNote = {
      note_id: "note-1",
      lesson_id: "lesson-1",
      text: "Key definition",
      timestamp_ms: null,
      created_at: "1",
      updated_at: "1",
    };
    const createLessonNote = vi.fn().mockResolvedValue(saved);
    const listLessonNotes = vi
      .fn()
      .mockResolvedValueOnce([])
      .mockResolvedValueOnce([saved]);

    render(
      <CourseCommandsProvider
        commands={noteCommands({ listLessonNotes, createLessonNote })}
      >
        <LessonNotesColumn lessonId="lesson-1" />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("region", { name: "Lesson notes" })
      ).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Add note" }));
    await user.type(
      screen.getByRole("textbox", { name: "Note text" }),
      "Key definition"
    );
    await user.click(screen.getByRole("button", { name: "Save note" }));

    await waitFor(() => {
      expect(createLessonNote).toHaveBeenCalledWith(
        "lesson-1",
        "Key definition",
        null
      );
      expect(screen.getByText("Key definition")).toBeInTheDocument();
    });
    expect(
      screen.getByRole("button", { name: "Make card" })
    ).toBeInTheDocument();
  });

  it("clicking a timestamp seeks playback", async () => {
    const user = userEvent.setup();
    const onSeek = vi.fn();
    const timestamped: LessonNote = {
      note_id: "note-ts",
      lesson_id: "lesson-1",
      text: "Important moment",
      timestamp_ms: 42_000,
      created_at: "1",
      updated_at: "1",
    };

    render(
      <CourseCommandsProvider
        commands={noteCommands({
          listLessonNotes: vi.fn().mockResolvedValue([timestamped]),
        })}
      >
        <LessonNotesColumn
          lessonId="lesson-1"
          playbackContext={{ currentMs: 0, onSeek }}
        />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByText("Important moment")).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Seek to 0:42" }));
    expect(onSeek).toHaveBeenCalledWith(42_000);
  });
});
