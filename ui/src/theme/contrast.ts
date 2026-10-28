function hexToRgb(hex: string): [number, number, number] {
  const n = Number.parseInt(hex.replace("#", "").trim(), 16);
  return [(n >> 16) & 255, (n >> 8) & 255, n & 255];
}

function channelLuminance(channel: number): number {
  const c = channel / 255;
  return c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4);
}

function relativeLuminance(hex: string): number {
  const [r, g, b] = hexToRgb(hex);
  return (
    0.2126 * channelLuminance(r) +
    0.7152 * channelLuminance(g) +
    0.0722 * channelLuminance(b)
  );
}

export function contrastRatio(foreground: string, background: string): number {
  const l1 = relativeLuminance(foreground);
  const l2 = relativeLuminance(background);
  const lighter = Math.max(l1, l2);
  const darker = Math.min(l1, l2);
  return (lighter + 0.05) / (darker + 0.05);
}

export const DESIGN_BODY_CONTRAST_MIN = 4.5;

export const DESIGN_BODY_TEXT_PAIRS: ReadonlyArray<{
  label: string;
  foreground: string;
  background: string;
}> = [
  { label: "primary on app", foreground: "#1C1917", background: "#F4F4F3" },
  { label: "primary on surface", foreground: "#1C1917", background: "#FAFAF9" },
  { label: "secondary on app", foreground: "#57534E", background: "#F4F4F3" },
  {
    label: "secondary on surface",
    foreground: "#57534E",
    background: "#FAFAF9",
  },
  { label: "muted on surface", foreground: "#78716C", background: "#FAFAF9" },
  { label: "muted on elevated", foreground: "#78716C", background: "#FFFFFF" },
  {
    label: "in_progress status on app",
    foreground: "#0D6E6A",
    background: "#F4F4F3",
  },
  {
    label: "on-accent on accent",
    foreground: "#FFFFFF",
    background: "#0D6E6A",
  },
];
