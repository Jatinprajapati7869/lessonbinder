import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { CaptureCommandPort } from "./src/api/captureCommands";
import type { CardDraft } from "./src/api/captureTypes";
import { captureCommandStubs } from "./src/test/captureCommandStubs";
import { CaptureCommandsProvider } from "./src/capture/CaptureCommandsContext";
import { SubtitleCueCardPanel } from "./src/screens/SubtitleCueCardPanel";

const draft: CardDraft = {
  course: "Linear Algebra",
  lesson_title: "Intro clip",
  front: "Definition of a vector space",
  back: "Intro clip\nLinear Algebra\n00:01:23",
  source_label: "subtitle",
  timestamp: "00:01:23",
};

describe("SubtitleCueCardPanel", () => {
  it("builds the draft and commits through the capture command port", async () => {
    const user = userEvent.setup();
    const buildSubtitleCueDraft = vi.fn().mockResolvedValue(draft);
    const commitSubtitleCueCard = vi.fn().mockResolvedValue({
      duplicate: false,
      note_id: 9,
      ...draft,
      lesson_title: draft.lesson_title,
    });
    const commands: CaptureCommandPort = {
      ...captureCommandStubs,
      buildSubtitleCueDraft,
      commitSubtitleCueCard,
    };

    render(
      <CaptureCommandsProvider commands={commands}>
        <SubtitleCueCardPanel
          lessonId="lesson-1"
          cue={{ start_ms: 83_000, end_ms: 90_000, text: draft.front }}
          onClose={() => {}}
        />
      </CaptureCommandsProvider>
    );

    await waitFor(() => {
      expect(buildSubtitleCueDraft).toHaveBeenCalledWith({
        lessonId: "lesson-1",
        timestampMs: 83_000,
        cueText: draft.front,
      });
    });

    expect(screen.getByDisplayValue(draft.front)).toBeInTheDocument();
    expect(screen.getByLabelText("Back")).toHaveValue(draft.back);

    await user.click(screen.getByRole("button", { name: "Save to Anki" }));

    await waitFor(() => {
      expect(commitSubtitleCueCard).toHaveBeenCalledWith({
        lessonId: "lesson-1",
        timestampMs: 83_000,
        cueText: draft.front,
      });
    });
  });

  it("surfaces empty cue rejection from the command layer", async () => {
    const commands: CaptureCommandPort = {
      ...captureCommandStubs,
      buildSubtitleCueDraft: vi
        .fn()
        .mockRejectedValue(new Error("card front must not be empty")),
    };

    render(
      <CaptureCommandsProvider commands={commands}>
        <SubtitleCueCardPanel
          lessonId="lesson-1"
          cue={{ start_ms: 0, end_ms: 1_000, text: "   " }}
          onClose={() => {}}
        />
      </CaptureCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole("alert")).toHaveTextContent(
        /card front must not be empty/i
      );
    });
  });
});
