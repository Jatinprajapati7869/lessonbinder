import { useCallback, useEffect, useRef, useState } from "react";

import { handleTableArrowKeys } from "../keyboard/tableListNavigation";

import type { LibraryContinueRow as LibraryContinueRowData } from "../api/libraryContinueTypes";
import type {
  LibrarySearchHit,
  OpenLessonFromSearch,
} from "../api/librarySearchTypes";
import type { CourseRow } from "../api/libraryTypes";
import { LibraryContinueRow } from "../library/LibraryContinueRow";
import {
  LIBRARY_SEARCH_DEBOUNCE_MS,
  LibrarySearchResults,
} from "../library/LibrarySearchResults";
import { useDebouncedValue } from "../library/useDebouncedValue";
import {
  formatRemainingMs,
  formatSourceKind,
  formatStatus,
} from "../library/format";
import { LibraryCourseTitleCell } from "../library/LibraryCourseTitleCell";
import { LibraryFirstRunEmpty } from "../library/LibraryFirstRunEmpty";
import { LibraryFolderScan } from "../library/LibraryFolderScan";
import { LibraryPastePlaylistPanel } from "../library/LibraryPastePlaylistPanel";
import { LibraryVisibilityFilter } from "../library/LibraryVisibilityFilter";
import type { LibraryVisibility } from "../api/libraryTypes";
import { libraryImportErrorMessage } from "../library/libraryImportError";
import { useLibraryCommands } from "../library/useLibraryCommands";

export type LibraryScreenProps = {
  onOpenCourse: (course: CourseRow) => void;
  onOpenLesson: (target: OpenLessonFromSearch) => void;
};

