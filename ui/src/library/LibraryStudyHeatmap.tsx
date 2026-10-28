import {
  formatStudyHeatmapSummary,
  type StudyHeatmap52Weeks,
} from "../api/libraryStudyHeatmapTypes";

export type LibraryStudyHeatmapProps = {
  heatmap: StudyHeatmap52Weeks;
};

export function LibraryStudyHeatmap({ heatmap }: LibraryStudyHeatmapProps) {
  const summary = formatStudyHeatmapSummary(
    heatmap.counted_days,
    heatmap.total_days
  );

  return (
    <section
      className="library-heatmap"
      aria-labelledby="library-heatmap-summary"
    >
      <p id="library-heatmap-summary" className="library-heatmap__summary">
        {summary}
      </p>
      <div className="library-heatmap__grid" aria-hidden="true">
        {heatmap.weeks.map((week, weekIndex) => (
          <div key={`week-${weekIndex}`} className="library-heatmap__week">
            {week.map((day) => (
              <span
                key={day.local_date}
                className={
                  day.counted
                    ? "library-heatmap__day library-heatmap__day--counted"
                    : "library-heatmap__day"
                }
                title={day.local_date}
              />
            ))}
          </div>
        ))}
      </div>
    </section>
  );
}
