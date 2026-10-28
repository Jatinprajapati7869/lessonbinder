import { useCallback, useEffect, useRef, useState } from "react";

import type {
  LessonRow,
  MediaChapter,
  SubtitleCue,
  UpNextTarget,
  WatchedRange,
} from "../api/courseTypes";
import type { VaultSettings } from "../api/settingsTypes";
import { invokeErrorMessage } from "../api/invokeError";
import {
  startMpvLessonHandoff,
  stopMpvLessonHandoff,
  updateMpvLessonEmbedRect,
  videoEmbedRect,
} from "../api/mpvCommands";
import { useCourseCommands } from "../course/useCourseCommands";
import { formatDurationMs, watchedPercent } from "../library/format";
import { isDocumentLessonPath } from "../lesson/lessonDocument";
import { PlayerCueList } from "../player/PlayerCueList";
import { PlayerChapterList } from "../player/PlayerChapterList";
import { PlayerLessonResources } from "../player/PlayerLessonResources";
import {
  PlayerTranscribeControl,
  type TranscribeJobState,
} from "../player/PlayerTranscribeControl";
import {
  isPictureInPictureSupported,
  toggleVideoPictureInPicture,
} from "../player/playerPictureInPicture";
import {
  playerMainClassName,
  playerScreenClassName,
} from "../player/playerZen";
import {
  PLAYER_THEATER_CONTROLS_HIDE_MS,
  playerFrameBarClassName,
} from "../player/playerTheater";
import {
  playerMarkCompleteAccessibleName,
  shouldHandlePlayerMarkComplete,
} from "../player/playerCompleteKeyboard";
import { PLAYER_EXIT_THEATER_OR_ZEN_KEY } from "../player/playerModeKeyboard";
import {
  clampSeekMs,
  playerArrowSeekDeltaMs,
  PLAYER_SEEK_STEP_MS,
  shouldHandlePlayerArrowSeek,
} from "../player/playerSeekKeyboard";
import { captureVideoFrameJpeg } from "../player/captureVideoFrameJpeg";
import {
  mapPlayTickToRecordPlayback,
  PLAYBACK_WATCH_INTERVAL_MS,
  watchTickAnchorAfterSeek,
  watchedMsFromRanges,
} from "../player/playbackWatch";
import { useSettingsCommands } from "../settings/useSettingsCommands";
import {
  normalizePlaybackSpeed,
  type PlaybackSpeedChoice,
} from "../player/playerPlaybackSpeed";
import { PlayerPlaybackSpeedSelect } from "../player/PlayerPlaybackSpeedSelect";
import {
  applyPlayerVolumeToVideo,
  PlayerVolumeControl,
} from "../player/PlayerVolumeControl";
import { MPV_NOT_ON_PATH_MESSAGE } from "../player/mpvHandoffMessage";
import { normalizePlayerVolume } from "../player/playerVolume";
import { LessonCardLinksList } from "../lesson/LessonCardLinksList";
import { LessonNotesColumn } from "../lesson/LessonNotesColumn";
import { CaptureCardPanel } from "./CaptureCardPanel";
import { AudioClipCardPanel } from "./AudioClipCardPanel";
import { SubtitleCueCardPanel } from "./SubtitleCueCardPanel";
import readerOutline from "../theme/icons/reader-outline.png";

export const RESUME_SAVE_INTERVAL_MS = 3000;

export type PlayerScreenProps = {
  lesson: LessonRow;
  courseId: string;
  initialSeekMs?: number | null;
  onBack: () => void;
  onPlayUpNext?: (target: UpNextTarget) => void;
};

type UnplayableProps = {
  lesson: LessonRow;
  lessonStatus: LessonRow["status"];
  courseId: string;
  mediaPath: string | null;
  onBack: () => void;
};

function PlayerUnplayable({
  lesson,
  lessonStatus,
  courseId,
  mediaPath,
  onBack,
}: UnplayableProps) {
  const commands = useCourseCommands();
  const [busy, setBusy] = useState(false);

  async function handleRecovery() {
    setBusy(true);
    try {
      if (lessonStatus === "broken") {
        await commands.rescanCourse(courseId);
      } else {
        await commands.markLessonBroken(lesson.lesson_id);
      }
      onBack();
    } finally {
      setBusy(false);
    }
  }

  const recoveryLabel =
    lessonStatus === "broken" ? "Rescan course" : "Mark as broken";

  return (
    <div className="player-screen">
      <button
        type="button"
        className="library-button-secondary"
        onClick={onBack}
      >
        Back to course
      </button>
      <h1 className="screen-title">{lesson.title}</h1>
      <p className="player-unplayable__lead" role="alert">
        No playable file for this lesson. The file may have moved or been
        deleted.
      </p>
      {mediaPath ? (
        <p className="player-unplayable__path" data-testid="player-media-path">
          {mediaPath}
        </p>
      ) : null}
      <button
        type="button"
        className="library-button-primary"
        disabled={busy}
        onClick={() => void handleRecovery()}
      >
        {recoveryLabel}
      </button>
    </div>
  );
}

