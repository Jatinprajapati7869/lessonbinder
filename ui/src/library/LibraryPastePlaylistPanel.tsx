export type LibraryPastePlaylistPanelProps = {
  busy: boolean;
  onCancel: () => void;
  onCreate: (courseTitle: string, text: string) => void;
};

export function LibraryPastePlaylistPanel({
  busy,
  onCancel,
  onCreate,
}: LibraryPastePlaylistPanelProps) {
  return (
    <form
      className="library-paste-playlist"
      aria-label="Paste playlist"
      onSubmit={(event) => {
        event.preventDefault();
        const data = new FormData(event.currentTarget);
        const courseTitle = String(data.get("courseTitle") ?? "").trim();
        const text = String(data.get("playlistLines") ?? "");
        if (!courseTitle || !text.trim()) {
          return;
        }
        onCreate(courseTitle, text);
      }}
    >
      <label
        className="library-paste-playlist__label"
        htmlFor="paste-course-title"
      >
        Course title
      </label>
      <input
        id="paste-course-title"
        name="courseTitle"
        type="text"
        className="library-paste-playlist__input"
        disabled={busy}
        autoComplete="off"
      />
      <label
        className="library-paste-playlist__label"
        htmlFor="paste-playlist-lines"
      >
        Playlist lines
      </label>
      <textarea
        id="paste-playlist-lines"
        name="playlistLines"
        className="library-paste-playlist__textarea"
        rows={8}
        placeholder={"Lesson title\thttps://…"}
        disabled={busy}
      />
      <div className="library-paste-playlist__actions">
        <button
          type="button"
          className="library-button-secondary"
          disabled={busy}
          onClick={onCancel}
        >
          Cancel
        </button>
        <button type="submit" className="study-button" disabled={busy}>
          Create
        </button>
      </div>
    </form>
  );
}
