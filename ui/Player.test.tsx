import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import type { CaptureCommandPort } from "./src/api/captureCommands";
import { CaptureCommandsProvider } from "./src/capture/CaptureCommandsContext";
import type { CourseCommandPort } from "./src/api/courseCommands";
import type { LessonRow } from "./src/api/courseTypes";
import { captureCommandStubs } from "./src/test/captureCommandStubs";
import { CourseCommandsProvider } from "./src/course/CourseCommandsContext";
import { SettingsCommandsProvider } from "./src/settings/SettingsCommandsContext";
import { PLAYBACK_WATCH_INTERVAL_MS } from "./src/player/playbackWatch";
import type { SettingsCommandPort } from "./src/api/settingsCommands";
import {
  defaultVaultSettings,
  settingsCommandStubs,
} from "./src/test/settingsPlaybackStubs";
import {
  PlayerScreen,
  RESUME_SAVE_INTERVAL_MS,
} from "./src/screens/PlayerScreen";

vi.mock("./src/player/captureVideoFrameJpeg", () => ({
  captureVideoFrameJpeg: vi
    .fn()
    .mockResolvedValue(new Uint8Array([0xff, 0xd8])),
}));
import { lessonNoteCommandStubs } from "./src/test/lessonNoteCommandStubs";
import { transcriptionCommandStubs } from "./src/test/transcriptionCommandStubs";

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

