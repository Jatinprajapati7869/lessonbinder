import { describe, expect, it } from "vitest";

import {
  applyUiTheme,
  readUiThemeFromDocument,
} from "./src/theme/applyUiTheme";

describe("applyUiTheme", () => {
  it("sets data-theme on the document root", () => {
    applyUiTheme("dark");
    expect(document.documentElement.dataset.theme).toBe("dark");
    expect(readUiThemeFromDocument()).toBe("dark");

    applyUiTheme("light");
    expect(document.documentElement.dataset.theme).toBe("light");
    expect(readUiThemeFromDocument()).toBe("light");
  });
});
