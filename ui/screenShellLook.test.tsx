import React from "react";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import type { CourseCommandPort } from "./src/api/courseCommands";
import type { CourseDetail, LessonRow } from "./src/api/courseTypes";
import type { CourseRow } from "./src/api/libraryTypes";
import type { LibraryCommandPort } from "./src/api/libraryCommands";
import { emptyStudyHeatmap52Weeks } from "./src/api/libraryStudyHeatmapTypes";
import type { ReviewCommandPort } from "./src/api/reviewCommands";
import type { RoadmapCommandPort } from "./src/api/roadmapCommands";
import type { DueSummary } from "./src/api/types";
import { CourseCommandsProvider } from "./src/course/CourseCommandsContext";
import { LibraryCommandsProvider } from "./src/library/LibraryCommandsContext";
import { ReviewCommandsProvider } from "./src/review/ReviewCommandsContext";
import { RoadmapCommandsProvider } from "./src/roadmap/RoadmapCommandsContext";
import { CourseScreen } from "./src/screens/CourseScreen";
import { LibraryScreen } from "./src/screens/LibraryScreen";
import { PlayerScreen } from "./src/screens/PlayerScreen";
import { ReviewScreen } from "./src/screens/ReviewScreen";
import { SettingsScreen } from "./src/screens/SettingsScreen";
import { RoadmapScreen } from "./src/screens/RoadmapScreen";
import { SettingsCommandsProvider } from "./src/settings/SettingsCommandsContext";
import { AppShell, type AppRoute } from "./src/shell/AppShell";
import { lessonNoteCommandStubs } from "./src/test/lessonNoteCommandStubs";
import {
  defaultVaultSettings,
  settingsCommandStubs,
} from "./src/test/settingsPlaybackStubs";
import { transcriptionCommandStubs } from "./src/test/transcriptionCommandStubs";

import "./src/theme/theme.css";

const uiRoot = dirname(fileURLToPath(import.meta.url));
const shellCss = readFileSync(join(uiRoot, "src/theme/shell.css"), "utf8");
const tokensCss = readFileSync(join(uiRoot, "src/theme/tokens.css"), "utf8");

vi.mock("./src/player/captureVideoFrameJpeg", () => ({
  captureVideoFrameJpeg: vi
    .fn()
    .mockResolvedValue(new Uint8Array([0xff, 0xd8])),
}));

const zeroDue: DueSummary = { new: 0, learning: 0, review: 0 };

const shellAnki = {
  kind: "unavailable" as const,
  message: "Anki is not installed.",
};

function shellProps(route: AppRoute, screenTitle: string) {
  return {
    route,
    screenTitle,
    onNavigate: vi.fn(),
    onStartReview: vi.fn(),
    due: zeroDue,
    anki: shellAnki,
  };
}

function expectScreenInsideShellWithWordmark(
  screenTitle: string,
  screenRoot: HTMLElement
): void {
  expect(document.querySelector(".app-shell")).toBeTruthy();
  const body = document.querySelector(".app-body");
  expect(body).toBeTruthy();
  expect((body as HTMLElement).contains(screenRoot)).toBe(true);

  const topTitle = document.querySelector(".top-bar__title");
  expect(topTitle).toHaveTextContent(screenTitle);

  const wordmark = document.querySelector(".top-bar__wordmark");
  expect(wordmark).toHaveTextContent("Lessonbinder");
  expect(shellCss).toMatch(
    /\.top-bar__wordmark[\s\S]*font-family:\s*var\(--font-wordmark\)/
  );
  expect(tokensCss).toMatch(/--font-wordmark:\s*"Andina"/);
}

function libraryCommandStubs(): LibraryCommandPort {
  return {
    listCourses: vi.fn().mockResolvedValue([]),
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
  };
}

const localLesson: LessonRow = {
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
  thumbnail_path: null,
};

const courseRow: CourseRow = {
  course_id: "course-1",
  title: "Demo Course",
  source_kind: "folder",
  status: "not_started",
  remaining_ms: 120_000,
  unknown_duration_count: 0,
  is_archived: false,
  thumbnail_path: null,
};

const courseDetail: CourseDetail = {
  course_id: "course-1",
  title: "Demo Course",
  course_status: "not_started",
  lessons: [localLesson],
  lesson_groups: [{ section: null, lessons: [localLesson] }],
  proposal: null,
};

function courseCommandStubs(): CourseCommandPort {
  return {
    getCourseDetail: vi.fn().mockResolvedValue(courseDetail),
    markLessonComplete: vi.fn(),
    markLessonIncomplete: vi.fn(),
    saveLessonResume: vi.fn(),
    acceptOrderProposal: vi.fn(),
    rejectOrderProposal: vi.fn(),
    reorderLessons: vi.fn().mockResolvedValue(undefined),
    setPlaylistItemWatched: vi.fn(),
    markLessonBroken: vi.fn(),
    guardLessonMediaForResume: vi.fn().mockResolvedValue("in_progress"),
    rescanCourse: vi.fn().mockResolvedValue(undefined),
    cancelFolderScan: vi.fn().mockResolvedValue(undefined),
    archiveCourse: vi.fn().mockResolvedValue(undefined),
    unarchiveCourse: vi.fn().mockResolvedValue(undefined),
    mediaUrlForPath: vi.fn().mockResolvedValue("asset://intro.mp4"),
    listLessonCues: vi.fn().mockResolvedValue([]),
    listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
    recordPlayback: vi.fn().mockResolvedValue([]),
    getTranscriptionModelPath: vi.fn().mockResolvedValue(""),
    transcribeLesson: vi.fn(),
    ...lessonNoteCommandStubs,
    ...transcriptionCommandStubs,
  };
}

