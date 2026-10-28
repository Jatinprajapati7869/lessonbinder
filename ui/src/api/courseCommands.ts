import { invoke } from "@tauri-apps/api/core";

import type {
  CourseDetail,
  LessonCardLink,
  LessonNote,
  LessonResource,
  LessonResourceKind,
  LessonChapters,
  LessonRow,
  UpNextTarget,
  SubtitleCue,
  WatchedRange,
} from "./courseTypes";

export type CourseCommandPort = {
  getCourseDetail(courseId: string): Promise<CourseDetail>;
  markLessonComplete(lessonId: string): Promise<void>;
  markLessonIncomplete(lessonId: string): Promise<void>;
  saveLessonResume(lessonId: string, positionMs: number): Promise<void>;
  saveDocumentPosition(lessonId: string, position: number): Promise<void>;
  acceptOrderProposal(proposalId: string): Promise<void>;
  rejectOrderProposal(proposalId: string): Promise<void>;
  reorderLessons(courseId: string, lessonIds: string[]): Promise<void>;
  setPlaylistItemWatched(
    playlistItemId: string,
    watched: boolean
  ): Promise<void>;
  markLessonBroken(lessonId: string): Promise<void>;
  guardLessonMediaForResume(lessonId: string): Promise<LessonRow["status"]>;
  rescanCourse(courseId: string): Promise<void>;
  cancelFolderScan(): Promise<void>;
  openCourseFolderInFileManager(courseId: string): Promise<void>;
  openLessonFileDirectoryInFileManager(lessonId: string): Promise<void>;
  archiveCourse(courseId: string): Promise<void>;
  unarchiveCourse(courseId: string): Promise<void>;
  mediaUrlForPath(path: string): Promise<string>;
  listLessonCues(lessonId: string): Promise<SubtitleCue[]>;
  listLessonChapters(lessonId: string): Promise<LessonChapters>;
  upNext(lessonId: string): Promise<UpNextTarget | null>;
  listLessonResources(lessonId: string): Promise<LessonResource[]>;
  listLessonCardLinks(lessonId: string): Promise<LessonCardLink[]>;
  unlinkLessonCard(lessonId: string, ankiNoteId: number): Promise<void>;
  captureLessonThumbnail(
    lessonId: string,
    positionMs: number,
    jpeg: Uint8Array
  ): Promise<void>;
  getTranscriptionModelPath(): Promise<string>;
  transcribeLesson(lessonId: string): Promise<void>;
  listLessonWatchedRanges(lessonId: string): Promise<WatchedRange[]>;
  recordPlayback(
    lessonId: string,
    startMs: number,
    endMs: number
  ): Promise<WatchedRange[]>;
  listLessonNotes(lessonId: string): Promise<LessonNote[]>;
  createLessonNote(
    lessonId: string,
    text: string,
    timestampMs: number | null
  ): Promise<LessonNote>;
  updateLessonNote(
    noteId: string,
    text: string,
    timestampMs: number | null
  ): Promise<LessonNote>;
  deleteLessonNote(noteId: string): Promise<void>;
  exportLessonNotesMarkdown(
    lessonId: string,
    lessonTitle: string
  ): Promise<void>;
  exportCourseNotesMarkdown(
    courseId: string,
    courseTitle: string
  ): Promise<void>;
};

