import { shouldHandlePlayerShortcutKey } from "./playerKeyboardGuard";

export const PLAYER_SEEK_STEP_MS = 5000;

export function playerArrowSeekDeltaMs(key: string): number | null {
  if (key === "ArrowLeft") {
    return -PLAYER_SEEK_STEP_MS;
  }
  if (key === "ArrowRight") {
    return PLAYER_SEEK_STEP_MS;
  }
  return null;
}

export function clampSeekMs(nextMs: number, maxMs: number): number {
  return Math.min(Math.max(0, nextMs), Math.max(maxMs, 0));
}

export function shouldHandlePlayerArrowSeek(
  event: KeyboardEvent,
  captureDialogOpen: boolean
): boolean {
  return shouldHandlePlayerShortcutKey(
    event,
    captureDialogOpen,
    (key) => playerArrowSeekDeltaMs(key) !== null
  );
}
