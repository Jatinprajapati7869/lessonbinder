export const LIBRARY_FIRST_RUN_COPY =
  "Import a folder on your disk or a playlist JSON file to add courses.";

export type LibraryFirstRunEmptyProps = {
  busy: boolean;
  error: string | null;
  onAddFolder: () => void;
  onAddPlaylistJson: () => void;
  onPastePlaylist: () => void;
};

export function LibraryFirstRunEmpty({
  busy,
  error,
  onAddFolder,
  onAddPlaylistJson,
  onPastePlaylist,
}: LibraryFirstRunEmptyProps) {
  return (
    <div className="library-empty" role="region" aria-label="Empty library">
      <p className="library-empty__hint">{LIBRARY_FIRST_RUN_COPY}</p>
      <div className="library-empty__actions">
        <button
          type="button"
          className="study-button"
          disabled={busy}
          onClick={onAddFolder}
        >
          Add folder
        </button>
        <button
          type="button"
          className="library-button-secondary"
          disabled={busy}
          onClick={onAddPlaylistJson}
        >
          Add playlist JSON
        </button>
        <button
          type="button"
          className="library-button-secondary"
          disabled={busy}
          onClick={onPastePlaylist}
        >
          Paste playlist
        </button>
      </div>
      {error ? (
        <p className="library-error" role="alert">
          {error}
        </p>
      ) : null}
    </div>
  );
}
