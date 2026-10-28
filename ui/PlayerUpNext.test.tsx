import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import type { CourseCommandPort } from "./src/api/courseCommands";
import type { LessonRow } from "./src/api/courseTypes";
import { CourseCommandsProvider } from "./src/course/CourseCommandsContext";
import { SettingsCommandsProvider } from "./src/settings/SettingsCommandsContext";
import { PlayerScreen } from "./src/screens/PlayerScreen";
import { settingsCommandStubs } from "./src/test/settingsPlaybackStubs";
import { lessonNoteCommandStubs } from "./src/test/lessonNoteCommandStubs";
import { transcriptionCommandStubs } from "./src/test/transcriptionCommandStubs";

const lesson: LessonRow = {
  lesson_id: "lesson-1",
  position: 1,
  title: "First lesson",
  kind: "local_file",
  status: "in_progress",
  duration_ms: 120_000,
  resume_position_ms: 0,
  max_position_reached_ms: 0,
  document_position: 0,
  playlist_item_id: null,
  playlist_url: null,
  local_media_path: "/tmp/first.mp4",
  thumbnail_path: null,
};

describe("PlayerScreen up next", () => {
  beforeEach(() => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
    HTMLMediaElement.prototype.play = vi.fn().mockResolvedValue(undefined);
    HTMLMediaElement.prototype.pause = vi.fn();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("shows Up next and calls the play handler when a target exists", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const onPlayUpNext = vi.fn();
    const upNext = { lesson_id: "lesson-3", title: "Third lesson" };
    const commands: CourseCommandPort = {
      getCourseDetail: vi.fn(),
      markLessonComplete: vi.fn(),
      markLessonIncomplete: vi.fn(),
      saveLessonResume: vi.fn(),
      acceptOrderProposal: vi.fn(),
      rejectOrderProposal: vi.fn(),
      reorderLessons: vi.fn(),
      setPlaylistItemWatched: vi.fn(),
      markLessonBroken: vi.fn(),
      guardLessonMediaForResume: vi.fn().mockResolvedValue("in_progress"),
      rescanCourse: vi.fn(),
      mediaUrlForPath: vi.fn().mockResolvedValue("asset://first.mp4"),
      listLessonCues: vi.fn().mockResolvedValue([]),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
      upNext: vi.fn().mockResolvedValue(upNext),
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen
            lesson={lesson}
            courseId="course-1"
            onBack={() => {}}
            onPlayUpNext={onPlayUpNext}
          />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByTestId("player-up-next-button")).toHaveTextContent(
        "Up next: Third lesson"
      );
    });

    await user.click(screen.getByTestId("player-up-next-button"));

    expect(onPlayUpNext).toHaveBeenCalledWith(upNext);
  });

  it("hides Up next when the command returns null", async () => {
    const commands: CourseCommandPort = {
      getCourseDetail: vi.fn(),
      markLessonComplete: vi.fn(),
      markLessonIncomplete: vi.fn(),
      saveLessonResume: vi.fn(),
      acceptOrderProposal: vi.fn(),
      rejectOrderProposal: vi.fn(),
      reorderLessons: vi.fn(),
      setPlaylistItemWatched: vi.fn(),
      markLessonBroken: vi.fn(),
      guardLessonMediaForResume: vi.fn().mockResolvedValue("in_progress"),
      rescanCourse: vi.fn(),
      mediaUrlForPath: vi.fn().mockResolvedValue("asset://first.mp4"),
      listLessonCues: vi.fn().mockResolvedValue([]),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
      upNext: vi.fn().mockResolvedValue(null),
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen
            lesson={lesson}
            courseId="course-1"
            onBack={() => {}}
            onPlayUpNext={() => {}}
          />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(commands.upNext).toHaveBeenCalledWith("lesson-1");
    });

    expect(
      screen.queryByTestId("player-up-next-button")
    ).not.toBeInTheDocument();
  });
});
