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
import { folderScanStatusLine } from "../library/folderScanProgress";
import { useFolderScanProgress } from "../library/useFolderScanProgress";
import { LibraryPastePlaylistPanel } from "../library/LibraryPastePlaylistPanel";
import { LibraryStreak } from "../library/LibraryStreak";
import { LibraryStudyHeatmap } from "../library/LibraryStudyHeatmap";
import type { StudyHeatmap52Weeks } from "../api/libraryStudyHeatmapTypes";
import { emptyStudyHeatmap52Weeks } from "../api/libraryStudyHeatmapTypes";
import { LibraryVisibilityFilter } from "../library/LibraryVisibilityFilter";
import type { LibraryVisibility } from "../api/libraryTypes";
import { libraryImportErrorMessage } from "../library/libraryImportError";
import { importCancellableFolderAtPath } from "../library/libraryCancellableFolderImport";
import { libraryFolderDropIntoLibrary } from "../library/libraryFolderDrop";
import { useLibraryCommands } from "../library/useLibraryCommands";
import searchOutline from "../theme/icons/search-outline.png";

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
  const folderScanProgress = useFolderScanProgress(scanningFolder);
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
  const [studyStreakDays, setStudyStreakDays] = useState(0);
  const [studyHeatmap, setStudyHeatmap] = useState<StudyHeatmap52Weeks>(
    emptyStudyHeatmap52Weeks
  );

  const refresh = useCallback(async () => {
    setError(null);
    try {
      const [rows, continueTarget, streakDays, heatmap] = await Promise.all([
        commands.listCourses(visibility),
        commands.getLibraryContinueRow(),
        commands.getLibraryStudyStreakDays(),
        commands.getLibraryStudyHeatmap52Weeks(),
      ]);
      setCourses(rows);
      setContinueRow(continueTarget);
      setStudyStreakDays(streakDays);
      setStudyHeatmap(heatmap);
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

  const importFolderAtPath = useCallback(
    async (path: string) => {
      setBusy(true);
      setError(null);
      try {
        const validationError = await importCancellableFolderAtPath(path, {
          validateImportFolderPath: (picked) =>
            commands.validateImportFolderPath(picked),
          importFolder: async (picked) => {
            setScanningFolder(true);
            await commands.importFolder(picked);
            await refresh();
          },
        });
        if (validationError !== null) {
          setError(validationError);
        }
      } catch (e) {
        const message = libraryImportErrorMessage(e, "Import folder failed.");
        if (message !== null) {
          setError(message);
        }
      } finally {
        setScanningFolder(false);
        setBusy(false);
      }
    },
    [commands, refresh]
  );

  useEffect(() => {
    if (typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) {
      return;
    }
    let unlisten: (() => void) | undefined;
    void (async () => {
      const { getCurrentWebview } = await import("@tauri-apps/api/webview");
      unlisten = await getCurrentWebview().onDragDropEvent((event) => {
        const payload = event.payload;
        if (payload.type !== "drop") {
          return;
        }
        const { paths } = payload;
        void (async () => {
          const dropError = await libraryFolderDropIntoLibrary(paths, {
            pathIsDirectory: (path) => commands.pathIsDirectory(path),
            importFolderAtPath,
          });
          if (dropError !== null) {
            setError(dropError);
          }
        })();
      });
    })();
    return () => {
      unlisten?.();
    };
  }, [commands, importFolderAtPath]);

  function handleSearchSelect(hit: LibrarySearchHit) {
    const target: OpenLessonFromSearch = {
      courseId: hit.course_id,
      lessonId: hit.lesson_id,
    };
    if (hit.cue_start_ms !== null) {
      target.seekMs = hit.cue_start_ms;
    }
    onOpenLesson(target);
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

  const studyStatsField = (
    <div className="library-study-stats">
      <LibraryStreak days={studyStreakDays} />
      <LibraryStudyHeatmap heatmap={studyHeatmap} />
    </div>
  );

  const searchField = (
    <div className="library-search">
      <label className="library-search__label" htmlFor="library-search-input">
        Search library
      </label>
      <div className="library-search__field theme-icon-label">
        <img
          src={searchOutline}
          alt=""
          aria-hidden
          className="theme-icon-label__icon"
        />
        <input
          id="library-search-input"
          type="search"
          className="library-search__input"
          value={searchQuery}
          placeholder="Lesson title, path, subtitle, or note text"
          onChange={(event) => setSearchQuery(event.target.value)}
        />
      </div>
      <LibrarySearchResults
        hits={searchHits}
        searching={searching}
        query={searchQuery}
        onSelect={handleSearchSelect}
      />
    </div>
  );

  async function handleImportFolder() {
    const path = await commands.pickFolder();
    if (!path) {
      return;
    }
    await importFolderAtPath(path);
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
    <LibraryFolderScan
      mode="import"
      statusLine={folderScanStatusLine("import", folderScanProgress)}
      onCancel={handleCancelFolderScan}
    />
  ) : null;

  if (courses.length === 0 && visibility === "active") {
    return (
      <div className="library">
        {folderScanStatus}
        {studyStatsField}
        <LibraryFirstRunEmpty
          busy={busy}
          error={error}
          onAddFolder={() => void handleImportFolder()}
        />
      </div>
    );
  }

  if (courses.length === 0 && visibility === "archived") {
    return (
      <div className="library">
        {folderScanStatus}
        <div className="library-header">
          {studyStatsField}
          {visibilityFilter}
        </div>
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
        {studyStatsField}
        <div className="library-toolbar">
          {visibilityFilter}
          <button
            type="button"
            className="library-button-primary"
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
      <div className="library-table-panel">
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
    </div>
  );
}
