import React from "react";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { CardDraft } from "./src/api/captureTypes";
import { CaptureCommandsProvider } from "./src/capture/CaptureCommandsContext";
import { AudioClipCardPanel } from "./src/screens/AudioClipCardPanel";
import { captureCommandStubs } from "./src/test/captureCommandStubs";

const draft: CardDraft = {
  course: "Linear Algebra",
  lesson_title: "Intro clip",
  front: "Definition of a vector space",
  back: "Intro clip\nLinear Algebra\n00:01:23",
  source_label: "audio_clip",
  timestamp: "00:01:23",
};

describe("AudioClipCardPanel", () => {
  it("surfaces clip_encoder_unavailable from the command layer", async () => {
    const commands = {
      ...captureCommandStubs,
      buildAudioClipDraft: vi
        .fn()
        .mockRejectedValue(new Error("clip_encoder_unavailable")),
    };

    render(
      <CaptureCommandsProvider commands={commands}>
        <AudioClipCardPanel
          lessonId="lesson-playlist"
          playheadMs={45_000}
          onClose={() => {}}
        />
      </CaptureCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole("alert")).toHaveTextContent(
        "clip_encoder_unavailable"
      );
    });
  });

  it("builds a draft and commits audio through the capture command port", async () => {
    const user = userEvent.setup();
    const buildAudioClipDraft = vi.fn().mockResolvedValue(draft);
    const commitAudioClipCard = vi.fn().mockResolvedValue({
      duplicate: false,
      note_id: 12,
      course: draft.course,
      lesson_title: draft.lesson_title,
      front: draft.front,
      back: draft.back,
      source_label: draft.source_label,
      timestamp: draft.timestamp,
    });
    const commands = {
      ...captureCommandStubs,
      buildAudioClipDraft,
      commitAudioClipCard,
    };

    render(
      <CaptureCommandsProvider commands={commands}>
        <AudioClipCardPanel
          lessonId="lesson-1"
          playheadMs={83_000}
          onClose={() => {}}
        />
      </CaptureCommandsProvider>
    );

    await waitFor(() => {
      expect(buildAudioClipDraft).toHaveBeenCalledWith({
        lessonId: "lesson-1",
        playheadMs: 83_000,
      });
    });

    await user.click(screen.getByRole("button", { name: "Save to Anki" }));

    await waitFor(() => {
      expect(commitAudioClipCard).toHaveBeenCalledWith({
        lessonId: "lesson-1",
        playheadMs: 83_000,
        front: draft.front,
        back: draft.back,
      });
    });
  });
});
