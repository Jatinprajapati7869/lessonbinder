/** Canonical values from `docs/DESIGN.md` (v1 light theme). Tests assert `tokens.css` matches. */

export const DESIGN_COLOR_TOKENS: Record<string, string> = {
  "--bg-app": "#F4F4F3",
  "--bg-surface": "#FAFAF9",
  "--bg-elevated": "#FFFFFF",
  "--border": "#E5E4E2",
  "--border-strong": "#D6D4D1",
  "--text-primary": "#1C1917",
  "--text-secondary": "#57534E",
  "--text-muted": "#78716C",
  "--accent": "#0D6E6A",
  "--accent-hover": "#0A5C58",
  "--accent-subtle": "#E6F2F1",
  "--on-accent": "#FFFFFF",
  "--progress-track": "#E7E5E4",
  "--status-broken": "#B45309",
  "--status-done": "#44403C",
  "--danger": "#B91C1C",
  "--focus-ring": "#0D6E6A",
};

/** Dark values from `:root[data-theme="dark"]` in `tokens.css` (same keys as light). */
export const DESIGN_DARK_COLOR_TOKENS: Record<string, string> = {
  "--bg-app": "#1C1917",
  "--bg-surface": "#292524",
  "--bg-elevated": "#44403C",
  "--border": "#57534E",
  "--border-strong": "#78716C",
  "--text-primary": "#FAFAF9",
  "--text-secondary": "#D6D3D1",
  "--text-muted": "#BDBAB5",
  "--accent": "#0D6E6A",
  "--accent-hover": "#0F857F",
  "--accent-subtle": "#134E4A",
  "--on-accent": "#FFFFFF",
  "--progress-track": "#44403C",
  "--status-broken": "#FBBF24",
  "--status-done": "#D6D3D1",
  "--danger": "#F87171",
  "--focus-ring": "#2DD4BF",
};

export const DESIGN_SPACE_TOKENS: Record<string, string> = {
  "--space-1": "4px",
  "--space-2": "8px",
  "--space-3": "12px",
  "--space-4": "16px",
  "--space-5": "24px",
  "--space-6": "32px",
};

export const DESIGN_FONT_TOKENS: Record<string, string> = {
  "--font-family": '"Google Sans", system-ui, sans-serif',
  "--font-wordmark": '"Andina", serif',
};

export const DESIGN_TYPE_TOKENS: Record<string, string> = {
  "--text-xs": "0.75rem",
  "--text-sm": "0.8125rem",
  "--text-base": "0.875rem",
  "--text-md": "1rem",
  "--text-lg": "1.125rem",
};

export const DESIGN_RADIUS_TOKENS: Record<string, string> = {
  "--radius-sm": "4px",
};

export const DESIGN_FOCUS_TOKENS: Record<string, string> = {
  "--focus-ring-width": "2px",
  "--focus-ring-offset": "2px",
};
