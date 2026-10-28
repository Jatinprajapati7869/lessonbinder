import { describe, expect, it } from "vitest";

import {
  contrastRatio,
  DESIGN_BODY_CONTRAST_MIN,
  DESIGN_BODY_TEXT_PAIRS,
} from "./src/theme/contrast";

describe("DESIGN.md body text contrast", () => {
  it("meets 4.5:1 for documented foreground/background pairs", () => {
    for (const pair of DESIGN_BODY_TEXT_PAIRS) {
      const ratio = contrastRatio(pair.foreground, pair.background);
      expect(ratio, pair.label).toBeGreaterThanOrEqual(
        DESIGN_BODY_CONTRAST_MIN
      );
    }
  });

  it("documents that muted on app background is below the body threshold", () => {
    const ratio = contrastRatio("#78716C", "#F4F4F3");
    expect(ratio).toBeLessThan(DESIGN_BODY_CONTRAST_MIN);
  });
});