export function LibraryScreen({
  onOpenCourse,
  onOpenLesson,
}: LibraryScreenProps) {
  const tableBodyRef = useRef<HTMLTableSectionElement>(null);
  const commands = useLibraryCommands();
  const [courses, setCourses] = useState<CourseRow[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [scanningFolder, setScanningFolder] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const debouncedSearch = useDebouncedValue(
    searchQuery,
    LIBRARY_SEARCH_DEBOUNCE_MS
  );
  const [searchHits, setSearchHits] = useState<LibrarySearchHit[]>([]);
  const [searching, setSearching] = useState(false);
  const [continueRow, setContinueRow] = useState<LibraryContinueRowData | null>(
    null
  );
  const [visibility, setVisibility] = useState<LibraryVisibility>("active");
  const [pastePlaylistOpen, setPastePlaylistOpen] = useState(false);

  const refresh = useCallback(async () => {
    setError(null);
    try {
      const [rows, continueTarget] = await Promise.all([
        commands.listCourses(visibility),
        commands.getLibraryContinueRow(),
      ]);
      setCourses(rows);
      setContinueRow(continueTarget);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Could not load courses.");
    }
  }, [commands, visibility]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  useEffect(() => {
    const query = debouncedSearch.trim();
    if (!query) {
      setSearchHits([]);
      setSearching(false);
      return;
    }
    let cancelled = false;
    setSearching(true);
    void commands.searchLibrary(query).then((hits) => {
      if (!cancelled) {
        setSearchHits(hits);
        setSearching(false);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [commands, debouncedSearch]);

  function handleSearchSelect(hit: LibrarySearchHit) {
    onOpenLesson({
      courseId: hit.course_id,
      lessonId: hit.lesson_id,
      seekMs: hit.cue_start_ms ?? undefined,
    });
  }

  function handleContinueActivate() {
    if (!continueRow) {
      return;
    }
    onOpenLesson({
      courseId: continueRow.course_id,
      lessonId: continueRow.lesson_id,
      seekMs:
        continueRow.resume_position_ms > 0
          ? continueRow.resume_position_ms
          : undefined,
    });
  }

  const continueField =
    continueRow === null ? null : (
      <LibraryContinueRow
        row={continueRow}
        onActivate={handleContinueActivate}
      />
    );

  const searchField = (
    <div className="library-search">
      <label className="library-search__label" htmlFor="library-search-input">
        Search library
      </label>
      <input
        id="library-search-input"
        type="search"
        className="library-search__input"
        value={searchQuery}
        placeholder="Lesson title, path, or subtitle text"
        onChange={(event) => setSearchQuery(event.target.value)}
      />
      <LibrarySearchResults
        hits={searchHits}
        searching={searching}
        query={searchQuery}
        onSelect={handleSearchSelect}
      />
    </div>
  );

  async function handleImportFolder() {
    setBusy(true);
    setError(null);
    try {
      const path = await commands.pickFolder();
      if (!path) {
        return;
      }
      setScanningFolder(true);
      await commands.importFolder(path);
      await refresh();
    } catch (e) {
      const message = libraryImportErrorMessage(e, "Import folder failed.");
      if (message !== null) {
        setError(message);
      }
    } finally {
      setScanningFolder(false);
      setBusy(false);
    }
  }

  function handleCancelFolderScan() {
    void commands.cancelImportFolder();
  }

  async function handleImportPlaylist() {
    setBusy(true);
    setError(null);
    try {
      const path = await commands.pickPlaylistJson();
      if (path) {
        await commands.importPlaylist(path);
        await refresh();
      }
    } catch (e) {
      setError(e instanceof Error ? e.message : "Import playlist failed.");
    } finally {
      setBusy(false);
    }
  }

  async function handleCreatePlaylistFromPaste(
    courseTitle: string,
    text: string
  ) {
    setBusy(true);
    setError(null);
    try {
      await commands.createPlaylistFromPaste(courseTitle, text);
      setPastePlaylistOpen(false);
      await refresh();
    } catch (e) {
      setError(e instanceof Error ? e.message : "Paste playlist failed.");
    } finally {
      setBusy(false);
    }
  }

  const pastePlaylistPanel = pastePlaylistOpen ? (
    <LibraryPastePlaylistPanel
      busy={busy}
      onCancel={() => setPastePlaylistOpen(false)}
      onCreate={(courseTitle, text) =>
        void handleCreatePlaylistFromPaste(courseTitle, text)
      }
    />
  ) : null;

  const pastePlaylistButton = (
    <button
      type="button"
      className="library-button-secondary"
      disabled={busy}
      onClick={() => setPastePlaylistOpen(true)}
    >
      Paste playlist
    </button>
  );

  const visibilityFilter = (
    <LibraryVisibilityFilter value={visibility} onChange={setVisibility} />
  );

  const folderScanStatus = scanningFolder ? (
    <LibraryFolderScan onCancel={handleCancelFolderScan} />
  ) : null;

  if (courses.length === 0 && visibility === "active") {
    return (
      <div className="library">
        {folderScanStatus}
        <LibraryFirstRunEmpty
          busy={busy}
          error={error}
          onAddFolder={() => void handleImportFolder()}
          onAddPlaylistJson={() => void handleImportPlaylist()}
          onPastePlaylist={() => setPastePlaylistOpen(true)}
        />
        {pastePlaylistPanel}
      </div>
    );
  }

  if (courses.length === 0 && visibility === "archived") {
    return (
      <div className="library">
        {folderScanStatus}
        <div className="library-header">{visibilityFilter}</div>
        {continueField}
        {searchField}
        {error ? (
          <p className="library-error" role="alert">
            {error}
          </p>
        ) : null}
        <div className="library-empty">
          <p className="library-empty__lead">No archived courses.</p>
        </div>
      </div>
    );
  }

  return (
    <div className="library">
      {folderScanStatus}
      <div className="library-header">
        <div className="library-toolbar">
          {visibilityFilter}
          <button
            type="button"
            className="study-button"
            disabled={busy}
            onClick={() => void handleImportFolder()}
          >
            Import folder
          </button>
          <button
            type="button"
            className="library-button-secondary"
            disabled={busy}
            onClick={() => void handleImportPlaylist()}
          >
            Add playlist
          </button>
          {pastePlaylistButton}
        </div>
      </div>
      {pastePlaylistPanel}
      {continueField}
      {searchField}
      {error ? (
        <p className="library-error" role="alert">
          {error}
        </p>
      ) : null}
      <table
        className="library-table"
        onKeyDown={(event) =>
          handleTableArrowKeys(
            event,
            tableBodyRef.current,
            "tbody tr",
            ".library-course-link"
          )
        }
      >
        <thead>
          <tr>
            <th scope="col">Title</th>
            <th scope="col">Source</th>
            <th scope="col">Status</th>
            <th scope="col">Remaining</th>
            <th scope="col">Unknown duration</th>
          </tr>
        </thead>
        <tbody ref={tableBodyRef}>
          {courses.map((course) => (
            <tr key={course.course_id}>
              <td>
                <LibraryCourseTitleCell
                  course={course}
                  onOpen={() => onOpenCourse(course)}
                  resolveMediaUrl={commands.mediaUrlForPath}
                />
              </td>
              <td>{formatSourceKind(course.source_kind)}</td>
              <td>
                <span
                  className={`library-status library-status--${course.status}`}
                >
                  {formatStatus(course.status)}
                </span>
              </td>
              <td className="library-numeric">
                {formatRemainingMs(course.remaining_ms)}
              </td>
              <td className="library-numeric">
                {course.unknown_duration_count}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
