export type UiTheme = "light" | "dark";

export function applyUiTheme(theme: UiTheme): void {
  document.documentElement.dataset.theme = theme;
}

export function readUiThemeFromDocument(): UiTheme {
  return document.documentElement.dataset.theme === "dark" ? "dark" : "light";
}
