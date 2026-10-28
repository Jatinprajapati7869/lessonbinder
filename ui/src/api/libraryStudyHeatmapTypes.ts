//! Copy for the 52-week study heatmap summary (kept in one place for tests).

export type StudyHeatmapDay = {
  local_date: string;
  counted: boolean;
};

export type StudyHeatmap52Weeks = {
  weeks: StudyHeatmapDay[][];
  counted_days: number;
  total_days: number;
};

export function formatStudyHeatmapSummary(
  countedDays: number,
  totalDays: number
): string {
  return `${countedDays} of ${totalDays} days with study`;
}

export function emptyStudyHeatmap52Weeks(): StudyHeatmap52Weeks {
  const weeks = Array.from({ length: 52 }, () =>
    Array.from({ length: 7 }, () => ({
      local_date: "1970-01-01",
      counted: false,
    }))
  );
  return {
    weeks,
    counted_days: 0,
    total_days: 364,
  };
}
