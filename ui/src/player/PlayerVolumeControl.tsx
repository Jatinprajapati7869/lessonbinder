import {
  normalizePlayerVolume,
  playerVolumePercent,
  volumeFromPercent,
} from "./playerVolume";

export type PlayerVolumeControlProps = {
  volume: number;
  muted: boolean;
  disabled?: boolean;
  onVolumeChange: (volume: number) => void;
  onMutedChange: (muted: boolean) => void;
};

export function PlayerVolumeControl({
  volume,
  muted,
  disabled = false,
  onVolumeChange,
  onMutedChange,
}: PlayerVolumeControlProps) {
  const percent = playerVolumePercent(volume);

  return (
    <div className="player-bar__volume">
      <button
        type="button"
        className="library-button-secondary player-bar__mute"
        aria-label={muted ? "Unmute" : "Mute"}
        aria-pressed={muted}
        disabled={disabled}
        onClick={() => onMutedChange(!muted)}
      >
        {muted ? "Unmute" : "Mute"}
      </button>
      <label className="player-bar__volume-slider">
        <span className="player-bar__volume-label">Volume</span>
        <input
          type="range"
          min={0}
          max={100}
          value={percent}
          aria-label="Volume"
          disabled={disabled}
          onChange={(event) =>
            onVolumeChange(volumeFromPercent(Number(event.target.value)))
          }
        />
      </label>
    </div>
  );
}

export function applyPlayerVolumeToVideo(
  video: HTMLVideoElement,
  volume: number,
  muted: boolean
): void {
  video.volume = normalizePlayerVolume(volume);
  video.muted = muted;
}
