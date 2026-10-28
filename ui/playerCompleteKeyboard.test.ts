import { describe, expect, it } from "vitest";

import {
  playerMarkCompleteAccessibleName,
  PLAYER_MARK_COMPLETE_KEY,
  shouldHandlePlayerMarkComplete,
} from "./src/player/playerCompleteKeyboard";

describe("playerCompleteKeyboard", () => {
  it("labels the control with the shortcut letter", () => {
    expect(PLAYER_MARK_COMPLETE_KEY).toBe("d");
    expect(playerMarkCompleteAccessibleName()).toBe("Mark complete (D)");
  });

  it("ignores mark complete in text fields but allows the scrubber range", () => {
    const textarea = document.createElement("textarea");
    const blocked = new KeyboardEvent("keydown", { key: "d", bubbles: true });
    Object.defineProperty(blocked, "target", { value: textarea });
    expect(shouldHandlePlayerMarkComplete(blocked, false)).toBe(false);

    const scrubber = document.createElement("input");
    scrubber.type = "range";
    scrubber.className = "player-scrubber__input";
    const allowed = new KeyboardEvent("keydown", { key: "d", bubbles: true });
    Object.defineProperty(allowed, "target", { value: scrubber });
    expect(shouldHandlePlayerMarkComplete(allowed, false)).toBe(true);
  });
});