describe("v1.3 look pass — screen inside shell", () => {
  it("Library renders inside the shell with the wordmark face", async () => {
    render(
      <LibraryCommandsProvider commands={libraryCommandStubs()}>
        <AppShell {...shellProps("library", "Library")}>
          <LibraryScreen onOpenCourse={() => {}} onOpenLesson={() => {}} />
        </AppShell>
      </LibraryCommandsProvider>
    );

    const screenRoot = await screen.findByRole("button", {
      name: "Add folder",
    });
    expectScreenInsideShellWithWordmark("Library", screenRoot);
  });

  it("Course renders inside the shell with the wordmark face", async () => {
    render(
      <CourseCommandsProvider commands={courseCommandStubs()}>
        <AppShell {...shellProps("library", "Demo Course")}>
          <CourseScreen
            course={courseRow}
            onBack={() => {}}
            onPlayLesson={() => {}}
          />
        </AppShell>
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(
        document.querySelector(".course-screen h1.screen-title")
      ).toHaveTextContent("Demo Course");
    });
    const screenRoot = document.querySelector(
      ".course-screen h1.screen-title"
    ) as HTMLElement;
    expectScreenInsideShellWithWordmark("Demo Course", screenRoot);
  });

  it("Player renders inside the shell with the wordmark face", async () => {
    HTMLMediaElement.prototype.play = vi.fn().mockResolvedValue(undefined);
    HTMLMediaElement.prototype.pause = vi.fn();

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={courseCommandStubs()}>
          <AppShell {...shellProps("library", "Intro")}>
            <PlayerScreen
              lesson={localLesson}
              courseId="course-1"
              onBack={() => {}}
            />
          </AppShell>
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    await waitFor(() => {
      expect(
        document.querySelector(".player-screen h1.screen-title")
      ).toHaveTextContent("Intro");
    });
    const screenRoot = document.querySelector(
      ".player-screen h1.screen-title"
    ) as HTMLElement;
    expectScreenInsideShellWithWordmark("Intro", screenRoot);
  });

  it("Review renders inside the shell with the wordmark face", async () => {
    const commands: ReviewCommandPort = {
      fetchNextCard: vi.fn().mockResolvedValue(null),
      fetchDueSummary: vi.fn().mockResolvedValue(zeroDue),
      fetchNoteBack: vi.fn(),
      readMedia: vi.fn(),
      answerCard: vi.fn(),
    };

    render(
      <ReviewCommandsProvider commands={commands}>
        <AppShell {...shellProps("review", "Review")}>
          <ReviewScreen
            onBackToLibrary={() => {}}
            onOpenInAnkiDesktop={() => {}}
            desktopAnkiAvailable={false}
          />
        </AppShell>
      </ReviewCommandsProvider>
    );

    const screenRoot = await screen.findByText("The review queue is empty.");
    expectScreenInsideShellWithWordmark("Review", screenRoot);
  });

  it("Settings renders inside the shell with the wordmark face", async () => {
    render(
      <SettingsCommandsProvider
        commands={{
          ...settingsCommandStubs,
          getVaultSettings: vi.fn().mockResolvedValue(defaultVaultSettings),
          getAnkiProfilePath: vi.fn().mockResolvedValue("/data/anki-profile"),
        }}
      >
        <AppShell {...shellProps("settings", "Settings")}>
          <SettingsScreen />
        </AppShell>
      </SettingsCommandsProvider>
    );

    const screenRoot = await screen.findByLabelText("Tesseract language");
    expectScreenInsideShellWithWordmark("Settings", screenRoot);
  });

  it("Roadmap renders inside the shell with the wordmark face", async () => {
    const roadmapCommands: RoadmapCommandPort = {
      listRoadmaps: vi
        .fn()
        .mockResolvedValue([{ roadmap_id: "roadmap-1", title: "Study path" }]),
      createRoadmap: vi.fn(),
      getRoadmapDetail: vi.fn().mockResolvedValue({
        roadmap_id: "roadmap-1",
        title: "Study path",
        status: "not_started",
        remaining_ms: 0,
        unknown_duration_count: 0,
        is_archived: false,
        steps: [],
        graph: { nodes: [], edges: [] },
      }),
      addRoadmapStep: vi.fn(),
      reorderRoadmapSteps: vi.fn(),
      updateRoadmapStepNote: vi.fn(),
      addPrerequisiteEdge: vi.fn(),
      removePrerequisiteEdge: vi.fn(),
      exportRoadmapGraphSvg: vi.fn(),
      relinkRoadmapStep: vi.fn(),
    };

    render(
      <LibraryCommandsProvider commands={libraryCommandStubs()}>
        <RoadmapCommandsProvider commands={roadmapCommands}>
          <AppShell {...shellProps("roadmap", "Roadmap")}>
            <RoadmapScreen />
          </AppShell>
        </RoadmapCommandsProvider>
      </LibraryCommandsProvider>
    );

    const screenRoot = await screen.findByRole("heading", {
      name: "Study path",
      level: 1,
    });
    expect(screenRoot).toHaveClass("screen-title");
    expectScreenInsideShellWithWordmark("Roadmap", screenRoot);
  });
});
