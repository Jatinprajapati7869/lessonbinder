import { useCallback, useEffect, useRef, useState } from "react";

import type { DueSummary } from "../api/types";
import type { ReviewCard, ReviewEase } from "../api/reviewTypes";
import {
  mapReviewKeyboardAction,
  shouldHandleReviewKeyboardEvent,
} from "../review/reviewKeyboard";
import { useReviewCommands } from "../review/useReviewCommands";

export type ReviewScreenProps = {
  onBackToLibrary: () => void;
  onOpenInAnkiDesktop: () => void;
  desktopAnkiAvailable: boolean;
  desktopLaunchError?: string | null;
  desktopLaunchBusy?: boolean;
  onQueueChanged?: () => void;
  courseId?: string | null;
};

function easeLabel(ease: ReviewEase): string {
  switch (ease) {
    case "again":
      return "Again";
    case "hard":
      return "Hard";
    case "good":
      return "Good";
    case "easy":
      return "Easy";
    default: {
      const _exhaustive: never = ease;
      return _exhaustive;
    }
  }
}

function easeDigit(ease: ReviewEase): number {
  switch (ease) {
    case "again":
      return 1;
    case "hard":
      return 2;
    case "good":
      return 3;
    case "easy":
      return 4;
    default: {
      const _exhaustive: never = ease;
      return _exhaustive;
    }
  }
}

function auravaultDueTotal(due: DueSummary): number {
  return due.new + due.learning + due.review;
}

function ReviewDueCounts({ due }: { due: DueSummary }) {
  return (
    <div className="due-counts" aria-label="AuraVault due counts">
      <span className="due-count">
        <span className="due-count__value">{due.new}</span>
        <span className="due-count__label">New</span>
      </span>
      <span className="due-count">
        <span className="due-count__value">{due.learning}</span>
        <span className="due-count__label">Learning</span>
      </span>
      <span className="due-count">
        <span className="due-count__value">{due.review}</span>
        <span className="due-count__label">Review</span>
      </span>
    </div>
  );
}

