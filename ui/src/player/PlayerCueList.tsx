import { useRef } from "react";

import type { SubtitleCue } from "../api/courseTypes";
import { formatDurationMs } from "../library/format";
import { indexOfActiveCue } from "./cueSelection";
import { handleCueListArrowKeys } from "./playerSidePanelKeyboard";

export type PlayerCueListProps = {
  cues: SubtitleCue[];
  currentMs: number;
  onCueClick: (startMs: number) => void;
  onMakeCard?: (cue: SubtitleCue) => void;
};

export function PlayerCueList({
  cues,
  currentMs,
  onCueClick,
  onMakeCard,
}: PlayerCueListProps) {
  const listRef = useRef<HTMLOListElement>(null);

  if (cues.length === 0) {
    return null;
  }

  const activeIndex = indexOfActiveCue(cues, currentMs);

  return (
    <aside className="player-cues" aria-label="Subtitle cues">
      <ol
        ref={listRef}
        className="player-cues__list"
        onKeyDown={(event) => handleCueListArrowKeys(event, listRef.current)}
      >
        {cues.map((cue, index) => {
          const active = index === activeIndex;
          return (
            <li
              key={`${cue.start_ms}-${cue.end_ms}`}
              className="player-cue-row"
            >
              <button
                type="button"
                className={
                  active ? "player-cue player-cue--active" : "player-cue"
                }
                aria-current={active ? "true" : undefined}
                onClick={() => onCueClick(cue.start_ms)}
              >
                <span className="player-cue__time">
                  {formatDurationMs(cue.start_ms)}
                </span>
                <span className="player-cue__text">{cue.text}</span>
              </button>
              {active && onMakeCard ? (
                <button
                  type="button"
                  className="player-cue__make-card"
                  onClick={() => onMakeCard(cue)}
                >
                  Make card
                </button>
              ) : null}
            </li>
          );
        })}
      </ol>
    </aside>
  );
}
