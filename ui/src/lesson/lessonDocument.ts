export function isMarkdownLessonPath(path: string | null): boolean {
  return path?.toLowerCase().endsWith(".md") ?? false;
}

export function isPlainTextLessonPath(path: string | null): boolean {
  return path?.toLowerCase().endsWith(".txt") ?? false;
}

export function isDocumentLessonPath(path: string | null): boolean {
  if (!path) {
    return false;
  }
  const lower = path.toLowerCase();
  return (
    lower.endsWith(".pdf") || lower.endsWith(".md") || lower.endsWith(".txt")
  );
}
