import { useCallback, useEffect, useRef, useState } from "react";

import type {
  LessonRow,
  MediaChapter,
  SubtitleCue,
  UpNextTarget,
  WatchedRange,
} from "../api/courseTypes";
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
import { captureVideoFrameJpeg } from "../player/captureVideoFrameJpeg";
import {
  mapPlayTickToRecordPlayback,
  PLAYBACK_WATCH_INTERVAL_MS,
  watchTickAnchorAfterSeek,
  watchedMsFromRanges,
} from "../player/playbackWatch";
import { LessonCardLinksList } from "../lesson/LessonCardLinksList";
import { LessonNotesColumn } from "../lesson/LessonNotesColumn";
import { CaptureCardPanel } from "./CaptureCardPanel";
import { AudioClipCardPanel } from "./AudioClipCardPanel";
import { SubtitleCueCardPanel } from "./SubtitleCueCardPanel";

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
  courseId: string;
  mediaPath: string | null;
  onBack: () => void;
};

function PlayerUnplayable({
  lesson,
  courseId,
  mediaPath,
  onBack,
}: UnplayableProps) {
  const commands = useCourseCommands();
  const [busy, setBusy] = useState(false);

  async function handleRecovery() {
    setBusy(true);
    try {
      if (lesson.status === "broken") {
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
    lesson.status === "broken" ? "Rescan course" : "Mark as broken";

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
  const videoRef = useRef<HTMLVideoElement>(null);
  const captureButtonRef = useRef<HTMLButtonElement>(null);
  const [mediaSrc, setMediaSrc] = useState<string | null>(null);
  const [playing, setPlaying] = useState(false);
  const [loadFailed, setLoadFailed] = useState(false);
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
  const lastWatchTickMsRef = useRef<number | null>(null);
  const pendingSeekMsRef = useRef<number | null>(initialSeekMs);

  useEffect(() => {
    pendingSeekMsRef.current = initialSeekMs;
  }, [initialSeekMs, lesson.lesson_id]);

  const unplayable =
    lesson.status === "broken" || !lesson.local_media_path || loadFailed;

  useEffect(() => {
    if (!lesson.local_media_path || lesson.status === "broken") {
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
  }, [commands, lesson.local_media_path, lesson.status]);

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
      lastWatchTickMsRef.current = watchTickAnchorAfterSeek(seekMs);
    }
  }

  function handleTimeUpdate() {
    const video = videoRef.current;
    if (!video) {
      return;
    }
    setPlaybackMs(Math.floor(video.currentTime * 1000));
  }

  function seekToMs(nextMs: number) {
    const video = videoRef.current;
    if (!video) {
      return;
    }
    video.currentTime = nextMs / 1000;
    setPlaybackMs(nextMs);
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
      void video.play();
      setPlaying(true);
    } else {
      video.pause();
      setPlaying(false);
      flushWatchTick();
      persistPosition();
    }
  }, [flushWatchTick, persistPosition]);

  const captureOpenRef = useRef(captureOpen);
  const audioClipOpenRef = useRef(audioClipOpen);
  const subtitleCardOpenRef = useRef(subtitleCardCue !== null);
  const togglePlayRef = useRef(togglePlay);

  useEffect(() => {
    captureOpenRef.current = captureOpen;
    audioClipOpenRef.current = audioClipOpen;
    subtitleCardOpenRef.current = subtitleCardCue !== null;
    togglePlayRef.current = togglePlay;
  }, [captureOpen, audioClipOpen, subtitleCardCue, togglePlay]);

  useEffect(() => {
    function onKeyDown(event: KeyboardEvent) {
      if (
        !shouldHandlePlayerSpace(
          event,
          captureOpenRef.current ||
            audioClipOpenRef.current ||
            subtitleCardOpenRef.current
        )
      ) {
        return;
      }
      event.preventDefault();
      togglePlayRef.current();
    }
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, []);

  const closeCapture = useCallback(() => {
    setCaptureOpen(false);
    captureButtonRef.current?.focus();
  }, []);

  async function handleMarkComplete() {
    await commands.markLessonComplete(lesson.lesson_id);
    onBack();
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

  if (unplayable) {
    return (
      <PlayerUnplayable
        lesson={lesson}
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
    <div className="player-screen">
      <button
        type="button"
        className="library-button-secondary"
        onClick={onBack}
      >
        Back to course
      </button>
      <h1 className="screen-title">{lesson.title}</h1>
      <div className="player-main">
        <video
          ref={videoRef}
          className="player-video"
          src={mediaSrc ?? undefined}
          controls={false}
          aria-label={`Lesson video: ${lesson.title}`}
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
        <PlayerChapterList chapters={chapters} onChapterClick={seekToMs} />
        <PlayerCueList
          cues={cues}
          currentMs={playbackMs}
          onCueClick={seekToMs}
          onMakeCard={setSubtitleCardCue}
        />
        <LessonNotesColumn
          lessonId={lesson.lesson_id}
          playbackContext={{
            currentMs: playbackMs,
            onSeek: seekToMs,
          }}
        />
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
      <div className="player-bar">
        <button type="button" className="study-button" onClick={togglePlay}>
          {playing ? "Pause" : "Play"}
        </button>
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
            value={Math.min(playbackMs, scrubMax)}
            aria-label="Playback position"
            onChange={handleScrubChange}
          />
          {watchedPct !== null ? (
            <span className="player-bar__watched">Watched {watchedPct}%</span>
          ) : null}
        </div>
        <button
          type="button"
          className="library-button-secondary"
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
      {thumbnailError ? (
        <p className="library-error" role="alert">
          {thumbnailError}
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
