import type { ReviewAnswerButtons } from "../api/reviewTypes";
import { PLAYER_MARK_COMPLETE_KEY } from "../player/playerCompleteKeyboard";
import { PLAYER_EXIT_THEATER_OR_ZEN_KEY } from "../player/playerModeKeyboard";
import {
  PLAYER_SEEK_STEP_MS,
  playerArrowSeekDeltaMs,
} from "../player/playerSeekKeyboard";
import {
  REVIEW_GRADE_DIGIT_KEYS,
  reviewEaseForDigit,
} from "../review/reviewKeyboard";

export type ShortcutScope = "player" | "review";

export type ImplementedShortcut = {
  scope: ShortcutScope;
  /** Values of `KeyboardEvent.key` for this shortcut. */
  keys: readonly string[];
  action: string;
};

const PLAYER_ARROW_SEEK_KEYS = ["ArrowLeft", "ArrowRight"] as const;

const ALL_REVIEW_BUTTONS: ReviewAnswerButtons = {
  again: true,
  hard: true,
  good: true,
  easy: true,
};

function reviewGradeShortcutRows(): ImplementedShortcut[] {
  return REVIEW_GRADE_DIGIT_KEYS.flatMap((digit) => {
    const ease = reviewEaseForDigit(digit, ALL_REVIEW_BUTTONS);
    if (!ease) {
      return [];
    }
    return [
      {
        scope: "review" as const,
        keys: [digit],
        action: `Grade: ${ease}`,
      },
    ];
  });
}

export function buildImplementedPlayerAndReviewShortcuts(): readonly ImplementedShortcut[] {
  return [
    {
      scope: "player",
      keys: PLAYER_ARROW_SEEK_KEYS.filter(
        (key) => playerArrowSeekDeltaMs(key) !== null
      ),
      action: `Seek ${PLAYER_SEEK_STEP_MS / 1000} seconds`,
    },
    {
      scope: "player",
      keys: [PLAYER_MARK_COMPLETE_KEY],
      action: "Mark complete",
    },
    {
      scope: "player",
      keys: [PLAYER_EXIT_THEATER_OR_ZEN_KEY],
      action: "Exit theater or zen",
    },
    ...reviewGradeShortcutRows(),
  ];
}

export const IMPLEMENTED_PLAYER_AND_REVIEW_SHORTCUTS =
  buildImplementedPlayerAndReviewShortcuts();

const SHORTCUT_KEY_DISPLAY: Record<string, string> = {
  ArrowLeft: "←",
  ArrowRight: "→",
  Escape: "Escape",
};

export function formatShortcutKeysForSettings(keys: readonly string[]): string {
  return keys
    .map((key) => {
      const mapped = SHORTCUT_KEY_DISPLAY[key];
      if (mapped) {
        return mapped;
      }
      if (key.length === 1) {
        return key.toUpperCase();
      }
      return key;
    })
    .join(" / ");
}

export function assertImplementedShortcutsMatchKeyModules(
  shortcuts: readonly ImplementedShortcut[]
): void {
  const expected = buildImplementedPlayerAndReviewShortcuts();
  if (JSON.stringify(shortcuts) !== JSON.stringify(expected)) {
    throw new Error("implemented shortcuts drifted from keyboard modules");
  }
}
