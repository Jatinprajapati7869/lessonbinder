import { invoke } from "@tauri-apps/api/core";

export type MpvEmbedRect = {
  x: number;
  y: number;
  width: number;
  height: number;
};

function tauriInvokeAvailable(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export function videoEmbedRect(video: HTMLVideoElement): MpvEmbedRect {
  const rect = video.getBoundingClientRect();
  const scale = window.devicePixelRatio || 1;
  return {
    x: Math.round(rect.left * scale),
    y: Math.round(rect.top * scale),
    width: Math.round(Math.max(1, rect.width) * scale),
    height: Math.round(Math.max(1, rect.height) * scale),
  };
}

export function startMpvLessonHandoff(
  mediaPath: string,
  rect: MpvEmbedRect,
  resumeSeconds: number
): Promise<void> {
  if (!tauriInvokeAvailable()) {
    return Promise.resolve();
  }
  return invoke("start_mpv_lesson_handoff", {
    mediaPath,
    rect,
    resumeSeconds,
  });
}

export function updateMpvLessonEmbedRect(rect: MpvEmbedRect): Promise<void> {
  if (!tauriInvokeAvailable()) {
    return Promise.resolve();
  }
  return invoke("update_mpv_lesson_embed_rect", { rect });
}

export function stopMpvLessonHandoff(): Promise<void> {
  if (!tauriInvokeAvailable()) {
    return Promise.resolve();
  }
  return invoke("stop_mpv_lesson_handoff");
}
