import { vi } from "vitest";

export const lessonNoteCommandStubs = {
  listLessonNotes: vi.fn().mockResolvedValue([]),
  createLessonNote: vi.fn(),
  updateLessonNote: vi.fn(),
  deleteLessonNote: vi.fn().mockResolvedValue(undefined),
  exportLessonNotesMarkdown: vi.fn().mockResolvedValue(undefined),
  exportCourseNotesMarkdown: vi.fn().mockResolvedValue(undefined),
  saveDocumentPosition: vi.fn().mockResolvedValue(undefined),
  listLessonResources: vi.fn().mockResolvedValue([]),
  listLessonCardLinks: vi.fn().mockResolvedValue([]),
  unlinkLessonCard: vi.fn().mockResolvedValue(undefined),
  captureLessonThumbnail: vi.fn().mockResolvedValue(undefined),
  listLessonChapters: vi.fn().mockResolvedValue({ chapters: [], reason: null }),
  upNext: vi.fn().mockResolvedValue(null),
};