export function PlayerScreen({
  lesson,
  courseId,
  initialSeekMs = null,
  onBack,
  onPlayUpNext,
}: PlayerScreenProps) {
  const commands = useCourseCommands();
  const settingsCommands = useSettingsCommands();
  const videoRef = useRef<HTMLVideoElement>(null);
  const captureButtonRef = useRef<HTMLButtonElement>(null);
  const [playbackStatus, setPlaybackStatus] = useState(lesson.status);
  const [mediaReady, setMediaReady] = useState(
    !lesson.local_media_path || lesson.status === "broken"
  );
  const [mediaSrc, setMediaSrc] = useState<string | null>(null);
  const [playing, setPlaying] = useState(false);
  const [loadFailed, setLoadFailed] = useState(false);
  const [mpvActive, setMpvActive] = useState(false);
  const [mpvHandoffMessage, setMpvHandoffMessage] = useState<string | null>(
    null
  );
  const mpvHandoffAttemptedRef = useRef(false);
  const previousLessonIdRef = useRef(lesson.lesson_id);
  const [captureOpen, setCaptureOpen] = useState(false);
  const [audioClipOpen, setAudioClipOpen] = useState(false);
  const [captureFrame, setCaptureFrame] = useState<string | null>(null);
  const [captureTimestampMs, setCaptureTimestampMs] = useState(0);
  const [playbackMs, setPlaybackMs] = useState(0);
  const [durationMs, setDurationMs] = useState(lesson.duration_ms ?? 0);
  const [cues, setCues] = useState<SubtitleCue[]>([]);
  const [chapters, setChapters] = useState<MediaChapter[]>([]);
  const [upNext, setUpNext] = useState<UpNextTarget | null>(null);
  const [subtitleCardCue, setSubtitleCardCue] = useState<SubtitleCue | null>(
    null
  );
  const [watchedRanges, setWatchedRanges] = useState<WatchedRange[]>([]);
  const [transcriptionModelPath, setTranscriptionModelPath] = useState("");
  const [transcribeState, setTranscribeState] =
    useState<TranscribeJobState>("idle");
  const [transcribeError, setTranscribeError] = useState<string | null>(null);
  const [thumbnailBusy, setThumbnailBusy] = useState(false);
  const [thumbnailError, setThumbnailError] = useState<string | null>(null);
  const [lessonFileOpenError, setLessonFileOpenError] = useState<string | null>(
    null
  );
  const [theaterMode, setTheaterMode] = useState(false);
  const [frameBarVisible, setFrameBarVisible] = useState(true);
  const [zenMode, setZenMode] = useState(false);
  const [playbackSpeed, setPlaybackSpeed] = useState<PlaybackSpeedChoice>(1);
  const [playerVolume, setPlayerVolume] = useState(1);
  const [playerMuted, setPlayerMuted] = useState(false);
  const pipSupported = isPictureInPictureSupported();
  const lastWatchTickMsRef = useRef<number | null>(null);
  const playbackMsRef = useRef(0);
  const pendingSeekMsRef = useRef<number | null>(initialSeekMs);
  const theaterModeRef = useRef(theaterMode);
  const zenModeRef = useRef(zenMode);
  const frameBarHideRef = useRef<number | null>(null);

  useEffect(() => {
    pendingSeekMsRef.current = initialSeekMs;
    playbackMsRef.current = lesson.resume_position_ms;
  }, [initialSeekMs, lesson.lesson_id, lesson.resume_position_ms]);

  useEffect(() => {
    setPlaybackStatus(lesson.status);
    setMediaReady(!lesson.local_media_path || lesson.status === "broken");
  }, [lesson.lesson_id, lesson.local_media_path, lesson.status]);

  useEffect(() => {
    if (!lesson.local_media_path || lesson.status === "broken") {
      return;
    }
    let cancelled = false;
    void commands.guardLessonMediaForResume(lesson.lesson_id).then((status) => {
      if (cancelled) {
        return;
      }
      setPlaybackStatus(status);
      setMediaReady(true);
    });
    return () => {
      cancelled = true;
    };
  }, [commands, lesson.lesson_id, lesson.local_media_path, lesson.status]);

  useEffect(() => {
    theaterModeRef.current = theaterMode;
  }, [theaterMode]);

  useEffect(() => {
    zenModeRef.current = zenMode;
  }, [zenMode]);

  const exitZenMode = useCallback(() => {
    setZenMode(false);
  }, []);

  const scheduleFrameBarHide = useCallback(() => {
    if (frameBarHideRef.current !== null) {
      window.clearTimeout(frameBarHideRef.current);
    }
    frameBarHideRef.current = window.setTimeout(() => {
      setFrameBarVisible(false);
    }, PLAYER_THEATER_CONTROLS_HIDE_MS);
  }, []);

  const revealFrameBar = useCallback(() => {
    setFrameBarVisible(true);
    scheduleFrameBarHide();
  }, [scheduleFrameBarHide]);

  const enterZenMode = useCallback(() => {
    setTheaterMode(false);
    setFrameBarVisible(true);
    if (frameBarHideRef.current !== null) {
      window.clearTimeout(frameBarHideRef.current);
      frameBarHideRef.current = null;
    }
    setZenMode(true);
    scheduleFrameBarHide();
  }, [scheduleFrameBarHide]);

  const toggleZenMode = useCallback(() => {
    if (zenModeRef.current) {
      exitZenMode();
      return;
    }
    enterZenMode();
  }, [enterZenMode, exitZenMode]);

  const revealTheaterControls = useCallback(() => {
    revealFrameBar();
  }, [revealFrameBar]);

  const exitTheaterMode = useCallback(() => {
    setTheaterMode(false);
    setFrameBarVisible(true);
    if (frameBarHideRef.current !== null) {
      window.clearTimeout(frameBarHideRef.current);
      frameBarHideRef.current = null;
    }
  }, []);

  const toggleTheaterMode = useCallback(() => {
    if (theaterModeRef.current) {
      exitTheaterMode();
      return;
    }
    setZenMode(false);
    setTheaterMode(true);
    setFrameBarVisible(true);
    scheduleFrameBarHide();
  }, [exitTheaterMode, scheduleFrameBarHide]);

  useEffect(() => {
    return () => {
      if (frameBarHideRef.current !== null) {
        window.clearTimeout(frameBarHideRef.current);
      }
    };
  }, []);

  useEffect(() => {
    if (
      !mediaReady ||
      !lesson.local_media_path ||
      playbackStatus === "broken"
    ) {
      return;
    }
    setFrameBarVisible(true);
    scheduleFrameBarHide();
  }, [lesson.lesson_id, mediaReady, playbackStatus, scheduleFrameBarHide]);

  useEffect(() => {
    function onTheaterKeyDown(event: KeyboardEvent) {
      if (!theaterModeRef.current || zenModeRef.current) {
        return;
      }
      if (
        captureOpenRef.current ||
        audioClipOpenRef.current ||
        subtitleCardOpenRef.current
      ) {
        return;
      }
      if (event.key === PLAYER_EXIT_THEATER_OR_ZEN_KEY) {
        event.preventDefault();
        event.stopImmediatePropagation();
        exitTheaterMode();
        return;
      }
      revealTheaterControls();
    }
    window.addEventListener("keydown", onTheaterKeyDown, true);
    return () => window.removeEventListener("keydown", onTheaterKeyDown, true);
  }, [exitTheaterMode, revealTheaterControls]);

  useEffect(() => {
    function onZenKeyDown(event: KeyboardEvent) {
      if (!zenModeRef.current) {
        return;
      }
      if (
        captureOpenRef.current ||
        audioClipOpenRef.current ||
        subtitleCardOpenRef.current
      ) {
        return;
      }
      if (event.key !== PLAYER_EXIT_THEATER_OR_ZEN_KEY) {
        return;
      }
      event.preventDefault();
      event.stopImmediatePropagation();
      exitZenMode();
    }
    window.addEventListener("keydown", onZenKeyDown, true);
    return () => window.removeEventListener("keydown", onZenKeyDown, true);
  }, [exitZenMode]);

  const unplayable =
    playbackStatus === "broken" || !lesson.local_media_path || loadFailed;

  useEffect(() => {
    if (previousLessonIdRef.current === lesson.lesson_id) {
      return;
    }
    previousLessonIdRef.current = lesson.lesson_id;
    mpvHandoffAttemptedRef.current = false;
    setMpvActive(false);
    setMpvHandoffMessage(null);
    void stopMpvLessonHandoff();
  }, [lesson.lesson_id]);

  useEffect(() => {
    return () => {
      void stopMpvLessonHandoff();
    };
  }, []);

  useEffect(() => {
    if (!mpvActive) {
      return;
    }
    const video = videoRef.current;
    if (!video) {
      return;
    }
    const syncRect = () => {
      void updateMpvLessonEmbedRect(videoEmbedRect(video));
    };
    const observer = new ResizeObserver(syncRect);
    observer.observe(video);
    window.addEventListener("resize", syncRect);
    return () => {
      observer.disconnect();
      window.removeEventListener("resize", syncRect);
    };
  }, [mpvActive]);

  useEffect(() => {
    if (
      !mediaReady ||
      !lesson.local_media_path ||
      playbackStatus === "broken"
    ) {
      return;
    }
    let cancelled = false;
    setLoadFailed(false);
    commands
      .mediaUrlForPath(lesson.local_media_path)
      .then((url) => {
        if (!cancelled) {
          setMediaSrc(url);
        }
      })
      .catch(() => {
        if (!cancelled) {
          setLoadFailed(true);
        }
      });
    return () => {
      cancelled = true;
    };
  }, [commands, lesson.local_media_path, mediaReady, playbackStatus]);

  useEffect(() => {
    let cancelled = false;
    void settingsCommands.getVaultSettings().then((settings) => {
      if (!cancelled) {
        setPlaybackSpeed(normalizePlaybackSpeed(settings.playback_speed));
        setPlayerVolume(normalizePlayerVolume(settings.player_volume));
        setPlayerMuted(settings.player_muted);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [settingsCommands]);

  useEffect(() => {
    const video = videoRef.current;
    if (video) {
      video.playbackRate = playbackSpeed;
      applyPlayerVolumeToVideo(video, playerVolume, playerMuted);
    }
  }, [playbackSpeed, playerVolume, playerMuted, mediaSrc]);

  const persistVaultSettings = useCallback(
    async (
      patch: Partial<
        Pick<VaultSettings, "playback_speed" | "player_volume" | "player_muted">
      >
    ) => {
      try {
        const settings = await settingsCommands.getVaultSettings();
        await settingsCommands.saveVaultSettings({ ...settings, ...patch });
      } catch {
        // keep local values when persistence fails
      }
    },
    [settingsCommands]
  );

  const handlePlaybackSpeedChange = useCallback(
    (speed: PlaybackSpeedChoice) => {
      setPlaybackSpeed(speed);
      const video = videoRef.current;
      if (video) {
        video.playbackRate = speed;
      }
      void persistVaultSettings({ playback_speed: speed });
    },
    [persistVaultSettings]
  );

  const handlePlayerVolumeChange = useCallback(
    (volume: number) => {
      const normalized = normalizePlayerVolume(volume);
      setPlayerVolume(normalized);
      const video = videoRef.current;
      if (video) {
        applyPlayerVolumeToVideo(video, normalized, playerMuted);
      }
      void persistVaultSettings({ player_volume: normalized });
    },
    [persistVaultSettings, playerMuted]
  );

  const handlePlayerMutedChange = useCallback(
    (muted: boolean) => {
      setPlayerMuted(muted);
      const video = videoRef.current;
      if (video) {
        applyPlayerVolumeToVideo(video, playerVolume, muted);
      }
      void persistVaultSettings({ player_muted: muted });
    },
    [persistVaultSettings, playerVolume]
  );

  const refreshCues = useCallback(async () => {
    const loaded = await commands.listLessonCues(lesson.lesson_id);
    setCues(loaded);
  }, [commands, lesson.lesson_id]);

  const refreshChapters = useCallback(async () => {
    const loaded = await commands.listLessonChapters(lesson.lesson_id);
    setChapters(loaded.chapters);
  }, [commands, lesson.lesson_id]);

  const refreshUpNext = useCallback(async () => {
    const next = await commands.upNext(lesson.lesson_id);
    setUpNext(next);
  }, [commands, lesson.lesson_id]);

  useEffect(() => {
    void refreshCues();
  }, [refreshCues]);

  useEffect(() => {
    void refreshChapters();
  }, [refreshChapters]);

  useEffect(() => {
    void refreshUpNext();
  }, [refreshUpNext]);

  useEffect(() => {
    let cancelled = false;
    void commands.getTranscriptionModelPath().then((path) => {
      if (!cancelled) {
        setTranscriptionModelPath(path);
      }
    });
    return () => {
      cancelled = true;
    };
  }, [commands]);

  const refreshWatchedRanges = useCallback(async () => {
    const ranges = await commands.listLessonWatchedRanges(lesson.lesson_id);
    setWatchedRanges(ranges);
  }, [commands, lesson.lesson_id]);

  useEffect(() => {
    void refreshWatchedRanges();
  }, [refreshWatchedRanges]);

  const flushWatchTick = useCallback(() => {
    const video = videoRef.current;
    if (!video) {
      return;
    }
    const currentMs = Math.floor(video.currentTime * 1000);
    const { nextLastTickMs, recordPlayback } = mapPlayTickToRecordPlayback(
      lastWatchTickMsRef.current,
      currentMs
    );
    lastWatchTickMsRef.current = nextLastTickMs;
    if (!recordPlayback) {
      return;
    }
    void commands
      .recordPlayback(
        lesson.lesson_id,
        recordPlayback.startMs,
        recordPlayback.endMs
      )
      .then((ranges) => {
        setWatchedRanges(ranges);
      });
  }, [commands, lesson.lesson_id]);

  const persistPosition = useCallback(() => {
    const video = videoRef.current;
    if (!video) {
      return;
    }
    const positionMs = Math.floor(video.currentTime * 1000);
    void commands.saveLessonResume(lesson.lesson_id, positionMs);
    playbackMsRef.current = positionMs;
  }, [commands, lesson.lesson_id]);

  useEffect(() => {
    if (!playing) {
      return;
    }
    const id = window.setInterval(persistPosition, RESUME_SAVE_INTERVAL_MS);
    return () => window.clearInterval(id);
  }, [playing, persistPosition]);

  useEffect(() => {
    if (!playing) {
      return;
    }
    const id = window.setInterval(flushWatchTick, PLAYBACK_WATCH_INTERVAL_MS);
    return () => window.clearInterval(id);
  }, [playing, flushWatchTick]);

  function handleLoadedMetadata() {
    const video = videoRef.current;
    if (!video) {
      return;
    }
    const fromVideo = Math.floor(video.duration * 1000);
    if (fromVideo > 0) {
      setDurationMs(fromVideo);
    }
    const seekMs = pendingSeekMsRef.current ?? lesson.resume_position_ms;
    pendingSeekMsRef.current = null;
    if (seekMs > 0) {
      video.currentTime = seekMs / 1000;
      setPlaybackMs(seekMs);
      playbackMsRef.current = seekMs;
      lastWatchTickMsRef.current = watchTickAnchorAfterSeek(seekMs);
    }
  }

  const handleVideoError = useCallback(() => {
    if (mpvHandoffAttemptedRef.current || !lesson.local_media_path) {
      return;
    }
    mpvHandoffAttemptedRef.current = true;
    const video = videoRef.current;
    if (!video) {
      return;
    }
    const resumeMs = pendingSeekMsRef.current ?? lesson.resume_position_ms ?? 0;
    void startMpvLessonHandoff(
      lesson.local_media_path,
      videoEmbedRect(video),
      resumeMs / 1000
    )
      .then(() => {
        setMpvActive(true);
        setMpvHandoffMessage(null);
        setPlaying(true);
      })
      .catch((error: unknown) => {
        setMpvHandoffMessage(
          invokeErrorMessage(error, MPV_NOT_ON_PATH_MESSAGE)
        );
      });
  }, [lesson.local_media_path, lesson.resume_position_ms]);

  function handleTimeUpdate() {
    const video = videoRef.current;
    if (!video) {
      return;
    }
    const nextMs = Math.floor(video.currentTime * 1000);
    setPlaybackMs(nextMs);
    playbackMsRef.current = nextMs;
  }

  function seekToMs(nextMs: number) {
    const video = videoRef.current;
    if (!video) {
      return;
    }
    video.currentTime = nextMs / 1000;
    setPlaybackMs(nextMs);
    playbackMsRef.current = nextMs;
    lastWatchTickMsRef.current = watchTickAnchorAfterSeek(nextMs);
    void commands.saveLessonResume(lesson.lesson_id, nextMs);
  }

  function handleScrubChange(event: React.ChangeEvent<HTMLInputElement>) {
    const nextMs = Number.parseInt(event.target.value, 10);
    seekToMs(nextMs);
  }

  const togglePlay = useCallback(() => {
    const video = videoRef.current;
    if (!video) {
      return;
    }
    if (video.paused) {
      lastWatchTickMsRef.current = watchTickAnchorAfterSeek(
        Math.floor(video.currentTime * 1000)
      );
      persistPosition();
      void video.play();
      setPlaying(true);
    } else {
      video.pause();
      setPlaying(false);
      flushWatchTick();
      persistPosition();
    }
  }, [flushWatchTick, persistPosition]);

  useEffect(() => {
    const lessonId = lesson.lesson_id;
    return () => {
      void commands.saveLessonResume(lessonId, playbackMsRef.current);
    };
  }, [commands, lesson.lesson_id]);

  const captureOpenRef = useRef(captureOpen);
  const audioClipOpenRef = useRef(audioClipOpen);
  const subtitleCardOpenRef = useRef(subtitleCardCue !== null);
  const togglePlayRef = useRef(togglePlay);
  const seekToMsRef = useRef(seekToMs);

  const handleMarkComplete = useCallback(async () => {
    await commands.markLessonComplete(lesson.lesson_id);
    onBack();
  }, [commands, lesson.lesson_id, onBack]);

  const markCompleteRef = useRef(handleMarkComplete);

  useEffect(() => {
    captureOpenRef.current = captureOpen;
    audioClipOpenRef.current = audioClipOpen;
    subtitleCardOpenRef.current = subtitleCardCue !== null;
    togglePlayRef.current = togglePlay;
    seekToMsRef.current = seekToMs;
    markCompleteRef.current = handleMarkComplete;
  }, [
    captureOpen,
    audioClipOpen,
    subtitleCardCue,
    togglePlay,
    seekToMs,
    handleMarkComplete,
  ]);

  useEffect(() => {
    function shortcutsBlocked() {
      return (
        captureOpenRef.current ||
        audioClipOpenRef.current ||
        subtitleCardOpenRef.current
      );
    }

    function onKeyDown(event: KeyboardEvent) {
      const blocked = shortcutsBlocked();
      const revealIfTheater = () => {
        if (theaterModeRef.current) {
          revealTheaterControls();
        }
      };

      if (
        (!theaterModeRef.current ||
          event.key === " " ||
          event.code === "Space") &&
        shouldHandlePlayerSpace(event, blocked)
      ) {
        revealIfTheater();
        event.preventDefault();
        togglePlayRef.current();
        return;
      }

      const seekDelta = playerArrowSeekDeltaMs(event.key);
      if (seekDelta !== null && shouldHandlePlayerArrowSeek(event, blocked)) {
        revealIfTheater();
        event.preventDefault();
        const maxMs = Math.max(durationMs, 1);
        seekToMsRef.current(
          clampSeekMs(playbackMsRef.current + seekDelta, maxMs)
        );
        return;
      }

      if (shouldHandlePlayerMarkComplete(event, blocked)) {
        revealIfTheater();
        event.preventDefault();
        void markCompleteRef.current();
      }
    }
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [durationMs, revealTheaterControls]);

  const closeCapture = useCallback(() => {
    setCaptureOpen(false);
    captureButtonRef.current?.focus();
  }, []);

  async function handlePictureInPicture() {
    const video = videoRef.current;
    if (!video) {
      return;
    }
    try {
      await toggleVideoPictureInPicture(video);
    } catch {
      // User dismissed or the webview rejected the request.
    }
  }

  async function handleTranscribe() {
    if (transcriptionModelPath.trim() === "") {
      return;
    }
    setTranscribeError(null);
    setTranscribeState("running");
    try {
      await commands.transcribeLesson(lesson.lesson_id);
      await refreshCues();
      setTranscribeState("done");
    } catch (e) {
      setTranscribeState("failed");
      setTranscribeError(
        e instanceof Error ? e.message : "Transcription failed."
      );
    }
  }

  const showTranscribe =
    lesson.kind === "local_file" &&
    lesson.local_media_path !== null &&
    !isDocumentLessonPath(lesson.local_media_path);

  async function handleUseFrameAsThumbnail() {
    const video = videoRef.current;
    if (!video || video.videoWidth === 0 || video.videoHeight === 0) {
      return;
    }
    setThumbnailBusy(true);
    setThumbnailError(null);
    try {
      const positionMs = Math.floor(video.currentTime * 1000);
      const jpeg = await captureVideoFrameJpeg(video);
      await commands.captureLessonThumbnail(lesson.lesson_id, positionMs, jpeg);
    } catch (e) {
      setThumbnailError(
        e instanceof Error ? e.message : "Could not save thumbnail."
      );
    } finally {
      setThumbnailBusy(false);
    }
  }

  async function handleShowLessonFileDirectory() {
    setLessonFileOpenError(null);
    try {
      await commands.openLessonFileDirectoryInFileManager(lesson.lesson_id);
    } catch (e) {
      setLessonFileOpenError(
        invokeErrorMessage(e, "Could not open lesson file folder.")
      );
    }
  }

  function openAudioClipCard() {
    const video = videoRef.current;
    if (!video) {
      return;
    }
    video.pause();
    setPlaying(false);
    flushWatchTick();
    persistPosition();
    setAudioClipOpen(true);
  }

  function openCapture() {
    const video = videoRef.current;
    if (!video || video.videoWidth === 0 || video.videoHeight === 0) {
      return;
    }
    video.pause();
    setPlaying(false);
    flushWatchTick();
    persistPosition();
    const canvas = document.createElement("canvas");
    canvas.width = video.videoWidth;
    canvas.height = video.videoHeight;
    const ctx = canvas.getContext("2d");
    if (!ctx) {
      return;
    }
    ctx.drawImage(video, 0, 0);
    setCaptureFrame(canvas.toDataURL("image/png"));
    setCaptureTimestampMs(Math.floor(video.currentTime * 1000));
    setCaptureOpen(true);
  }

  if (lesson.local_media_path && lesson.status !== "broken" && !mediaReady) {
    return (
      <div className="player-screen">
        <button
          type="button"
          className="library-button-secondary"
          onClick={onBack}
        >
          Back to course
        </button>
        <p className="library-empty__hint">Loading lesson…</p>
      </div>
    );
  }

  if (unplayable) {
    return (
      <PlayerUnplayable
        lesson={lesson}
        lessonStatus={playbackStatus}
        courseId={courseId}
        mediaPath={lesson.local_media_path}
        onBack={onBack}
      />
    );
  }

  const watchedUnionMs = watchedMsFromRanges(watchedRanges);
  const watchedPct = watchedPercent(
    watchedUnionMs,
    durationMs > 0 ? durationMs : null
  );
  const scrubMax = Math.max(durationMs, 1);

  return (
    <div
      className={playerScreenClassName(zenMode, theaterMode)}
      data-testid={zenMode ? "player-zen-screen" : undefined}
    >
      {!zenMode && !theaterMode ? (
        <button
          type="button"
          className="library-button-secondary"
          onClick={onBack}
        >
          Back to course
        </button>
      ) : null}
      {!zenMode && !theaterMode ? (
        <h1 className="screen-title">{lesson.title}</h1>
      ) : null}
      <div
        className={playerMainClassName(zenMode, theaterMode)}
        onMouseMove={revealFrameBar}
      >
        <div className="player-frame">
          {mpvHandoffMessage ? (
            <p
              className="player-mpv-handoff-message"
              role="alert"
              data-testid="player-mpv-handoff-message"
            >
              {mpvHandoffMessage}
            </p>
          ) : null}
          <video
            ref={videoRef}
            className={
              mpvActive
                ? "player-video player-video--mpv-handoff"
                : "player-video"
            }
            src={mediaSrc ?? undefined}
            controls={false}
            aria-label={`Lesson video: ${lesson.title}`}
            onError={handleVideoError}
            onLoadedMetadata={handleLoadedMetadata}
            onTimeUpdate={handleTimeUpdate}
            onPause={() => {
              flushWatchTick();
              setPlaying(false);
            }}
            onPlay={() => {
              const video = videoRef.current;
              if (video) {
                lastWatchTickMsRef.current = watchTickAnchorAfterSeek(
                  Math.floor(video.currentTime * 1000)
                );
              }
              setPlaying(true);
            }}
          />
          <div
            className={playerFrameBarClassName(frameBarVisible)}
            data-testid="player-bar"
          >
            <button
              type="button"
              className="library-button-secondary"
              data-testid="player-theater-toggle"
              onClick={toggleTheaterMode}
            >
              {theaterMode ? "Exit theater" : "Theater"}
            </button>
            <button
              type="button"
              className="library-button-secondary"
              data-testid="player-zen-toggle"
              onClick={toggleZenMode}
            >
              Zen
            </button>
            <button
              type="button"
              className="library-button-primary"
              onClick={togglePlay}
            >
              {playing ? "Pause" : "Play"}
            </button>
            <PlayerPlaybackSpeedSelect
              value={playbackSpeed}
              onChange={handlePlaybackSpeedChange}
            />
            <PlayerVolumeControl
              volume={playerVolume}
              muted={playerMuted}
              onVolumeChange={handlePlayerVolumeChange}
              onMutedChange={handlePlayerMutedChange}
            />
            <span className="player-bar__time">
              {formatDurationMs(playbackMs)} / {formatDurationMs(durationMs)}
            </span>
            <div className="player-scrubber">
              <div className="player-scrubber__track" aria-hidden="true">
                <div className="player-scrubber__watched-ranges">
                  {watchedRanges.map((range) => (
                    <div
                      key={`${range.start_ms}-${range.end_ms}`}
                      className="player-scrubber__watched-segment"
                      style={{
                        left: `${(range.start_ms / scrubMax) * 100}%`,
                        width: `${((range.end_ms - range.start_ms) / scrubMax) * 100}%`,
                      }}
                    />
                  ))}
                </div>
              </div>
              <input
                type="range"
                className="player-scrubber__input"
                min={0}
                max={scrubMax}
                step={PLAYER_SEEK_STEP_MS}
                value={Math.min(playbackMs, scrubMax)}
                aria-label="Playback position"
                onChange={handleScrubChange}
              />
              {watchedPct !== null ? (
                <span className="player-bar__watched">
                  Watched {watchedPct}%
                </span>
              ) : null}
            </div>
          </div>
        </div>
        {!zenMode && !theaterMode ? (
          <>
            <PlayerChapterList chapters={chapters} onChapterClick={seekToMs} />
            <PlayerCueList
              cues={cues}
              currentMs={playbackMs}
              onCueClick={seekToMs}
              onMakeCard={setSubtitleCardCue}
            />
            <LessonNotesColumn
              lessonId={lesson.lesson_id}
              lessonTitle={lesson.title}
              playbackContext={{
                currentMs: playbackMs,
                onSeek: seekToMs,
              }}
            />
          </>
        ) : null}
      </div>
      {!zenMode && !theaterMode ? (
        <>
          <div className="player-actions">
            {pipSupported ? (
              <button
                type="button"
                className="library-button-secondary"
                data-testid="player-pip-button"
                onClick={() => void handlePictureInPicture()}
              >
                Picture in picture
              </button>
            ) : null}
            <button
              type="button"
              className="library-button-secondary"
              aria-label={playerMarkCompleteAccessibleName()}
              onClick={() => void handleMarkComplete()}
            >
              Mark complete
            </button>
            {upNext && onPlayUpNext ? (
              <button
                type="button"
                className="library-button-secondary"
                data-testid="player-up-next-button"
                onClick={() => onPlayUpNext(upNext)}
              >
                Up next: {upNext.title}
              </button>
            ) : null}
            <button
              ref={captureButtonRef}
              type="button"
              className="library-button-secondary"
              data-testid="player-capture-button"
              onClick={openCapture}
            >
              Capture card
            </button>
            {lesson.kind === "local_file" && lesson.local_media_path ? (
              <button
                type="button"
                className="library-button-secondary theme-icon-label"
                aria-label="Show lesson file"
                onClick={() => void handleShowLessonFileDirectory()}
              >
                <img
                  src={readerOutline}
                  alt=""
                  aria-hidden
                  className="theme-icon-label__icon"
                />
                Show lesson file
              </button>
            ) : null}
            {lesson.kind === "local_file" ? (
              <button
                type="button"
                className="library-button-secondary"
                data-testid="player-audio-card-button"
                onClick={openAudioClipCard}
              >
                Add audio to card
              </button>
            ) : null}
            {lesson.kind === "local_file" ? (
              <button
                type="button"
                className="library-button-secondary"
                disabled={thumbnailBusy}
                data-testid="player-thumbnail-button"
                onClick={() => void handleUseFrameAsThumbnail()}
              >
                Use this frame as thumbnail
              </button>
            ) : null}
          </div>
          <PlayerLessonResources lessonId={lesson.lesson_id} />
          <LessonCardLinksList lessonId={lesson.lesson_id} />
          {showTranscribe ? (
            <PlayerTranscribeControl
              modelPath={transcriptionModelPath}
              jobState={transcribeState}
              errorMessage={transcribeError}
              onTranscribe={() => void handleTranscribe()}
            />
          ) : null}
        </>
      ) : null}
      {!zenMode && thumbnailError ? (
        <p className="library-error" role="alert">
          {thumbnailError}
        </p>
      ) : null}
      {!zenMode && lessonFileOpenError ? (
        <p className="library-error" role="alert">
          {lessonFileOpenError}
        </p>
      ) : null}
      {audioClipOpen ? (
        <AudioClipCardPanel
          lessonId={lesson.lesson_id}
          playheadMs={playbackMs}
          onClose={() => setAudioClipOpen(false)}
        />
      ) : null}
      {captureOpen && captureFrame ? (
        <CaptureCardPanel
          lessonId={lesson.lesson_id}
          lessonTitle={lesson.title}
          timestampMs={captureTimestampMs}
          frameDataUrl={captureFrame}
          onClose={closeCapture}
        />
      ) : null}
      {subtitleCardCue ? (
        <SubtitleCueCardPanel
          lessonId={lesson.lesson_id}
          cue={subtitleCardCue}
          onClose={() => setSubtitleCardCue(null)}
        />
      ) : null}
    </div>
  );
}

function shouldHandlePlayerSpace(
  event: KeyboardEvent,
  captureDialogOpen: boolean
): boolean {
  if (event.key !== " " && event.code !== "Space") {
    return false;
  }
  if (captureDialogOpen) {
    return false;
  }
  if (
    event.defaultPrevented ||
    event.metaKey ||
    event.ctrlKey ||
    event.altKey
  ) {
    return false;
  }
  const target = event.target;
  if (!(target instanceof HTMLElement)) {
    return true;
  }
  const tag = target.tagName;
  if (tag === "TEXTAREA" || tag === "INPUT" || tag === "SELECT") {
    return false;
  }
  return !target.isContentEditable;
}
