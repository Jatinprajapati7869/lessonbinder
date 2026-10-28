import { useEffect, useState } from "react";

import {
  FOLDER_SCAN_PROGRESS_EVENT,
  type FolderScanProgressPayload,
} from "./folderScanProgress";

export function useFolderScanProgress(
  active: boolean
): FolderScanProgressPayload | null {
  const [progress, setProgress] = useState<FolderScanProgressPayload | null>(
    null
  );

  useEffect(() => {
    if (!active) {
      setProgress(null);
      return;
    }
    if (typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) {
      return;
    }
    let unlisten: (() => void) | undefined;
    void (async () => {
      const { listen } = await import("@tauri-apps/api/event");
      unlisten = await listen<FolderScanProgressPayload>(
        FOLDER_SCAN_PROGRESS_EVENT,
        (event) => {
          setProgress(event.payload);
        }
      );
    })();
    return () => {
      unlisten?.();
    };
  }, [active]);

  return progress;
}
