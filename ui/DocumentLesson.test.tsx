import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import type { CourseCommandPort } from "./src/api/courseCommands";
import type { LessonRow } from "./src/api/courseTypes";
import { CourseCommandsProvider } from "./src/course/CourseCommandsContext";
import { DocumentLessonScreen } from "./src/screens/DocumentLessonScreen";
import fixture from "./src/test/fixtures/lesson-with-remote-image.md?raw";
import { lessonNoteCommandStubs } from "./src/test/lessonNoteCommandStubs";

function documentCommands(
  overrides: Partial<CourseCommandPort> = {}
): CourseCommandPort {
  return {
    getCourseDetail: vi.fn(),
    markLessonComplete: vi.fn(),
    markLessonIncomplete: vi.fn(),
    saveLessonResume: vi.fn(),
    saveDocumentPosition: vi.fn().mockResolvedValue(undefined),
    acceptOrderProposal: vi.fn(),
    rejectOrderProposal: vi.fn(),
    reorderLessons: vi.fn(),
    setPlaylistItemWatched: vi.fn(),
    markLessonBroken: vi.fn(),
    rescanCourse: vi.fn(),
    mediaUrlForPath: vi.fn().mockResolvedValue("asset://chapter.md"),
    listLessonCues: vi.fn().mockResolvedValue([]),
    getTranscriptionModelPath: vi.fn().mockResolvedValue(""),
    transcribeLesson: vi.fn(),
    listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
    recordPlayback: vi.fn().mockResolvedValue([]),
    ...lessonNoteCommandStubs,
    ...overrides,
  };
}

const markdownLesson: LessonRow = {
  lesson_id: "lesson-md",
  position: 1,
  title: "Chapter",
  kind: "local_file",
  status: "in_progress",
  duration_ms: null,
  resume_position_ms: 0,
  max_position_reached_ms: 0,
  document_position: 0,
  playlist_item_id: null,
  playlist_url: null,
  local_media_path: "/tmp/chapter.md",
};

const pdfLesson: LessonRow = {
  ...markdownLesson,
  lesson_id: "lesson-pdf",
  title: "Slides",
  local_media_path: "/tmp/slides.pdf",
  document_position: 5,
};

describe("DocumentLessonScreen", () => {
  it("renders markdown in the player area without video or remote image fetch", async () => {
    const fetchMock = vi
      .spyOn(globalThis, "fetch")
      .mockImplementation(async (input) => {
        const url = String(input);
        if (url === "asset://chapter.md") {
          return { text: async () => fixture } as Response;
        }
        throw new Error(`unexpected fetch: ${url}`);
      });

    const { container } = render(
      <CourseCommandsProvider commands={documentCommands()}>
        <DocumentLessonScreen lesson={markdownLesson} onBack={() => {}} />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("heading", { name: "Fixture heading" })
      ).toBeInTheDocument();
    });

    expect(container.querySelector("video")).toBeNull();
    expect(screen.queryByRole("img")).not.toBeInTheDocument();
    expect(fetchMock).toHaveBeenCalledTimes(1);
    expect(fetchMock).toHaveBeenCalledWith("asset://chapter.md");
    expect(screen.getByTestId("document-lesson-scroll")).toBeInTheDocument();
    fetchMock.mockRestore();
  });

  it("shows saved pdf page number", async () => {
    render(
      <CourseCommandsProvider commands={documentCommands()}>
        <DocumentLessonScreen lesson={pdfLesson} onBack={() => {}} />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByLabelText("PDF page")).toHaveValue(5);
      expect(screen.getByTestId("document-path")).toHaveTextContent(
        "/tmp/slides.pdf"
      );
    });
  });

  it("shows empty cards line when the lesson has no links", async () => {
    const listLessonCardLinks = vi.fn().mockResolvedValue([]);

    render(
      <CourseCommandsProvider
        commands={documentCommands({ listLessonCardLinks })}
      >
        <DocumentLessonScreen lesson={markdownLesson} onBack={() => {}} />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByText("No cards from this lesson.")
      ).toBeInTheDocument();
    });
    expect(listLessonCardLinks).toHaveBeenCalledWith("lesson-md");
  });
});
