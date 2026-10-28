import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { ReviewCommandPort } from "./src/api/reviewCommands";
import type { DueSummary } from "./src/api/types";
import { ReviewCommandsProvider } from "./src/review/ReviewCommandsContext";
import { ReviewScreen } from "./src/screens/ReviewScreen";

const zeroDue: DueSummary = { new: 0, learning: 0, review: 0 };

function reviewCommands(
  overrides: Partial<ReviewCommandPort> = {}
): ReviewCommandPort {
  return {
    fetchNextCard: vi.fn().mockResolvedValue(null),
    fetchDueSummary: vi.fn().mockResolvedValue(zeroDue),
    fetchNoteBack: vi.fn(),
    readMedia: vi.fn(),
    answerCard: vi.fn(),
    ...overrides,
  };
}

function renderReview(commands: ReviewCommandPort) {
  return render(
    <ReviewCommandsProvider commands={commands}>
      <ReviewScreen
        onBackToLibrary={() => {}}
        onOpenInAnkiDesktop={() => {}}
        desktopAnkiAvailable={false}
      />
    </ReviewCommandsProvider>
  );
}

describe("ReviewScreen", () => {
  it("passes courseId to fetchNextCard when reviewing a single course", async () => {
    const fetchNextCard = vi.fn().mockResolvedValue(null);
    const commands = reviewCommands({
      fetchNextCard,
      fetchDueSummary: vi.fn().mockResolvedValue(zeroDue),
    });

    render(
      <ReviewCommandsProvider commands={commands}>
        <ReviewScreen
          courseId="course-physics"
          onBackToLibrary={() => {}}
          onOpenInAnkiDesktop={() => {}}
          desktopAnkiAvailable={false}
        />
      </ReviewCommandsProvider>
    );

    await waitFor(() => {
      expect(fetchNextCard).toHaveBeenCalledWith("course-physics");
    });
  });

  it("reveals the back and records Good via the command port", async () => {
    const user = userEvent.setup();
    const answerCard = vi.fn().mockResolvedValue(undefined);
    const commands = reviewCommands({
      fetchNextCard: vi.fn().mockResolvedValue({
        card_id: 42,
        note_id: 7,
        front_text: "What is 2+2?",
        screenshot_filename: null,
        buttons: { again: true, hard: true, good: true, easy: false },
      }),
      fetchNoteBack: vi.fn().mockResolvedValue("4"),
      answerCard,
    });

    renderReview(commands);

    expect(await screen.findByText("What is 2+2?")).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Show answer" }));

    expect(await screen.findByText("4")).toBeInTheDocument();
    expect(
      screen.queryByRole("button", { name: "Easy" })
    ).not.toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: /Good \(3\)/ })
    ).toBeInTheDocument();

    await user.keyboard("3");

    await waitFor(() => {
      expect(answerCard).toHaveBeenCalledOnce();
      expect(answerCard).toHaveBeenCalledWith(42, "good");
    });
  });

  it("reveals on space and leaves on Escape without answering", async () => {
    const user = userEvent.setup();
    const answerCard = vi.fn();
    const onBack = vi.fn();
    const commands = reviewCommands({
      fetchNextCard: vi.fn().mockResolvedValue({
        card_id: 1,
        note_id: 2,
        front_text: "Front",
        screenshot_filename: null,
        buttons: { again: true, hard: true, good: true, easy: true },
      }),
      fetchNoteBack: vi.fn().mockResolvedValue("Back"),
      answerCard,
    });

    render(
      <ReviewCommandsProvider commands={commands}>
        <ReviewScreen
          onBackToLibrary={onBack}
          onOpenInAnkiDesktop={() => {}}
          desktopAnkiAvailable={false}
        />
      </ReviewCommandsProvider>
    );

    await screen.findByText("Front");
    await user.keyboard(" ");
    expect(await screen.findByText("Back")).toBeInTheDocument();

    await user.keyboard("{Escape}");
    expect(onBack).toHaveBeenCalledOnce();
    expect(answerCard).not.toHaveBeenCalled();
  });

  it("shows unreadable queue with due counts when next card is missing but dues exist", async () => {
    renderReview(
      reviewCommands({
        fetchNextCard: vi.fn().mockResolvedValue(null),
        fetchDueSummary: vi.fn().mockResolvedValue({
          new: 2,
          learning: 1,
          review: 0,
        }),
      })
    );

    expect(
      await screen.findByText("The review queue could not be read.")
    ).toBeInTheDocument();
    expect(screen.getByLabelText("AuraVault due counts")).toHaveTextContent(
      "2"
    );
    expect(screen.getByLabelText("AuraVault due counts")).toHaveTextContent(
      "1"
    );
    expect(
      screen.queryByText("The review queue is empty.")
    ).not.toBeInTheDocument();
  });

  it("shows empty state when there is no next card and due counts are zero", async () => {
    renderReview(
      reviewCommands({
        fetchNextCard: vi.fn().mockResolvedValue(null),
        fetchDueSummary: vi.fn().mockResolvedValue(zeroDue),
      })
    );

    expect(
      await screen.findByText("The review queue is empty.")
    ).toBeInTheDocument();
    expect(
      screen.queryByText("The review queue could not be read.")
    ).not.toBeInTheDocument();
  });
});
