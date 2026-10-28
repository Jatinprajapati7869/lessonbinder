export type LessonKind = "local_file" | "playlist_item";

export type LessonStatus = "not_started" | "in_progress" | "done" | "broken";

export type DerivedStatus = "not_started" | "in_progress" | "done" | "broken";

export type SubtitleCue = {
  start_ms: number;
  end_ms: number;
  text: string;
};

export type MediaChapter = {
  start_ms: number;
  title: string;
};

export type LessonChapters = {
  chapters: MediaChapter[];
  reason: string | null;
};

export type WatchedRange = {
  start_ms: number;
  end_ms: number;
};

export type LessonNote = {
  note_id: string;
  lesson_id: string;
  text: string;
  timestamp_ms: number | null;
  created_at: string;
  updated_at: string;
};

export type LessonResourceKind = "pdf" | "image" | "text";

export type LessonResource = {
  resource_id: string;
  filename: string;
  kind: LessonResourceKind;
  absolute_path: string;
};

export type LessonCardLink = {
  anki_note_id: number;
  timestamp_ms: number;
  front_preview: string;
};

export type UpNextTarget = {
  lesson_id: string;
  title: string;
};

export type LessonRow = {
  lesson_id: string;
  position: number;
  title: string;
  kind: LessonKind;
  status: LessonStatus;
  duration_ms: number | null;
  resume_position_ms: number;
  max_position_reached_ms: number;
  document_position: number;
  playlist_item_id: string | null;
  playlist_url: string | null;
  local_media_path: string | null;
  thumbnail_path: string | null;
};

export type OrderProposal = {
  proposal_id: string;
  current_titles: string[];
  proposed_titles: string[];
};

export type LessonSectionGroup = {
  section: string | null;
  lessons: LessonRow[];
};

export type CourseDetail = {
  course_id: string;
  title: string;
  course_status: DerivedStatus;
  lessons: LessonRow[];
  lesson_groups: LessonSectionGroup[];
  proposal: OrderProposal | null;
};
