export const PLAYBACK_WATCH_INTERVAL_MS = 1000;

export type PlaybackRecordRequest = {
  startMs: number;
  endMs: number;
};

/** Map a periodic play tick to an optional `record_playback` span. */
export function mapPlayTickToRecordPlayback(
  lastTickMs: number | null,
  currentMs: number
): {
  nextLastTickMs: number;
  recordPlayback: PlaybackRecordRequest | null;
} {
  if (lastTickMs === null) {
    return { nextLastTickMs: currentMs, recordPlayback: null };
  }
  if (currentMs <= lastTickMs) {
    return { nextLastTickMs: currentMs, recordPlayback: null };
  }
  return {
    nextLastTickMs: currentMs,
    recordPlayback: { startMs: lastTickMs, endMs: currentMs },
  };
}

/** After a seek, anchor the next watch tick here so the skipped gap is not recorded. */
export function watchTickAnchorAfterSeek(seekMs: number): number {
  return seekMs;
}

export function watchedMsFromRanges(
  ranges: { start_ms: number; end_ms: number }[]
): number {
  return ranges.reduce(
    (sum, range) => sum + (range.end_ms - range.start_ms),
    0
  );
}
