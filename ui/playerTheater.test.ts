import { describe, expect, it } from "vitest";

import { playerTheaterBarClassName } from "./src/player/playerTheater";

describe("playerTheaterBarClassName", () => {
  it("hides the theater bar until controls are revealed", () => {
    expect(playerTheaterBarClassName(true, false)).toBe(
      "player-bar player-bar--theater player-bar--theater-hidden"
    );
    expect(playerTheaterBarClassName(true, true)).toBe(
      "player-bar player-bar--theater"
    );
  });
});
