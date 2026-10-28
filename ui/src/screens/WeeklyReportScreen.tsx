import { useCallback, useEffect, useState } from "react";

import type { WeeklyReport } from "../api/weeklyReportTypes";
import {
  formatWeeklyReportWeekRange,
  WEEKLY_REPORT_CARDS_GRADED_LABEL,
  WEEKLY_REPORT_LESSONS_DONE_LABEL,
  WEEKLY_REPORT_WATCHED_MINUTES_LABEL,
} from "../api/weeklyReportTypes";
import { useWeeklyReportCommands } from "../report/WeeklyReportCommandsContext";

export function WeeklyReportScreen() {
  const commands = useWeeklyReportCommands();
  const [report, setReport] = useState<WeeklyReport | null>(null);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setError(null);
    try {
      setReport(await commands.getWeeklyReport());
    } catch (e) {
      setError(
        e instanceof Error ? e.message : "Could not load weekly report."
      );
    }
  }, [commands]);

  useEffect(() => {
    void load();
  }, [load]);

  if (error !== null) {
    return (
      <div className="weekly-report">
        <p className="weekly-report__error" role="alert">
          {error}
        </p>
      </div>
    );
  }

  if (report === null) {
    return (
      <div className="weekly-report">
        <p className="weekly-report__loading">Loading…</p>
      </div>
    );
  }

  return (
    <div className="weekly-report">
      <p className="weekly-report__week">
        {formatWeeklyReportWeekRange(report.week_monday, report.week_sunday)}
      </p>
      <dl className="weekly-report__metrics">
        <div className="weekly-report__metric">
          <dt className="weekly-report__metric-label">
            {WEEKLY_REPORT_LESSONS_DONE_LABEL}
          </dt>
          <dd className="weekly-report__metric-value">
            {report.lessons_marked_done}
          </dd>
        </div>
        <div className="weekly-report__metric">
          <dt className="weekly-report__metric-label">
            {WEEKLY_REPORT_CARDS_GRADED_LABEL}
          </dt>
          <dd className="weekly-report__metric-value">{report.cards_graded}</dd>
        </div>
        {report.watched_minutes !== null ? (
          <div className="weekly-report__metric">
            <dt className="weekly-report__metric-label">
              {WEEKLY_REPORT_WATCHED_MINUTES_LABEL}
            </dt>
            <dd className="weekly-report__metric-value">
              {report.watched_minutes}
            </dd>
          </div>
        ) : null}
      </dl>
    </div>
  );
}
