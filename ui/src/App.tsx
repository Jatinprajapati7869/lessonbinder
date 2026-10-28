import { useCallback, useEffect, useState } from "react";

import {
  fetchStudyBarState,
  fetchVaultBootstrap,
  launchAnkiProfile,
  retryOpenVault,
} from "./api/tauri";
import type { LessonRow, UpNextTarget } from "./api/courseTypes";
import type { CourseRow } from "./api/libraryTypes";
import type { AnkiLaunchStatus, DueSummary, VaultBootstrap } from "./api/types";
import { CaptureCommandsProvider } from "./capture/CaptureCommandsContext";
import type { OpenLessonFromSearch } from "./api/librarySearchTypes";
import { CourseCommandsProvider } from "./course/CourseCommandsContext";
import { useCourseCommands } from "./course/useCourseCommands";
import { LibraryCommandsProvider } from "./library/LibraryCommandsContext";
import { useLibraryCommands } from "./library/useLibraryCommands";
import { ReviewCommandsProvider } from "./review/ReviewCommandsContext";
import { RoadmapCommandsProvider } from "./roadmap/RoadmapCommandsContext";
import { CourseScreen } from "./screens/CourseScreen";
import { LibraryScreen } from "./screens/LibraryScreen";
import { isDocumentLessonPath } from "./lesson/lessonDocument";
import { DocumentLessonScreen } from "./screens/DocumentLessonScreen";
import { PlayerScreen } from "./screens/PlayerScreen";
import { ReviewScreen } from "./screens/ReviewScreen";
import { RoadmapScreen } from "./screens/RoadmapScreen";
import { SettingsScreen } from "./screens/SettingsScreen";
import { SettingsCommandsProvider } from "./settings/SettingsCommandsContext";
import { VaultUnavailableScreen } from "./screens/VaultUnavailableScreen";
import { AppShell, type AppRoute } from "./shell/AppShell";

const zeroDue: DueSummary = { new: 0, learning: 0, review: 0 };

const unavailableAnki: AnkiLaunchStatus = {
  kind: "unavailable",
  message:
    "Anki is not installed. Install Anki or check Settings for the executable path.",
};

export type AuraVaultAppProps = {
  fetchVaultBootstrapImpl?: () => Promise<VaultBootstrap>;
  fetchStudyBarStateImpl?: () => Promise<{
    due: DueSummary;
    anki: AnkiLaunchStatus;
  }>;
  launchAnkiProfileImpl?: () => Promise<void>;
};

