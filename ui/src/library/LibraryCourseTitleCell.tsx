import type { CourseRow } from "../api/libraryTypes";
import { StoredLessonThumbnail } from "../lesson/StoredLessonThumbnail";

export type LibraryCourseTitleCellProps = {
  course: CourseRow;
  onOpen: () => void;
  resolveMediaUrl: (path: string) => Promise<string>;
};

export function LibraryCourseTitleCell({
  course,
  onOpen,
  resolveMediaUrl,
}: LibraryCourseTitleCellProps) {
  return (
    <div className="library-course-title-cell">
      <StoredLessonThumbnail
        path={course.thumbnail_path}
        resolveUrl={resolveMediaUrl}
      />
      <button type="button" className="library-course-link" onClick={onOpen}>
        {course.title}
      </button>
    </div>
  );
}
