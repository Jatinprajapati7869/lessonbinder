import { useEffect, useState } from "react";

import type { LessonResource } from "../api/courseTypes";
import { useCourseCommands } from "../course/useCourseCommands";
import { pdfSrcWithPage } from "./documentPosition";
import { isDocumentLessonPath, isMarkdownLessonPath } from "./lessonDocument";
import { LessonMarkdownBody } from "./lessonMarkdown";

export type LessonResourceReaderProps = {
  resource: LessonResource;
};

export function LessonResourceReader({ resource }: LessonResourceReaderProps) {
  const commands = useCourseCommands();
  const path = resource.absolute_path;
  const [docUrl, setDocUrl] = useState<string | null>(null);
  const [textContent, setTextContent] = useState<string | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);

  const isPdf = resource.kind === "pdf";
  const isImage = resource.kind === "image";
  const isMarkdown = isMarkdownLessonPath(path);

  useEffect(() => {
    if (isImage) {
      void commands.mediaUrlForPath(path).then(setDocUrl);
      return;
    }
    if (!isDocumentLessonPath(path)) {
      setLoadError("Unsupported resource type.");
      return;
    }
    let cancelled = false;
    void commands.mediaUrlForPath(path).then(async (url) => {
      if (cancelled) {
        return;
      }
      if (isPdf) {
        setDocUrl(url);
        return;
      }
      try {
        const response = await fetch(url);
        const text = await response.text();
        if (!cancelled) {
          setTextContent(text);
        }
      } catch {
        if (!cancelled) {
          setLoadError("Could not load document.");
        }
      }
    });
    return () => {
      cancelled = true;
    };
  }, [commands, isImage, isPdf, path]);

  if (loadError) {
    return (
      <p className="library-error" role="alert">
        {loadError}
      </p>
    );
  }

  if (isImage) {
    return docUrl ? (
      <img
        className="lesson-resource__image"
        src={docUrl}
        alt={resource.filename}
        data-testid="lesson-resource-image"
      />
    ) : (
      <p className="library-empty__hint">Loading image…</p>
    );
  }

  if (isPdf) {
    const iframeSrc = docUrl ? pdfSrcWithPage(docUrl, 1) : null;
    return iframeSrc ? (
      <iframe
        className="document-lesson__frame"
        src={iframeSrc}
        title={resource.filename}
        data-testid="document-pdf-frame"
      />
    ) : (
      <p className="library-empty__hint">Loading document…</p>
    );
  }

  if (textContent !== null) {
    return (
      <div
        className="document-lesson__scroll lesson-resource__scroll"
        data-testid="document-lesson-scroll"
      >
        {isMarkdown ? (
          <LessonMarkdownBody source={textContent} />
        ) : (
          <pre className="document-lesson__text" data-testid="document-text">
            {textContent}
          </pre>
        )}
      </div>
    );
  }

  return <p className="library-empty__hint">Loading document…</p>;
}
