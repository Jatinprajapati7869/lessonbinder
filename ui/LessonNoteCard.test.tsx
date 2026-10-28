import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { CaptureCommandPort } from "./src/api/captureCommands";
import type { CardDraft } from "./src/api/captureTypes";
import type { CourseCommandPort } from "./src/api/courseCommands";
import type { LessonNote } from "./src/api/courseTypes";
import { CaptureCommandsProvider } from "./src/capture/CaptureCommandsContext";
import { CourseCommandsProvider } from "./src/course/CourseCommandsContext";
import { LessonNotesColumn } from "./src/lesson/LessonNotesColumn";
import { lessonNoteCommandStubs } from "./src/test/lessonNoteCommandStubs";

const note: LessonNote = {
  note_id: "note-1",
  lesson_id: "lesson-1",
  text: "Key definition",
  timestamp_ms: 42_000,
  created_at: "1",
  updated_at: "1",
};

const draft: CardDraft = {
  course: "Linear Algebra",
  lesson_title: "Intro clip",
  front: "Key definition",
  back: "Intro clip\nLinear Algebra\n00:00:42",
  source_label: "lesson_note",
  timestamp: "00:00:42",
};

function courseCommands(
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
    archiveCourse: vi.fn(),
    unarchiveCourse: vi.fn(),
    mediaUrlForPath: vi.fn(),
    listLessonCues: vi.fn().mockResolvedValue([]),
    getTranscriptionModelPath: vi.fn().mockResolvedValue(""),
    transcribeLesson: vi.fn(),
    listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
    recordPlayback: vi.fn().mockResolvedValue([]),
    ...lessonNoteCommandStubs,
    listLessonNotes: vi.fn().mockResolvedValue([note]),
    ...overrides,
  };
}

describe("Lesson note card", () => {
  it("does not commit until Save to Anki", async () => {
    const user = userEvent.setup();
    const buildLessonNoteDraft = vi.fn().mockResolvedValue(draft);
    const commitLessonNoteCard = vi.fn().mockResolvedValue({
      duplicate: false,
      note_id: 1,
      ...draft,
      lesson_title: draft.lesson_title,
    });
    const captureCommands: CaptureCommandPort = {
      captureOcrStatus: vi.fn(),
      buildCaptureDraft: vi.fn(),
      commitCaptureCard: vi.fn(),
      buildSubtitleCueDraft: vi.fn(),
      commitSubtitleCueCard: vi.fn(),
      buildLessonNoteDraft,
      commitLessonNoteCard,
      buildAudioClipDraft: vi.fn(),
      commitAudioClipCard: vi.fn(),
    };

    render(
      <CaptureCommandsProvider commands={captureCommands}>
        <CourseCommandsProvider commands={courseCommands()}>
          <LessonNotesColumn lessonId="lesson-1" />
        </CourseCommandsProvider>
      </CaptureCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByText("Key definition")).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Make card" }));

    await waitFor(() => {
      expect(buildLessonNoteDraft).toHaveBeenCalledWith({
        lessonId: "lesson-1",
        noteText: "Key definition",
        timestampMs: 42_000,
      });
      expect(screen.getByLabelText("Front")).toHaveValue("Key definition");
    });
    expect(commitLessonNoteCard).not.toHaveBeenCalled();

    await user.click(screen.getByRole("button", { name: "Close" }));
    expect(commitLessonNoteCard).not.toHaveBeenCalled();
  });
});
