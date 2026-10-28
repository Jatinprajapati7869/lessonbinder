import { useCallback, useEffect, useState } from "react";

import type { LessonCardLink } from "../api/courseTypes";
import { useCourseCommands } from "../course/useCourseCommands";
import { formatDurationMs } from "../library/format";

export type LessonCardLinksListProps = {
  lessonId: string;
};

export function LessonCardLinksList({ lessonId }: LessonCardLinksListProps) {
  const commands = useCourseCommands();
  const [links, setLinks] = useState<LessonCardLink[] | null>(null);
  const [selectedNoteId, setSelectedNoteId] = useState<number | null>(null);
  const [unlinkConfirmOpen, setUnlinkConfirmOpen] = useState(false);
  const [unlinkBusy, setUnlinkBusy] = useState(false);

  const refreshLinks = useCallback(async () => {
    const rows = await commands.listLessonCardLinks(lessonId);
    setLinks(rows);
    setSelectedNoteId((current) =>
      current !== null && rows.some((row) => row.anki_note_id === current)
        ? current
        : null
    );
  }, [commands, lessonId]);

  useEffect(() => {
    let cancelled = false;
    void commands.listLessonCardLinks(lessonId).then((rows) => {
      if (!cancelled) {
        setLinks(rows);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [commands, lessonId]);

  async function handleConfirmUnlink() {
    if (selectedNoteId === null) {
      return;
    }
    setUnlinkBusy(true);
    try {
      await commands.unlinkLessonCard(lessonId, selectedNoteId);
      setUnlinkConfirmOpen(false);
      await refreshLinks();
    } finally {
      setUnlinkBusy(false);
    }
  }

  return (
    <section
      className="player-cards"
      aria-label="Cards"
      data-testid="lesson-card-links"
    >
      <h2 className="player-cards__title">Cards</h2>
      {links === null ? (
        <p className="library-empty__hint" role="status">
          Loading cards…
        </p>
      ) : links.length === 0 ? (
        <p className="player-cards__empty" role="status">
          No cards from this lesson.
        </p>
      ) : (
        <>
          <ul className="player-cards__list">
            {links.map((link) => (
              <li key={`${link.anki_note_id}-${link.timestamp_ms}`}>
                <button
                  type="button"
                  className="player-cards__row"
                  aria-pressed={selectedNoteId === link.anki_note_id}
                  onClick={() => {
                    setSelectedNoteId(link.anki_note_id);
                  }}
                >
                  <span className="player-cards__time">
                    {formatDurationMs(link.timestamp_ms)}
                  </span>
                  <span className="player-cards__preview">
                    {link.front_preview}
                  </span>
                </button>
              </li>
            ))}
          </ul>
          {selectedNoteId !== null ? (
            <button
              type="button"
              className="library-button-secondary player-cards__unlink"
              onClick={() => setUnlinkConfirmOpen(true)}
            >
              Unlink
            </button>
          ) : null}
        </>
      )}

      {unlinkConfirmOpen ? (
        <div
          className="player-cards__confirm"
          role="alertdialog"
          aria-labelledby="player-cards-unlink-title"
        >
          <p
            id="player-cards-unlink-title"
            className="player-cards__confirm-line"
          >
            The Anki note is kept.
          </p>
          <div className="player-cards__confirm-actions">
            <button
              type="button"
              className="library-button-secondary"
              disabled={unlinkBusy}
              onClick={() => setUnlinkConfirmOpen(false)}
            >
              Cancel
            </button>
            <button
              type="button"
              className="study-button"
              disabled={unlinkBusy}
              onClick={() => void handleConfirmUnlink()}
            >
              Unlink card
            </button>
          </div>
        </div>
      ) : null}
    </section>
  );
}
