import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { emptyStudyHeatmap52Weeks } from "./src/api/libraryStudyHeatmapTypes";
import { LessonbinderApp } from "./src/App";
import { defaultVaultSettings } from "./src/test/settingsPlaybackStubs";

const PROFILE_IN_USE =
  "Lessonbinder Anki profile is in use (close Anki or wait for another operation to finish)";

vi.mock("@tauri-apps/api/core", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@tauri-apps/api/core")>();
  return {
    ...actual,
    invoke: vi.fn(async (cmd: string) => {
      switch (cmd) {
        case "list_library_courses":
          return [];
        case "get_library_continue_row":
          return null;
        case "get_library_study_streak_days":
          return 0;
        case "get_library_study_heatmap_52_weeks":
          return emptyStudyHeatmap52Weeks();
        case "get_vault_settings":
          return defaultVaultSettings;
        default:
          throw new Error(`unexpected invoke ${cmd}`);
      }
    }),
  };
});

describe("AnkiProfileInUse", () => {
  it("shows the profile lock sentence when study bar state fails", async () => {
    render(
      <LessonbinderApp
        fetchVaultBootstrapImpl={async () => ({
          ready: true,
          path: "/tmp/isolated/dev.lessonbinder.app/vault.db",
          error: null,
        })}
        fetchStudyBarStateImpl={async () => {
          throw new Error(PROFILE_IN_USE);
        }}
      />
    );

    await waitFor(() => {
      expect(screen.getByText(PROFILE_IN_USE)).toBeInTheDocument();
    });
    expect(screen.getByRole("button", { name: "Library" })).toBeInTheDocument();
  });
});
