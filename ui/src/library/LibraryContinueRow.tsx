import type { LibraryContinueRow as LibraryContinueRowData } from "../api/libraryContinueTypes";

export type LibraryContinueRowProps = {
  row: LibraryContinueRowData;
  onActivate: () => void;
};

export function formatContinueWatchedPercent(
  watchedPercent: number | null
): string {
  if (watchedPercent === null) {
    return "Unknown";
  }
  return `${watchedPercent}%`;
}

export function LibraryContinueRow({
  row,
  onActivate,
}: LibraryContinueRowProps) {
  return (
    <button
      type="button"
      className="library-continue-row"
      aria-label={`Continue ${row.lesson_title}`}
      onClick={onActivate}
    >
      <span className="library-continue-row__label">Continue</span>
      <span className="library-continue-row__course">{row.course_title}</span>
      <span className="library-continue-row__lesson">{row.lesson_title}</span>
      <span className="library-continue-row__watched">
        {formatContinueWatchedPercent(row.watched_percent)}
      </span>
    </button>
  );
}