function MainContent({
  route,
  openCourse,
  playerLesson,
  playerSeekMs,
  onOpenCourse,
  onBackToLibrary,
  onPlayLesson,
  onBackFromPlayer,
  onOpenInAnkiDesktop,
  desktopAnkiAvailable,
  desktopLaunchError,
  desktopLaunchBusy,
  onReviewQueueChanged,
  reviewCourseId,
  onReviewCourse,
  onBackFromReview,
}: {
  route: AppRoute;
  openCourse: CourseRow | null;
  playerLesson: LessonRow | null;
  playerSeekMs: number | null;
  onOpenCourse: (course: CourseRow) => void;
  onBackToLibrary: () => void;
  onPlayLesson: (lesson: LessonRow, seekMs?: number) => void;
  onBackFromPlayer: () => void;
  onOpenInAnkiDesktop: () => void;
  desktopAnkiAvailable: boolean;
  desktopLaunchError: string | null;
  desktopLaunchBusy: boolean;
  onReviewQueueChanged: () => void;
  reviewCourseId: string | null;
  onReviewCourse: (courseId: string) => void;
  onBackFromReview: () => void;
}) {
  const courseCommands = useCourseCommands();
  const libraryCommands = useLibraryCommands();

  const handlePlayUpNext = useCallback(
    async (target: UpNextTarget) => {
      if (!openCourse) {
        return;
      }
      const detail = await courseCommands.getCourseDetail(openCourse.course_id);
      const lesson = detail.lessons.find(
        (l) => l.lesson_id === target.lesson_id
      );
      if (lesson) {
        onPlayLesson(lesson);
      }
    },
    [courseCommands, onPlayLesson, openCourse]
  );

  const handleOpenLessonFromSearch = useCallback(
    async (target: OpenLessonFromSearch) => {
      const courses = await libraryCommands.listCourses("active");
      const course = courses.find((c) => c.course_id === target.courseId);
      if (!course) {
        return;
      }
      const detail = await courseCommands.getCourseDetail(target.courseId);
      const lesson = detail.lessons.find(
        (l) => l.lesson_id === target.lessonId
      );
      if (!lesson) {
        return;
      }
      onOpenCourse(course);
      onPlayLesson(lesson, target.seekMs);
    },
    [courseCommands, libraryCommands, onOpenCourse, onPlayLesson]
  );

  if (playerLesson && openCourse) {
    if (isDocumentLessonPath(playerLesson.local_media_path)) {
      return (
        <DocumentLessonScreen lesson={playerLesson} onBack={onBackFromPlayer} />
      );
    }
    return (
      <PlayerScreen
        lesson={playerLesson}
        courseId={openCourse.course_id}
        initialSeekMs={playerSeekMs}
        onBack={onBackFromPlayer}
        onPlayUpNext={(target) => void handlePlayUpNext(target)}
      />
    );
  }
  if (openCourse) {
    return (
      <CourseScreen
        course={openCourse}
        onBack={onBackToLibrary}
        onPlayLesson={onPlayLesson}
        onReviewThisCourse={onReviewCourse}
      />
    );
  }
  if (route === "review") {
    return (
      <ReviewScreen
        onBackToLibrary={onBackFromReview}
        onOpenInAnkiDesktop={onOpenInAnkiDesktop}
        desktopAnkiAvailable={desktopAnkiAvailable}
        desktopLaunchError={desktopLaunchError}
        desktopLaunchBusy={desktopLaunchBusy}
        onQueueChanged={onReviewQueueChanged}
        courseId={reviewCourseId}
      />
    );
  }
  if (route === "roadmap") {
    return <RoadmapScreen />;
  }
  if (route === "settings") {
    return <SettingsScreen />;
  }
  return (
    <LibraryScreen
      onOpenCourse={onOpenCourse}
      onOpenLesson={(target) => void handleOpenLessonFromSearch(target)}
    />
  );
}

