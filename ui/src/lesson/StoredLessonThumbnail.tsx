import { useEffect, useState } from "react";

export type StoredLessonThumbnailProps = {
  path: string | null;
  resolveUrl: (path: string) => Promise<string>;
};

export function StoredLessonThumbnail({
  path,
  resolveUrl,
}: StoredLessonThumbnailProps) {
  const [src, setSrc] = useState<string | null>(null);

  useEffect(() => {
    if (!path) {
      setSrc(null);
      return;
    }
    let cancelled = false;
    void resolveUrl(path).then((url) => {
      if (!cancelled) {
        setSrc(url);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [path, resolveUrl]);

  if (!path || !src) {
    return null;
  }

  return (
    <img
      className="stored-lesson-thumbnail"
      src={src}
      alt=""
      data-testid="lesson-thumbnail"
    />
  );
}
