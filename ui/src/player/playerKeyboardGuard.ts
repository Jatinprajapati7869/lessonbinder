export function shouldHandlePlayerShortcutKey(
  event: KeyboardEvent,
  captureDialogOpen: boolean,
  matchesKey: (key: string) => boolean
): boolean {
  if (!matchesKey(event.key)) {
    return false;
  }
  if (captureDialogOpen) {
    return false;
  }
  if (
    event.defaultPrevented ||
    event.metaKey ||
    event.ctrlKey ||
    event.altKey
  ) {
    return false;
  }
  const target = event.target;
  if (!(target instanceof HTMLElement)) {
    return true;
  }
  if (target.isContentEditable) {
    return false;
  }
  const tag = target.tagName;
  if (tag === "TEXTAREA" || tag === "SELECT") {
    return false;
  }
  if (tag === "INPUT") {
    const input = target as HTMLInputElement;
    return (
      input.type === "range" &&
      input.classList.contains("player-scrubber__input")
    );
  }
  return true;
}
