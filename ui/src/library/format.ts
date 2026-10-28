import type { LessonStatus } from "../api/courseTypes";
import type { DerivedStatus } from "../api/libraryTypes";

export function formatRemainingMs(ms: number): string {
  if (ms <= 0) {
    return "0m";
  }
  const totalMinutes = Math.floor(ms / 60_000);
  const hours = Math.floor(totalMinutes / 60);
  const minutes = totalMinutes % 60;
  if (hours > 0) {
    return `${hours}h ${minutes}m`;
  }
  return `${minutes}m`;
}

export function formatStatus(status: DerivedStatus | LessonStatus): string {
  switch (status) {
    case "not_started":
      return "Not started";
    case "in_progress":
      return "In progress";
    case "done":
      return "Done";
    case "broken":
      return "Broken";
    default:
      return status;
  }
}

export function formatSourceKind(kind: "folder" | "playlist"): string {
  return kind === "folder" ? "Folder" : "Playlist";
}

export function formatDurationMs(ms: number | null): string {
  if (ms === null || ms <= 0) {
    return "—";
  }
  const totalSeconds = Math.floor(ms / 1000);
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  if (minutes >= 60) {
    const hours = Math.floor(minutes / 60);
    const remMin = minutes % 60;
    return `${hours}h ${remMin}m`;
  }
  return `${minutes}:${seconds.toString().padStart(2, "0")}`;
}

export function watchedPercent(
  maxMs: number,
  durationMs: number | null
): number | null {
  if (durationMs === null || durationMs <= 0) {
    return null;
  }
  return Math.min(100, Math.round((maxMs / durationMs) * 100));
}
