function sourceLengthFor(element: HTMLElement, sourceLength?: number): number {
  if (sourceLength !== undefined && sourceLength > 0) {
    return sourceLength;
  }
  return element.textContent?.length ?? 0;
}

/** Map a stored source character offset to scroll position. */
export function scrollToCharOffset(
  element: HTMLElement,
  offset: number,
  sourceLength?: number
): void {
  const length = sourceLengthFor(element, sourceLength);
  if (length === 0 || offset <= 0) {
    element.scrollTop = 0;
    return;
  }
  const clamped = Math.min(offset, length);
  const maxScroll = element.scrollHeight - element.clientHeight;
  if (maxScroll <= 0) {
    return;
  }
  const ratio = clamped / length;
  element.scrollTop = ratio * maxScroll;
}

/** Character offset in the source file from scroll position. */
export function charOffsetFromScroll(
  element: HTMLElement,
  sourceLength?: number
): number {
  const length = sourceLengthFor(element, sourceLength);
  if (length === 0) {
    return 0;
  }
  const maxScroll = element.scrollHeight - element.clientHeight;
  if (maxScroll <= 0) {
    return 0;
  }
  const ratio = element.scrollTop / maxScroll;
  return Math.round(ratio * length);
}

export function pdfPageFromPosition(position: number): number {
  return Math.max(1, position);
}

export function pdfSrcWithPage(url: string, page: number): string {
  if (page <= 1) {
    return url;
  }
  const hash = `page=${page}`;
  return url.includes("#") ? `${url}&${hash}` : `${url}#${hash}`;
}
