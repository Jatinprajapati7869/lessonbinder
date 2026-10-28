import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import React from "react";
import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import {
  formatStudyHeatmapSummary,
  type StudyHeatmap52Weeks,
  type StudyHeatmapDay,
} from "./src/api/libraryStudyHeatmapTypes";
import { LibraryStudyHeatmap } from "./src/library/LibraryStudyHeatmap";

const sampleHeatmap: StudyHeatmap52Weeks = {
  weeks: [
    [
      { local_date: "2026-10-01", counted: false },
      { local_date: "2026-10-02", counted: true },
      { local_date: "2026-10-03", counted: false },
      { local_date: "2026-10-04", counted: false },
      { local_date: "2026-10-05", counted: false },
      { local_date: "2026-10-06", counted: false },
      { local_date: "2026-10-07", counted: false },
    ],
  ],
  counted_days: 1,
  total_days: 364,
};

const emptyWeek: StudyHeatmapDay[] = Array.from({ length: 7 }, (_, index) => ({
  local_date: `2026-01-0${index + 1}`,
  counted: false,
}));

const here = dirname(fileURLToPath(import.meta.url));
const libraryCss = readFileSync(join(here, "src/theme/library.css"), "utf8");

describe("formatStudyHeatmapSummary", () => {
  it("states how many days counted in the window", () => {
    expect(formatStudyHeatmapSummary(3, 364)).toBe("3 of 364 days with study");
  });

  it("shows zero when no days counted", () => {
    expect(formatStudyHeatmapSummary(0, 364)).toBe("0 of 364 days with study");
  });
});

describe("LibraryStudyHeatmap", () => {
  it("shows counted-day summary text before the grid", () => {
    render(<LibraryStudyHeatmap heatmap={sampleHeatmap} />);
    const summary = screen.getByText("1 of 364 days with study");
    const grid = document.querySelector(".library-heatmap__grid");
    expect(summary).toBeInTheDocument();
    expect(grid).not.toBeNull();
    expect(
      summary.compareDocumentPosition(grid!) & Node.DOCUMENT_POSITION_FOLLOWING
    ).toBeTruthy();
  });

  it("marks counted days for accent styling", () => {
    const { container } = render(
      <LibraryStudyHeatmap heatmap={sampleHeatmap} />
    );
    expect(
      container.querySelectorAll(".library-heatmap__day--counted")
    ).toHaveLength(1);
    expect(container.querySelectorAll(".library-heatmap__week")).toHaveLength(
      1
    );
  });

  it("renders a week with zero counted days as seven track cells", () => {
    const heatmap: StudyHeatmap52Weeks = {
      weeks: [emptyWeek],
      counted_days: 0,
      total_days: 364,
    };
    render(<LibraryStudyHeatmap heatmap={heatmap} />);
    expect(screen.getByText("0 of 364 days with study")).toBeInTheDocument();
    const days = document.querySelectorAll(".library-heatmap__day");
    expect(days).toHaveLength(7);
    expect(
      document.querySelectorAll(".library-heatmap__day--counted")
    ).toHaveLength(0);
    expect(libraryCss).toMatch(
      /\.library-heatmap__day\s*\{[^}]*background:\s*var\(--progress-track\)/
    );
  });
});
