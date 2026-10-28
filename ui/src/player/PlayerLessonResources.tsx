import { useEffect, useState } from "react";

import type { LessonResource, LessonResourceKind } from "../api/courseTypes";
import { useCourseCommands } from "../course/useCourseCommands";
import { LessonResourceReader } from "../lesson/LessonResourceReader";

function lessonResourceKindLabel(kind: LessonResourceKind): string {
  switch (kind) {
    case "pdf":
      return "PDF";
    case "image":
      return "Image";
    case "text":
      return "Text";
    default: {
      const _exhaustive: never = kind;
      return _exhaustive;
    }
  }
}

export type PlayerLessonResourcesProps = {
  lessonId: string;
};

export function PlayerLessonResources({
  lessonId,
}: PlayerLessonResourcesProps) {
  const commands = useCourseCommands();
  const [resources, setResources] = useState<LessonResource[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    void commands.listLessonResources(lessonId).then((rows) => {
      if (!cancelled) {
        setResources(rows);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [commands, lessonId]);

  const selected =
    resources.find((resource) => resource.resource_id === selectedId) ?? null;

  if (resources.length === 0) {
    return null;
  }

  return (
    <section className="player-resources" aria-label="Lesson resources">
      <h2 className="player-resources__title">Resources</h2>
      <ul className="player-resources__list">
        {resources.map((resource) => {
          const kindLabel = lessonResourceKindLabel(resource.kind);
          return (
            <li key={resource.resource_id}>
              <button
                type="button"
                className="player-resources__item"
                aria-pressed={selectedId === resource.resource_id}
                onClick={() => setSelectedId(resource.resource_id)}
              >
                <span className="player-resources__filename">
                  {resource.filename}
                </span>
                <span className="player-resources__kind">{kindLabel}</span>
              </button>
            </li>
          );
        })}
      </ul>
      {selected ? (
        <div
          className="player-resources__viewer"
          data-testid="lesson-resource-viewer"
        >
          <LessonResourceReader resource={selected} />
        </div>
      ) : null}
    </section>
  );
}
