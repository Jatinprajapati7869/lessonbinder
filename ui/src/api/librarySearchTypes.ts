export type SearchMatchKind =
  "lesson_title" | "relative_path" | "subtitle_cue" | "lesson_note";

export type LibrarySearchHit = {
  lesson_id: string;
  lesson_title: string;
  course_id: string;
  course_title: string;
  match_kind: SearchMatchKind;
  snippet: string;
  cue_start_ms: number | null;
};

export type OpenLessonFromSearch = {
  courseId: string;
  lessonId: string;
  seekMs?: number;
};
