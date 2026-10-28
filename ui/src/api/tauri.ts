import { invoke } from "@tauri-apps/api/core";

import type { ReviewCard, ReviewEase } from "./reviewTypes";
import type { StudyBarState, VaultBootstrap } from "./types";

export async function fetchVaultBootstrap(): Promise<VaultBootstrap> {
  return invoke<VaultBootstrap>("vault_bootstrap");
}

export async function retryOpenVault(): Promise<VaultBootstrap> {
  return invoke<VaultBootstrap>("retry_open_vault");
}

export async function fetchStudyBarState(): Promise<StudyBarState> {
  return invoke<StudyBarState>("study_bar_state");
}

export async function launchAnkiProfile(): Promise<void> {
  return invoke<void>("launch_anki_profile");
}

export async function fetchReviewNextCard(
  courseId?: string | null
): Promise<ReviewCard | null> {
  if (courseId) {
    return invoke<ReviewCard | null>("review_next_card_for_course", {
      courseId,
    });
  }
  return invoke<ReviewCard | null>("review_next_card");
}

export async function fetchReviewNoteBack(
  noteId: number
): Promise<{ back: string }> {
  return invoke<{ back: string }>("review_note_back", { noteId });
}

export async function submitReviewAnswer(
  cardId: number,
  ease: ReviewEase
): Promise<void> {
  return invoke<void>("review_answer_card", { cardId, ease });
}

export async function readReviewMedia(filename: string): Promise<Uint8Array> {
  const bytes = await invoke<number[]>("review_read_media", { filename });
  return Uint8Array.from(bytes);
}
