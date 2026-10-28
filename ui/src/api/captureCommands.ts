import { invoke } from "@tauri-apps/api/core";

import type {
  CaptureOcrStatus,
  CardDraft,
  CommittedCard,
} from "./captureTypes";

export type CaptureCommandPort = {
  captureOcrStatus(): Promise<CaptureOcrStatus>;
  buildCaptureDraft(params: {
    lessonId: string;
    timestampMs: number;
    png: Uint8Array;
    editedFront?: string;
    editedBack?: string;
  }): Promise<CardDraft>;
  commitCaptureCard(params: {
    lessonId: string;
    timestampMs: number;
    png: Uint8Array;
    front: string;
    back: string;
  }): Promise<CommittedCard>;
  buildSubtitleCueDraft(params: {
    lessonId: string;
    timestampMs: number;
    cueText: string;
  }): Promise<CardDraft>;
  commitSubtitleCueCard(params: {
    lessonId: string;
    timestampMs: number;
    cueText: string;
  }): Promise<CommittedCard>;
  buildLessonNoteDraft(params: {
    lessonId: string;
    noteText: string;
    timestampMs: number | null;
  }): Promise<CardDraft>;
  commitLessonNoteCard(params: {
    lessonId: string;
    timestampMs: number | null;
    front: string;
    back: string;
  }): Promise<CommittedCard>;
  buildAudioClipDraft(params: {
    lessonId: string;
    playheadMs: number;
  }): Promise<CardDraft>;
  commitAudioClipCard(params: {
    lessonId: string;
    playheadMs: number;
    front: string;
    back: string;
  }): Promise<CommittedCard>;
};

export const tauriCaptureCommands: CaptureCommandPort = {
  captureOcrStatus: () => invoke<CaptureOcrStatus>("capture_ocr_status"),
  buildCaptureDraft: ({
    lessonId,
    timestampMs,
    png,
    editedFront,
    editedBack,
  }) =>
    invoke<CardDraft>("build_capture_draft", {
      lessonId,
      timestampMs,
      png: Array.from(png),
      editedFront,
      editedBack,
    }),
  commitCaptureCard: ({ lessonId, timestampMs, png, front, back }) =>
    invoke<CommittedCard>("commit_capture_card", {
      lessonId,
      timestampMs,
      png: Array.from(png),
      front,
      back,
    }),
  buildSubtitleCueDraft: ({ lessonId, timestampMs, cueText }) =>
    invoke<CardDraft>("build_subtitle_cue_draft", {
      lessonId,
      timestampMs,
      cueText,
    }),
  commitSubtitleCueCard: ({ lessonId, timestampMs, cueText }) =>
    invoke<CommittedCard>("commit_subtitle_cue_card", {
      lessonId,
      timestampMs,
      cueText,
    }),
  buildLessonNoteDraft: ({ lessonId, noteText, timestampMs }) =>
    invoke<CardDraft>("build_lesson_note_draft", {
      lessonId,
      noteText,
      timestampMs,
    }),
  commitLessonNoteCard: ({ lessonId, timestampMs, front, back }) =>
    invoke<CommittedCard>("commit_lesson_note_card", {
      lessonId,
      timestampMs,
      front,
      back,
    }),
  buildAudioClipDraft: ({ lessonId, playheadMs }) =>
    invoke<CardDraft>("build_audio_clip_draft", {
      lessonId,
      playheadMs,
    }),
  commitAudioClipCard: ({ lessonId, playheadMs, front, back }) =>
    invoke<CommittedCard>("commit_audio_clip_card", {
      lessonId,
      playheadMs,
      front,
      back,
    }),
};
