export type CourseSourceKind = "folder" | "playlist";

export type LibraryVisibility = "active" | "archived";

export type DerivedStatus = "not_started" | "in_progress" | "done" | "broken";

export type CourseRow = {
  course_id: string;
  title: string;
  source_kind: CourseSourceKind;
  status: DerivedStatus;
  remaining_ms: number;
  unknown_duration_count: number;
  is_archived: boolean;
  thumbnail_path: string | null;
};
