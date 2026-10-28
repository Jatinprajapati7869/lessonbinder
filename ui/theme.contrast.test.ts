import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

import {
  contrastRatio,
  DESIGN_ACCENT_HEX,
  DESIGN_BODY_CONTRAST_MIN,
  DESIGN_BODY_TEXT_PAIRS,
  DESIGN_DARK_BODY_TEXT_PAIRS,
  DESIGN_DARK_PLAYER_CONTROL_PAIRS,
} from "./src/theme/contrast";

const here = dirname(fileURLToPath(import.meta.url));
const tokensCss = readFileSync(join(here, "src/theme/tokens.css"), "utf8");

function accentFromBlock(css: string): string | undefined {
  const match = css.match(/--accent:\s*(#[0-9a-f]{6})/i);
  return match?.[1]?.toLowerCase();
}

describe("DESIGN.md body text contrast", () => {
  it("meets 4.5:1 for documented foreground/background pairs", () => {
    for (const pair of DESIGN_BODY_TEXT_PAIRS) {
      const ratio = contrastRatio(pair.foreground, pair.background);
      expect(ratio, pair.label).toBeGreaterThanOrEqual(
        DESIGN_BODY_CONTRAST_MIN
      );
    }
  });

  it("meets 4.5:1 for dark theme body pairs on stone surfaces", () => {
    for (const pair of DESIGN_DARK_BODY_TEXT_PAIRS) {
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

describe("dark theme player", () => {
  it("keeps progress fill on the accent token in dark theme", () => {
    const lightBlock = tokensCss.slice(
      0,
      tokensCss.indexOf(':root[data-theme="dark"]')
    );
    const darkBlock = tokensCss.slice(
      tokensCss.indexOf(':root[data-theme="dark"]')
    );
    expect(accentFromBlock(lightBlock)).toBe(DESIGN_ACCENT_HEX.toLowerCase());
    expect(accentFromBlock(darkBlock)).toBe(DESIGN_ACCENT_HEX.toLowerCase());
  });

  it("meets 4.5:1 for player control text on dark elevated surfaces", () => {
    expect(DESIGN_DARK_PLAYER_CONTROL_PAIRS.length).toBeGreaterThan(0);
    for (const pair of DESIGN_DARK_PLAYER_CONTROL_PAIRS) {
      const ratio = contrastRatio(pair.foreground, pair.background);
      expect(ratio, pair.label).toBeGreaterThanOrEqual(
        DESIGN_BODY_CONTRAST_MIN
      );
    }
  });
});
