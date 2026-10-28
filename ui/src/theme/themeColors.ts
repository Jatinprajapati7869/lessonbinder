import { DESIGN_COLOR_TOKENS, DESIGN_DARK_COLOR_TOKENS } from "./tokens";

export type UiTheme = "light" | "dark";

export function activeUiTheme(): UiTheme {
  if (typeof document === "undefined") {
    return "light";
  }
  return document.documentElement.getAttribute("data-theme") === "dark"
    ? "dark"
    : "light";
}

export function colorTokensForTheme(theme: UiTheme): Record<string, string> {
  return theme === "dark" ? DESIGN_DARK_COLOR_TOKENS : DESIGN_COLOR_TOKENS;
}

export function colorTokenHex(
  tokens: Record<string, string>,
  name: keyof typeof DESIGN_COLOR_TOKENS
): string {
  return tokens[name].toLowerCase();
}
