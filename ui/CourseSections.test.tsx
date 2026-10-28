import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import type { CourseCommandPort } from "./src/api/courseCommands";
import type { CourseDetail, LessonRow } from "./src/api/courseTypes";
import type { CourseRow } from "./src/api/libraryTypes";
import { CourseCommandsProvider } from "./src/course/CourseCommandsContext";
import { CourseScreen } from "./src/screens/CourseScreen";
import { lessonNoteCommandStubs } from "./src/test/lessonNoteCommandStubs";

function lesson(id: string, position: number, title: string): LessonRow {
  return {
    lesson_id: id,
    position,
    title,
    kind: "local_file",
    status: "not_started",
    duration_ms: 60_000,
    resume_position_ms: 0,
    max_position_reached_ms: 0,
    document_position: 0,
    playlist_item_id: null,
    playlist_url: null,
    local_media_path: `/tmp/${title}.mp4`,
    thumbnail_path: null,
  };
}

const alphaOne = lesson("l-a1", 1, "a1");
const alphaTwo = lesson("l-a2", 2, "a2");
const betaOne = lesson("l-b1", 3, "b1");

const twoSectionDetail: CourseDetail = {
  course_id: "course-1",
  title: "Demo Course",
  course_status: "not_started",
  lessons: [alphaOne, alphaTwo, betaOne],
  lesson_groups: [
    { section: "alpha", lessons: [alphaOne, alphaTwo] },
    { section: "beta", lessons: [betaOne] },
  ],
  proposal: null,
};

const courseRow: CourseRow = {
  course_id: "course-1",
  title: "Demo Course",
  source_kind: "folder",
  status: "not_started",
  remaining_ms: 180_000,
  unknown_duration_count: 0,
  is_archived: false,
  thumbnail_path: null,
};

describe("CourseScreen section grouping", () => {
  it("renders section headings and lessons in group order", async () => {
    const commands: CourseCommandPort = {
      getCourseDetail: vi.fn().mockResolvedValue(twoSectionDetail),
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
      mediaUrlForPath: vi.fn(),
      listLessonCues: vi.fn().mockResolvedValue([]),
      getTranscriptionModelPath: vi.fn().mockResolvedValue(""),
      transcribeLesson: vi.fn(),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      ...lessonNoteCommandStubs,
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

    await waitFor(() => {
      expect(screen.getByText("alpha")).toBeInTheDocument();
      expect(screen.getByText("beta")).toBeInTheDocument();
    });

    const rows = screen.getAllByRole("row");
    const titles = rows
      .map((row) => row.textContent ?? "")
      .filter((text) => /a1|a2|b1/.test(text))
      .map((text) => {
        if (text.includes("a1")) return "a1";
        if (text.includes("a2")) return "a2";
        return "b1";
      });
    expect(titles).toEqual(["a1", "a2", "b1"]);
  });
});
