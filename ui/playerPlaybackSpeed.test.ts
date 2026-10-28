import { describe, expect, it } from "vitest";

import {
  formatPlaybackSpeedLabel,
  normalizePlaybackSpeed,
  PLAYBACK_SPEED_CHOICES,
} from "./src/player/playerPlaybackSpeed";

describe("playerPlaybackSpeed", () => {
  it("offers the fixed speed choices with default 1", () => {
    expect(PLAYBACK_SPEED_CHOICES).toEqual([0.75, 1, 1.25, 1.5, 2]);
    expect(normalizePlaybackSpeed(999)).toBe(1);
  });

  it("formats labels for the speed select", () => {
    expect(formatPlaybackSpeedLabel(1)).toBe("1×");
    expect(formatPlaybackSpeedLabel(1.25)).toBe("1.25×");
  });
});
