import React, { useCallback, useEffect, useState } from "react";
import { act, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { LibraryCommandPort } from "./src/api/libraryCommands";
import type { CourseRow } from "./src/api/libraryTypes";
import {
  LIBRARY_FIRST_RUN_COPY,
  LibraryFirstRunEmpty,
} from "./src/library/LibraryFirstRunEmpty";
import { emptyStudyHeatmap52Weeks } from "./src/api/libraryStudyHeatmapTypes";
import { LibraryCommandsProvider } from "./src/library/LibraryCommandsContext";
import { libraryFolderDropIntoLibrary } from "./src/library/libraryFolderDrop";
import { LibraryScreen } from "./src/screens/LibraryScreen";

function baseCommands(
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
    mediaUrlForPath: vi.fn().mockResolvedValue("asset://file"),
    pathIsDirectory: vi.fn().mockResolvedValue(true),
    ...overrides,
  };
}

function renderEmptyLibrary(commands: LibraryCommandPort) {
  render(
    <LibraryCommandsProvider commands={commands}>
      <LibraryScreen onOpenCourse={vi.fn()} onOpenLesson={vi.fn()} />
    </LibraryCommandsProvider>
  );
}

describe("Library empty state", () => {
  it("shows one sentence and Add folder", async () => {
    renderEmptyLibrary(baseCommands());

    await waitFor(() => {
      expect(screen.getByText(LIBRARY_FIRST_RUN_COPY)).toBeInTheDocument();
    });

    expect(screen.getByText("0 days")).toBeInTheDocument();
    expect(screen.getAllByRole("paragraph")).toHaveLength(3);
    expect(
      screen.getByRole("button", { name: "Add folder" })
    ).toBeInTheDocument();
    expect(
      screen.queryByRole("button", { name: "Add playlist JSON" })
    ).not.toBeInTheDocument();
    expect(
      screen.queryByRole("button", { name: "Paste playlist" })
    ).not.toBeInTheDocument();

    expect(screen.queryByLabelText("Search library")).not.toBeInTheDocument();
    expect(screen.queryByText(/welcome/i)).not.toBeInTheDocument();
    expect(screen.queryByText(/journey/i)).not.toBeInTheDocument();
    expect(screen.queryByRole("img")).not.toBeInTheDocument();
  });

  it("folder drop on empty library imports and leaves the empty state (stub path; Vitest has no Tauri drag event)", async () => {
    const sampleCourse: CourseRow = {
      course_id: "course-1",
      title: "Dropped Course",
      source_kind: "folder",
      status: "not_started",
      remaining_ms: 0,
      unknown_duration_count: 0,
      is_archived: false,
      thumbnail_path: null,
    };
    let courses: CourseRow[] = [];
    const listCourses = vi.fn(async () => courses);
    const importFolder = vi.fn(async () => {
      courses = [sampleCourse];
    });

    function EmptyLibraryDropHarness({
      onReady,
    }: {
      onReady: (drop: (paths: readonly string[]) => Promise<void>) => void;
    }) {
      const [rows, setRows] = useState<CourseRow[]>([]);
      const [error, setError] = useState<string | null>(null);
      const [busy, setBusy] = useState(false);
      const commands = baseCommands({ listCourses, importFolder });

      const refresh = useCallback(async () => {
        setRows(await commands.listCourses());
      }, [commands]);

      useEffect(() => {
        void refresh();
      }, [refresh]);

      const importFolderAtPath = useCallback(
        async (path: string) => {
          setBusy(true);
          try {
            await commands.importFolder(path);
            await refresh();
          } finally {
            setBusy(false);
          }
        },
        [commands, refresh]
      );

      const dropHandler = useCallback(
        async (paths: readonly string[]) => {
          const dropError = await libraryFolderDropIntoLibrary(paths, {
            pathIsDirectory: (path) => commands.pathIsDirectory(path),
            importFolderAtPath,
          });
          if (dropError !== null) {
            setError(dropError);
          }
        },
        [commands, importFolderAtPath]
      );

      useEffect(() => {
        onReady(dropHandler);
      }, [dropHandler, onReady]);

      if (rows.length === 0) {
        return (
          <LibraryFirstRunEmpty
            busy={busy}
            error={error}
            onAddFolder={() => {}}
          />
        );
      }
      return <p role="status">{rows[0]!.title}</p>;
    }

    let dropPaths: ((paths: readonly string[]) => Promise<void>) | null = null;
    const onDropReady = (drop: (paths: readonly string[]) => Promise<void>) => {
      dropPaths = drop;
    };

    render(<EmptyLibraryDropHarness onReady={onDropReady} />);

    await waitFor(() => {
      expect(screen.getByText(LIBRARY_FIRST_RUN_COPY)).toBeInTheDocument();
    });

    await act(async () => {
      await dropPaths!(["/tmp/dropped-course"]);
    });

    await waitFor(() => {
      expect(
        screen.queryByText(LIBRARY_FIRST_RUN_COPY)
      ).not.toBeInTheDocument();
      expect(screen.getByRole("status")).toHaveTextContent("Dropped Course");
    });
    expect(importFolder).toHaveBeenCalledWith("/tmp/dropped-course");
  });
});

describe("Library with courses", () => {
  it("add playlist JSON opens picker and imports from the toolbar", async () => {
    const user = userEvent.setup();
    const commands = baseCommands({
      listCourses: vi.fn().mockResolvedValue([
        {
          course_id: "c1",
          title: "Sample",
          source_kind: "folder",
          status: "not_started",
          remaining_ms: 0,
          unknown_duration_count: 0,
          is_archived: false,
          thumbnail_path: null,
        },
      ]),
      pickPlaylistJson: vi.fn().mockResolvedValue("/tmp/list.json"),
      importPlaylist: vi.fn().mockResolvedValue(undefined),
    });

    render(
      <LibraryCommandsProvider commands={commands}>
        <LibraryScreen onOpenCourse={vi.fn()} onOpenLesson={vi.fn()} />
      </LibraryCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Add playlist" })
      ).toBeEnabled();
    });

    await user.click(screen.getByRole("button", { name: "Add playlist" }));

    await waitFor(() => {
      expect(commands.pickPlaylistJson).toHaveBeenCalled();
      expect(commands.importPlaylist).toHaveBeenCalledWith("/tmp/list.json");
    });
  });
});
