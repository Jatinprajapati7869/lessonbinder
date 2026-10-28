import { describe, expect, it } from "vitest";

import {
  playerMainClassName,
  playerScreenClassName,
} from "./src/player/playerZen";

describe("player zen class names", () => {
  it("prefers zen over theater when both would be active", () => {
    expect(playerScreenClassName(true, true)).toBe(
      "player-screen player-screen--zen"
    );
    expect(playerMainClassName(true, true)).toBe(
      "player-main player-main--zen"
    );
  });
});
