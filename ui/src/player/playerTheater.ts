export const PLAYER_THEATER_CONTROLS_HIDE_MS = 2500;

export function playerFrameBarClassName(controlsVisible: boolean): string {
  const classes = ["player-bar", "player-bar--overlay"];
  if (!controlsVisible) {
    classes.push("player-bar--overlay-hidden");
  }
  return classes.join(" ");
}

/** @deprecated use {@link playerFrameBarClassName} */
export function playerTheaterBarClassName(
  _theaterMode: boolean,
  controlsVisible: boolean
): string {
  return playerFrameBarClassName(controlsVisible);
}
