import { useCallback, useEffect, useRef, useState } from "react";

import type { LessonRow } from "../api/courseTypes";
import { useCourseCommands } from "../course/useCourseCommands";
import {
  charOffsetFromScroll,
  pdfPageFromPosition,
  pdfSrcWithPage,
  scrollToCharOffset,
} from "../lesson/documentPosition";
import {
  isDocumentLessonPath,
  isMarkdownLessonPath,
} from "../lesson/lessonDocument";
import { LessonCardLinksList } from "../lesson/LessonCardLinksList";
import { LessonMarkdownBody } from "../lesson/lessonMarkdown";
import { LessonNotesColumn } from "../lesson/LessonNotesColumn";

export const DOCUMENT_POSITION_SAVE_MS = 1500;

export type DocumentLessonScreenProps = {
  lesson: LessonRow;
  onBack: () => void;
};

export function DocumentLessonScreen({
  lesson,
  onBack,
}: DocumentLessonScreenProps) {
  const commands = useCourseCommands();
  const scrollRef = useRef<HTMLDivElement>(null);
  const [docUrl, setDocUrl] = useState<string | null>(null);
  const [textContent, setTextContent] = useState<string | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [pdfPage, setPdfPage] = useState(
    pdfPageFromPosition(lesson.document_position)
  );
  const [textRestored, setTextRestored] = useState(false);

  const path = lesson.local_media_path;
  const isPdf = path?.toLowerCase().endsWith(".pdf") ?? false;
  const isMarkdown = isMarkdownLessonPath(path);

  const persistPosition = useCallback(
    (position: number) => {
      if (position === lesson.document_position) {
        return;
      }
      void commands.saveDocumentPosition(lesson.lesson_id, position);
    },
    [commands, lesson.document_position, lesson.lesson_id]
  );

  useEffect(() => {
    if (!path || !isDocumentLessonPath(path)) {
      return;
    }
    let cancelled = false;
    void commands.mediaUrlForPath(path).then(async (url) => {
      if (cancelled) {
        return;
      }
      if (isPdf) {
        setDocUrl(url);
        return;
      }
      try {
        const response = await fetch(url);
        const text = await response.text();
        if (!cancelled) {
          setTextContent(text);
        }
      } catch {
        if (!cancelled) {
          setLoadError("Could not load document.");
        }
      }
    });
    return () => {
      cancelled = true;
    };
  }, [commands, path, isPdf]);

  useEffect(() => {
    if (isPdf || textContent === null) {
      return;
    }
    const scrollEl = scrollRef.current;
    if (!scrollEl || textContent === null) {
      return;
    }
    scrollToCharOffset(scrollEl, lesson.document_position, textContent.length);
    setTextRestored(true);
  }, [isPdf, lesson.document_position, textContent]);

  useEffect(() => {
    if (isPdf || !textRestored || textContent === null) {
      return;
    }
    const scrollEl = scrollRef.current;
    if (!scrollEl) {
      return;
    }
    const onScroll = () => {
      persistPosition(charOffsetFromScroll(scrollEl, textContent.length));
    };
    scrollEl.addEventListener("scroll", onScroll, { passive: true });
    return () => scrollEl.removeEventListener("scroll", onScroll);
  }, [isPdf, persistPosition, textContent, textRestored]);

  useEffect(() => {
    if (!isPdf) {
      return;
    }
    const id = window.setInterval(() => {
      persistPosition(pdfPage);
    }, DOCUMENT_POSITION_SAVE_MS);
    return () => window.clearInterval(id);
  }, [isPdf, pdfPage, persistPosition]);

  async function handleMarkComplete() {
    await commands.markLessonComplete(lesson.lesson_id);
    onBack();
  }

  const iframeSrc = docUrl && isPdf ? pdfSrcWithPage(docUrl, pdfPage) : null;

  return (
    <div className="player-screen document-lesson">
      <button
        type="button"
        className="library-button-secondary"
        onClick={onBack}
      >
        Back to course
      </button>
      <h1 className="screen-title">{lesson.title}</h1>
      <div className="player-main">
        <div className="document-lesson__content">
          {loadError ? (
            <p className="library-error" role="alert">
              {loadError}
            </p>
          ) : null}
          {isPdf ? (
            <div className="document-lesson__pdf">
              {path ? (
                <p
                  className="document-lesson__path"
                  data-testid="document-path"
                >
                  {path}
                </p>
              ) : null}
              <label className="document-lesson__page-label">
                Page
                <input
                  type="number"
                  className="document-lesson__page-input"
                  min={1}
                  value={pdfPage}
                  aria-label="PDF page"
                  onChange={(event) => {
                    const next = Number.parseInt(event.target.value, 10);
                    if (!Number.isNaN(next) && next >= 1) {
                      setPdfPage(next);
                    }
                  }}
                />
              </label>
              {iframeSrc ? (
                <iframe
                  className="document-lesson__frame"
                  src={iframeSrc}
                  title={lesson.title}
                  data-testid="document-pdf-frame"
                />
              ) : (
                <p className="library-empty__hint">Loading document…</p>
              )}
            </div>
          ) : null}
          {!isPdf && textContent !== null ? (
            <div
              ref={scrollRef}
              className="document-lesson__scroll"
              data-testid="document-lesson-scroll"
            >
              {isMarkdown ? (
                <LessonMarkdownBody source={textContent} />
              ) : (
                <pre
                  className="document-lesson__text"
                  data-testid="document-text"
                >
                  {textContent}
                </pre>
              )}
            </div>
          ) : null}
          {!isPdf && !loadError && textContent === null ? (
            <p className="library-empty__hint">Loading document…</p>
          ) : null}
        </div>
        <LessonNotesColumn lessonId={lesson.lesson_id} />
      </div>
      <LessonCardLinksList lessonId={lesson.lesson_id} />
      <div className="player-bar">
        <button
          type="button"
          className="library-button-secondary"
          onClick={() => void handleMarkComplete()}
        >
          Mark complete
        </button>
      </div>
    </div>
  );
}
