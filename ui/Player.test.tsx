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
import { PLAYBACK_WATCH_INTERVAL_MS } from "./src/player/playbackWatch";
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
      rescanCourse: vi.fn(),
      mediaUrlForPath: vi.fn().mockResolvedValue("asset://intro.mp4"),
      listLessonCues: vi.fn().mockResolvedValue([]),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
    };

    render(
      <CourseCommandsProvider commands={commands}>
        <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
      </CourseCommandsProvider>
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
      rescanCourse: vi.fn(),
      mediaUrlForPath: vi.fn().mockResolvedValue("asset://intro.mp4"),
      listLessonCues: vi.fn().mockResolvedValue([]),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
    };

    render(
      <CourseCommandsProvider commands={commands}>
        <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
      </CourseCommandsProvider>
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
      <CaptureCommandsProvider commands={captureCommands}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
        </CourseCommandsProvider>
      </CaptureCommandsProvider>
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

  it("clicking a cue seeks and saves resume immediately", async () => {
    const user = userEvent.setup({ advanceTimers: vi.advanceTimersByTime });
    const saveLessonResume = vi.fn().mockResolvedValue(undefined);
    const cues = [
      { start_ms: 1_000, end_ms: 2_000, text: "First" },
      { start_ms: 3_000, end_ms: 4_500, text: "Second" },
      { start_ms: 10_000, end_ms: 12_000, text: "Third" },
    ];
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
      rescanCourse: vi.fn(),
      mediaUrlForPath: vi.fn().mockResolvedValue("asset://intro.mp4"),
      listLessonCues: vi.fn().mockResolvedValue(cues),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
    };

    render(
      <CourseCommandsProvider commands={commands}>
        <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
      </CourseCommandsProvider>
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
      <CourseCommandsProvider commands={commands}>
        <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
      </CourseCommandsProvider>
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
      rescanCourse: vi.fn(),
      mediaUrlForPath: vi.fn().mockResolvedValue("asset://intro.mp4"),
      listLessonCues: vi.fn().mockResolvedValue([]),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback,
      ...lessonNoteCommandStubs,
      ...transcriptionCommandStubs,
    };

    render(
      <CourseCommandsProvider commands={commands}>
        <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
      </CourseCommandsProvider>
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
      <CourseCommandsProvider commands={commands}>
        <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
      </CourseCommandsProvider>
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
      <CourseCommandsProvider commands={commands}>
        <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
      </CourseCommandsProvider>
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
      <CourseCommandsProvider commands={commands}>
        <PlayerScreen lesson={lesson} courseId="course-1" onBack={() => {}} />
      </CourseCommandsProvider>
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
});