describe("PlayerScreen", () => {
  beforeEach(() => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
    HTMLMediaElement.prototype.play = vi.fn().mockResolvedValue(undefined);
    HTMLMediaElement.prototype.pause = vi.fn();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("persists resume on interval while playing", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const saveLessonResume = vi.fn().mockResolvedValue(undefined);
    const commands: CourseCommandPort = {
      getCourseDetail: vi.fn(),
      markLessonComplete: vi.fn(),
      markLessonIncomplete: vi.fn(),
      saveLessonResume,
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
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole("button", { name: "Play" })).toBeInTheDocument();
    });

    const video = document.querySelector("video") as HTMLVideoElement;
    Object.defineProperty(video, "currentTime", {
      writable: true,
      value: 12,
    });

    await user.click(screen.getByRole("button", { name: "Play" }));

    await vi.advanceTimersByTimeAsync(RESUME_SAVE_INTERVAL_MS);

    await waitFor(() => {
      expect(saveLessonResume).toHaveBeenCalledWith("lesson-1", 12_000);
    });
  });

  it("space toggles play and pause when capture is closed", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
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
      mediaUrlForPath: vi.fn().mockResolvedValue("asset://intro.mp4"),
      listLessonCues: vi.fn().mockResolvedValue([]),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole("button", { name: "Play" })).toBeInTheDocument();
    });

    const video = document.querySelector("video") as HTMLVideoElement;
    let paused = true;
    Object.defineProperty(video, "paused", {
      configurable: true,
      get: () => paused,
    });
    video.play = vi.fn().mockImplementation(() => {
      paused = false;
      return Promise.resolve();
    });
    video.pause = vi.fn().mockImplementation(() => {
      paused = true;
    });

    await user.keyboard(" ");
    expect(video.play).toHaveBeenCalled();
    expect(screen.getByRole("button", { name: "Pause" })).toBeInTheDocument();

    await user.keyboard(" ");
    expect(video.pause).toHaveBeenCalled();
    expect(screen.getByRole("button", { name: "Play" })).toBeInTheDocument();
  });

  it("d key marks complete and exposes the shortcut on the control", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const markLessonComplete = vi.fn().mockResolvedValue(undefined);
    const onBack = vi.fn();
    const commands: CourseCommandPort = {
      getCourseDetail: vi.fn(),
      markLessonComplete,
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
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={onBack} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Mark complete (D)" })
      ).toBeInTheDocument();
    });

    await user.keyboard("d");

    await waitFor(() => {
      expect(markLessonComplete).toHaveBeenCalledWith("lesson-1");
      expect(onBack).toHaveBeenCalled();
    });
  });

  it("escape closes capture and returns focus to the capture button", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
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
      mediaUrlForPath: vi.fn().mockResolvedValue("asset://intro.mp4"),
      listLessonCues: vi.fn().mockResolvedValue([]),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
    };

    const drawImage = vi.fn();
    HTMLCanvasElement.prototype.getContext = vi.fn(
      () => ({ drawImage }) as unknown as CanvasRenderingContext2D
    );
    HTMLCanvasElement.prototype.toDataURL = vi.fn(
      () => "data:image/png;base64,abc"
    );

    const captureCommands: CaptureCommandPort = {
      ...captureCommandStubs,
      captureOcrStatus: vi.fn().mockResolvedValue({ available: true }),
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CaptureCommandsProvider commands={captureCommands}>
          <CourseCommandsProvider commands={commands}>
            <PlayerScreen
              lesson={lesson}
              courseId="course-1"
              onBack={() => {}}
            />
          </CourseCommandsProvider>
        </CaptureCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByTestId("player-capture-button")).toBeInTheDocument();
    });

    const video = document.querySelector("video") as HTMLVideoElement;
    Object.defineProperty(video, "videoWidth", {
      value: 640,
      configurable: true,
    });
    Object.defineProperty(video, "videoHeight", {
      value: 360,
      configurable: true,
    });

    await user.click(screen.getByTestId("player-capture-button"));

    await waitFor(() => {
      expect(
        screen.getByRole("dialog", { name: /capture card/i })
      ).toBeInTheDocument();
    });

    await user.keyboard("{Escape}");

    await waitFor(() => {
      expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    });
    expect(screen.getByTestId("player-capture-button")).toHaveFocus();
  });

  it("clicking a cue seeks local playback and saves resume without opening a URL or marking complete", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const saveLessonResume = vi.fn().mockResolvedValue(undefined);
    const markLessonComplete = vi.fn();
    const openWindow = vi.spyOn(window, "open").mockImplementation(() => null);
    const cues = [
      { start_ms: 1_000, end_ms: 2_000, text: "First" },
      { start_ms: 3_000, end_ms: 4_500, text: "Second" },
      { start_ms: 10_000, end_ms: 12_000, text: "Third" },
    ];
    const commands: CourseCommandPort = {
      getCourseDetail: vi.fn(),
      markLessonComplete,
      markLessonIncomplete: vi.fn(),
      saveLessonResume,
      acceptOrderProposal: vi.fn(),
      rejectOrderProposal: vi.fn(),
      reorderLessons: vi.fn(),
      setPlaylistItemWatched: vi.fn(),
      markLessonBroken: vi.fn(),
      guardLessonMediaForResume: vi.fn().mockResolvedValue("in_progress"),
      rescanCourse: vi.fn(),
      mediaUrlForPath: vi.fn().mockResolvedValue("asset://intro.mp4"),
      listLessonCues: vi.fn().mockResolvedValue(cues),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByText("Second")).toBeInTheDocument();
    });

    const video = document.querySelector("video") as HTMLVideoElement;
    Object.defineProperty(video, "currentTime", {
      writable: true,
      value: 0,
    });

    await user.click(screen.getByRole("button", { name: /Second/ }));

    expect(video.currentTime).toBe(3);
    expect(saveLessonResume).toHaveBeenCalledWith("lesson-1", 3_000);
    expect(markLessonComplete).not.toHaveBeenCalled();
    expect(openWindow).not.toHaveBeenCalled();
    openWindow.mockRestore();
  });

  it("seeks to initialSeekMs when the lesson loads (search hit navigation)", async () => {
    const saveLessonResume = vi.fn().mockResolvedValue(undefined);
    const commands: CourseCommandPort = {
      getCourseDetail: vi.fn(),
      markLessonComplete: vi.fn(),
      markLessonIncomplete: vi.fn(),
      saveLessonResume,
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
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen
            lesson={lesson}
            courseId="course-1"
            initialSeekMs={12_500}
            onBack={() => {}}
          />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(document.querySelector("video")).toBeTruthy();
    });

    const video = document.querySelector("video") as HTMLVideoElement;
    Object.defineProperty(video, "duration", {
      configurable: true,
      value: 120,
    });
    Object.defineProperty(video, "currentTime", {
      configurable: true,
      writable: true,
      value: 0,
    });

    video.dispatchEvent(new Event("loadedmetadata"));

    expect(video.currentTime).toBe(12.5);
    expect(HTMLMediaElement.prototype.play).not.toHaveBeenCalled();
  });

  it("persists resume when playback starts without waiting for the interval", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const saveLessonResume = vi.fn().mockResolvedValue(undefined);
    const commands: CourseCommandPort = {
      getCourseDetail: vi.fn(),
      markLessonComplete: vi.fn(),
      markLessonIncomplete: vi.fn(),
      saveLessonResume,
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
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(document.querySelector("video")).toBeTruthy();
    });

    const video = document.querySelector("video") as HTMLVideoElement;
    Object.defineProperty(video, "duration", {
      configurable: true,
      value: 120,
    });
    Object.defineProperty(video, "currentTime", {
      configurable: true,
      writable: true,
      value: 5,
    });
    video.dispatchEvent(new Event("loadedmetadata"));

    await user.click(screen.getByRole("button", { name: /play/i }));

    expect(saveLessonResume).toHaveBeenCalledWith("lesson-1", 5_000);
  });

  it("ArrowLeft and ArrowRight seek five seconds while the player scrubber is focused", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const saveLessonResume = vi.fn().mockResolvedValue(undefined);
    const lessonAtThirty: LessonRow = {
      ...lesson,
      resume_position_ms: 30_000,
      max_position_reached_ms: 30_000,
    };
    const commands: CourseCommandPort = {
      getCourseDetail: vi.fn(),
      markLessonComplete: vi.fn(),
      markLessonIncomplete: vi.fn(),
      saveLessonResume,
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
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen
            lesson={lessonAtThirty}
            courseId="course-1"
            onBack={() => {}}
          />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(document.querySelector("video")).toBeTruthy();
    });

    const video = document.querySelector("video") as HTMLVideoElement;
    Object.defineProperty(video, "duration", {
      configurable: true,
      value: 120,
    });
    Object.defineProperty(video, "currentTime", {
      configurable: true,
      writable: true,
      value: 30,
    });
    video.dispatchEvent(new Event("loadedmetadata"));
    video.dispatchEvent(new Event("timeupdate"));

    const scrubber = screen.getByRole("slider", {
      name: "Playback position",
    });
    scrubber.focus();

    await user.keyboard("{ArrowLeft}");
    expect(saveLessonResume).toHaveBeenCalledWith("lesson-1", 25_000);
    expect(video.currentTime).toBe(25);

    saveLessonResume.mockClear();
    await user.keyboard("{ArrowRight}");
    expect(saveLessonResume).toHaveBeenCalledWith("lesson-1", 30_000);
    expect(video.currentTime).toBe(30);
  });

  it("persists resume when the player unmounts", async () => {
    const saveLessonResume = vi.fn().mockResolvedValue(undefined);
    const commands: CourseCommandPort = {
      getCourseDetail: vi.fn(),
      markLessonComplete: vi.fn(),
      markLessonIncomplete: vi.fn(),
      saveLessonResume,
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
    };

    const { unmount } = render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(document.querySelector("video")).toBeTruthy();
    });

    const video = document.querySelector("video") as HTMLVideoElement;
    Object.defineProperty(video, "currentTime", {
      configurable: true,
      writable: true,
      value: 7.25,
    });
    video.dispatchEvent(new Event("timeupdate"));

    unmount();

    expect(saveLessonResume).toHaveBeenCalledWith("lesson-1", 7_250);
  });

  it("clicking a chapter seeks and saves resume immediately", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const saveLessonResume = vi.fn().mockResolvedValue(undefined);
    const listLessonChapters = vi.fn().mockResolvedValue({
      chapters: [
        { start_ms: 0, title: "Opening" },
        { start_ms: 125_500, title: "Main topic" },
      ],
      reason: null,
    });
    const commands: CourseCommandPort = {
      getCourseDetail: vi.fn(),
      markLessonComplete: vi.fn(),
      markLessonIncomplete: vi.fn(),
      saveLessonResume,
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
      listLessonChapters,
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: /Main topic/i })
      ).toBeInTheDocument();
    });

    const video = document.querySelector("video") as HTMLVideoElement;
    Object.defineProperty(video, "currentTime", {
      writable: true,
      value: 0,
    });

    await user.click(screen.getByRole("button", { name: /Main topic/i }));

    expect(video.currentTime).toBe(125.5);
    expect(saveLessonResume).toHaveBeenCalledWith("lesson-1", 125_500);
  });

  it("records watched playback on interval without marking complete", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const recordPlayback = vi
      .fn()
      .mockResolvedValue([{ start_ms: 12_000, end_ms: 13_000 }]);
    const markLessonComplete = vi.fn();
    const commands: CourseCommandPort = {
      getCourseDetail: vi.fn(),
      markLessonComplete,
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
      recordPlayback,
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole("button", { name: "Play" })).toBeInTheDocument();
    });

    const video = document.querySelector("video") as HTMLVideoElement;
    let current = 12;
    Object.defineProperty(video, "currentTime", {
      configurable: true,
      get: () => current,
      set: (value: number) => {
        current = value;
      },
    });

    await user.click(screen.getByRole("button", { name: "Play" }));
    current = 13;
    await vi.advanceTimersByTimeAsync(PLAYBACK_WATCH_INTERVAL_MS);

    await waitFor(() => {
      expect(recordPlayback).toHaveBeenCalledWith("lesson-1", 12_000, 13_000);
    });
    expect(markLessonComplete).not.toHaveBeenCalled();
  });

  it("lists lesson resources and opens pdf and image viewers", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const listLessonResources = vi.fn().mockResolvedValue([
      {
        resource_id: "res-pdf",
        filename: "slides.pdf",
        kind: "pdf",
        absolute_path: "/tmp/course/slides.pdf",
      },
      {
        resource_id: "res-img",
        filename: "diagram.png",
        kind: "image",
        absolute_path: "/tmp/course/diagram.png",
      },
    ]);
    const mediaUrlForPath = vi.fn().mockImplementation((path: string) => {
      return Promise.resolve(`asset://${path}`);
    });
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
      mediaUrlForPath,
      listLessonCues: vi.fn().mockResolvedValue([]),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
      listLessonResources,
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByText("slides.pdf")).toBeInTheDocument();
      expect(screen.getByText("diagram.png")).toBeInTheDocument();
      expect(screen.getByText("PDF")).toBeInTheDocument();
      expect(screen.getByText("Image")).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: /slides\.pdf/i }));

    await waitFor(() => {
      expect(screen.getByTestId("document-pdf-frame")).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: /diagram\.png/i }));

    await waitFor(() => {
      const image = screen.getByTestId("lesson-resource-image");
      expect(image).toHaveAttribute("src", "asset:///tmp/course/diagram.png");
    });

    expect(listLessonResources).toHaveBeenCalledWith("lesson-1");
  });

  it("use this frame as thumbnail calls command with current playback time", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const captureLessonThumbnail = vi.fn().mockResolvedValue(undefined);
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
      mediaUrlForPath: vi.fn().mockResolvedValue("asset://intro.mp4"),
      listLessonCues: vi.fn().mockResolvedValue([]),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
      captureLessonThumbnail,
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByTestId("player-thumbnail-button")).toBeInTheDocument();
    });

    const video = document.querySelector("video") as HTMLVideoElement;
    Object.defineProperty(video, "videoWidth", {
      value: 640,
      configurable: true,
    });
    Object.defineProperty(video, "videoHeight", {
      value: 360,
      configurable: true,
    });
    Object.defineProperty(video, "currentTime", {
      writable: true,
      value: 4.5,
    });

    await user.click(screen.getByTestId("player-thumbnail-button"));

    await waitFor(() => {
      expect(captureLessonThumbnail).toHaveBeenCalledWith(
        "lesson-1",
        4_500,
        new Uint8Array([0xff, 0xd8])
      );
    });
  });

  it("lists a lesson card link without opening Anki", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const openSpy = vi.spyOn(window, "open").mockImplementation(() => null);
    const listLessonCardLinks = vi.fn().mockResolvedValue([
      {
        anki_note_id: 9001,
        timestamp_ms: 83_000,
        front_preview: "What is X?",
      },
    ]);
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
      mediaUrlForPath: vi.fn().mockResolvedValue("asset://intro.mp4"),
      listLessonCues: vi.fn().mockResolvedValue([]),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
      listLessonCardLinks,
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByText("What is X?")).toBeInTheDocument();
      expect(screen.getByText("1:23")).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: /What is X\?/ }));

    expect(openSpy).not.toHaveBeenCalled();
    expect(listLessonCardLinks).toHaveBeenCalledWith("lesson-1");
    openSpy.mockRestore();
  });

  it("theater mode hides lesson notes and Escape exits without marking complete", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const markLessonComplete = vi.fn();
    const onBack = vi.fn();
    const commands: CourseCommandPort = {
      getCourseDetail: vi.fn(),
      markLessonComplete,
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
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={onBack} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("region", { name: "Lesson notes" })
      ).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Theater" }));

    expect(
      screen.queryByRole("region", { name: "Lesson notes" })
    ).not.toBeInTheDocument();
    expect(markLessonComplete).not.toHaveBeenCalled();

    await user.keyboard("{Escape}");

    expect(
      screen.getByRole("region", { name: "Lesson notes" })
    ).toBeInTheDocument();
    expect(markLessonComplete).not.toHaveBeenCalled();
    expect(onBack).not.toHaveBeenCalled();
  });

  it("reveals theater controls after a key press when they were hidden", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
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
      mediaUrlForPath: vi.fn().mockResolvedValue("asset://intro.mp4"),
      listLessonCues: vi.fn().mockResolvedValue([]),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByTestId("player-bar")).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Theater" }));
    await vi.advanceTimersByTimeAsync(2_600);

    await waitFor(() => {
      expect(screen.getByTestId("player-bar")).toHaveClass(
        "player-bar--theater-hidden"
      );
    });

    await user.keyboard("{ArrowDown}");

    expect(screen.getByTestId("player-bar")).not.toHaveClass(
      "player-bar--theater-hidden"
    );
  });

  it("hides picture-in-picture when the video API is unavailable", async () => {
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
      mediaUrlForPath: vi.fn().mockResolvedValue("asset://intro.mp4"),
      listLessonCues: vi.fn().mockResolvedValue([]),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole("button", { name: "Play" })).toBeInTheDocument();
    });

    expect(screen.queryByTestId("player-pip-button")).not.toBeInTheDocument();
  });

  it("zen mode shows only the video and Escape exits without marking complete", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const markLessonComplete = vi.fn();
    const onBack = vi.fn();
    const commands: CourseCommandPort = {
      getCourseDetail: vi.fn(),
      markLessonComplete,
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
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={onBack} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByTestId("player-bar")).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Zen" }));

    expect(screen.getByTestId("player-zen-screen")).toBeInTheDocument();
    expect(screen.queryByTestId("player-bar")).not.toBeInTheDocument();
    expect(
      screen.queryByRole("region", { name: "Lesson notes" })
    ).not.toBeInTheDocument();
    expect(markLessonComplete).not.toHaveBeenCalled();

    await user.keyboard("{Escape}");

    expect(screen.queryByTestId("player-zen-screen")).not.toBeInTheDocument();
    expect(screen.getByTestId("player-bar")).toBeInTheDocument();
    expect(markLessonComplete).not.toHaveBeenCalled();
    expect(onBack).not.toHaveBeenCalled();
  });

  it("entering zen exits theater", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
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
      mediaUrlForPath: vi.fn().mockResolvedValue("asset://intro.mp4"),
      listLessonCues: vi.fn().mockResolvedValue([]),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByTestId("player-bar")).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Theater" }));
    expect(
      screen.getByRole("button", { name: "Exit theater" })
    ).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Zen" }));

    expect(screen.getByTestId("player-zen-screen")).toBeInTheDocument();
    expect(screen.queryByTestId("player-bar")).not.toBeInTheDocument();
  });

  it("loads persisted playback speed and saves changes to vault settings", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const getVaultSettings = vi.fn().mockResolvedValue({
      ...defaultVaultSettings,
      playback_speed: 1.25,
    });
    const saveVaultSettings = vi.fn().mockResolvedValue(undefined);
    const settingsCommands: SettingsCommandPort = {
      ...settingsCommandStubs,
      getVaultSettings,
      saveVaultSettings,
    };
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
      mediaUrlForPath: vi.fn().mockResolvedValue("asset://intro.mp4"),
      listLessonCues: vi.fn().mockResolvedValue([]),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
    };

    render(
      <SettingsCommandsProvider commands={settingsCommands}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByLabelText("Playback speed")).toHaveValue("1.25");
    });

    const video = document.querySelector("video") as HTMLVideoElement;
    await waitFor(() => {
      expect(video.playbackRate).toBe(1.25);
    });

    await user.selectOptions(screen.getByLabelText("Playback speed"), "1.5");

    await waitFor(() => {
      expect(video.playbackRate).toBe(1.5);
      expect(saveVaultSettings).toHaveBeenCalledWith({
        ...defaultVaultSettings,
        playback_speed: 1.5,
      });
    });
  });

  it("loads persisted volume and mute and saves changes", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const getVaultSettings = vi.fn().mockResolvedValue({
      ...defaultVaultSettings,
      player_volume: 0.4,
      player_muted: true,
    });
    const saveVaultSettings = vi.fn().mockResolvedValue(undefined);
    const settingsCommands: SettingsCommandPort = {
      ...settingsCommandStubs,
      getVaultSettings,
      saveVaultSettings,
    };
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
      mediaUrlForPath: vi.fn().mockResolvedValue("asset://intro.mp4"),
      listLessonCues: vi.fn().mockResolvedValue([]),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
    };

    render(
      <SettingsCommandsProvider commands={settingsCommands}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByLabelText("Volume")).toHaveValue("40");
      expect(
        screen.getByRole("button", { name: "Unmute" })
      ).toBeInTheDocument();
    });

    const video = document.querySelector("video") as HTMLVideoElement;
    await waitFor(() => {
      expect(video.volume).toBeCloseTo(0.4);
      expect(video.muted).toBe(true);
    });

    await user.click(screen.getByRole("button", { name: "Unmute" }));

    await waitFor(() => {
      expect(video.muted).toBe(false);
      expect(saveVaultSettings).toHaveBeenCalledWith({
        ...defaultVaultSettings,
        player_volume: 0.4,
        player_muted: false,
      });
    });
  });

  it("shows one sentence when the file manager cannot open the lesson file folder", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const openLessonFileDirectoryInFileManager = vi.fn().mockRejectedValue(
      new Error(
        JSON.stringify({
          code: "opener_open_path_failed",
          message: "Could not open that folder in your file manager.",
        })
      )
    );
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
      mediaUrlForPath: vi.fn().mockResolvedValue("asset://intro.mp4"),
      listLessonCues: vi.fn().mockResolvedValue([]),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
      openLessonFileDirectoryInFileManager,
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Show lesson file" })
      ).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Show lesson file" }));

    await waitFor(() => {
      expect(screen.getByRole("alert")).toHaveTextContent(
        "Could not open that folder in your file manager."
      );
    });
    expect(screen.getByRole("alert").textContent).not.toMatch(/stack/i);
  });
});
