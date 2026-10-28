import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { CaptureCommandPort } from "./src/api/captureCommands";
import type { CardDraft, CommittedCard } from "./src/api/captureTypes";
import { captureCommandStubs } from "./src/test/captureCommandStubs";
import { CaptureCommandsProvider } from "./src/capture/CaptureCommandsContext";
import { CaptureCardPanel } from "./src/screens/CaptureCardPanel";

vi.mock("./src/capture/cropFrame", () => ({
  cropPngFromDataUrl: vi
    .fn()
    .mockResolvedValue(new Uint8Array([137, 80, 78, 71])),
}));

const frameDataUrl =
  "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";

const testSelection = { x: 8, y: 8, width: 80, height: 48 };

const ocrDraft: CardDraft = {
  course: "Linear Algebra",
  lesson_title: "Intro",
  front: "What is a matrix?",
  back: "",
  source_label: "lecture.mp4",
  timestamp: "00:01:23",
};

describe("CaptureCardPanel", () => {
  it("shows tesseract missing when OCR is unavailable", async () => {
    const buildCaptureDraft = vi
      .fn()
      .mockRejectedValue(
        new Error(
          "Tesseract is not installed. Install Tesseract on your system to capture text from video frames. Images are not sent anywhere else."
        )
      );
    const commands: CaptureCommandPort = {
      ...captureCommandStubs,
      captureOcrStatus: vi.fn().mockResolvedValue({ available: false }),
      buildCaptureDraft,
    };

    render(
      <CaptureCommandsProvider commands={commands}>
        <CaptureCardPanel
          lessonId="lesson-1"
          lessonTitle="Intro"
          timestampMs={83_000}
          frameDataUrl={frameDataUrl}
          initialSelectionRect={testSelection}
          onClose={() => {}}
        />
        ,
      </CaptureCommandsProvider>
    );

    await waitFor(() => {
      expect(buildCaptureDraft).toHaveBeenCalled();
    });
    expect(screen.getByRole("alert")).toHaveTextContent(
      /Tesseract is not installed/i
    );
  });

  it("escape closes the dialog", async () => {
    const onClose = vi.fn();
    const commands: CaptureCommandPort = {
      ...captureCommandStubs,
      captureOcrStatus: vi.fn().mockResolvedValue({ available: true }),
    };

    render(
      <CaptureCommandsProvider commands={commands}>
        <CaptureCardPanel
          lessonId="lesson-1"
          lessonTitle="Intro"
          timestampMs={83_000}
          frameDataUrl={frameDataUrl}
          onClose={onClose}
        />
      </CaptureCommandsProvider>
    );

    const user = userEvent.setup();
    await user.keyboard("{Escape}");
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it("duplicate commit shows the existing card instead of inserting", async () => {
    const user = userEvent.setup();
    const duplicateResult: CommittedCard = {
      duplicate: true,
      note_id: 99,
      course: "Linear Algebra",
      lesson_title: "Intro",
      front: "What is a matrix?",
      back: "Saved definition",
      source_label: "lecture.mp4",
      timestamp: "00:01:23",
    };
    const buildCaptureDraft = vi.fn().mockResolvedValue(ocrDraft);
    const commitCaptureCard = vi.fn().mockResolvedValue(duplicateResult);
    const commands: CaptureCommandPort = {
      ...captureCommandStubs,
      captureOcrStatus: vi.fn().mockResolvedValue({ available: true }),
      buildCaptureDraft,
      commitCaptureCard,
    };

    render(
      <CaptureCommandsProvider commands={commands}>
        <CaptureCardPanel
          lessonId="lesson-1"
          lessonTitle="Intro"
          timestampMs={83_000}
          frameDataUrl={frameDataUrl}
          initialSelectionRect={testSelection}
          onClose={() => {}}
        />
        ,
      </CaptureCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByLabelText("Front")).toHaveValue("What is a matrix?");
    });

    await user.type(screen.getByLabelText("Back"), "My draft back");
    await user.click(screen.getByRole("button", { name: "Save card" }));

    await waitFor(() => {
      expect(commitCaptureCard).toHaveBeenCalledWith(
        expect.objectContaining({
          lessonId: "lesson-1",
          timestampMs: 83_000,
          front: "What is a matrix?",
          back: "My draft back",
        })
      );
      expect(screen.getByText(/This card already exists/i)).toBeInTheDocument();
      expect(screen.getByLabelText("Back")).toHaveValue("Saved definition");
      expect(screen.getByLabelText("Back")).toBeDisabled();
    });
  });
});
