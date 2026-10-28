import { describe, expect, it } from "vitest";

import {
  clampSeekMs,
  PLAYER_SEEK_STEP_MS,
  playerArrowSeekDeltaMs,
  shouldHandlePlayerArrowSeek,
} from "./src/player/playerSeekKeyboard";

describe("playerSeekKeyboard", () => {
  it("steps five seconds per arrow key", () => {
    expect(PLAYER_SEEK_STEP_MS).toBe(5000);
    expect(playerArrowSeekDeltaMs("ArrowLeft")).toBe(-5000);
    expect(playerArrowSeekDeltaMs("ArrowRight")).toBe(5000);
  });

  it("ignores arrow seek in text fields but allows the scrubber range", () => {
    const textarea = document.createElement("textarea");
    const blocked = new KeyboardEvent("keydown", {
      key: "ArrowRight",
      bubbles: true,
    });
    Object.defineProperty(blocked, "target", { value: textarea });
    expect(shouldHandlePlayerArrowSeek(blocked, false)).toBe(false);

    const scrubber = document.createElement("input");
    scrubber.type = "range";
    scrubber.className = "player-scrubber__input";
    const allowed = new KeyboardEvent("keydown", {
      key: "ArrowLeft",
      bubbles: true,
    });
    Object.defineProperty(allowed, "target", { value: scrubber });
    expect(shouldHandlePlayerArrowSeek(allowed, false)).toBe(true);
  });

  it("clamps seek inside the media duration", () => {
    expect(clampSeekMs(30_000, 120_000)).toBe(30_000);
    expect(clampSeekMs(-1_000, 120_000)).toBe(0);
    expect(clampSeekMs(130_000, 120_000)).toBe(120_000);
  });
});
