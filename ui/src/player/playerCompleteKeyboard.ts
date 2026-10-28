import { shouldHandlePlayerShortcutKey } from "./playerKeyboardGuard";

export const PLAYER_MARK_COMPLETE_KEY = "d";

export function playerMarkCompleteAccessibleName(
  shortcutKey = PLAYER_MARK_COMPLETE_KEY
): string {
  return `Mark complete (${shortcutKey.toUpperCase()})`;
}

export function isPlayerMarkCompleteKey(key: string): boolean {
  return key.toLowerCase() === PLAYER_MARK_COMPLETE_KEY;
}

export function shouldHandlePlayerMarkComplete(
  event: KeyboardEvent,
  captureDialogOpen: boolean
): boolean {
  return shouldHandlePlayerShortcutKey(
    event,
    captureDialogOpen,
    isPlayerMarkCompleteKey
  );
}
