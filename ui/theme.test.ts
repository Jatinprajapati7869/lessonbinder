import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

import {
  DESIGN_COLOR_TOKENS,
  DESIGN_FOCUS_TOKENS,
  DESIGN_FONT_TOKENS,
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

function collectSourceFiles(dir: string, ext: string): string[] {
  const out: string[] = [];
  for (const entry of readdirSync(dir)) {
    const path = join(dir, entry);
    const stat = statSync(path);
    if (stat.isDirectory()) {
      out.push(...collectSourceFiles(path, ext));
    } else if (path.endsWith(ext)) {
      out.push(path);
    }
  }
  return out;
}

describe("DESIGN.md CSS tokens", () => {
  it("declares color tokens in tokens.css", () => {
    for (const [name, value] of Object.entries(DESIGN_COLOR_TOKENS)) {
      expectTokenInCss(name, normalizeHex(value));
    }
  });

  it("declares font family tokens", () => {
    expect(DESIGN_FONT_TOKENS["--font-family"]).toContain("Google Sans");
    expect(DESIGN_FONT_TOKENS["--font-wordmark"]).toContain("Andina");
    expect(tokensCss).toMatch(/--font-family:\s*"Google Sans"/);
    expect(tokensCss).toMatch(/--font-wordmark:\s*"Andina"/);
    for (const [name, value] of Object.entries(DESIGN_FONT_TOKENS)) {
      expectTokenInCss(name, value);
    }
    expect(tokensCss).not.toContain("IBM Plex");
    expect(tokensCss).not.toMatch(/fonts\.googleapis\.com/);
    expect(tokensCss).not.toMatch(/fonts\.gstatic\.com/);
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
    expect(shellCss).toMatch(
      /\.top-bar__wordmark[\s\S]*font-family:\s*var\(--font-wordmark\)/
    );
    expect(shellCss).not.toMatch(/font-family:\s*["']/);
  });

  it("uses the wordmark face only on the shell wordmark", () => {
    const themeDir = join(here, "src/theme");
    for (const file of readdirSync(themeDir).filter((name) =>
      name.endsWith(".css")
    )) {
      const css = readFileSync(join(themeDir, file), "utf8");
      if (file === "tokens.css") {
        const withoutDefinitions = css
          .replace(/@font-face[\s\S]*?\}/g, "")
          .replace(/--font-wordmark:[^;]+;/g, "");
        expect(withoutDefinitions).not.toContain("var(--font-wordmark)");
        continue;
      }
      if (!css.includes("var(--font-wordmark)")) {
        continue;
      }
      expect(file).toBe("shell.css");
      const stripped = css.replace(/\.top-bar__wordmark[\s\S]*?\}/, "");
      expect(stripped).not.toContain("var(--font-wordmark)");
    }

    const wordmarkNodes = collectSourceFiles(join(here, "src"), ".tsx").filter(
      (path) => readFileSync(path, "utf8").includes("top-bar__wordmark")
    );
    expect(wordmarkNodes).toEqual([join(here, "src/shell/AppShell.tsx")]);
  });
});
