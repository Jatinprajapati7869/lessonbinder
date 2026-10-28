import React from "react";
import { render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";

import type { CourseCommandPort } from "./src/api/courseCommands";
import type { CourseDetail, LessonRow } from "./src/api/courseTypes";
import type { CourseRow } from "./src/api/libraryTypes";
import { CourseCommandsProvider } from "./src/course/CourseCommandsContext";
import { CourseScreen } from "./src/screens/CourseScreen";
import { lessonNoteCommandStubs } from "./src/test/lessonNoteCommandStubs";

const localLesson: LessonRow = {
  lesson_id: "lesson-1",
  position: 1,
  title: "Intro",
  kind: "local_file",
  status: "not_started",
  duration_ms: 120_000,
  resume_position_ms: 0,
  max_position_reached_ms: 0,
  document_position: 0,
  playlist_item_id: null,
  playlist_url: null,
  local_media_path: "/tmp/intro.mp4",
  thumbnail_path: null,
};

const courseRow: CourseRow = {
  course_id: "course-1",
  title: "Demo Course",
  source_kind: "folder",
  status: "not_started",
  remaining_ms: 120_000,
  unknown_duration_count: 0,
  is_archived: false,
  thumbnail_path: null,
};

const detail: CourseDetail = {
  course_id: "course-1",
  title: "Demo Course",
  course_status: "not_started",
  lessons: [localLesson],
  lesson_groups: [{ section: null, lessons: [localLesson] }],
  proposal: null,
};

function baseCommands(
  overrides: Partial<CourseCommandPort> = {}
): CourseCommandPort {
  return {
    getCourseDetail: vi.fn().mockResolvedValue(detail),
    markLessonComplete: vi.fn(),
    markLessonIncomplete: vi.fn(),
    saveLessonResume: vi.fn(),
    acceptOrderProposal: vi.fn(),
    rejectOrderProposal: vi.fn(),
    reorderLessons: vi.fn().mockResolvedValue(undefined),
    setPlaylistItemWatched: vi.fn(),
    markLessonBroken: vi.fn(),
    rescanCourse: vi.fn().mockResolvedValue(undefined),
    archiveCourse: vi.fn().mockResolvedValue(undefined),
    unarchiveCourse: vi.fn().mockResolvedValue(undefined),
    mediaUrlForPath: vi.fn(),
    listLessonCues: vi.fn().mockResolvedValue([]),
    listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
    recordPlayback: vi.fn().mockResolvedValue([]),
    getTranscriptionModelPath: vi.fn().mockResolvedValue(""),
    transcribeLesson: vi.fn(),
    ...lessonNoteCommandStubs,
    ...overrides,
  };
}

describe("CourseScreen", () => {
  it("Review this course passes the course id to the review handler", async () => {
    const user = userEvent.setup();
    const onReviewThisCourse = vi.fn();

    render(
      <CourseCommandsProvider commands={baseCommands()}>
        <CourseScreen
          course={courseRow}
          onBack={() => {}}
          onPlayLesson={() => {}}
          onReviewThisCourse={onReviewThisCourse}
        />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByTestId("course-review-button")).toBeInTheDocument();
    });

    await user.click(
      screen.getByRole("button", { name: "Review this course" })
    );

    expect(onReviewThisCourse).toHaveBeenCalledWith("course-1");
  });

  it("folder course rescan calls command layer and refreshes lessons", async () => {
    const user = userEvent.setup();
    const rescanCourse = vi.fn().mockResolvedValue(undefined);
    const refreshedLessons: LessonRow[] = [
      localLesson,
      {
        ...localLesson,
        lesson_id: "lesson-2",
        position: 2,
        title: "Part two",
      },
    ];
    const getCourseDetail = vi
      .fn()
      .mockResolvedValueOnce(detail)
      .mockResolvedValueOnce({
        ...detail,
        lessons: refreshedLessons,
        lesson_groups: [{ section: null, lessons: refreshedLessons }],
      });

    render(
      <CourseCommandsProvider
        commands={baseCommands({ getCourseDetail, rescanCourse })}
      >
        <CourseScreen
          course={courseRow}
          onBack={() => {}}
          onPlayLesson={() => {}}
        />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Rescan folder" })
      ).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Rescan folder" }));

    await waitFor(() => {
      expect(rescanCourse).toHaveBeenCalledWith("course-1");
      expect(getCourseDetail).toHaveBeenCalledTimes(2);
      expect(screen.getByText("Part two")).toBeInTheDocument();
    });
  });

  it("shows stored thumbnail on lesson row when present", async () => {
    const lessonWithThumb: LessonRow = {
      ...localLesson,
      thumbnail_path: "/app/thumbnails/lesson-1.jpg",
    };
    const mediaUrlForPath = vi.fn().mockResolvedValue("asset://thumb");
    render(
      <CourseCommandsProvider
        commands={baseCommands({
          getCourseDetail: vi.fn().mockResolvedValue({
            ...detail,
            lessons: [lessonWithThumb],
            lesson_groups: [{ section: null, lessons: [lessonWithThumb] }],
          }),
          mediaUrlForPath,
        })}
      >
        <CourseScreen
          course={courseRow}
          onBack={() => {}}
          onPlayLesson={() => {}}
        />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByTestId("lesson-thumbnail")).toHaveAttribute(
        "src",
        "asset://thumb"
      );
    });
    expect(mediaUrlForPath).toHaveBeenCalledWith(
      "/app/thumbnails/lesson-1.jpg"
    );
  });

  it("lesson row without thumbnail stays text only", async () => {
    render(
      <CourseCommandsProvider commands={baseCommands()}>
        <CourseScreen
          course={courseRow}
          onBack={() => {}}
          onPlayLesson={() => {}}
        />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(screen.getByRole("button", { name: "Intro" })).toBeInTheDocument();
    });
    expect(screen.queryByTestId("lesson-thumbnail")).not.toBeInTheDocument();
    expect(screen.queryByRole("img")).not.toBeInTheDocument();
  });

  it("playlist course does not offer rescan and shows typed not-supported hint", async () => {
    const playlistCourse: CourseRow = {
      ...courseRow,
      source_kind: "playlist",
    };
    const emptyPlaylistDetail: CourseDetail = {
      ...detail,
      lessons: [],
      lesson_groups: [],
    };

    render(
      <CourseCommandsProvider
        commands={baseCommands({
          getCourseDetail: vi.fn().mockResolvedValue(emptyPlaylistDetail),
        })}
      >
        <CourseScreen
          course={playlistCourse}
          onBack={() => {}}
          onPlayLesson={() => {}}
        />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.queryByRole("button", { name: "Rescan folder" })
      ).not.toBeInTheDocument();
      expect(screen.getByText(/rescan_not_supported/i)).toBeInTheDocument();
    });
  });

  it("archive confirm keeps Anki notes line and calls command", async () => {
    const user = userEvent.setup();
    const archiveCourse = vi.fn().mockResolvedValue(undefined);
    const onBack = vi.fn();

    render(
      <CourseCommandsProvider commands={baseCommands({ archiveCourse })}>
        <CourseScreen
          course={courseRow}
          onBack={onBack}
          onPlayLesson={() => {}}
        />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Archive" })
      ).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Archive" }));
    expect(screen.getByText("Notes in Anki are kept.")).toBeInTheDocument();

    await user.click(screen.getByRole("button", { name: "Archive course" }));

    await waitFor(() => {
      expect(archiveCourse).toHaveBeenCalledWith("course-1");
      expect(onBack).toHaveBeenCalled();
    });
  });

  it("move down on first lesson swaps titles with second", async () => {
    const user = userEvent.setup();
    const secondLesson: LessonRow = {
      ...localLesson,
      lesson_id: "lesson-2",
      position: 2,
      title: "Second",
    };
    const twoLessons: LessonRow[] = [localLesson, secondLesson];
    const twoLessonDetail: CourseDetail = {
      ...detail,
      lessons: twoLessons,
      lesson_groups: [{ section: null, lessons: twoLessons }],
    };
    const reorderLessons = vi.fn().mockResolvedValue(undefined);
    const getCourseDetail = vi
      .fn()
      .mockResolvedValueOnce(twoLessonDetail)
      .mockResolvedValueOnce({
        ...twoLessonDetail,
        lessons: [secondLesson, localLesson],
        lesson_groups: [
          { section: null, lessons: [secondLesson, localLesson] },
        ],
      });

    render(
      <CourseCommandsProvider
        commands={baseCommands({ getCourseDetail, reorderLessons })}
      >
        <CourseScreen
          course={courseRow}
          onBack={() => {}}
          onPlayLesson={() => {}}
        />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Move down" })
      ).toBeInTheDocument();
    });
    const lessonRows = screen.getAllByRole("row").slice(1);
    expect(
      within(lessonRows[0]).queryByRole("button", { name: "Move up" })
    ).not.toBeInTheDocument();
    expect(
      within(lessonRows[1]).queryByRole("button", { name: "Move down" })
    ).not.toBeInTheDocument();

    await user.click(
      within(lessonRows[0]).getByRole("button", { name: "Move down" })
    );

    await waitFor(() => {
      expect(reorderLessons).toHaveBeenCalledWith("course-1", [
        "lesson-2",
        "lesson-1",
      ]);
      const titleButtons = screen.getAllByRole("button", {
        name: /Intro|Second/,
      });
      expect(titleButtons[0]).toHaveAccessibleName("Second");
      expect(titleButtons[1]).toHaveAccessibleName("Intro");
    });
  });

  it("move down then rescan folder keeps swapped lesson titles", async () => {
    const user = userEvent.setup();
    const secondLesson: LessonRow = {
      ...localLesson,
      lesson_id: "lesson-2",
      position: 2,
      title: "Second",
    };
    const pathOrderDetail: CourseDetail = {
      ...detail,
      lessons: [localLesson, secondLesson],
      lesson_groups: [{ section: null, lessons: [localLesson, secondLesson] }],
    };
    const swappedLessons: LessonRow[] = [
      { ...secondLesson, position: 1 },
      { ...localLesson, position: 2 },
    ];
    const swappedDetail: CourseDetail = {
      ...detail,
      lessons: swappedLessons,
      lesson_groups: [{ section: null, lessons: swappedLessons }],
    };
    const rescanCourse = vi.fn().mockResolvedValue(undefined);
    const getCourseDetail = vi
      .fn()
      .mockResolvedValueOnce(pathOrderDetail)
      .mockResolvedValueOnce(swappedDetail)
      .mockResolvedValueOnce(swappedDetail);
    const reorderLessons = vi.fn().mockResolvedValue(undefined);

    render(
      <CourseCommandsProvider
        commands={baseCommands({
          getCourseDetail,
          reorderLessons,
          rescanCourse,
        })}
      >
        <CourseScreen
          course={courseRow}
          onBack={() => {}}
          onPlayLesson={() => {}}
        />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Move down" })
      ).toBeInTheDocument();
    });

    const lessonRows = () => screen.getAllByRole("row").slice(1);
    await user.click(
      within(lessonRows()[0]).getByRole("button", { name: "Move down" })
    );

    await waitFor(() => {
      expect(reorderLessons).toHaveBeenCalledWith("course-1", [
        "lesson-2",
        "lesson-1",
      ]);
      expect(
        screen.getAllByRole("button", { name: /Intro|Second/ })[0]
      ).toHaveAccessibleName("Second");
    });

    await user.click(screen.getByRole("button", { name: "Rescan folder" }));

    await waitFor(() => {
      expect(rescanCourse).toHaveBeenCalledWith("course-1");
      expect(getCourseDetail).toHaveBeenCalledTimes(3);
      const titleButtons = screen.getAllByRole("button", {
        name: /Intro|Second/,
      });
      expect(titleButtons[0]).toHaveAccessibleName("Second");
      expect(titleButtons[1]).toHaveAccessibleName("Intro");
    });
  });

  it("mark complete calls command and refreshes", async () => {
    const user = userEvent.setup();
    const markLessonComplete = vi.fn().mockResolvedValue(undefined);
    const getCourseDetail = vi
      .fn()
      .mockResolvedValueOnce(detail)
      .mockResolvedValueOnce({
        ...detail,
        lessons: [{ ...localLesson, status: "done" }],
        lesson_groups: [
          { section: null, lessons: [{ ...localLesson, status: "done" }] },
        ],
        course_status: "done",
      });

    const commands: CourseCommandPort = {
      getCourseDetail,
      markLessonComplete,
      markLessonIncomplete: vi.fn(),
      saveLessonResume: vi.fn(),
      acceptOrderProposal: vi.fn(),
      rejectOrderProposal: vi.fn(),
      reorderLessons: vi.fn(),
      setPlaylistItemWatched: vi.fn(),
      markLessonBroken: vi.fn(),
      rescanCourse: vi.fn(),
      archiveCourse: vi.fn(),
      unarchiveCourse: vi.fn(),
      mediaUrlForPath: vi.fn(),
      listLessonCues: vi.fn().mockResolvedValue([]),
      listLessonWatchedRanges: vi.fn().mockResolvedValue([]),
      recordPlayback: vi.fn().mockResolvedValue([]),
      getTranscriptionModelPath: vi.fn().mockResolvedValue(""),
      transcribeLesson: vi.fn(),
      ...lessonNoteCommandStubs,
    };

    render(
      <CourseCommandsProvider commands={commands}>
        <CourseScreen
          course={courseRow}
          onBack={() => {}}
          onPlayLesson={() => {}}
        />
      </CourseCommandsProvider>
    );

    await waitFor(() => {
      expect(
        screen.getByRole("button", { name: "Mark complete" })
      ).toBeInTheDocument();
    });

    await user.click(screen.getByRole("button", { name: "Mark complete" }));

    await waitFor(() => {
      expect(markLessonComplete).toHaveBeenCalledWith("lesson-1");
      expect(
        screen.getByRole("button", { name: "Mark incomplete" })
      ).toBeInTheDocument();
    });
  });
});
