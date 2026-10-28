import { describe, expect, it } from "vitest";

import {
  folderScanCountLabel,
  folderScanStatusLine,
} from "./src/library/folderScanProgress";

describe("folderScanCountLabel", () => {
  it('shows "n of m" when the planned total is known', () => {
    expect(folderScanCountLabel(1, 3)).toBe("1 of 3");
    expect(folderScanCountLabel(3, 3)).toBe("3 of 3");
  });

  it("uses lesson file counts while scanning before the total is known", () => {
    expect(folderScanCountLabel(2, 0)).toBe("2 lesson files found");
    expect(folderScanCountLabel(1, 0)).toBe("1 lesson file found");
  });
});

describe("folderScanStatusLine", () => {
  it("does not invent a total from a timer", () => {
    expect(folderScanStatusLine("import", { seen: 1, planned: 4 })).toBe(
      "Scanning folder… 1 of 4"
    );
    expect(folderScanStatusLine("rescan", { seen: 2, planned: 5 })).toBe(
      "Rescanning folder… 2 of 5"
    );
  });
});
