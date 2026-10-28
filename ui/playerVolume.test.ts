import { describe, expect, it } from "vitest";

import {
  DEFAULT_PLAYER_VOLUME,
  normalizePlayerVolume,
  playerVolumePercent,
} from "./src/player/playerVolume";

describe("playerVolume", () => {
  it("defaults new profiles to full volume", () => {
    expect(DEFAULT_PLAYER_VOLUME).toBe(1);
    expect(normalizePlayerVolume(Number.NaN)).toBe(1);
  });

  it("clamps volume to zero through one", () => {
    expect(normalizePlayerVolume(1.5)).toBe(1);
    expect(playerVolumePercent(0.4)).toBe(40);
  });
});
