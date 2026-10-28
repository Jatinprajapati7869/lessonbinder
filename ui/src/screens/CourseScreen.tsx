import { useCallback, useEffect, useRef, useState } from "react";

import { handleTableArrowKeys } from "../keyboard/tableListNavigation";

import type { CourseDetail, LessonRow } from "../api/courseTypes";
import { CourseLessonTitleCell } from "../course/CourseLessonTitleCell";
import { libraryImportErrorMessage } from "../library/libraryImportError";
import { LibraryFolderScan } from "../library/LibraryFolderScan";
import { folderScanStatusLine } from "../library/folderScanProgress";
import { useFolderScanProgress } from "../library/useFolderScanProgress";
import type { CourseRow } from "../api/libraryTypes";
import { useCourseCommands } from "../course/useCourseCommands";
import {
  formatDurationMs,
  formatStatus,
  watchedPercent,
} from "../library/format";
import { OrderProposalPanel } from "./OrderProposalPanel";

export type CourseScreenProps = {
  course: CourseRow;
  onBack: () => void;
  onPlayLesson: (lesson: LessonRow) => void;
  onReviewThisCourse?: (courseId: string) => void;
};

function courseProgressPercent(lessons: LessonRow[]): number {
  if (lessons.length === 0) {
    return 0;
  }
  const done = lessons.filter((l) => l.status === "done").length;
  return Math.round((done / lessons.length) * 100);
}

