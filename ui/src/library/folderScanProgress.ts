export const FOLDER_SCAN_PROGRESS_EVENT = "folder_scan_progress";

export type FolderScanProgressPayload = {
  seen: number;
  planned: number;
};

/** Formats scan/import progress from backend counts (never a timer). */
export function folderScanCountLabel(seen: number, planned: number): string {
  if (planned > 0) {
    return `${seen} of ${planned}`;
  }
  if (seen > 0) {
    return `${seen} lesson file${seen === 1 ? "" : "s"} found`;
  }
  return "";
}

export function folderScanStatusLine(
  mode: "import" | "rescan",
  progress: FolderScanProgressPayload | null
): string {
  const prefix = mode === "import" ? "Scanning folder" : "Rescanning folder";
  if (progress === null) {
    return `${prefix}…`;
  }
  const count = folderScanCountLabel(progress.seen, progress.planned);
  if (!count) {
    return `${prefix}…`;
  }
  return `${prefix}… ${count}`;
}
