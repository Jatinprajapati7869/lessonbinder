import { describe, expect, it } from "vitest";

import {
  charOffsetFromScroll,
  scrollToCharOffset,
} from "./src/lesson/documentPosition";

describe("documentPosition", () => {
  it("scrollToCharOffset restores a saved character offset", () => {
    const pre = document.createElement("pre");
    pre.textContent = "A".repeat(1_000);
    Object.defineProperty(pre, "clientHeight", {
      value: 100,
      configurable: true,
    });
    Object.defineProperty(pre, "scrollHeight", {
      value: 500,
      configurable: true,
    });

    scrollToCharOffset(pre, 500, 1_000);
    expect(pre.scrollTop).toBe(200);

    const roundTrip = charOffsetFromScroll(pre, 1_000);
    expect(roundTrip).toBe(500);
  });
});
