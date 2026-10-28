import type { KeyboardEvent } from "react";

import { handleTableArrowKeys } from "../keyboard/tableListNavigation";

export function handleCueListArrowKeys(
  event: KeyboardEvent,
  container: HTMLElement | null
): void {
  handleTableArrowKeys(
    event,
    container,
    ".player-cue-row",
    "button.player-cue"
  );
}

export function handleLessonNotesArrowKeys(
  event: KeyboardEvent,
  container: HTMLElement | null
): void {
  handleTableArrowKeys(event, container, "li.lesson-notes__item", "button");
}
