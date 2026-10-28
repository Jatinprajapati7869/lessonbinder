export type LibraryFolderScanProps = {
  onCancel: () => void;
};

export function LibraryFolderScan({ onCancel }: LibraryFolderScanProps) {
  return (
    <div className="library-scan" role="status" aria-live="polite">
      <p className="library-scan__label">Scanning folder…</p>
      <button
        type="button"
        className="library-button-secondary"
        onClick={onCancel}
      >
        Cancel
      </button>
    </div>
  );
}
