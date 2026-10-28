export type WeeklyReport = {
  week_monday: string;
  week_sunday: string;
  lessons_marked_done: number;
  cards_graded: number;
  watched_minutes: number | null;
};

export const WEEKLY_REPORT_LESSONS_DONE_LABEL = "Lessons marked done";
export const WEEKLY_REPORT_CARDS_GRADED_LABEL = "Cards graded";
export const WEEKLY_REPORT_WATCHED_MINUTES_LABEL = "Minutes watched";

export function formatWeeklyReportWeekRange(
  monday: string,
  sunday: string
): string {
  return `${monday} – ${sunday}`;
}
