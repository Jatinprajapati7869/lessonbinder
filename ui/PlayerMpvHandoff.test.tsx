import React from "react";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import type { CourseCommandPort } from "./src/api/courseCommands";
import type { LessonRow } from "./src/api/courseTypes";
import { CourseCommandsProvider } from "./src/course/CourseCommandsContext";
import { SettingsCommandsProvider } from "./src/settings/SettingsCommandsContext";
import { MPV_NOT_ON_PATH_MESSAGE } from "./src/player/mpvHandoffMessage";
import { PlayerScreen } from "./src/screens/PlayerScreen";
import { lessonNoteCommandStubs } from "./src/test/lessonNoteCommandStubs";
import { settingsCommandStubs } from "./src/test/settingsPlaybackStubs";
import { transcriptionCommandStubs } from "./src/test/transcriptionCommandStubs";

vi.mock("./src/player/captureVideoFrameJpeg", () => ({
  captureVideoFrameJpeg: vi
    .fn()
    .mockResolvedValue(new Uint8Array([0xff, 0xd8])),
}));

vi.mock("./src/api/mpvCommands", async (importOriginal) => {
  const mod = await importOriginal<typeof import("./src/api/mpvCommands")>();
  return {
    ...mod,
    startMpvLessonHandoff: vi.fn(),
    stopMpvLessonHandoff: vi.fn().mockResolvedValue(undefined),
    updateMpvLessonEmbedRect: vi.fn().mockResolvedValue(undefined),
  };
});

import { startMpvLessonHandoff } from "./src/api/mpvCommands";

const lesson: LessonRow = {
  lesson_id: "lesson-1",
  position: 1,
  title: "Intro",
  kind: "local_file",
  status: "in_progress",
  duration_ms: 120_000,
  resume_position_ms: 5_000,
  max_position_reached_ms: 5_000,
  document_position: 0,
  playlist_item_id: null,
  playlist_url: null,
  local_media_path: "/tmp/intro.mp4",
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
    guardLessonMediaForResume: vi.fn().mockResolvedValue("in_progress"),
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

function renderPlayer(commands: CourseCommandPort) {
  return render(
    <SettingsCommandsProvider commands={settingsCommandStubs}>
      <CourseCommandsProvider commands={commands}>
        <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
      </CourseCommandsProvider>
    </SettingsCommandsProvider>
  );
}

describe("PlayerScreen mpv handoff", () => {
  beforeEach(() => {
    vi.mocked(startMpvLessonHandoff).mockReset();
    vi.mocked(startMpvLessonHandoff).mockResolvedValue(undefined);
    HTMLMediaElement.prototype.play = vi.fn().mockResolvedValue(undefined);
    HTMLMediaElement.prototype.pause = vi.fn();
    class ResizeObserverMock {
      observe = vi.fn();
      unobserve = vi.fn();
      disconnect = vi.fn();
    }
    vi.stubGlobal("ResizeObserver", ResizeObserverMock);
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it("starts mpv handoff on video error without marking the lesson done", async () => {
    const markLessonComplete = vi.fn();
    renderPlayer(courseCommands({ markLessonComplete }));

    await waitFor(() => {
      expect(document.querySelector("video")).toBeTruthy();
    });

    const video = document.querySelector("video") as HTMLVideoElement;
    fireEvent.error(video);

    await waitFor(() => {
      expect(startMpvLessonHandoff).toHaveBeenCalledTimes(1);
    });

    expect(startMpvLessonHandoff).toHaveBeenCalledWith(
      "/tmp/intro.mp4",
      expect.objectContaining({
        width: expect.any(Number),
        height: expect.any(Number),
      }),
      5
    );
    expect(markLessonComplete).not.toHaveBeenCalled();
  });

  it("shows the one-sentence message when mpv is missing", async () => {
    vi.mocked(startMpvLessonHandoff).mockRejectedValue(
      new Error(
        JSON.stringify({
          code: "mpv_not_found",
          message: MPV_NOT_ON_PATH_MESSAGE,
        })
      )
    );
    const markLessonComplete = vi.fn();
    renderPlayer(courseCommands({ markLessonComplete }));

    await waitFor(() => {
      expect(document.querySelector("video")).toBeTruthy();
    });

    fireEvent.error(document.querySelector("video") as HTMLVideoElement);

    expect(
      await screen.findByTestId("player-mpv-handoff-message")
    ).toHaveTextContent(MPV_NOT_ON_PATH_MESSAGE);
    expect(markLessonComplete).not.toHaveBeenCalled();
  });
});

describe("PlayerScreen overlay transport bar", () => {
  beforeEach(() => {
    HTMLMediaElement.prototype.play = vi.fn().mockResolvedValue(undefined);
    HTMLMediaElement.prototype.pause = vi.fn();
    class ResizeObserverMock {
      observe = vi.fn();
      unobserve = vi.fn();
      disconnect = vi.fn();
    }
    vi.stubGlobal("ResizeObserver", ResizeObserverMock);
  });

  it("keeps play, speed, volume, theater, and zen on the frame overlay only", async () => {
    renderPlayer(courseCommands());

    await waitFor(() => {
      expect(screen.getByTestId("player-bar")).toBeInTheDocument();
    });

    const bar = screen.getByTestId("player-bar");
    expect(bar).toHaveClass("player-bar--overlay");
    expect(bar.closest(".player-frame")).toBeTruthy();
    expect(document.querySelectorAll(".player-bar")).toHaveLength(1);

    expect(bar).toContainElement(screen.getByRole("button", { name: "Play" }));
    expect(bar).toContainElement(screen.getByTestId("player-theater-toggle"));
    expect(bar).toContainElement(screen.getByTestId("player-zen-toggle"));
    expect(bar).toContainElement(screen.getByText("Speed"));
    expect(bar).toContainElement(screen.getByText("Volume"));

    const title = document.querySelector(".player-screen h1.screen-title");
    expect(title).toHaveTextContent("Intro");
    const titleIndex = Array.from(
      document.querySelector(".player-screen")!.children
    ).indexOf(title as Element);
    const main = document.querySelector(".player-main");
    expect(main).toBeTruthy();
    const mainIndex = Array.from(
      document.querySelector(".player-screen")!.children
    ).indexOf(main as Element);
    expect(mainIndex).toBeGreaterThan(titleIndex);
    expect(document.querySelector(".player-screen > .player-bar")).toBeNull();
  });
});
