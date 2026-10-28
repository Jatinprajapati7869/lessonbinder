export function playerScreenClassName(
  zenMode: boolean,
  theaterMode: boolean
): string {
  if (zenMode) {
    return "player-screen player-screen--zen";
  }
  if (theaterMode) {
    return "player-screen player-screen--theater";
  }
  return "player-screen";
}

export function playerMainClassName(
  zenMode: boolean,
  theaterMode: boolean
): string {
  if (zenMode) {
    return "player-main player-main--zen";
  }
  if (theaterMode) {
    return "player-main player-main--theater";
  }
  return "player-main";
}
