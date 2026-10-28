import React from "react";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { CourseCommandPort } from "./src/api/courseCommands";
import type { CourseDetail, LessonRow } from "./src/api/courseTypes";
import type { CourseRow } from "./src/api/libraryTypes";
import { CourseCommandsProvider } from "./src/course/CourseCommandsContext";
import { SettingsCommandsProvider } from "./src/settings/SettingsCommandsContext";
import { CourseScreen } from "./src/screens/CourseScreen";
import { PlayerScreen } from "./src/screens/PlayerScreen";
import { settingsCommandStubs } from "./src/test/settingsPlaybackStubs";

const brokenLesson: LessonRow = {
  lesson_id: "lesson-broken",
  position: 1,
  title: "Missing clip",
  kind: "local_file",
  status: "broken",
  duration_ms: 60_000,
  resume_position_ms: 0,
  max_position_reached_ms: 0,
  document_position: 0,
  playlist_item_id: null,
  playlist_url: null,
  local_media_path: "/courses/linear/missing.mp4",
};

const courseRow: CourseRow = {
  course_id: "course-1",
  title: "Linear Algebra",
  source_kind: "folder",
  status: "broken",
  remaining_ms: 60_000,
  unknown_duration_count: 0,
  is_archived: false,
};

const detail: CourseDetail = {
  course_id: "course-1",
  title: "Linear Algebra",
  course_status: "broken",
  lessons: [brokenLesson],
  lesson_groups: [{ section: null, lessons: [brokenLesson] }],
  proposal: null,
};

const noopCommands: Pick<
  CourseCommandPort,
  | "markLessonComplete"
  | "markLessonIncomplete"
  | "saveLessonResume"
  | "acceptOrderProposal"
  | "rejectOrderProposal"
  | "setPlaylistItemWatched"
  | "mediaUrlForPath"
  | "listLessonCues"
  | "listLessonChapters"
  | "listLessonWatchedRanges"
  | "recordPlayback"
  | "listLessonNotes"
  | "createLessonNote"
  | "updateLessonNote"
  | "deleteLessonNote"
  | "saveDocumentPosition"
> = {
  markLessonComplete: vi.fn(),
  markLessonIncomplete: vi.fn(),
  saveLessonResume: vi.fn(),
  acceptOrderProposal: vi.fn(),
  rejectOrderProposal: vi.fn(),
  reorderLessons: vi.fn(),
  setPlaylistItemWatched: vi.fn(),
  mediaUrlForPath: vi.fn(),
  listLessonCues: vi.fn().mockResolvedValue([]),
  listLessonChapters: vi.fn().mockResolvedValue({ chapters: [], reason: null }),
  upNext: vi.fn().mockResolvedValue(null),
  listLessonResources: vi.fn().mockResolvedValue([]),
  captureLessonThumbnail: vi.fn().mockResolvedValue(undefined),
  getTranscriptionModelPath: vi.fn().mockResolvedValue(""),
  transcribeLesson: vi.fn(),
  listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
  recordPlayback: vi.fn().mockResolvedValue([]),
  listLessonNotes: vi.fn().mockResolvedValue([]),
  createLessonNote: vi.fn(),
  updateLessonNote: vi.fn(),
  deleteLessonNote: vi.fn().mockResolvedValue(undefined),
  exportLessonNotesMarkdown: vi.fn().mockResolvedValue(undefined),
  exportCourseNotesMarkdown: vi.fn().mockResolvedValue(undefined),
  saveDocumentPosition: vi.fn().mockResolvedValue(undefined),
};

describe("BrokenLesson", () => {
  it("course row shows broken status", async () => {
    const commands: CourseCommandPort = {
      getCourseDetail: vi.fn().mockResolvedValue(detail),
      markLessonBroken: vi.fn(),
      rescanCourse: vi.fn(),
      ...noopCommands,
    };

    render(
      <CourseCommandsProvider commands={commands}>
        <CourseScreen
          course={courseRow}
          onBack={() => {}}
          onPlayLesson={() => {}}
        />
      </CourseCommandsProvider>
    );

    expect(
      await screen.findByTestId("lesson-status-lesson-broken")
    ).toHaveTextContent("Broken");
  });

  it("player explains missing file path", async () => {
    const user = userEvent.setup();
    const rescanCourse = vi.fn().mockResolvedValue(undefined);
    const commands: CourseCommandPort = {
      getCourseDetail: vi.fn(),
      markLessonBroken: vi.fn(),
      rescanCourse,
      ...noopCommands,
    };

    render(
      <SettingsCommandsProvider commands={settingsCommandStubs}>
        <CourseCommandsProvider commands={commands}>
          <PlayerScreen
            lesson={brokenLesson}
            courseId="course-1"
            onBack={() => {}}
          />
        </CourseCommandsProvider>
      </SettingsCommandsProvider>
    );

    expect(
      screen.getByText(/No playable file for this lesson/i)
    ).toBeInTheDocument();
    expect(screen.getByTestId("player-media-path")).toHaveTextContent(
      "/courses/linear/missing.mp4"
    );

    await user.click(screen.getByRole("button", { name: "Rescan course" }));
    expect(rescanCourse).toHaveBeenCalledWith("course-1");
  });
});
