import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { LibraryCommandPort } from "./src/api/libraryCommands";
import { emptyStudyHeatmap52Weeks } from "./src/api/libraryStudyHeatmapTypes";
import type { CourseRow } from "./src/api/libraryTypes";
import { LibraryCommandsProvider } from "./src/library/LibraryCommandsContext";
import { LibraryScreen } from "./src/screens/LibraryScreen";

const sampleCourse: CourseRow = {
  course_id: "course-1",
  title: "Linear Algebra",
  source_kind: "folder",
  status: "not_started",
  remaining_ms: 0,
  unknown_duration_count: 2,
  is_archived: false,
  thumbnail_path: null,
};

function libraryCommands(
  overrides: Partial<LibraryCommandPort> = {}
): LibraryCommandPort {
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
    mediaUrlForPath: vi
      .fn()
      .mockImplementation((path: string) => Promise.resolve(`asset://${path}`)),
    pathIsDirectory: vi.fn().mockResolvedValue(true),
    ...overrides,
  };
}

function renderLibrary(commands: LibraryCommandPort) {
  const onOpenCourse = vi.fn();
  const onOpenLesson = vi.fn();
  render(
    <LibraryCommandsProvider commands={commands}>
      <LibraryScreen onOpenCourse={onOpenCourse} onOpenLesson={onOpenLesson} />
    </LibraryCommandsProvider>
  );
  return { onOpenCourse, onOpenLesson };
}