export const tauriCourseCommands: CourseCommandPort = {
  getCourseDetail: (courseId) =>
    invoke<CourseDetail>("get_course_detail", { courseId }),
  markLessonComplete: (lessonId) =>
    invoke("mark_lesson_complete", { lessonId }),
  markLessonIncomplete: (lessonId) =>
    invoke("mark_lesson_incomplete", { lessonId }),
  saveLessonResume: (lessonId, positionMs) =>
    invoke("save_lesson_resume", { lessonId, positionMs }),
  saveDocumentPosition: (lessonId, position) =>
    invoke("save_document_position", { lessonId, position }),
  acceptOrderProposal: (proposalId) =>
    invoke("accept_order_proposal", { proposalId }),
  rejectOrderProposal: (proposalId) =>
    invoke("reject_order_proposal", { proposalId }),
  reorderLessons: (courseId, lessonIds) =>
    invoke("reorder_course_lessons", { courseId, lessonIds }),
  setPlaylistItemWatched: (playlistItemId, watched) =>
    invoke("set_playlist_item_watched", { playlistItemId, watched }),
  markLessonBroken: (lessonId) => invoke("mark_lesson_broken", { lessonId }),
  guardLessonMediaForResume: (lessonId) =>
    invoke<LessonRow["status"]>("guard_lesson_media_for_resume", { lessonId }),
  rescanCourse: (courseId) => invoke("rescan_course", { courseId }),
  cancelFolderScan: () => invoke("cancel_import_course_folder"),
  openCourseFolderInFileManager: (courseId) =>
    invoke("open_course_folder_in_file_manager", { courseId }),
  openLessonFileDirectoryInFileManager: (lessonId) =>
    invoke("open_lesson_file_directory_in_file_manager", { lessonId }),
  archiveCourse: (courseId) => invoke("archive_course_entry", { courseId }),
  unarchiveCourse: (courseId) => invoke("unarchive_course_entry", { courseId }),
  mediaUrlForPath: async (path) => {
    const { convertFileSrc } = await import("@tauri-apps/api/core");
    return convertFileSrc(path);
  },
  listLessonCues: (lessonId) =>
    invoke<SubtitleCue[]>("list_lesson_cues", { lessonId }),
  listLessonChapters: (lessonId) =>
    invoke<LessonChapters>("list_chapters", { lessonId }),
  upNext: (lessonId) => invoke<UpNextTarget | null>("up_next", { lessonId }),
  captureLessonThumbnail: (lessonId, positionMs, jpeg) =>
    invoke("capture_lesson_thumbnail", {
      lessonId,
      positionMs,
      jpeg: Array.from(jpeg),
    }),
  listLessonResources: async (lessonId) => {
    const rows = await invoke<
      {
        resource_id: string;
        filename: string;
        kind: string;
        absolute_path: string;
      }[]
    >("list_lesson_resources_entry", { lessonId });
    return rows.map((row) => ({
      resource_id: row.resource_id,
      filename: row.filename,
      kind: row.kind as LessonResourceKind,
      absolute_path: row.absolute_path,
    }));
  },
  listLessonCardLinks: async (lessonId) =>
    invoke<LessonCardLink[]>("list_lesson_card_links_entry", { lessonId }),
  unlinkLessonCard: (lessonId, ankiNoteId) =>
    invoke("unlink_lesson_card_entry", { lessonId, ankiNoteId }),
  getTranscriptionModelPath: () =>
    invoke<string>("get_transcription_model_path"),
  transcribeLesson: (lessonId) => invoke("transcribe_lesson", { lessonId }),
  listLessonWatchedRanges: (lessonId) =>
    invoke<WatchedRange[]>("list_lesson_watched_ranges", { lessonId }),
  recordPlayback: (lessonId, startMs, endMs) =>
    invoke<WatchedRange[]>("record_lesson_playback", {
      lessonId,
      startMs,
      endMs,
    }),
  listLessonNotes: (lessonId) =>
    invoke<LessonNote[]>("list_lesson_notes", { lessonId }),
  createLessonNote: (lessonId, text, timestampMs) =>
    invoke<LessonNote>("create_lesson_note_entry", {
      lessonId,
      text,
      timestampMs,
    }),
  updateLessonNote: (noteId, text, timestampMs) =>
    invoke<LessonNote>("update_lesson_note_entry", {
      noteId,
      text,
      timestampMs,
    }),
  deleteLessonNote: (noteId) => invoke("delete_lesson_note_entry", { noteId }),
  exportLessonNotesMarkdown: (lessonId, lessonTitle) => {
    const safeName = `${lessonTitle.replace(/[^\w.-]+/g, "-").replace(/^-+|-+$/g, "") || "lesson-notes"}.md`;
    return invoke("export_lesson_notes_markdown", {
      lessonId,
      defaultFileName: safeName,
    });
  },
  exportCourseNotesMarkdown: (courseId, courseTitle) => {
    const safeName = `${courseTitle.replace(/[^\w.-]+/g, "-").replace(/^-+|-+$/g, "") || "course-notes"}.md`;
    return invoke("export_course_notes_markdown", {
      courseId,
      defaultFileName: safeName,
    });
  },
};
