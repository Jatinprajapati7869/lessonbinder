export const PLAYER_THEATER_CONTROLS_HIDE_MS = 2500;

export function playerTheaterBarClassName(
  theaterMode: boolean,
  controlsVisible: boolean
): string {
  const classes = ["player-bar"];
  if (theaterMode) {
    classes.push("player-bar--theater");
    if (!controlsVisible) {
      classes.push("player-bar--theater-hidden");
    }
  }
  return classes.join(" ");
}
