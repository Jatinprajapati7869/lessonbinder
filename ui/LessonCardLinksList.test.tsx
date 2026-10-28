import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { CourseCommandPort } from "./src/api/courseCommands";
import { CourseCommandsProvider } from "./src/course/CourseCommandsContext";
import { LessonCardLinksList } from "./src/lesson/LessonCardLinksList";
import { lessonNoteCommandStubs } from "./src/test/lessonNoteCommandStubs";

function cardCommands(
  overrides: Partial<CourseCommandPort> = {}
): CourseCommandPort {
  return {
    getCourseDetail: vi.fn(),
    markLessonComplete: vi.fn(),
    markLessonIncomplete: vi.fn(),
    saveLessonResume: vi.fn(),
    saveDocumentPosition: vi.fn(),
    acceptOrderProposal: vi.fn(),
    rejectOrderProposal: vi.fn(),
    reorderLessons: vi.fn(),
    setPlaylistItemWatched: vi.fn(),
    markLessonBroken: vi.fn(),
    guardLessonMediaForResume: vi.fn().mockResolvedValue("in_progress"),
    rescanCourse: vi.fn(),
    archiveCourse: vi.fn(),
    unarchiveCourse: vi.fn(),
    mediaUrlForPath: vi.fn(),
    listLessonCues: vi.fn().mockResolvedValue([]),
    getTranscriptionModelPath: vi.fn().mockResolvedValue(""),
    transcribeLesson: vi.fn(),
    listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
    recordPlayback: vi.fn().mockResolvedValue([]),
    ...lessonNoteCommandStubs,
    ...overrides,
  };
}

describe("LessonCardLinksList", () => {
  it("unlink confirm removes the card row", async () => {
    const user = userEvent.setup();
    const link = {
      anki_note_id: 9001,
      timestamp_ms: 83_000,
      front_preview: "What is X?",
    };
    const listLessonCardLinks = vi
      .fn()
      .mockResolvedValueOnce([link])
      .mockResolvedValueOnce([]);
    const unlinkLessonCard = vi.fn().mockResolvedValue(undefined);

    render(
      <CourseCommandsProvider
        commands={cardCommands({ listLessonCardLinks, unlinkLessonCard })}
      >
        <LessonCardLinksList lessonId="lesson-1" />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByText("What is X?")).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: /What is X\?/ }));
    await user.click(screen.getByRole("button", { name: "Unlink" }));

    expect(screen.getByText("The Anki note is kept.")).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Unlink card" }));

    await waitFor(() => {
      expect(unlinkLessonCard).toHaveBeenCalledWith("lesson-1", 9001);
      expect(screen.queryByText("What is X?")).not.toBeInTheDocument();
      expect(
        screen.getByText("No cards from this lesson.")
      ).toBeInTheDocument();
    });
  });
});
