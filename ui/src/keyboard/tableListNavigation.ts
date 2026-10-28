export function handleTableArrowKeys(
  event: React.KeyboardEvent,
  container: HTMLElement | null,
  rowSelector: string,
  controlSelector: string
): void {
  if (!container || (event.key !== "ArrowDown" && event.key !== "ArrowUp")) {
    return;
  }
  const rows = Array.from(container.querySelectorAll<HTMLElement>(rowSelector));
  const controls = rows
    .map((row) => row.querySelector<HTMLElement>(controlSelector))
    .filter((el): el is HTMLElement => el !== null);
  const index = controls.findIndex((el) => el === document.activeElement);
  if (index < 0) {
    return;
  }
  event.preventDefault();
  const nextIndex = event.key === "ArrowDown" ? index + 1 : index - 1;
  if (nextIndex >= 0 && nextIndex < controls.length) {
    controls[nextIndex].focus();
  }
}
