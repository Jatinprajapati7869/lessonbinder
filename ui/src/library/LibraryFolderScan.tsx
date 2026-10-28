export const LIBRARY_CANCEL_IMPORT_LABEL = "Cancel import";
export const LIBRARY_CANCEL_RESCAN_LABEL = "Cancel rescan";

export type LibraryFolderScanMode = "import" | "rescan";

export type LibraryFolderScanProps = {
  mode: LibraryFolderScanMode;
  onCancel: () => void;
};

function scanStatusLabel(mode: LibraryFolderScanMode): string {
  switch (mode) {
    case "import":
      return "Scanning folder…";
    case "rescan":
      return "Rescanning folder…";
    default: {
      const unreachable: never = mode;
      return unreachable;
    }
  }
}

function cancelLabel(mode: LibraryFolderScanMode): string {
  switch (mode) {
    case "import":
      return LIBRARY_CANCEL_IMPORT_LABEL;
    case "rescan":
      return LIBRARY_CANCEL_RESCAN_LABEL;
    default: {
      const unreachable: never = mode;
      return unreachable;
    }
  }
}

export function LibraryFolderScan({ mode, onCancel }: LibraryFolderScanProps) {
  const cancelText = cancelLabel(mode);
  return (
    <div className="library-scan" role="status" aria-live="polite">
      <p className="library-scan__label">{scanStatusLabel(mode)}</p>
      <button
        type="button"
        className="library-button-secondary"
        aria-label={cancelText}
        onClick={onCancel}
      >
        {cancelText}
      </button>
    </div>
  );
}
