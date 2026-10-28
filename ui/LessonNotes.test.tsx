import React from "react";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { CourseCommandPort } from "./src/api/courseCommands";
import type { LessonNote } from "./src/api/courseTypes";
import { CourseCommandsProvider } from "./src/course/CourseCommandsContext";
import { LessonNotesColumn } from "./src/lesson/LessonNotesColumn";

function noteCommands(
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
    mediaUrlForPath: vi.fn(),
    listLessonCues: vi.fn().mockResolvedValue([]),
    listLessonResources: vi.fn().mockResolvedValue([]),
    captureLessonThumbnail: vi.fn().mockResolvedValue(undefined),
    getTranscriptionModelPath: vi.fn().mockResolvedValue(""),
    transcribeLesson: vi.fn(),
    listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
    recordPlayback: vi.fn().mockResolvedValue([]),
    listLessonCardLinks: vi.fn().mockResolvedValue([]),
    unlinkLessonCard: vi.fn().mockResolvedValue(undefined),
    listLessonNotes: vi.fn().mockResolvedValue([]),
    createLessonNote: vi.fn(),
    updateLessonNote: vi.fn(),
    deleteLessonNote: vi.fn(),
    exportLessonNotesMarkdown: vi.fn().mockResolvedValue(undefined),
    saveDocumentPosition: vi.fn().mockResolvedValue(undefined),
    ...overrides,
  };
}

