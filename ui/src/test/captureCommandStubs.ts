import { vi } from "vitest";

import type { CaptureCommandPort } from "../api/captureCommands";

export const captureCommandStubs: CaptureCommandPort = {
  captureOcrStatus: vi.fn().mockResolvedValue({ available: true }),
  buildCaptureDraft: vi.fn(),
  commitCaptureCard: vi.fn(),
  buildSubtitleCueDraft: vi.fn(),
  commitSubtitleCueCard: vi.fn(),
  buildLessonNoteDraft: vi.fn(),
  commitLessonNoteCard: vi.fn(),
  buildAudioClipDraft: vi.fn(),
  commitAudioClipCard: vi.fn(),
};
