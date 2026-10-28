import { describe, expect, it } from "vitest";

import {
  mapPlayTickToRecordPlayback,
  watchTickAnchorAfterSeek,
} from "./src/player/playbackWatch";

describe("mapPlayTickToRecordPlayback", () => {
  it("anchors the first tick without recording", () => {
    expect(mapPlayTickToRecordPlayback(null, 5_000)).toEqual({
      nextLastTickMs: 5_000,
      recordPlayback: null,
    });
  });

  it("records the span since the previous tick while playing forward", () => {
    expect(mapPlayTickToRecordPlayback(5_000, 6_200)).toEqual({
      nextLastTickMs: 6_200,
      recordPlayback: { startMs: 5_000, endMs: 6_200 },
    });
  });

  it("does not record when time moves backward after a seek", () => {
    let last = watchTickAnchorAfterSeek(30_000);
    expect(mapPlayTickToRecordPlayback(last, 31_000)).toEqual({
      nextLastTickMs: 31_000,
      recordPlayback: { startMs: 30_000, endMs: 31_000 },
    });

    last = watchTickAnchorAfterSeek(60_000);
    expect(mapPlayTickToRecordPlayback(last, 61_500)).toEqual({
      nextLastTickMs: 61_500,
      recordPlayback: { startMs: 60_000, endMs: 61_500 },
    });
    expect(mapPlayTickToRecordPlayback(61_500, 40_000)).toEqual({
      nextLastTickMs: 40_000,
      recordPlayback: null,
    });
  });
});