describe("LibraryScreen", () => {
  it("shows study heatmap summary text", async () => {
    renderLibrary(
      libraryCommands({
        listCourses: vi.fn().mockResolvedValue([sampleCourse]),
        getLibraryStudyHeatmap52Weeks: vi.fn().mockResolvedValue({
          weeks: [],
          counted_days: 5,
          total_days: 364,
        }),
      })
    );

    await waitFor(() => {
      expect(screen.getByText("5 of 364 days with study")).toBeInTheDocument();
    });
  });

  it("shows study streak as a number and the word days", async () => {
    renderLibrary(
      libraryCommands({
        listCourses: vi.fn().mockResolvedValue([sampleCourse]),
        getLibraryStudyStreakDays: vi.fn().mockResolvedValue(12),
      })
    );

    await waitFor(() => {
      expect(screen.getByText("12 days")).toBeInTheDocument();
    });
  });

  it("lists courses from mock commands", async () => {
    const listCourses = vi.fn().mockResolvedValue([sampleCourse]);
    renderLibrary(libraryCommands({ listCourses }));

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Linear Algebra" })
      ).toBeInTheDocument();
    });
    expect(screen.getByText("Not started")).toBeInTheDocument();
    expect(screen.getByText("2")).toBeInTheDocument();
    expect(listCourses).toHaveBeenCalledWith("active");
  });

  it("shows stored thumbnail on course row when present", async () => {
    const mediaUrlForPath = vi.fn().mockResolvedValue("asset://thumb");
    renderLibrary(
      libraryCommands({
        listCourses: vi.fn().mockResolvedValue([
          {
            ...sampleCourse,
            thumbnail_path: "/app/thumbnails/lesson-1.jpg",
          },
        ]),
        mediaUrlForPath,
      })
    );

    await waitFor(() => {
      expect(screen.getByTestId("lesson-thumbnail")).toHaveAttribute(
        "src",
        "asset://thumb"
      );
    });
    expect(mediaUrlForPath).toHaveBeenCalledWith(
      "/app/thumbnails/lesson-1.jpg"
    );
  });

  it("course row without thumbnail stays text only", async () => {
    renderLibrary(
      libraryCommands({
        listCourses: vi.fn().mockResolvedValue([sampleCourse]),
      })
    );

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Linear Algebra" })
      ).toBeInTheDocument();
    });
    expect(screen.queryByTestId("lesson-thumbnail")).not.toBeInTheDocument();
    expect(screen.queryByRole("img")).not.toBeInTheDocument();
  });

  it("import folder triggers command after picker returns path", async () => {
    const user = userEvent.setup();
    const commands = libraryCommands({
      listCourses: vi
        .fn()
        .mockResolvedValueOnce([])
        .mockResolvedValueOnce([sampleCourse]),
      importFolder: vi.fn().mockResolvedValue(undefined),
      pickFolder: vi.fn().mockResolvedValue("/tmp/course"),
    });

    renderLibrary(commands);

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Add folder" })
      ).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Add folder" }));

    await waitFor(() => {
      expect(commands.pickFolder).toHaveBeenCalled();
      expect(commands.importFolder).toHaveBeenCalledWith("/tmp/course");
    });

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Linear Algebra" })
      ).toBeInTheDocument();
    });
  });

  it("shows one sentence when import rejects a risky folder path", async () => {
    const user = userEvent.setup();
    const listCourses = vi
      .fn()
      .mockResolvedValueOnce([])
      .mockResolvedValueOnce([sampleCourse]);
    const validateImportFolderPath = vi.fn().mockRejectedValue(
      new Error(
        JSON.stringify({
          code: "risky_import_path",
          message: "risky_import_path",
        })
      )
    );
    const importFolder = vi.fn();
    renderLibrary(
      libraryCommands({
        listCourses,
        validateImportFolderPath,
        importFolder,
        pickFolder: vi.fn().mockResolvedValue("/"),
      })
    );

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Add folder" })
      ).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Add folder" }));

    await waitFor(() => {
      expect(validateImportFolderPath).toHaveBeenCalledWith("/");
      expect(importFolder).not.toHaveBeenCalled();
      expect(screen.getByRole("alert")).toHaveTextContent(
        "That folder is too broad; pick the course directory itself."
      );
    });

    expect(listCourses).toHaveBeenCalledTimes(1);
  });

  it("does not show Cancel import while folder import is idle", async () => {
    renderLibrary(libraryCommands());

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Add folder" })
      ).toBeInTheDocument();
    });

    expect(
      screen.queryByRole("button", { name: "Cancel import" })
    ).not.toBeInTheDocument();
  });

  it("cancel folder scan calls command layer while import is in flight", async () => {
    const user = userEvent.setup();
    let finishImport: (() => void) | undefined;
    const importFolder = vi.fn(
      () =>
        new Promise<void>((resolve) => {
          finishImport = resolve;
        })
    );
    const cancelImportFolder = vi.fn().mockResolvedValue(undefined);
    renderLibrary(
      libraryCommands({
        importFolder,
        cancelImportFolder,
        pickFolder: vi.fn().mockResolvedValue("/tmp/course"),
      })
    );

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Add folder" })
      ).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Add folder" }));

    await waitFor(() => {
      expect(screen.getByRole("status")).toHaveTextContent("Scanning folder");
      expect(importFolder).toHaveBeenCalledWith("/tmp/course");
    });

    await user.click(screen.getByRole("button", { name: "Cancel import" }));

    expect(cancelImportFolder).toHaveBeenCalledTimes(1);

    finishImport?.();
  });

  it("Escape during folder import does not call cancelImportFolder", async () => {
    const user = userEvent.setup();
    let finishImport: (() => void) | undefined;
    const importFolder = vi.fn(
      () =>
        new Promise<void>((resolve) => {
          finishImport = resolve;
        })
    );
    const cancelImportFolder = vi.fn().mockResolvedValue(undefined);
    renderLibrary(
      libraryCommands({
        importFolder,
        cancelImportFolder,
        pickFolder: vi.fn().mockResolvedValue("/tmp/course"),
      })
    );

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Add folder" })
      ).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Add folder" }));

    await waitFor(() => {
      expect(screen.getByRole("status")).toHaveTextContent("Scanning folder");
    });

    await user.keyboard("{Escape}");

    expect(cancelImportFolder).not.toHaveBeenCalled();
    expect(
      screen.getByRole("button", { name: "Cancel import" })
    ).toBeInTheDocument();

    finishImport?.();
  });
});
