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

const PLAYER_ARROW_SEEK_KEY_CANDIDATES = ["ArrowLeft", "ArrowRight"] as const;

function playerArrowSeekKeys(): readonly string[] {
  return PLAYER_ARROW_SEEK_KEY_CANDIDATES.filter(
    (key) => playerArrowSeekDeltaMs(key) !== null
  );
}

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

/** Player and review shortcuts wired in the existing keyboard modules. */
export function buildImplementedPlayerAndReviewShortcuts(): readonly ImplementedShortcut[] {
  return [
    {
      scope: "player",
      keys: playerArrowSeekKeys(),
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

/** Settings row label for one or more `KeyboardEvent.key` values. */
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

/** Throws when `shortcuts` is not aligned with the keyboard modules. */
export function assertImplementedShortcutsMatchKeyModules(
  shortcuts: readonly ImplementedShortcut[]
): void {
  const expected = buildImplementedPlayerAndReviewShortcuts();
  if (shortcuts.length !== expected.length) {
    throw new Error(
      `shortcut list length ${shortcuts.length} !== ${expected.length} from key modules`
    );
  }
  for (let i = 0; i < expected.length; i += 1) {
    const row = shortcuts[i];
    const want = expected[i];
    if (row.scope !== want.scope || row.action !== want.action) {
      throw new Error(
        `shortcut row ${i}: expected ${want.scope} "${want.action}", got ${row.scope} "${row.action}"`
      );
    }
    if (
      row.keys.length !== want.keys.length ||
      row.keys.some((k, j) => k !== want.keys[j])
    ) {
      throw new Error(
        `shortcut row ${i} keys ${row.keys.join(",")} !== ${want.keys.join(",")} from key modules`
      );
    }
  }

  for (const row of shortcuts) {
    if (row.action.startsWith("Seek")) {
      if (row.keys.some((key) => playerArrowSeekDeltaMs(key) === null)) {
        throw new Error(`seek row includes non-seek key`);
      }
    }
    if (
      row.action === "Mark complete" &&
      row.keys[0] !== PLAYER_MARK_COMPLETE_KEY
    ) {
      throw new Error("mark complete key mismatch");
    }
    if (
      row.action === "Exit theater or zen" &&
      row.keys[0] !== PLAYER_EXIT_THEATER_OR_ZEN_KEY
    ) {
      throw new Error("exit theater/zen key mismatch");
    }
    if (row.scope === "review") {
      if (!REVIEW_GRADE_DIGIT_KEYS.includes(row.keys[0])) {
        throw new Error(
          `review grade key ${row.keys[0]} not in reviewKeyboard`
        );
      }
    }
  }
}