describe("LessonNotesColumn", () => {
  it("add note calls command layer and lists the saved note", async () => {
    const user = userEvent.setup();
    const saved: LessonNote = {
      note_id: "note-1",
      lesson_id: "lesson-1",
      text: "Key definition",
      timestamp_ms: null,
      created_at: "1",
      updated_at: "1",
    };
    const createLessonNote = vi.fn().mockResolvedValue(saved);
    const listLessonNotes = vi
      .fn()
      .mockResolvedValueOnce([])
      .mockResolvedValueOnce([saved]);

    render(
      <CourseCommandsProvider
        commands={noteCommands({ listLessonNotes, createLessonNote })}
      >
        <LessonNotesColumn lessonId="lesson-1" lessonTitle="Intro clip" />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("region", { name: "Lesson notes" })
      ).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Add note" }));
    await user.type(
      screen.getByRole("textbox", { name: "Note text" }),
      "Key definition"
    );
    await user.click(screen.getByRole("button", { name: "Save note" }));

    await waitFor(() => {
      expect(createLessonNote).toHaveBeenCalledWith(
        "lesson-1",
        "Key definition",
        null
      );
      expect(screen.getByText("Key definition")).toBeInTheDocument();
    });
    expect(
      screen.getByRole("button", { name: "Make card" })
    ).toBeInTheDocument();
  });

  it("clicking a timestamp seeks playback", async () => {
    const user = userEvent.setup();
    const onSeek = vi.fn();
    const timestamped: LessonNote = {
      note_id: "note-ts",
      lesson_id: "lesson-1",
      text: "Important moment",
      timestamp_ms: 42_000,
      created_at: "1",
      updated_at: "1",
    };

    render(
      <CourseCommandsProvider
        commands={noteCommands({
          listLessonNotes: vi.fn().mockResolvedValue([timestamped]),
        })}
      >
        <LessonNotesColumn
          lessonId="lesson-1"
          lessonTitle="Intro clip"
          playbackContext={{ currentMs: 0, onSeek }}
        />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByText("Important moment")).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Seek to 0:42" }));
    expect(onSeek).toHaveBeenCalledWith(42_000);
  });

  it("export notes calls the command layer with lesson id and title", async () => {
    const user = userEvent.setup();
    const exportLessonNotesMarkdown = vi.fn().mockResolvedValue(undefined);

    render(
      <CourseCommandsProvider
        commands={noteCommands({ exportLessonNotesMarkdown })}
      >
        <LessonNotesColumn lessonId="lesson-1" lessonTitle="Eigenvalues" />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Export notes" })
      ).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Export notes" }));

    expect(exportLessonNotesMarkdown).toHaveBeenCalledWith(
      "lesson-1",
      "Eigenvalues"
    );
  });

  it("saves an edited note when the field blurs", async () => {
    const user = userEvent.setup();
    const existing: LessonNote = {
      note_id: "note-1",
      lesson_id: "lesson-1",
      text: "Before",
      timestamp_ms: null,
      created_at: "1",
      updated_at: "1",
    };
    const updated: LessonNote = {
      ...existing,
      text: "After blur",
      updated_at: "2",
    };
    const updateLessonNote = vi.fn().mockResolvedValue(updated);
    const listLessonNotes = vi
      .fn()
      .mockResolvedValueOnce([existing])
      .mockResolvedValueOnce([updated]);

    render(
      <CourseCommandsProvider
        commands={noteCommands({ listLessonNotes, updateLessonNote })}
      >
        <LessonNotesColumn lessonId="lesson-1" lessonTitle="Intro clip" />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByText("Before")).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Edit" }));
    const field = screen.getByRole("textbox", { name: "Note text" });
    await user.clear(field);
    await user.type(field, "After blur");
    fireEvent.blur(field);

    await waitFor(() => {
      expect(updateLessonNote).toHaveBeenCalledWith(
        "note-1",
        "After blur",
        null
      );
      expect(screen.getByText("After blur")).toBeInTheDocument();
    });
  });

  it("reload keeps saved note text after blur save", async () => {
    const user = userEvent.setup();
    const existing: LessonNote = {
      note_id: "note-1",
      lesson_id: "lesson-1",
      text: "Before",
      timestamp_ms: null,
      created_at: "1",
      updated_at: "1",
    };
    const updated: LessonNote = {
      ...existing,
      text: "Persists after reload",
      updated_at: "2",
    };
    const updateLessonNote = vi.fn().mockResolvedValue(updated);
    let notesOnDisk: LessonNote[] = [existing];
    const listLessonNotes = vi.fn().mockImplementation(async () => notesOnDisk);

    const view = ({ key }: { key: string }) => (
      <CourseCommandsProvider
        commands={noteCommands({ listLessonNotes, updateLessonNote })}
      >
        <LessonNotesColumn
          key={key}
          lessonId="lesson-1"
          lessonTitle="Intro clip"
        />
      </CourseCommandsProvider>
    );

    const { unmount } = render(view({ key: "first" }));

    await waitFor(() => {
      expect(screen.getByText("Before")).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Edit" }));
    const field = screen.getByRole("textbox", { name: "Note text" });
    await user.clear(field);
    await user.type(field, "Persists after reload");
    notesOnDisk = [updated];
    fireEvent.blur(field);

    await waitFor(() => {
      expect(updateLessonNote).toHaveBeenCalled();
    });

    unmount();
    render(view({ key: "reload" }));

    await waitFor(() => {
      expect(screen.getByText("Persists after reload")).toBeInTheDocument();
    });
    expect(screen.queryByText("Before")).not.toBeInTheDocument();
  });

  it("cancel does not save when the field blurs", async () => {
    const user = userEvent.setup();
    const existing: LessonNote = {
      note_id: "note-1",
      lesson_id: "lesson-1",
      text: "Keep me",
      timestamp_ms: null,
      created_at: "1",
      updated_at: "1",
    };
    const updateLessonNote = vi.fn();

    render(
      <CourseCommandsProvider
        commands={noteCommands({
          listLessonNotes: vi.fn().mockResolvedValue([existing]),
          updateLessonNote,
        })}
      >
        <LessonNotesColumn lessonId="lesson-1" lessonTitle="Intro clip" />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByText("Keep me")).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Edit" }));
    await user.type(
      screen.getByRole("textbox", { name: "Note text" }),
      " changed"
    );
    await user.click(screen.getByRole("button", { name: "Cancel" }));

    expect(updateLessonNote).not.toHaveBeenCalled();
    expect(screen.getByText("Keep me")).toBeInTheDocument();
  });
});
