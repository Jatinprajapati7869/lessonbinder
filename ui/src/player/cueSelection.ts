import type { SubtitleCue } from "../api/courseTypes";

/** Index of the cue that contains `currentMs`, or -1 when between cues or before the first. */
export function indexOfActiveCue(
  cues: SubtitleCue[],
  currentMs: number
): number {
  return cues.findIndex(
    (cue) => currentMs >= cue.start_ms && currentMs < cue.end_ms
  );
}
