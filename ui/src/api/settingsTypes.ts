export type UiTheme = "light" | "dark";

export type VaultSettings = {
  tesseract_language: string;
  transcription_model_path: string;
  anki_desktop_binary_path: string;
  ui_theme: UiTheme;
  playback_speed: number;
  player_volume: number;
  player_muted: boolean;
};
