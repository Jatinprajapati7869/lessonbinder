import type { LessonRow } from "../api/courseTypes";
import { StoredLessonThumbnail } from "../lesson/StoredLessonThumbnail";

export type CourseLessonTitleCellProps = {
  lesson: LessonRow;
  onOpen: () => void;
  resolveMediaUrl: (path: string) => Promise<string>;
};

export function CourseLessonTitleCell({
  lesson,
  onOpen,
  resolveMediaUrl,
}: CourseLessonTitleCellProps) {
  return (
    <div className="course-lesson-title-cell">
      <StoredLessonThumbnail
        path={lesson.thumbnail_path}
        resolveUrl={resolveMediaUrl}
      />
      <button type="button" className="library-course-link" onClick={onOpen}>
        {lesson.title}
      </button>
    </div>
  );
}
