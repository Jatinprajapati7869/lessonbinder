import { useRef } from "react";

import type { MediaChapter } from "../api/courseTypes";
import { formatDurationMs } from "../library/format";
import { handleCueListArrowKeys } from "./playerSidePanelKeyboard";

export type PlayerChapterListProps = {
  chapters: MediaChapter[];
  onChapterClick: (startMs: number) => void;
};

export function PlayerChapterList({
  chapters,
  onChapterClick,
}: PlayerChapterListProps) {
  const listRef = useRef<HTMLOListElement>(null);

  if (chapters.length === 0) {
    return null;
  }

  return (
    <aside className="player-chapters" aria-label="Chapters">
      <h2 className="player-chapters__heading">Chapters</h2>
      <ol
        ref={listRef}
        className="player-chapters__list"
        onKeyDown={(event) => handleCueListArrowKeys(event, listRef.current)}
      >
        {chapters.map((chapter) => (
          <li key={`${chapter.start_ms}-${chapter.title}`}>
            <button
              type="button"
              className="player-chapter"
              onClick={() => onChapterClick(chapter.start_ms)}
            >
              <span className="player-chapter__time">
                {formatDurationMs(chapter.start_ms)}
              </span>
              <span className="player-chapter__title">{chapter.title}</span>
            </button>
          </li>
        ))}
      </ol>
    </aside>
  );
}
