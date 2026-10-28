import { existsSync } from "node:fs";
import { join } from "node:path";

export const BUNDLED_FONT_FILE_NAMES = [
  "googlesans-regular.ttf",
  "googlesans-medium.ttf",
  "googlesans-bold.ttf",
  "andina.ttf",
] as const;

export function assertBundledFontFilesPresent(uiRoot: string): void {
  const fontsDir = join(uiRoot, "src/theme/fonts");
  const missing = BUNDLED_FONT_FILE_NAMES.filter(
    (name) => !existsSync(join(fontsDir, name))
  );
  if (missing.length === 0) {
    return;
  }
  if (missing.length === 1) {
    throw new Error(`Missing licensed font file: ${missing[0]}`);
  }
  throw new Error(`Missing licensed font files: ${missing.join(", ")}`);
}