export function AuraVaultApp({
  fetchVaultBootstrapImpl = fetchVaultBootstrap,
  fetchStudyBarStateImpl = fetchStudyBarState,
  launchAnkiProfileImpl = launchAnkiProfile,
}: AuraVaultAppProps = {}) {
  const [bootstrap, setBootstrap] = useState<VaultBootstrap | null>(null);
  const [bootstrapBusy, setBootstrapBusy] = useState(false);
  const [route, setRoute] = useState<AppRoute>("library");
  const [openCourse, setOpenCourse] = useState<CourseRow | null>(null);
  const [playerLesson, setPlayerLesson] = useState<LessonRow | null>(null);
  const [playerSeekMs, setPlayerSeekMs] = useState<number | null>(null);
  const [reviewCourseId, setReviewCourseId] = useState<string | null>(null);
  const [due, setDue] = useState<DueSummary>(zeroDue);
  const [anki, setAnki] = useState<AnkiLaunchStatus>(unavailableAnki);
  const [desktopLaunchBusy, setDesktopLaunchBusy] = useState(false);
  const [desktopLaunchError, setDesktopLaunchError] = useState<string | null>(
    null
  );

  const refreshStudyBar = useCallback(async () => {
    try {
      const bar = await fetchStudyBarStateImpl();
      setDue(bar.due);
      setAnki(bar.anki);
    } catch {
      setDue(zeroDue);
      setAnki(unavailableAnki);
    }
  }, [fetchStudyBarStateImpl]);

  const loadBootstrap = useCallback(async () => {
    const next = await fetchVaultBootstrapImpl();
    setBootstrap(next);
    return next;
  }, [fetchVaultBootstrapImpl]);

  useEffect(() => {
    void loadBootstrap();
  }, [loadBootstrap]);

  useEffect(() => {
    if (!bootstrap?.ready) {
      return;
    }
    void refreshStudyBar();
  }, [bootstrap?.ready, refreshStudyBar]);

  async function handleRetryVault() {
    setBootstrapBusy(true);
    try {
      const next = await retryOpenVault();
      setBootstrap(next);
    } catch (e) {
      setBootstrap({
        ready: false,
        path: bootstrap?.path ?? "",
        error:
          e instanceof Error ? e.message : "Could not open vault database.",
      });
    } finally {
      setBootstrapBusy(false);
    }
  }

  function handleNavigate(next: AppRoute) {
    setRoute(next);
    setOpenCourse(null);
    setPlayerLesson(null);
    setPlayerSeekMs(null);
    setReviewCourseId(null);
  }

  function handleStartReview() {
    setReviewCourseId(null);
    setOpenCourse(null);
    setPlayerLesson(null);
    setPlayerSeekMs(null);
    setRoute("review");
  }

  function handleReviewCourse(courseId: string) {
    setReviewCourseId(courseId);
    setPlayerLesson(null);
    setPlayerSeekMs(null);
    setRoute("review");
  }

  function handleBackFromReview() {
    setReviewCourseId(null);
    if (openCourse) {
      setRoute("library");
    } else {
      setOpenCourse(null);
      setPlayerLesson(null);
      setPlayerSeekMs(null);
      setRoute("library");
    }
  }

  function handlePlayLesson(lesson: LessonRow, seekMs?: number) {
    setPlayerSeekMs(seekMs ?? null);
    setPlayerLesson(lesson);
  }

  useEffect(() => {
    function onKeyDown(event: KeyboardEvent) {
      if (event.key !== "Escape" || event.defaultPrevented) {
        return;
      }
      if (document.querySelector(".capture-card[role='dialog']")) {
        return;
      }
      if (playerLesson) {
        event.preventDefault();
        setPlayerLesson(null);
        return;
      }
      if (openCourse) {
        event.preventDefault();
        setOpenCourse(null);
        return;
      }
      if (route === "review" || route === "settings") {
        event.preventDefault();
        setRoute("library");
      }
    }
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [openCourse, playerLesson, route]);

  async function handleOpenInAnkiDesktop() {
    setDesktopLaunchError(null);
    setDesktopLaunchBusy(true);
    try {
      await launchAnkiProfileImpl();
    } catch (e) {
      setDesktopLaunchError(
        e instanceof Error ? e.message : "Could not launch Anki."
      );
    } finally {
      setDesktopLaunchBusy(false);
    }
  }

  if (!bootstrap) {
    return null;
  }

  if (!bootstrap.ready) {
    return (
      <VaultUnavailableScreen
        path={bootstrap.path}
        error={bootstrap.error}
        busy={bootstrapBusy}
        onRetry={() => void handleRetryVault()}
      />
    );
  }

  const desktopAnkiAvailable = anki.kind === "available";
  const screenTitle =
    playerLesson?.title ??
    openCourse?.title ??
    (route === "review"
      ? "Review"
      : route === "roadmap"
        ? "Roadmap"
        : route === "settings"
          ? "Settings"
          : "Library");

  return (
    <LibraryCommandsProvider>
      <CourseCommandsProvider>
        <CaptureCommandsProvider>
          <RoadmapCommandsProvider>
            <ReviewCommandsProvider>
              <SettingsCommandsProvider>
                <AppShell
                  route={route}
                  screenTitle={screenTitle}
                  onNavigate={handleNavigate}
                  due={due}
                  anki={anki}
                  onStartReview={handleStartReview}
                >
                  <MainContent
                    route={route}
                    openCourse={openCourse}
                    playerLesson={playerLesson}
                    playerSeekMs={playerSeekMs}
                    onOpenCourse={setOpenCourse}
                    onBackToLibrary={() => {
                      setOpenCourse(null);
                      setPlayerLesson(null);
                      setPlayerSeekMs(null);
                      setReviewCourseId(null);
                      setRoute("library");
                    }}
                    onPlayLesson={handlePlayLesson}
                    onBackFromPlayer={() => {
                      setPlayerLesson(null);
                      setPlayerSeekMs(null);
                    }}
                    onOpenInAnkiDesktop={() => void handleOpenInAnkiDesktop()}
                    desktopAnkiAvailable={desktopAnkiAvailable}
                    desktopLaunchError={desktopLaunchError}
                    desktopLaunchBusy={desktopLaunchBusy}
                    onReviewQueueChanged={() => void refreshStudyBar()}
                    reviewCourseId={reviewCourseId}
                    onReviewCourse={handleReviewCourse}
                    onBackFromReview={handleBackFromReview}
                  />
                </AppShell>
              </SettingsCommandsProvider>
            </ReviewCommandsProvider>
          </RoadmapCommandsProvider>
        </CaptureCommandsProvider>
      </CourseCommandsProvider>
    </LibraryCommandsProvider>
  );
}

export default function App() {
  return <AuraVaultApp />;
}
