import type { JSX, ReactNode } from "react";

const HEADING_RE = /^(#{1,6})\s+(.+)$/;
const IMAGE_RE = /^!\[([^\]]*)\]\(([^)]+)\)\s*$/;

function headingLevel(markers: string): 1 | 2 | 3 | 4 | 5 | 6 {
  return Math.min(6, markers.length) as 1 | 2 | 3 | 4 | 5 | 6;
}

function renderImagePlaceholder(alt: string, url: string): ReactNode {
  const label = alt.trim() || "Image";
  return (
    <p
      className="lesson-markdown__image-placeholder"
      data-testid="markdown-image-placeholder"
    >
      {label} (not loaded)
      <span className="lesson-markdown__image-url">{url}</span>
    </p>
  );
}

/** Render lesson markdown without fetching remote images. */
export function LessonMarkdownBody({ source }: { source: string }) {
  const blocks: ReactNode[] = [];
  const paragraph: string[] = [];

  function flushParagraph() {
    if (paragraph.length === 0) {
      return;
    }
    blocks.push(
      <p key={`p-${blocks.length}`} className="lesson-markdown__paragraph">
        {paragraph.join(" ")}
      </p>
    );
    paragraph.length = 0;
  }

  for (const line of source.split(/\r?\n/)) {
    const trimmed = line.trimEnd();
    if (trimmed.length === 0) {
      flushParagraph();
      continue;
    }
    const heading = HEADING_RE.exec(trimmed);
    if (heading) {
      flushParagraph();
      const level = headingLevel(heading[1]);
      const Tag = `h${level}` as keyof JSX.IntrinsicElements;
      blocks.push(
        <Tag key={`h-${blocks.length}`} className="lesson-markdown__heading">
          {heading[2]}
        </Tag>
      );
      continue;
    }
    const image = IMAGE_RE.exec(trimmed);
    if (image) {
      flushParagraph();
      blocks.push(
        <span key={`img-${blocks.length}`}>
          {renderImagePlaceholder(image[1], image[2])}
        </span>
      );
      continue;
    }
    paragraph.push(trimmed);
  }
  flushParagraph();

  return (
    <article className="lesson-markdown" data-testid="document-markdown">
      {blocks}
    </article>
  );
}
