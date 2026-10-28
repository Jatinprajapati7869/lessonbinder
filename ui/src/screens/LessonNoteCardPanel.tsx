import { useEffect, useRef, useState } from "react";

import type { LessonNote } from "../api/courseTypes";
import type { CardDraft, CommittedCard } from "../api/captureTypes";
import { useCaptureCommands } from "../capture/useCaptureCommands";

export type LessonNoteCardPanelProps = {
  lessonId: string;
  note: LessonNote;
  onClose: () => void;
};

export function LessonNoteCardPanel({
  lessonId,
  note,
  onClose,
}: LessonNoteCardPanelProps) {
  const commands = useCaptureCommands();
  const dialogRef = useRef<HTMLDivElement>(null);
  const [draft, setDraft] = useState<CardDraft | null>(null);
  const [front, setFront] = useState("");
  const [back, setBack] = useState("");
  const [committed, setCommitted] = useState<CommittedCard | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    let cancelled = false;
    void commands
      .buildLessonNoteDraft({
        lessonId,
        noteText: note.text,
        timestampMs: note.timestamp_ms,
      })
      .then((next) => {
        if (!cancelled) {
          setDraft(next);
          setFront(next.front);
          setBack(next.back);
        }
      })
      .catch((e) => {
        if (!cancelled) {
          setError(
            e instanceof Error ? e.message : "Could not build a card draft."
          );
        }
      });
    return () => {
      cancelled = true;
    };
  }, [commands, lessonId, note.text, note.timestamp_ms]);

  async function handleCommit() {
    if (!draft) {
      return;
    }
    setBusy(true);
    setError(null);
    try {
      const result = await commands.commitLessonNoteCard({
        lessonId,
        timestampMs: note.timestamp_ms,
        front,
        back,
      });
      setCommitted(result);
    } catch (e) {
      setError(
        e instanceof Error ? e.message : "Could not save the card to Anki."
      );
    } finally {
      setBusy(false);
    }
  }

  return (
    <div
      ref={dialogRef}
      className="capture-card"
      role="dialog"
      aria-labelledby="lesson-note-card-title"
      aria-modal="true"
    >
      <h2 id="lesson-note-card-title" className="capture-card__title">
        Card from note
      </h2>
      {error ? (
        <p className="capture-card__error" role="alert">
          {error}
        </p>
      ) : null}
      {committed ? (
        <p className="capture-card__success" role="status">
          {committed.duplicate
            ? "Card already exists in Anki for this note."
            : "Card saved to Anki."}
        </p>
      ) : draft ? (
        <div className="capture-card__fields">
          <label className="capture-card__label">
            Front
            <textarea
              className="capture-card__input"
              value={front}
              rows={3}
              disabled={busy}
              onChange={(event) => setFront(event.target.value)}
            />
          </label>
          <label className="capture-card__label">
            Back
            <textarea
              className="capture-card__input"
              value={back}
              rows={3}
              disabled={busy}
              onChange={(event) => setBack(event.target.value)}
            />
          </label>
        </div>
      ) : (
        <p className="capture-card__hint" role="status">
          Loading draft…
        </p>
      )}
      <div className="capture-card__actions">
        <button
          type="button"
          className="library-button-secondary"
          onClick={onClose}
        >
          Close
        </button>
        {!committed && draft ? (
          <button
            type="button"
            className="library-button-primary"
            disabled={busy || !front.trim()}
            onClick={() => void handleCommit()}
          >
            Save to Anki
          </button>
        ) : null}
      </div>
    </div>
  );
}
