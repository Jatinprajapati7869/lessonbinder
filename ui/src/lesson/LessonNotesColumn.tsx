import { useCallback, useEffect, useRef, useState } from "react";

import type { LessonNote } from "../api/courseTypes";
import { useCourseCommands } from "../course/useCourseCommands";
import { formatDurationMs } from "../library/format";
import { handleLessonNotesArrowKeys } from "../player/playerSidePanelKeyboard";
import { LessonNoteCardPanel } from "../screens/LessonNoteCardPanel";

export type LessonNotesPlaybackContext = {
  currentMs: number;
  onSeek: (ms: number) => void;
};

export type LessonNotesColumnProps = {
  lessonId: string;
  playbackContext?: LessonNotesPlaybackContext;
};

type EditorState =
  { mode: "closed" } | { mode: "create" } | { mode: "edit"; note: LessonNote };

export function LessonNotesColumn({
  lessonId,
  playbackContext,
}: LessonNotesColumnProps) {
  const commands = useCourseCommands();
  const [notes, setNotes] = useState<LessonNote[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [editor, setEditor] = useState<EditorState>({ mode: "closed" });
  const [draftText, setDraftText] = useState("");
  const [linkTimestamp, setLinkTimestamp] = useState(false);
  const [cardNote, setCardNote] = useState<LessonNote | null>(null);
  const listRef = useRef<HTMLOListElement>(null);

  const refresh = useCallback(async () => {
    setError(null);
    try {
      const loaded = await commands.listLessonNotes(lessonId);
      setNotes(loaded);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not load notes.");
    }
  }, [commands, lessonId]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  function openCreate() {
    setDraftText("");
    setLinkTimestamp(playbackContext !== undefined);
    setEditor({ mode: "create" });
  }

  function openEdit(note: LessonNote) {
    setDraftText(note.text);
    setLinkTimestamp(note.timestamp_ms !== null);
    setEditor({ mode: "edit", note });
  }

  function closeEditor() {
    setEditor({ mode: "closed" });
    setDraftText("");
  }

  async function handleSave() {
    const text = draftText.trim();
    if (!text) {
      return;
    }
    const timestampMs =
      linkTimestamp && playbackContext ? playbackContext.currentMs : null;
    setBusy(true);
    setError(null);
    try {
      if (editor.mode === "create") {
        await commands.createLessonNote(lessonId, text, timestampMs);
      } else if (editor.mode === "edit") {
        await commands.updateLessonNote(
          editor.note.note_id,
          text,
          linkTimestamp && playbackContext ? timestampMs : null
        );
      }
      closeEditor();
      await refresh();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not save note.");
    } finally {
      setBusy(false);
    }
  }

  async function handleDelete(noteId: string) {
    setBusy(true);
    setError(null);
    try {
      await commands.deleteLessonNote(noteId);
      await refresh();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not delete note.");
    } finally {
      setBusy(false);
    }
  }

  return (
    <aside className="lesson-notes" role="region" aria-label="Lesson notes">
      <div className="lesson-notes__header">
        <h2 className="lesson-notes__title">Notes</h2>
        <button
          type="button"
          className="library-button-secondary lesson-notes__add"
          disabled={busy || editor.mode !== "closed"}
          onClick={openCreate}
        >
          Add note
        </button>
      </div>

      {error ? (
        <p className="library-error" role="alert">
          {error}
        </p>
      ) : null}

      {editor.mode !== "closed" ? (
        <form
          className="lesson-notes__editor"
          onSubmit={(event) => {
            event.preventDefault();
            void handleSave();
          }}
        >
          <label className="lesson-notes__label" htmlFor="lesson-note-text">
            Note text
          </label>
          <textarea
            id="lesson-note-text"
            className="lesson-notes__textarea"
            rows={4}
            value={draftText}
            disabled={busy}
            onChange={(event) => setDraftText(event.target.value)}
          />
          {playbackContext ? (
            <label className="lesson-notes__timestamp-toggle">
              <input
                type="checkbox"
                checked={linkTimestamp}
                disabled={busy}
                onChange={(event) => setLinkTimestamp(event.target.checked)}
              />
              Link to current time (
              {formatDurationMs(playbackContext.currentMs)})
            </label>
          ) : null}
          <div className="lesson-notes__editor-actions">
            <button
              type="button"
              className="library-button-secondary"
              disabled={busy}
              onClick={closeEditor}
            >
              Cancel
            </button>
            <button
              type="submit"
              className="library-button-primary"
              disabled={busy || !draftText.trim()}
            >
              Save note
            </button>
          </div>
        </form>
      ) : null}

      <ol
        ref={listRef}
        className="lesson-notes__list"
        onKeyDown={(event) =>
          handleLessonNotesArrowKeys(event, listRef.current)
        }
      >
        {notes.map((note) => (
          <li key={note.note_id} className="lesson-notes__item">
            <div className="lesson-notes__item-body">
              {note.timestamp_ms !== null && playbackContext ? (
                <button
                  type="button"
                  className="lesson-notes__timestamp"
                  onClick={() => playbackContext.onSeek(note.timestamp_ms!)}
                >
                  Seek to {formatDurationMs(note.timestamp_ms)}
                </button>
              ) : note.timestamp_ms !== null ? (
                <span className="lesson-notes__timestamp-static">
                  {formatDurationMs(note.timestamp_ms)}
                </span>
              ) : null}
              <p className="lesson-notes__text">{note.text}</p>
            </div>
            <div className="lesson-notes__item-actions">
              <button
                type="button"
                className="library-button-secondary"
                disabled={busy || !note.text.trim()}
                onClick={() => setCardNote(note)}
              >
                Make card
              </button>
              <button
                type="button"
                className="library-button-secondary"
                disabled={busy}
                onClick={() => openEdit(note)}
              >
                Edit
              </button>
              <button
                type="button"
                className="library-button-secondary"
                disabled={busy}
                onClick={() => void handleDelete(note.note_id)}
              >
                Delete
              </button>
            </div>
          </li>
        ))}
      </ol>
      {notes.length === 0 && editor.mode === "closed" ? (
        <p className="lesson-notes__empty">No notes yet.</p>
      ) : null}

      {cardNote ? (
        <LessonNoteCardPanel
          lessonId={lessonId}
          note={cardNote}
          onClose={() => setCardNote(null)}
        />
      ) : null}
    </aside>
  );
}
