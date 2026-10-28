export const DEFAULT_PLAYER_VOLUME = 1;

export function normalizePlayerVolume(value: number): number {
  if (!Number.isFinite(value)) {
    return DEFAULT_PLAYER_VOLUME;
  }
  return Math.min(1, Math.max(0, value));
}

export function playerVolumePercent(volume: number): number {
  return Math.round(normalizePlayerVolume(volume) * 100);
}

export function volumeFromPercent(percent: number): number {
  return normalizePlayerVolume(percent / 100);
}
