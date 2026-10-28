import { describe, expect, it } from "vitest";

import { playerFrameBarClassName } from "./src/player/playerTheater";

describe("playerFrameBarClassName", () => {
  it("hides the frame overlay bar until controls are revealed", () => {
    expect(playerFrameBarClassName(false)).toBe(
      "player-bar player-bar--overlay player-bar--overlay-hidden"
    );
    expect(playerFrameBarClassName(true)).toBe(
      "player-bar player-bar--overlay"
    );
  });
});
