import { useCallback, useEffect, useRef, useState } from "react";

import type { CardDraft, CommittedCard } from "../api/captureTypes";
import { cropPngFromDataUrl, type PixelRect } from "../capture/cropFrame";
import { useCaptureCommands } from "../capture/useCaptureCommands";

type DragState = {
  startX: number;
  startY: number;
  currentX: number;
  currentY: number;
};

export type CaptureCardPanelProps = {
  lessonId: string;
  lessonTitle: string;
  timestampMs: number;
  frameDataUrl: string;
  onClose: () => void;
  /** Test seam: run OCR for a fixed region without pointer dragging. */
  initialSelectionRect?: PixelRect;
};

function rectFromDrag(drag: DragState): PixelRect | null {
  const x1 = Math.min(drag.startX, drag.currentX);
  const y1 = Math.min(drag.startY, drag.currentY);
  const x2 = Math.max(drag.startX, drag.currentX);
  const y2 = Math.max(drag.startY, drag.currentY);
  const width = x2 - x1;
  const height = y2 - y1;
  if (width < 4 || height < 4) {
    return null;
  }
  return { x: x1, y: y1, width, height };
}

export function CaptureCardPanel({
  lessonId,
  lessonTitle,
  timestampMs,
  frameDataUrl,
  onClose,
  initialSelectionRect,
}: CaptureCardPanelProps) {
  const commands = useCaptureCommands();
  const cancelRef = useRef<HTMLButtonElement>(null);
  const dialogRef = useRef<HTMLDivElement>(null);
  const frameRef = useRef<HTMLDivElement>(null);
  const [drag, setDrag] = useState<DragState | null>(null);
  const [selection, setSelection] = useState<PixelRect | null>(null);
  const cropPngRef = useRef<Uint8Array | null>(null);
  const [draft, setDraft] = useState<CardDraft | null>(null);
  const [front, setFront] = useState("");
  const [back, setBack] = useState("");
  const [committed, setCommitted] = useState<CommittedCard | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [ocrHint, setOcrHint] = useState<string | null>(null);

  useEffect(() => {
    void commands.captureOcrStatus().then((status) => {
      if (!status.available) {
        setOcrHint(
          "Tesseract is not installed. Install Tesseract on your system to capture text from video frames. Images are not sent anywhere else."
        );
      }
    });
  }, [commands]);

  const runOcr = useCallback(
    async (rect: PixelRect) => {
      setBusy(true);
      setError(null);
      try {
        const png = await cropPngFromDataUrl(frameDataUrl, rect);
        cropPngRef.current = png;
        const next = await commands.buildCaptureDraft({
          lessonId,
          timestampMs,
          png,
        });
        setDraft(next);
        setFront(next.front);
        setBack(next.back);
      } catch (e) {
        const message =
          e instanceof Error ? e.message : "Could not run OCR on that region.";
        setError(message);
        setDraft(null);
      } finally {
        setBusy(false);
      }
    },
    [commands, frameDataUrl, lessonId, timestampMs]
  );

  useEffect(() => {
    if (!initialSelectionRect) {
      return;
    }
    setSelection(initialSelectionRect);
    void runOcr(initialSelectionRect);
  }, [initialSelectionRect, runOcr]);

  useEffect(() => {
    cancelRef.current?.focus();
    function onKeyDown(event: KeyboardEvent) {
      if (event.key === "Escape") {
        event.preventDefault();
        onClose();
        return;
      }
      const dialog = dialogRef.current;
      if (dialog) {
        trapFocusWithin(dialog, event);
      }
    }
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [onClose]);

  const applyDefaultRegion = useCallback(() => {
    const frame = frameRef.current;
    if (!frame || busy || draft) {
      return;
    }
    const rect: PixelRect = {
      x: Math.max(8, frame.clientWidth * 0.15),
      y: Math.max(8, frame.clientHeight * 0.2),
      width: Math.min(160, Math.max(80, frame.clientWidth * 0.5)),
      height: Math.min(120, Math.max(48, frame.clientHeight * 0.25)),
    };
    setSelection(rect);
    void runOcr(rect);
  }, [busy, draft, runOcr]);

  function pointerPosition(event: React.PointerEvent): {
    x: number;
    y: number;
  } {
    const frame = frameRef.current;
    if (!frame) {
      return { x: 0, y: 0 };
    }
    const bounds = frame.getBoundingClientRect();
    return {
      x: (event.clientX || 0) - bounds.left,
      y: (event.clientY || 0) - bounds.top,
    };
  }

  function handlePointerDown(event: React.PointerEvent) {
    if (busy || draft) {
      return;
    }
    const pos = pointerPosition(event);
    setDrag({
      startX: pos.x,
      startY: pos.y,
      currentX: pos.x,
      currentY: pos.y,
    });
    setSelection(null);
    setError(null);
    if (
      "setPointerCapture" in event.currentTarget &&
      typeof event.currentTarget.setPointerCapture === "function"
    ) {
      event.currentTarget.setPointerCapture(event.pointerId);
    }
  }

  function handlePointerMove(event: React.PointerEvent) {
    if (!drag || busy || draft) {
      return;
    }
    const pos = pointerPosition(event);
    setDrag({ ...drag, currentX: pos.x, currentY: pos.y });
  }

  function handlePointerUp(event: React.PointerEvent) {
    if (!drag || busy || draft) {
      return;
    }
    const pos = pointerPosition(event);
    const nextDrag = { ...drag, currentX: pos.x, currentY: pos.y };
    const rect = rectFromDrag(nextDrag);
    setDrag(null);
    if (!rect) {
      setError("Drag a larger region over the text you want to capture.");
      return;
    }
    setSelection(rect);
    void runOcr(rect);
  }

  function clearDrag() {
    setDrag(null);
  }

  function handleFrameKeyDown(event: React.KeyboardEvent) {
    if (event.key === "Enter" && !busy && !draft) {
      event.preventDefault();
      applyDefaultRegion();
    }
  }

  async function handleSave() {
    const png = cropPngRef.current;
    if (!png) {
      return;
    }
    setBusy(true);
    setError(null);
    try {
      const result = await commands.commitCaptureCard({
        lessonId,
        timestampMs,
        png,
        front,
        back,
      });
      setCommitted(result);
      if (result.duplicate) {
        setFront(result.front);
        setBack(result.back);
      }
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not save card.");
    } finally {
      setBusy(false);
    }
  }

  const activeRect = drag !== null ? rectFromDrag(drag) : selection;

  return (
    <div
      ref={dialogRef}
      className="capture-card"
      role="dialog"
      aria-modal="true"
      aria-labelledby="capture-card-title"
    >
      <header className="capture-card__header">
        <h2 id="capture-card-title" className="screen-title">
          Capture card
        </h2>
        <button
          ref={cancelRef}
          type="button"
          className="library-button-secondary"
          onClick={onClose}
        >
          Cancel
        </button>
      </header>

      <p className="capture-card__context">
        {lessonTitle} · {draft?.timestamp ?? formatTimestamp(timestampMs)}
      </p>

      {ocrHint ? (
        <p className="capture-card__hint" role="status">
          {ocrHint}
        </p>
      ) : null}
      {error ? (
        <p className="library-error" role="alert">
          {error}
        </p>
      ) : null}

      {committed?.duplicate ? (
        <p className="capture-card__duplicate" role="status">
          This card already exists in your Lessonbinder profile. Showing the saved
          note.
        </p>
      ) : null}

      {committed && !committed.duplicate ? (
        <p className="capture-card__success" role="status">
          Card saved to Anki.
        </p>
      ) : null}

      {!draft && !busy ? (
        <button
          type="button"
          className="library-button-secondary capture-card__select-region"
          onClick={() => frameRef.current?.focus()}
        >
          Select region
        </button>
      ) : null}

      <div className="capture-card__layout">
        <div
          ref={frameRef}
          className="capture-frame"
          data-testid="capture-frame"
          tabIndex={0}
          role="img"
          aria-label="Paused video frame. Drag to select text, or press Enter for a default region."
          onKeyDown={handleFrameKeyDown}
          onPointerDown={handlePointerDown}
          onPointerMove={handlePointerMove}
          onPointerUp={handlePointerUp}
          onPointerCancel={clearDrag}
          onLostPointerCapture={clearDrag}
        >
          <img
            className="capture-frame__image"
            src={frameDataUrl}
            alt="Paused video frame"
            draggable={false}
          />
          {!draft && !busy ? (
            <p className="capture-frame__hint">
              {selection
                ? "Running OCR…"
                : "Pause the video and drag to select text on screen."}
            </p>
          ) : null}
          {activeRect ? (
            <div
              className="capture-frame__selection"
              style={{
                left: activeRect.x,
                top: activeRect.y,
                width: activeRect.width,
                height: activeRect.height,
              }}
            />
          ) : null}
        </div>

        <div className="capture-card__editor">
          {draft || committed ? (
            <>
              <p className="capture-card__meta">
                {draft?.course ?? committed?.course}
              </p>
              <label className="capture-card__label" htmlFor="capture-front">
                Front
              </label>
              <textarea
                id="capture-front"
                className="capture-card__field"
                rows={4}
                value={front}
                onChange={(e) => setFront(e.target.value)}
                disabled={busy || Boolean(committed?.duplicate)}
              />
              <label className="capture-card__label" htmlFor="capture-back">
                Back
              </label>
              <textarea
                id="capture-back"
                className="capture-card__field"
                rows={4}
                value={back}
                onChange={(e) => setBack(e.target.value)}
                disabled={busy || Boolean(committed?.duplicate)}
              />
              <button
                type="button"
                className="library-button-primary"
                disabled={busy || !front.trim() || Boolean(committed)}
                onClick={() => void handleSave()}
              >
                Save card
              </button>
            </>
          ) : (
            <p className="library-empty__hint">
              Select a region on the frame to capture text.
            </p>
          )}
        </div>
      </div>
    </div>
  );
}

const FOCUSABLE =
  'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';

function trapFocusWithin(container: HTMLElement, event: KeyboardEvent): void {
  if (event.key !== "Tab") {
    return;
  }
  const nodes = Array.from(
    container.querySelectorAll<HTMLElement>(FOCUSABLE)
  ).filter((el) => el.offsetParent !== null || el === document.activeElement);
  if (nodes.length === 0) {
    return;
  }
  const first = nodes[0];
  const last = nodes[nodes.length - 1];
  const active = document.activeElement as HTMLElement | null;
  if (event.shiftKey) {
    if (active === first || !container.contains(active)) {
      event.preventDefault();
      last.focus();
    }
    return;
  }
  if (active === last) {
    event.preventDefault();
    first.focus();
  }
}

function formatTimestamp(timestampMs: number): string {
  const totalSecs = Math.max(0, Math.floor(timestampMs / 1000));
  const hours = Math.floor(totalSecs / 3600);
  const minutes = Math.floor((totalSecs % 3600) / 60);
  const seconds = totalSecs % 60;
  return `${hours.toString().padStart(2, "0")}:${minutes
    .toString()
    .padStart(2, "0")}:${seconds.toString().padStart(2, "0")}`;
}
