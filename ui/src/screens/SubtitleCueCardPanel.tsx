import { useEffect, useRef, useState } from "react";

import type { CardDraft, CommittedCard } from "../api/captureTypes";
import type { SubtitleCue } from "../api/courseTypes";
import { useCaptureCommands } from "../capture/useCaptureCommands";

export type SubtitleCueCardPanelProps = {
  lessonId: string;
  cue: SubtitleCue;
  onClose: () => void;
};

export function SubtitleCueCardPanel({
  lessonId,
  cue,
  onClose,
}: SubtitleCueCardPanelProps) {
  const commands = useCaptureCommands();
  const dialogRef = useRef<HTMLDivElement>(null);
  const [draft, setDraft] = useState<CardDraft | null>(null);
  const [committed, setCommitted] = useState<CommittedCard | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    let cancelled = false;
    void commands
      .buildSubtitleCueDraft({
        lessonId,
        timestampMs: cue.start_ms,
        cueText: cue.text,
      })
      .then((next) => {
        if (!cancelled) {
          setDraft(next);
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
  }, [commands, cue.start_ms, cue.text, lessonId]);

  async function handleCommit() {
    if (!draft) {
      return;
    }
    setBusy(true);
    setError(null);
    try {
      const result = await commands.commitSubtitleCueCard({
        lessonId,
        timestampMs: cue.start_ms,
        cueText: cue.text,
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
      aria-labelledby="subtitle-card-title"
      aria-modal="true"
    >
      <h2 id="subtitle-card-title" className="capture-card__title">
        Card from subtitle
      </h2>
      {error ? (
        <p className="capture-card__error" role="alert">
          {error}
        </p>
      ) : null}
      {committed ? (
        <p className="capture-card__success" role="status">
          {committed.duplicate
            ? "Card already exists in Anki for this cue."
            : "Card saved to Anki."}
        </p>
      ) : draft ? (
        <div className="capture-card__fields">
          <label className="capture-card__label">
            Front
            <textarea
              className="capture-card__input"
              readOnly
              value={draft.front}
              rows={3}
            />
          </label>
          <label className="capture-card__label">
            Back
            <textarea
              className="capture-card__input"
              readOnly
              value={draft.back}
              rows={3}
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
            disabled={busy}
            onClick={() => void handleCommit()}
          >
            Save to Anki
          </button>
        ) : null}
      </div>
    </div>
  );
}
