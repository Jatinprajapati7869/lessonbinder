import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const scanRoots = [join(repoRoot, "ui/src"), join(repoRoot, "src-tauri/src")];

const ANALYTICS_RE =
  /analytics|telemetry|mixpanel|sentry|posthog|google-analytics|gtag\(|hotjar|fullstory|amplitude/i;

const HTTP_URL_RE = /https?:\/\/[^\s"'`)]+/g;

const ALLOWED_HTTP_SUBSTRINGS = ["http://www.w3.org/2000/svg", "https://…"];

function collectSourceFiles(dir: string): string[] {
  const out: string[] = [];
  for (const entry of readdirSync(dir)) {
    const path = join(dir, entry);
    if (statSync(path).isDirectory()) {
      out.push(...collectSourceFiles(path));
      continue;
    }
    if (/\.(tsx?|rs)$/.test(entry)) {
      out.push(path);
    }
  }
  return out;
}

function isAllowedHttpMatch(match: string, line: string): boolean {
  if (ALLOWED_HTTP_SUBSTRINGS.some((allowed) => line.includes(allowed))) {
    return true;
  }
  try {
    const { hostname } = new URL(match);
    return (
      hostname === "localhost" ||
      hostname === "127.0.0.1" ||
      hostname === "asset.localhost"
    );
  } catch {
    return false;
  }
}

describe("ui/src and src-tauri/src have no analytics or off-localhost fetch targets", () => {
  it("matches the SITTING.md ripgrep audit (no removable phone-home hits)", () => {
    const violations: string[] = [];

    for (const root of scanRoots) {
      for (const file of collectSourceFiles(root)) {
        const text = readFileSync(file, "utf8");
        const rel = file.slice(repoRoot.length + 1);
        if (ANALYTICS_RE.test(text)) {
          violations.push(`${rel}: analytics/telemetry pattern`);
        }
        for (const line of text.split("\n")) {
          for (const match of line.matchAll(HTTP_URL_RE)) {
            if (!isAllowedHttpMatch(match[0], line)) {
              violations.push(`${rel}: ${match[0]}`);
            }
          }
        }
      }
    }

    expect(violations).toEqual([]);
  });
});
