import { useEffect, useRef, useState } from "react";

import type { CardDraft, CommittedCard } from "../api/captureTypes";
import { useCaptureCommands } from "../capture/useCaptureCommands";

export type AudioClipCardPanelProps = {
  lessonId: string;
  playheadMs: number;
  onClose: () => void;
};

export function AudioClipCardPanel({
  lessonId,
  playheadMs,
  onClose,
}: AudioClipCardPanelProps) {
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
      .buildAudioClipDraft({ lessonId, playheadMs })
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
            e instanceof Error
              ? e.message
              : "Could not build an audio card draft."
          );
        }
      });
    return () => {
      cancelled = true;
    };
  }, [commands, lessonId, playheadMs]);

  async function handleCommit() {
    if (!draft) {
      return;
    }
    setBusy(true);
    setError(null);
    try {
      const result = await commands.commitAudioClipCard({
        lessonId,
        playheadMs,
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
      aria-labelledby="audio-clip-card-title"
      aria-modal="true"
    >
      <h2 id="audio-clip-card-title" className="capture-card__title">
        Add audio to card
      </h2>
      {error ? (
        <p className="capture-card__error" role="alert">
          {error}
        </p>
      ) : null}
      {committed ? (
        <p className="capture-card__success" role="status">
          {committed.duplicate
            ? "Card already exists in Anki for this clip."
            : "Card saved to Anki with audio clip."}
        </p>
      ) : draft ? (
        <div className="capture-card__fields">
          <label className="capture-card__label">
            Front
            <textarea
              className="capture-card__input"
              value={front}
              rows={3}
              onChange={(e) => setFront(e.target.value)}
              disabled={busy}
            />
          </label>
          <label className="capture-card__label">
            Back
            <textarea
              className="capture-card__input"
              value={back}
              rows={3}
              onChange={(e) => setBack(e.target.value)}
              disabled={busy}
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
