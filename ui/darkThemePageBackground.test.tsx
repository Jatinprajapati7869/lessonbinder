import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

import {
  DESIGN_COLOR_TOKENS,
  DESIGN_DARK_COLOR_TOKENS,
} from "./src/theme/tokens";

const here = dirname(fileURLToPath(import.meta.url));
const tokensCss = readFileSync(join(here, "src/theme/tokens.css"), "utf8");

describe("dark theme page background", () => {
  it("declares a dark app background in tokens.css, not the light app hex", () => {
    const darkBlock = tokensCss.slice(
      tokensCss.indexOf(':root[data-theme="dark"]')
    );
    const lightApp = DESIGN_COLOR_TOKENS["--bg-app"].toLowerCase();
    const darkApp = DESIGN_DARK_COLOR_TOKENS["--bg-app"].toLowerCase();
    expect(darkBlock.toLowerCase()).toContain(`--bg-app: ${darkApp}`);
    expect(darkBlock.toLowerCase()).not.toContain(`--bg-app: ${lightApp}`);
  });

  it("paints the main pane from --bg-app without light hex literals in shell CSS", () => {
    const shellCss = readFileSync(join(here, "src/theme/shell.css"), "utf8");
    expect(shellCss).toMatch(/\.main-pane[\s\S]*background:\s*var\(--bg-app\)/);
    expect(shellCss).not.toContain(DESIGN_COLOR_TOKENS["--bg-app"]);
    expect(shellCss).not.toContain(DESIGN_COLOR_TOKENS["--bg-surface"]);
  });
});