export function ReviewScreen({
  onBackToLibrary,
  onOpenInAnkiDesktop,
  desktopAnkiAvailable,
  desktopLaunchError = null,
  desktopLaunchBusy = false,
  onQueueChanged,
  courseId = null,
}: ReviewScreenProps) {
  const commands = useReviewCommands();
  const [card, setCard] = useState<ReviewCard | null>(null);
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [revealed, setRevealed] = useState(false);
  const [backText, setBackText] = useState<string | null>(null);
  const [screenshotUrl, setScreenshotUrl] = useState<string | null>(null);
  const screenshotUrlRef = useRef<string | null>(null);
  const [answerBusy, setAnswerBusy] = useState(false);
  const [dueSummary, setDueSummary] = useState<DueSummary | null>(null);

  const clearScreenshot = useCallback(() => {
    if (screenshotUrlRef.current) {
      URL.revokeObjectURL(screenshotUrlRef.current);
      screenshotUrlRef.current = null;
      setScreenshotUrl(null);
    }
  }, []);

  const loadCard = useCallback(async () => {
    setLoading(true);
    setLoadError(null);
    setRevealed(false);
    setBackText(null);
    clearScreenshot();
    try {
      const [next, due] = await Promise.all([
        commands.fetchNextCard(courseId),
        commands.fetchDueSummary(),
      ]);
      setDueSummary(due);
      setCard(next);
      if (next?.screenshot_filename) {
        const bytes = await commands.readMedia(next.screenshot_filename);
        const blob = new Blob([bytes], { type: "image/png" });
        const url = URL.createObjectURL(blob);
        screenshotUrlRef.current = url;
        setScreenshotUrl(url);
      }
    } catch (e) {
      setCard(null);
      setDueSummary(null);
      setLoadError(
        e instanceof Error ? e.message : "Could not load the review queue."
      );
    } finally {
      setLoading(false);
    }
  }, [clearScreenshot, commands, courseId]);

  useEffect(() => {
    void loadCard();
    return () => {
      if (screenshotUrlRef.current) {
        URL.revokeObjectURL(screenshotUrlRef.current);
      }
    };
  }, [loadCard]);

  async function handleReveal() {
    if (!card || revealed) {
      return;
    }
    try {
      const back = await commands.fetchNoteBack(card.note_id);
      setBackText(back);
      setRevealed(true);
    } catch (e) {
      setLoadError(
        e instanceof Error ? e.message : "Could not load the card back."
      );
    }
  }

  async function handleAnswer(ease: ReviewEase) {
    if (!card || !revealed || answerBusy) {
      return;
    }
    setAnswerBusy(true);
    setLoadError(null);
    try {
      await commands.answerCard(card.card_id, ease);
      onQueueChanged?.();
      await loadCard();
    } catch (e) {
      setLoadError(
        e instanceof Error ? e.message : "Could not record your answer."
      );
    } finally {
      setAnswerBusy(false);
    }
  }

  const offered: ReviewEase[] = card
    ? (["again", "hard", "good", "easy"] as const).filter(
        (ease) => card.buttons[ease]
      )
    : [];

  const handleRevealRef = useRef(handleReveal);
  const handleAnswerRef = useRef(handleAnswer);

  useEffect(() => {
    handleRevealRef.current = handleReveal;
    handleAnswerRef.current = handleAnswer;
  });

  useEffect(() => {
    function onKeyDown(event: KeyboardEvent) {
      if (!shouldHandleReviewKeyboardEvent(event.target, event.key)) {
        return;
      }
      const action = mapReviewKeyboardAction(event.key, {
        hasCard: card !== null && !loading,
        revealed,
        answerBusy,
        buttons: card?.buttons ?? {
          again: false,
          hard: false,
          good: false,
          easy: false,
        },
      });
      if (action.type === "none") {
        return;
      }
      event.preventDefault();
      if (action.type === "leave") {
        onBackToLibrary();
        return;
      }
      if (action.type === "reveal") {
        void handleRevealRef.current();
        return;
      }
      void handleAnswerRef.current(action.ease);
    }
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [answerBusy, card, loading, onBackToLibrary, revealed]);

  const queueUnreadable =
    !loading &&
    !loadError &&
    !card &&
    dueSummary !== null &&
    auravaultDueTotal(dueSummary) > 0;
  const queueEmpty =
    !loading &&
    !loadError &&
    !card &&
    dueSummary !== null &&
    auravaultDueTotal(dueSummary) === 0;

  return (
    <div className="review-screen">
      <div className="review-screen__fallback">
        <button
          type="button"
          className="review-fallback-button"
          disabled={!desktopAnkiAvailable || desktopLaunchBusy}
          onClick={() => onOpenInAnkiDesktop()}
        >
          Open in Anki Desktop
        </button>
        {!desktopAnkiAvailable ? (
          <p className="review-fallback-hint" role="status">
            Anki Desktop is not available. Review here or install Anki and set
            the path in Settings.
          </p>
        ) : null}
        {desktopLaunchError ? (
          <p className="review-error" role="alert">
            {desktopLaunchError}
          </p>
        ) : null}
      </div>

      {loading ? (
        <p className="review-status" role="status">
          Loading review queue…
        </p>
      ) : null}
      {loadError ? (
        <p className="review-error" role="alert">
          {loadError}
        </p>
      ) : null}

      {!loading && !card && !loadError ? (
        queueUnreadable ? (
          <div className="review-queue-unreadable" role="alert">
            <p className="review-queue-unreadable__message">
              The review queue could not be read.
            </p>
            <ReviewDueCounts due={dueSummary!} />
            <button
              type="button"
              className="review-back-button"
              onClick={onBackToLibrary}
            >
              Back to Library
            </button>
          </div>
        ) : queueEmpty ? (
          <div className="review-empty">
            <p className="review-empty__message">The review queue is empty.</p>
            <button
              type="button"
              className="review-back-button"
              onClick={onBackToLibrary}
            >
              Back to Library
            </button>
          </div>
        ) : null
      ) : null}

      {card && !loading ? (
        <article className="review-card" aria-label="Review card">
          <section className="review-card__front" aria-label="Front">
            <p className="review-card__text">{card.front_text}</p>
            {screenshotUrl ? (
              <img
                className="review-card__screenshot"
                src={screenshotUrl}
                alt="Card screenshot"
              />
            ) : null}
          </section>

          {!revealed ? (
            <button
              type="button"
              className="review-reveal-button"
              onClick={() => void handleReveal()}
            >
              Show answer
            </button>
          ) : (
            <section className="review-card__back" aria-label="Back">
              <p className="review-card__text">{backText}</p>
            </section>
          )}

          {revealed ? (
            <div className="review-grades" role="group" aria-label="Grade card">
              {offered.map((ease) => (
                <button
                  key={ease}
                  type="button"
                  className={`review-grade review-grade--${ease}`}
                  disabled={answerBusy}
                  onClick={() => void handleAnswer(ease)}
                >
                  {easeLabel(ease)} ({easeDigit(ease)})
                </button>
              ))}
            </div>
          ) : null}
        </article>
      ) : null}
    </div>
  );
}