export function CourseScreen({
  course,
  onBack,
  onPlayLesson,
  onReviewThisCourse,
}: CourseScreenProps) {
  const lessonTableRef = useRef<HTMLTableSectionElement>(null);
  const commands = useCourseCommands();
  const [detail, setDetail] = useState<CourseDetail | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [rescanningFolder, setRescanningFolder] = useState(false);
  const folderScanProgress = useFolderScanProgress(rescanningFolder);
  const [archiveConfirmOpen, setArchiveConfirmOpen] = useState(false);

  const refresh = useCallback(async () => {
    setError(null);
    try {
      const next = await commands.getCourseDetail(course.course_id);
      setDetail(next);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not load course.");
    }
  }, [commands, course.course_id]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  async function toggleComplete(lesson: LessonRow) {
    setBusy(true);
    try {
      if (lesson.status === "done") {
        await commands.markLessonIncomplete(lesson.lesson_id);
      } else {
        await commands.markLessonComplete(lesson.lesson_id);
      }
      await refresh();
    } catch (e) {
      setError(
        e instanceof Error ? e.message : "Could not update lesson status."
      );
    } finally {
      setBusy(false);
    }
  }

  async function togglePlaylistWatched(lesson: LessonRow, watched: boolean) {
    if (!lesson.playlist_item_id) {
      return;
    }
    setBusy(true);
    try {
      await commands.setPlaylistItemWatched(lesson.playlist_item_id, watched);
      await refresh();
    } catch (e) {
      setError(
        e instanceof Error ? e.message : "Could not update watched state."
      );
    } finally {
      setBusy(false);
    }
  }

  async function handleAcceptProposal(proposalId: string) {
    setBusy(true);
    try {
      await commands.acceptOrderProposal(proposalId);
      await refresh();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not accept proposal.");
    } finally {
      setBusy(false);
    }
  }

  async function handleRescanCourse() {
    setBusy(true);
    setError(null);
    try {
      setRescanningFolder(true);
      await commands.rescanCourse(course.course_id);
      await refresh();
    } catch (e) {
      const message = libraryImportErrorMessage(e, "Could not rescan folder.");
      if (message !== null) {
        setError(message);
      }
    } finally {
      setRescanningFolder(false);
      setBusy(false);
    }
  }

  function handleCancelFolderRescan() {
    void commands.cancelFolderScan();
  }

  async function handleArchiveCourse() {
    setBusy(true);
    setError(null);
    try {
      await commands.archiveCourse(course.course_id);
      setArchiveConfirmOpen(false);
      onBack();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not archive course.");
    } finally {
      setBusy(false);
    }
  }

  async function handleUnarchiveCourse() {
    setBusy(true);
    setError(null);
    try {
      await commands.unarchiveCourse(course.course_id);
      onBack();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not restore course.");
    } finally {
      setBusy(false);
    }
  }

  async function handleRejectProposal(proposalId: string) {
    setBusy(true);
    try {
      await commands.rejectOrderProposal(proposalId);
      await refresh();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not reject proposal.");
    } finally {
      setBusy(false);
    }
  }

  async function moveLesson(lessonId: string, direction: "up" | "down") {
    if (!detail) {
      return;
    }
    const ids = detail.lessons.map((l) => l.lesson_id);
    const index = ids.indexOf(lessonId);
    if (index < 0) {
      return;
    }
    const swapWith = direction === "up" ? index - 1 : index + 1;
    if (swapWith < 0 || swapWith >= ids.length) {
      return;
    }
    const next = [...ids];
    [next[index], next[swapWith]] = [next[swapWith], next[index]];
    setBusy(true);
    try {
      await commands.reorderLessons(course.course_id, next);
      await refresh();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not reorder lessons.");
    } finally {
      setBusy(false);
    }
  }

  async function handleExportCourseNotes() {
    if (!detail) {
      return;
    }
    setBusy(true);
    setError(null);
    try {
      await commands.exportCourseNotesMarkdown(course.course_id, detail.title);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not export notes.");
    } finally {
      setBusy(false);
    }
  }

  if (!detail) {
    return (
      <div className="course-screen">
        <button
          type="button"
          className="library-button-secondary"
          onClick={onBack}
        >
          Back to library
        </button>
        {error ? (
          <p className="library-error" role="alert">
            {error}
          </p>
        ) : null}
      </div>
    );
  }

  const progress = courseProgressPercent(detail.lessons);
  const isFolderCourse = course.source_kind === "folder";
  const lessonGroups =
    detail.lesson_groups.length > 0
      ? detail.lesson_groups
      : [{ section: null, lessons: detail.lessons }];

  function renderLessonRow(lesson: LessonRow) {
    const orderIndex = detail.lessons.findIndex(
      (row) => row.lesson_id === lesson.lesson_id
    );
    const isFirst = orderIndex === 0;
    const isLast = orderIndex === detail.lessons.length - 1;
    const reorderControls = (
      <>
        {!isFirst ? (
          <button
            type="button"
            className="library-button-secondary"
            disabled={busy}
            onClick={() => void moveLesson(lesson.lesson_id, "up")}
          >
            Move up
          </button>
        ) : null}
        {!isLast ? (
          <button
            type="button"
            className="library-button-secondary"
            disabled={busy}
            onClick={() => void moveLesson(lesson.lesson_id, "down")}
          >
            Move down
          </button>
        ) : null}
      </>
    );
    const pct = watchedPercent(
      lesson.max_position_reached_ms,
      lesson.duration_ms
    );
    return (
      <tr key={lesson.lesson_id}>
        <td className="library-numeric">{lesson.position}</td>
        <td>
          {lesson.kind === "playlist_item" ? (
            <div className="playlist-lesson-cell">
              <span>{lesson.title}</span>
              {lesson.playlist_url ? (
                <a
                  className="playlist-lesson-cell__url"
                  href={lesson.playlist_url}
                  target="_blank"
                  rel="noreferrer"
                >
                  {lesson.playlist_url}
                </a>
              ) : null}
            </div>
          ) : (
            <CourseLessonTitleCell
              lesson={lesson}
              onOpen={() => onPlayLesson(lesson)}
              resolveMediaUrl={commands.mediaUrlForPath}
            />
          )}
        </td>
        <td className="library-numeric">
          {formatDurationMs(lesson.duration_ms)}
        </td>
        <td className="library-numeric">{pct === null ? "—" : `${pct}%`}</td>
        <td>
          <span
            className={`library-status library-status--${lesson.status}`}
            data-testid={`lesson-status-${lesson.lesson_id}`}
          >
            {formatStatus(lesson.status)}
          </span>
        </td>
        <td>
          {lesson.kind === "playlist_item" ? (
            <div className="lesson-actions">
              {reorderControls}
              <label className="playlist-watched-toggle">
                <input
                  type="checkbox"
                  checked={lesson.status === "done"}
                  disabled={busy}
                  onChange={(e) =>
                    void togglePlaylistWatched(lesson, e.target.checked)
                  }
                />
                Watched
              </label>
            </div>
          ) : (
            <div className="lesson-actions">
              {reorderControls}
              <button
                type="button"
                className="library-button-secondary"
                disabled={busy}
                onClick={() => onPlayLesson(lesson)}
              >
                Open
              </button>
              <button
                type="button"
                className="library-button-secondary"
                disabled={busy}
                onClick={() => void toggleComplete(lesson)}
              >
                {lesson.status === "done" ? "Mark incomplete" : "Mark complete"}
              </button>
            </div>
          )}
        </td>
      </tr>
    );
  }

  return (
    <div className="course-screen">
      {rescanningFolder ? (
        <LibraryFolderScan
          mode="rescan"
          statusLine={folderScanStatusLine("rescan", folderScanProgress)}
          onCancel={handleCancelFolderRescan}
        />
      ) : null}
      <button
        type="button"
        className="library-button-secondary"
        onClick={onBack}
      >
        Back to library
      </button>
      <header className="course-header">
        <h1 className="screen-title">{detail.title}</h1>
        <p className="course-header__status">
          {formatStatus(detail.course_status)}
        </p>
        <div
          className="course-progress"
          role="progressbar"
          aria-label="Course completion"
          aria-valuenow={progress}
          aria-valuemin={0}
          aria-valuemax={100}
        >
          <div
            className="course-progress__fill"
            style={{ width: `${progress}%` }}
          />
        </div>
        <div className="course-header__actions">
          {onReviewThisCourse ? (
            <button
              type="button"
              className="library-button-primary"
              data-testid="course-review-button"
              disabled={busy}
              onClick={() => onReviewThisCourse(course.course_id)}
            >
              Review this course
            </button>
          ) : null}
          {isFolderCourse ? (
            <button
              type="button"
              className="library-button-secondary"
              disabled={busy}
              onClick={() => void handleRescanCourse()}
            >
              Rescan folder
            </button>
          ) : null}
          <button
            type="button"
            className="library-button-secondary"
            disabled={busy}
            onClick={() => void handleExportCourseNotes()}
          >
            Export notes
          </button>
          {course.is_archived ? (
            <button
              type="button"
              className="library-button-secondary"
              disabled={busy}
              onClick={() => void handleUnarchiveCourse()}
            >
              Restore to library
            </button>
          ) : (
            <button
              type="button"
              className="library-button-secondary"
              disabled={busy}
              onClick={() => setArchiveConfirmOpen(true)}
            >
              Archive
            </button>
          )}
        </div>
      </header>

      {archiveConfirmOpen ? (
        <div
          className="course-archive-confirm"
          role="alertdialog"
          aria-labelledby="course-archive-confirm-title"
        >
          <p
            id="course-archive-confirm-title"
            className="course-archive-confirm__line"
          >
            Notes in Anki are kept.
          </p>
          <div className="course-archive-confirm__actions">
            <button
              type="button"
              className="library-button-secondary"
              disabled={busy}
              onClick={() => setArchiveConfirmOpen(false)}
            >
              Cancel
            </button>
            <button
              type="button"
              className="study-button"
              disabled={busy}
              onClick={() => void handleArchiveCourse()}
            >
              Archive course
            </button>
          </div>
        </div>
      ) : null}

      {detail.proposal ? (
        <OrderProposalPanel
          proposal={detail.proposal}
          busy={busy}
          onAccept={() =>
            void handleAcceptProposal(detail.proposal!.proposal_id)
          }
          onReject={() =>
            void handleRejectProposal(detail.proposal!.proposal_id)
          }
        />
      ) : null}

      {error ? (
        <p className="library-error" role="alert">
          {error}
        </p>
      ) : null}

      {detail.lessons.length === 0 ? (
        <div className="library-empty">
          <p className="library-empty__lead">This course has no lessons.</p>
          <p className="library-empty__hint">
            Check that the folder contains supported files (mp4, mkv, pdf, …).
          </p>
          {isFolderCourse ? (
            <button
              type="button"
              className="library-button-primary"
              disabled={busy}
              onClick={() => void handleRescanCourse()}
            >
              Rescan folder
            </button>
          ) : (
            <p className="library-empty__hint" role="status">
              rescan_not_supported
            </p>
          )}
        </div>
      ) : (
        <div className="library-table-panel">
          <table
            className="library-table course-lesson-table"
            onKeyDown={(event) =>
              handleTableArrowKeys(
                event,
                lessonTableRef.current,
                "tbody tr",
                "button.library-course-link, .lesson-actions button, .playlist-watched-toggle input"
              )
            }
          >
            <thead>
              <tr>
                <th scope="col">#</th>
                <th scope="col">Title</th>
                <th scope="col">Duration</th>
                <th scope="col">Watched</th>
                <th scope="col">Status</th>
                <th scope="col">Actions</th>
              </tr>
            </thead>
            <tbody ref={lessonTableRef}>
              {lessonGroups.flatMap((group, groupIndex) => {
                const heading =
                  group.section === null
                    ? []
                    : [
                        <tr key={`section-${group.section}-${groupIndex}`}>
                          <th
                            colSpan={6}
                            scope="rowgroup"
                            className="course-section-heading"
                          >
                            {group.section}
                          </th>
                        </tr>,
                      ];
                return [...heading, ...group.lessons.map(renderLessonRow)];
              })}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
