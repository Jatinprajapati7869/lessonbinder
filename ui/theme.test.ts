import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

import {
  DESIGN_COLOR_TOKENS,
  DESIGN_FOCUS_TOKENS,
  DESIGN_RADIUS_TOKENS,
  DESIGN_SPACE_TOKENS,
  DESIGN_TYPE_TOKENS,
} from "./src/theme/tokens";

const here = dirname(fileURLToPath(import.meta.url));
const tokensCss = readFileSync(join(here, "src/theme/tokens.css"), "utf8");

function normalizeHex(value: string): string {
  return value.trim().toLowerCase();
}

function expectTokenInCss(name: string, value: string) {
  const pattern = new RegExp(
    `${name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}:\\s*${value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}`,
    "i"
  );
  if (value.startsWith("#")) {
    expect(tokensCss).toMatch(pattern);
    return;
  }
  expect(tokensCss).toContain(`${name}: ${value}`);
}

describe("DESIGN.md CSS tokens", () => {
  it("declares color tokens in tokens.css", () => {
    for (const [name, value] of Object.entries(DESIGN_COLOR_TOKENS)) {
      expectTokenInCss(name, normalizeHex(value));
    }
  });

  it("declares spacing and type scale tokens", () => {
    for (const [name, value] of Object.entries(DESIGN_SPACE_TOKENS)) {
      expectTokenInCss(name, value);
    }
    for (const [name, value] of Object.entries(DESIGN_TYPE_TOKENS)) {
      expectTokenInCss(name, value);
    }
    for (const [name, value] of Object.entries(DESIGN_RADIUS_TOKENS)) {
      expectTokenInCss(name, value);
    }
    for (const [name, value] of Object.entries(DESIGN_FOCUS_TOKENS)) {
      expectTokenInCss(name, value);
    }
  });

  it("shell styles avoid literal colors outside tokens.css", () => {
    const shellCss = readFileSync(join(here, "src/theme/shell.css"), "utf8");
    const hex = /#[0-9a-f]{3,8}/gi;
    expect(shellCss.match(hex)).toBeNull();
    expect(shellCss).not.toMatch(/font-family\s*:/);
  });
});
