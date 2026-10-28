import {
  fetchReviewNextCard,
  fetchReviewNoteBack,
  fetchStudyBarState,
  readReviewMedia,
  submitReviewAnswer,
} from "./tauri";
import type { DueSummary } from "./types";
import type { ReviewCard, ReviewEase } from "./reviewTypes";

export type ReviewCommandPort = {
  fetchNextCard(courseId?: string | null): Promise<ReviewCard | null>;
  fetchDueSummary(): Promise<DueSummary>;
  fetchNoteBack(noteId: number): Promise<string>;
  readMedia(filename: string): Promise<Uint8Array>;
  answerCard(cardId: number, ease: ReviewEase): Promise<void>;
};

export const tauriReviewCommands: ReviewCommandPort = {
  fetchNextCard: fetchReviewNextCard,
  fetchDueSummary: async () => {
    const bar = await fetchStudyBarState();
    return bar.due;
  },
  fetchNoteBack: async (noteId) => {
    const { back } = await fetchReviewNoteBack(noteId);
    return back;
  },
  readMedia: readReviewMedia,
  answerCard: submitReviewAnswer,
};
