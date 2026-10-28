import { describe, expect, it } from "vitest";

import {
  assertImplementedShortcutsMatchKeyModules,
  buildImplementedPlayerAndReviewShortcuts,
  formatShortcutKeysForSettings,
  IMPLEMENTED_PLAYER_AND_REVIEW_SHORTCUTS,
} from "./src/keyboard/implementedShortcuts";
import { PLAYER_MARK_COMPLETE_KEY } from "./src/player/playerCompleteKeyboard";
import { PLAYER_EXIT_THEATER_OR_ZEN_KEY } from "./src/player/playerModeKeyboard";
import {
  PLAYER_SEEK_STEP_MS,
  playerArrowSeekDeltaMs,
} from "./src/player/playerSeekKeyboard";
import { REVIEW_GRADE_DIGIT_KEYS } from "./src/review/reviewKeyboard";

describe("IMPLEMENTED_PLAYER_AND_REVIEW_SHORTCUTS", () => {
  it("matches key modules (fails when the list drifts)", () => {
    expect(() =>
      assertImplementedShortcutsMatchKeyModules(
        IMPLEMENTED_PLAYER_AND_REVIEW_SHORTCUTS
      )
    ).not.toThrow();
    expect(() =>
      assertImplementedShortcutsMatchKeyModules([
        {
          scope: "player",
          keys: ["ArrowLeft"],
          action: "Mark complete",
        },
      ])
    ).toThrow();
  });

  it("exported list equals buildImplementedPlayerAndReviewShortcuts()", () => {
    expect(IMPLEMENTED_PLAYER_AND_REVIEW_SHORTCUTS).toEqual(
      buildImplementedPlayerAndReviewShortcuts()
    );
  });

  it("lists arrow seek keys from playerSeekKeyboard", () => {
    const seekRow = IMPLEMENTED_PLAYER_AND_REVIEW_SHORTCUTS.find(
      (row) => row.scope === "player" && row.action.startsWith("Seek")
    );
    expect(seekRow).toBeDefined();
    expect(
      seekRow!.keys.every((key) => playerArrowSeekDeltaMs(key) !== null)
    ).toBe(true);
    expect(seekRow!.keys).toContain("ArrowLeft");
    expect(seekRow!.keys).toContain("ArrowRight");
    expect(seekRow!.action).toBe(`Seek ${PLAYER_SEEK_STEP_MS / 1000} seconds`);
  });

  it("lists mark complete key from playerCompleteKeyboard", () => {
    const row = IMPLEMENTED_PLAYER_AND_REVIEW_SHORTCUTS.find(
      (row) => row.action === "Mark complete"
    );
    expect(row?.keys).toEqual([PLAYER_MARK_COMPLETE_KEY]);
  });

  it("lists theater and zen exit key from playerModeKeyboard", () => {
    const row = IMPLEMENTED_PLAYER_AND_REVIEW_SHORTCUTS.find(
      (row) => row.action === "Exit theater or zen"
    );
    expect(row?.keys).toEqual([PLAYER_EXIT_THEATER_OR_ZEN_KEY]);
  });

  it("lists review grade digits from reviewKeyboard", () => {
    const gradeRows = IMPLEMENTED_PLAYER_AND_REVIEW_SHORTCUTS.filter(
      (row) => row.scope === "review"
    );
    expect(gradeRows.map((row) => row.keys[0]).sort()).toEqual(
      [...REVIEW_GRADE_DIGIT_KEYS].sort()
    );
    expect(gradeRows).toHaveLength(REVIEW_GRADE_DIGIT_KEYS.length);
  });
});

describe("formatShortcutKeysForSettings", () => {
  it("formats arrow and letter keys for display", () => {
    expect(formatShortcutKeysForSettings(["ArrowLeft", "ArrowRight"])).toBe(
      "← / →"
    );
    expect(formatShortcutKeysForSettings(["d"])).toBe("D");
  });
});
