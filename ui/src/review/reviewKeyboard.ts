import type { ReviewAnswerButtons, ReviewEase } from "../api/reviewTypes";

export type ReviewKeyboardAction =
  | { type: "none" }
  | { type: "reveal" }
  | { type: "answer"; ease: ReviewEase }
  | { type: "leave" };

const DIGIT_TO_EASE: Record<string, ReviewEase> = {
  "1": "again",
  "2": "hard",
  "3": "good",
  "4": "easy",
};

/** Digit keys used to grade a revealed card (see `reviewEaseForDigit`). */
export const REVIEW_GRADE_DIGIT_KEYS: readonly string[] =
  Object.keys(DIGIT_TO_EASE);

export function reviewEaseForDigit(
  digit: string,
  buttons: ReviewAnswerButtons
): ReviewEase | null {
  const ease = DIGIT_TO_EASE[digit];
  if (!ease || !buttons[ease]) {
    return null;
  }
  return ease;
}

export function mapReviewKeyboardAction(
  key: string,
  context: {
    hasCard: boolean;
    revealed: boolean;
    answerBusy: boolean;
    buttons: ReviewAnswerButtons;
  }
): ReviewKeyboardAction {
  if (key === "Escape") {
    return { type: "leave" };
  }

  if (!context.hasCard || context.answerBusy) {
    return { type: "none" };
  }

  if (key === " " || key === "Spacebar") {
    if (!context.revealed) {
      return { type: "reveal" };
    }
    return { type: "none" };
  }

  if (context.revealed) {
    const ease = reviewEaseForDigit(key, context.buttons);
    if (ease) {
      return { type: "answer", ease };
    }
  }

  return { type: "none" };
}

export function shouldHandleReviewKeyboardEvent(
  target: EventTarget | null,
  key: string
): boolean {
  if (!(target instanceof HTMLElement)) {
    return true;
  }
  if (target.closest(".side-nav") || target.closest(".top-bar")) {
    return false;
  }
  const tag = target.tagName;
  if (tag === "TEXTAREA" || tag === "INPUT" || tag === "SELECT") {
    return false;
  }
  if (target.isContentEditable) {
    return false;
  }
  if (tag === "BUTTON") {
    if (key === " " || key === "Spacebar") {
      return false;
    }
    if (key === "1" || key === "2" || key === "3" || key === "4") {
      return false;
    }
  }
  return true;
}
