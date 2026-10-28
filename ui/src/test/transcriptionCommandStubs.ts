import { vi } from "vitest";

export const transcriptionCommandStubs = {
  getTranscriptionModelPath: vi.fn().mockResolvedValue(""),
  transcribeLesson: vi.fn().mockResolvedValue(undefined),
};
