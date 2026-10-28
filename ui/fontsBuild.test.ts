import { execSync } from "node:child_process";
import {
  existsSync,
  mkdtempSync,
  readFileSync,
  readdirSync,
  renameSync,
  rmSync,
  statSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

import {
  assertBundledFontFilesPresent,
  BUNDLED_FONT_FILE_NAMES,
} from "./build/assertBundledFonts";

const here = dirname(fileURLToPath(import.meta.url));
const forbiddenFontHost = /fonts\.googleapis\.com|fonts\.gstatic\.com/;

function collectFiles(dir: string, extensions: string[]): string[] {
  const out: string[] = [];
  for (const entry of readdirSync(dir)) {
    const path = join(dir, entry);
    if (statSync(path).isDirectory()) {
      out.push(...collectFiles(path, extensions));
      continue;
    }
    if (extensions.some((ext) => path.endsWith(ext))) {
      out.push(path);
    }
  }
  return out;
}

describe("fontsBuild", () => {
  it("ships four local font files referenced from tokens.css", () => {
    assertBundledFontFilesPresent(here);

    const tokensCss = readFileSync(join(here, "src/theme/tokens.css"), "utf8");
    expect(tokensCss.match(/@font-face/g)?.length).toBe(4);
    for (const name of BUNDLED_FONT_FILE_NAMES) {
      expect(tokensCss).toContain(`./fonts/${name}`);
    }
    expect(tokensCss).not.toMatch(forbiddenFontHost);
  });

  it("does not reference Google Fonts hosts in UI source or index.html", () => {
    const roots = [
      join(here, "index.html"),
      ...collectFiles(join(here, "src"), [".css", ".tsx", ".ts", ".html"]),
    ];
    for (const path of roots) {
      expect(readFileSync(path, "utf8"), path).not.toMatch(forbiddenFontHost);
    }
  });

  it("includes hashed font assets in ui/dist when a production build exists", () => {
    const distAssets = join(here, "dist/assets");
    if (!existsSync(distAssets)) {
      return;
    }
    const builtFonts = readdirSync(distAssets).filter((name) =>
      name.endsWith(".ttf")
    );
    expect(builtFonts.length).toBeGreaterThanOrEqual(4);
  });
});

describe("missing bundled font", () => {
  it("reports the missing file name from assertBundledFontFilesPresent", () => {
    const emptyUiRoot = mkdtempSync(join(tmpdir(), "lessonbinder-ui-"));
    try {
      expect(() => assertBundledFontFilesPresent(emptyUiRoot)).toThrow(
        /Missing licensed font files:.*googlesans-regular\.ttf/
      );
    } finally {
      rmSync(emptyUiRoot, { recursive: true, force: true });
    }
  });

  it("fails vite build with the missing font file name", () => {
    const fontPath = join(here, "src/theme/fonts/googlesans-regular.ttf");
    if (!existsSync(fontPath)) {
      return;
    }
    const hiddenPath = `${fontPath}.vitest-hidden`;
    renameSync(fontPath, hiddenPath);
    try {
      execSync("npx vite build", {
        cwd: here,
        encoding: "utf8",
        stdio: ["ignore", "pipe", "pipe"],
      });
      expect.fail("expected vite build to fail when a font file is missing");
    } catch (error) {
      const err = error as {
        stderr?: string;
        stdout?: string;
        message?: string;
      };
      const output = `${err.stderr ?? ""}${err.stdout ?? ""}${err.message ?? ""}`;
      expect(output).toContain("googlesans-regular.ttf");
    } finally {
      renameSync(hiddenPath, fontPath);
    }
  });
});
