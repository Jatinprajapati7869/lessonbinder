export const LIBRARY_FIRST_RUN_COPY =
  "Add a folder on your disk to import a course.";

export type LibraryFirstRunEmptyProps = {
  busy: boolean;
  error: string | null;
  onAddFolder: () => void;
};

export function LibraryFirstRunEmpty({
  busy,
  error,
  onAddFolder,
}: LibraryFirstRunEmptyProps) {
  return (
    <div className="library-empty" role="region" aria-label="Empty library">
      <p className="library-empty__hint">{LIBRARY_FIRST_RUN_COPY}</p>
      <div className="library-empty__actions">
        <button
          type="button"
          className="library-button-primary"
          disabled={busy}
          onClick={onAddFolder}
        >
          Add folder
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
