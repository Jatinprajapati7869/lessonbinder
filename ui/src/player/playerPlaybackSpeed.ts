export const PLAYBACK_SPEED_CHOICES = [0.75, 1, 1.25, 1.5, 2] as const;

export type PlaybackSpeedChoice = (typeof PLAYBACK_SPEED_CHOICES)[number];

export function normalizePlaybackSpeed(value: number): PlaybackSpeedChoice {
  for (const choice of PLAYBACK_SPEED_CHOICES) {
    if (Math.abs(value - choice) < Number.EPSILON) {
      return choice;
    }
  }
  return 1;
}

export function formatPlaybackSpeedLabel(speed: PlaybackSpeedChoice): string {
  return speed === 1 ? "1×" : `${speed}×`;
}
