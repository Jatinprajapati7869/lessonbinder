import { describe, expect, it } from "vitest";

import type { ReviewAnswerButtons } from "./src/api/reviewTypes";
import {
  mapReviewKeyboardAction,
  reviewEaseForDigit,
  shouldHandleReviewKeyboardEvent,
} from "./src/review/reviewKeyboard";

const allButtons: ReviewAnswerButtons = {
  again: true,
  hard: true,
  good: true,
  easy: true,
};

const noEasy: ReviewAnswerButtons = {
  again: true,
  hard: true,
  good: true,
  easy: false,
};

describe("reviewEaseForDigit", () => {
  it("maps 1–4 to eases when offered", () => {
    expect(reviewEaseForDigit("1", allButtons)).toBe("again");
    expect(reviewEaseForDigit("3", allButtons)).toBe("good");
    expect(reviewEaseForDigit("4", noEasy)).toBeNull();
  });
});

describe("mapReviewKeyboardAction", () => {
  it("reveals on space before the back is shown", () => {
    expect(
      mapReviewKeyboardAction(" ", {
        hasCard: true,
        revealed: false,
        answerBusy: false,
        buttons: allButtons,
      })
    ).toEqual({ type: "reveal" });
  });

  it("does not reveal on space after the back is shown", () => {
    expect(
      mapReviewKeyboardAction(" ", {
        hasCard: true,
        revealed: true,
        answerBusy: false,
        buttons: allButtons,
      })
    ).toEqual({ type: "none" });
  });

  it("answers with 1–4 only when revealed and the button is legal", () => {
    expect(
      mapReviewKeyboardAction("3", {
        hasCard: true,
        revealed: true,
        answerBusy: false,
        buttons: allButtons,
      })
    ).toEqual({ type: "answer", ease: "good" });

    expect(
      mapReviewKeyboardAction("4", {
        hasCard: true,
        revealed: true,
        answerBusy: false,
        buttons: noEasy,
      })
    ).toEqual({ type: "none" });
  });

  it("ignores grade keys while answering", () => {
    expect(
      mapReviewKeyboardAction("2", {
        hasCard: true,
        revealed: true,
        answerBusy: true,
        buttons: allButtons,
      })
    ).toEqual({ type: "none" });
  });

  it("leaves the session on Escape without grading", () => {
    expect(
      mapReviewKeyboardAction("Escape", {
        hasCard: true,
        revealed: false,
        answerBusy: false,
        buttons: allButtons,
      })
    ).toEqual({ type: "leave" });
  });
});

describe("shouldHandleReviewKeyboardEvent", () => {
  it("ignores space and digits when focus is on a button", () => {
    const button = document.createElement("button");
    expect(shouldHandleReviewKeyboardEvent(button, " ")).toBe(false);
    expect(shouldHandleReviewKeyboardEvent(button, "3")).toBe(false);
  });

  it("allows shortcuts when focus is on the document body", () => {
    expect(shouldHandleReviewKeyboardEvent(document.body, " ")).toBe(true);
    expect(shouldHandleReviewKeyboardEvent(document.body, "3")).toBe(true);
  });

  it("ignores shortcuts when focus is in app chrome", () => {
    const nav = document.createElement("nav");
    nav.className = "side-nav";
    const button = document.createElement("button");
    nav.appendChild(button);
    document.body.appendChild(nav);

    expect(shouldHandleReviewKeyboardEvent(button, "Escape")).toBe(false);
    expect(shouldHandleReviewKeyboardEvent(button, "3")).toBe(false);

    nav.remove();
  });
});
