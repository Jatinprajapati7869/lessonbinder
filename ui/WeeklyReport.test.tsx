import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import type { WeeklyReportCommandPort } from "./src/api/weeklyReportCommands";
import {
  WEEKLY_REPORT_CARDS_GRADED_LABEL,
  WEEKLY_REPORT_LESSONS_DONE_LABEL,
  WEEKLY_REPORT_WATCHED_MINUTES_LABEL,
} from "./src/api/weeklyReportTypes";
import { WeeklyReportCommandsProvider } from "./src/report/WeeklyReportCommandsContext";
import { WeeklyReportScreen } from "./src/screens/WeeklyReportScreen";

function renderReport(commands: WeeklyReportCommandPort) {
  const view = render(
    <WeeklyReportCommandsProvider commands={commands}>
      <WeeklyReportScreen />
    </WeeklyReportCommandsProvider>
  );
  return view;
}

const FORBIDDEN_WEEKLY_REPORT_COPY =
  /keep (it up|going)|great (job|week)|well done|you'?re on|building a habit|stay consistent|congratulations/i;

describe("WeeklyReportScreen", () => {
  it("shows week range with labeled lesson and card counts", async () => {
    const { container } = renderReport({
      getWeeklyReport: vi.fn().mockResolvedValue({
        week_monday: "2026-10-26",
        week_sunday: "2026-11-01",
        lessons_marked_done: 2,
        cards_graded: 5,
        watched_minutes: null,
      }),
    });

    await waitFor(() => {
      expect(screen.getByText("2026-10-26 – 2026-11-01")).toBeInTheDocument();
    });

    expect(
      screen.getByText(WEEKLY_REPORT_LESSONS_DONE_LABEL)
    ).toBeInTheDocument();
    expect(
      screen.getByText(WEEKLY_REPORT_CARDS_GRADED_LABEL)
    ).toBeInTheDocument();

    const metrics = container.querySelectorAll(".weekly-report__metric");
    expect(metrics).toHaveLength(2);
    expect(
      metrics[0].querySelector(".weekly-report__metric-value")
    ).toHaveTextContent("2");
    expect(
      metrics[1].querySelector(".weekly-report__metric-value")
    ).toHaveTextContent("5");

    expect(container.textContent).not.toMatch(FORBIDDEN_WEEKLY_REPORT_COPY);
    expect(screen.queryByText(/minutes watched/i)).not.toBeInTheDocument();
    expect(screen.queryByText(/advice/i)).not.toBeInTheDocument();
    expect(screen.queryByText(/email/i)).not.toBeInTheDocument();
  });

  it("shows watched minutes with label when the report includes them", async () => {
    const { container } = renderReport({
      getWeeklyReport: vi.fn().mockResolvedValue({
        week_monday: "2026-10-26",
        week_sunday: "2026-11-01",
        lessons_marked_done: 0,
        cards_graded: 0,
        watched_minutes: 42,
      }),
    });

    await waitFor(() => {
      expect(
        screen.getByText(WEEKLY_REPORT_WATCHED_MINUTES_LABEL)
      ).toBeInTheDocument();
    });
    expect(
      container.querySelector(
        ".weekly-report__metric:last-of-type .weekly-report__metric-value"
      )
    ).toHaveTextContent("42");
  });
});
