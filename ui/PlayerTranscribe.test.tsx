import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import type { CourseCommandPort } from "./src/api/courseCommands";
import type { LessonRow } from "./src/api/courseTypes";
import { CourseCommandsProvider } from "./src/course/CourseCommandsContext";
import { PlayerScreen } from "./src/screens/PlayerScreen";
import { lessonNoteCommandStubs } from "./src/test/lessonNoteCommandStubs";
import { transcriptionCommandStubs } from "./src/test/transcriptionCommandStubs";
import { TRANSCRIPTION_MODEL_PATH_REQUIRED } from "./src/player/PlayerTranscribeControl";

const lesson: LessonRow = {
  lesson_id: "lesson-1",
  position: 1,
  title: "Intro",
  kind: "local_file",
  status: "in_progress",
  duration_ms: 120_000,
  resume_position_ms: 0,
  max_position_reached_ms: 0,
  document_position: 0,
  playlist_item_id: null,
  playlist_url: null,
  local_media_path: "/tmp/intro.mp4",
};

function baseCommands(
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
    mediaUrlForPath: vi.fn().mockResolvedValue("asset://intro.mp4"),
    listLessonCues: vi.fn().mockResolvedValue([]),
    listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
    recordPlayback: vi.fn().mockResolvedValue([]),
    ...lessonNoteCommandStubs,
    ...transcriptionCommandStubs,
    ...overrides,
  };
}

describe("Player transcription", () => {
  beforeEach(() => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
    HTMLMediaElement.prototype.play = vi.fn().mockResolvedValue(undefined);
    HTMLMediaElement.prototype.pause = vi.fn();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("empty model path shows requirement and does not start transcription", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const transcribeLesson = vi.fn();

    render(
      <CourseCommandsProvider
        commands={baseCommands({
          getTranscriptionModelPath: vi.fn().mockResolvedValue(""),
          transcribeLesson,
        })}
      >
        <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Transcribe" })
      ).toBeInTheDocument();
    });

    expect(
      screen.getByText(TRANSCRIPTION_MODEL_PATH_REQUIRED)
    ).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Transcribe" }));

    expect(transcribeLesson).not.toHaveBeenCalled();
    expect(
      screen.queryByText("Transcription running.")
    ).not.toBeInTheDocument();
  });
});
