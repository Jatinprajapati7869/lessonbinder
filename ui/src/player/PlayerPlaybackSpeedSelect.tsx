import {
  formatPlaybackSpeedLabel,
  normalizePlaybackSpeed,
  PLAYBACK_SPEED_CHOICES,
  type PlaybackSpeedChoice,
} from "./playerPlaybackSpeed";

export type PlayerPlaybackSpeedSelectProps = {
  value: PlaybackSpeedChoice;
  disabled?: boolean;
  onChange: (speed: PlaybackSpeedChoice) => void;
};

export function PlayerPlaybackSpeedSelect({
  value,
  disabled = false,
  onChange,
}: PlayerPlaybackSpeedSelectProps) {
  return (
    <label className="player-bar__speed">
      <span className="player-bar__speed-label">Speed</span>
      <select
        className="player-bar__speed-select"
        aria-label="Playback speed"
        disabled={disabled}
        value={String(value)}
        onChange={(event) =>
          onChange(normalizePlaybackSpeed(Number(event.target.value)))
        }
      >
        {PLAYBACK_SPEED_CHOICES.map((speed) => (
          <option key={speed} value={speed}>
            {formatPlaybackSpeedLabel(speed)}
          </option>
        ))}
      </select>
    </label>
  );
}
