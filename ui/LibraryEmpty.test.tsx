import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { LibraryCommandPort } from "./src/api/libraryCommands";
import { LIBRARY_FIRST_RUN_COPY } from "./src/library/LibraryFirstRunEmpty";
import { LibraryCommandsProvider } from "./src/library/LibraryCommandsContext";
import { LibraryScreen } from "./src/screens/LibraryScreen";

function baseCommands(
  overrides: Partial<LibraryCommandPort> = {}
): LibraryCommandPort {
  return {
    listCourses: vi.fn().mockResolvedValue([]),
    importFolder: vi.fn(),
    cancelImportFolder: vi.fn(),
    importPlaylist: vi.fn(),
    createPlaylistFromPaste: vi.fn(),
    pickFolder: vi.fn(),
    pickPlaylistJson: vi.fn(),
    searchLibrary: vi.fn().mockResolvedValue([]),
    getLibraryContinueRow: vi.fn().mockResolvedValue(null),
    mediaUrlForPath: vi.fn().mockResolvedValue("asset://file"),
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
  it("shows one sentence and add folder / playlist actions", async () => {
    renderEmptyLibrary(baseCommands());

    await waitFor(() => {
      expect(screen.getByText(LIBRARY_FIRST_RUN_COPY)).toBeInTheDocument();
    });

    expect(screen.getAllByRole("paragraph")).toHaveLength(1);
    expect(
      screen.getByRole("button", { name: "Add folder" })
    ).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: "Add playlist JSON" })
    ).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: "Paste playlist" })
    ).toBeInTheDocument();

    expect(screen.queryByLabelText("Search library")).not.toBeInTheDocument();
    expect(screen.queryByText(/welcome/i)).not.toBeInTheDocument();
    expect(screen.queryByText(/journey/i)).not.toBeInTheDocument();
    expect(screen.queryByRole("img")).not.toBeInTheDocument();
  });

  it("add playlist JSON opens picker and imports", async () => {
    const user = userEvent.setup();
    const commands = baseCommands({
      pickPlaylistJson: vi.fn().mockResolvedValue("/tmp/list.json"),
      importPlaylist: vi.fn().mockResolvedValue(undefined),
    });

    renderEmptyLibrary(commands);

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Add playlist JSON" })
      ).toBeEnabled();
    });

    await user.click(screen.getByRole("button", { name: "Add playlist JSON" }));

    await waitFor(() => {
      expect(commands.pickPlaylistJson).toHaveBeenCalled();
      expect(commands.importPlaylist).toHaveBeenCalledWith("/tmp/list.json");
    });
  });
});
