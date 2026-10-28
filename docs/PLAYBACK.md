# Playback diagnosis (prompts 352–363)

Engine and UI rules for later prompts live here. Prompt **353** defines the playback engine rule; implementation follows in **354+**.

---

## Engine rule (prompt 353)

1. **Primary:** The Tauri **webview** `<video>` element in `ui/src/screens/PlayerScreen.tsx` plays the lesson file when WebKit can decode it (`convertFileSrc` / `mediaUrlForPath` on the same **`local_media_path`**).
2. **Fallback:** On the element’s **`error`** event (and when prompt **355+** defines equivalent “cannot play” cases for stuck loads), hand the **same local filesystem path**—not a remote URL—to the **`mpv`** program on **`PATH`**, driven through **mpv’s JSON IPC** (stdin/stdout or socket per mpv manual). Lessonbinder owns transport and state; **Lessonbinder draws the only on-screen controls** (play, scrubber, speed, volume, theater, zen, etc.).
3. **Out of bounds for this repo:** Do **not** copy the **mpv** source tree into Lessonbinder. Do **not** enable or ship **mpv’s OSC** (on-screen controller); the embedded UI remains the sole control surface.
4. **Prompt 353:** Rule documented here only—**no player or IPC code** yet.

Watch percent still does not auto-mark a lesson **done**. Course files stay on disk where imported.

---

## Prompt 352 — First course video does not start (2026-10-28)

**Environment:** Lessonbinder **0.0.0** dev (`npm run tauri:dev`), Linux WebKitGTK (Tauri webview). App data: `/tmp/lessonbinder-sitting-data` (sitting profile also contains a **Ventoy** folder course from manual import).

**Reproduction (failure):** Open library course **Dandadan S02 1080p Dual Audio WEBRip DD+ x265-EMBER** → open the **first lesson row** in the course table (**`[EMBER] Dandadan - 13`**, sort index **0**) → press **Play** in the player bar. The `<video>` in `ui/src/screens/PlayerScreen.tsx` never reaches playback (scrubber stays at zero; no audible output).

The **first lesson in this course is a video** (`.mkv`), not markdown or PDF—no “first row is non-video” branch was needed for this course.

| Field                                   | Value                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| --------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Lesson path** (`local_media_path`)    | `/run/media/xingzi/Ventoy/Dandadan S02 1080p Dual Audio WEBRip DD+ x265-EMBER/[EMBER] Dandadan - 13.mkv`                                                                                                                                                                                                                                                                                                                                                                |
| **Extension**                           | `.mkv`                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| **Container / video codec** (ffprobe)   | Matroska; **HEVC** (`hevc` / H.265)                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| **UI row opened**                       | First lesson row **`[EMBER] Dandadan - 13`** (matches sort index **0**)                                                                                                                                                                                                                                                                                                                                                                                                 |
| **`convertFileSrc` URL**                | Set on `<video src>` via `commands.mediaUrlForPath` → `convertFileSrc` in `ui/src/api/courseCommands.ts`. Tauri 2 **asset** protocol (default): `http://asset.localhost/` + URL-encoded absolute path. For this file: `http://asset.localhost/%2Frun%2Fmedia%2Fxingzi%2FVentoy%2FDandadan%20S02%201080p%20Dual%20Audio%20WEBRip%20DD%2B%20x265-EMBER%2F%5BEMBER%5D%20Dandadan%20-%2013.mkv` (confirm on the live `<video src>` in Web Inspector if the prefix differs). |
| **`<video>` error `code`**              | **`null`** — no `error` event observed within **20s** (Lessonbinder does not register `onError` on the element today).                                                                                                                                                                                                                                                                                                                                                  |
| **`<video>` error `message`**           | **`""`** (empty — `HTMLMediaElement.error` stays **null**).                                                                                                                                                                                                                                                                                                                                                                                                             |
| **Observed state while “not starting”** | `readyState` **0** (`HAVE_NOTHING`); `networkState` **2** (`NETWORK_LOADING`) — stuck loading, never `loadeddata`.                                                                                                                                                                                                                                                                                                                                                      |

**Control (same WebKitGTK stack, same directory as file):** Sitting fixture **`/tmp/lessonbinder-sitting/intro.mp4`** (H.264 in `.mp4`) loads in a local `<video src="intro.mp4">` probe (`readyState` **4**, playback can start). That path is the first video in course **lessonbinder-sitting**; first table row is also **`intro`** (video), not **`notes.md`**.

**Likely cause (352 only):** WebKitGTK in the embedded player does not usefully decode this **HEVC-in-MKV** local file through the `<video>` + **asset** URL path; playback never starts. (mpv hand-off is **out of scope** for prompt 352.)

---

## Implementation notes

- **Prompt 354:** Rust JSON IPC client in `crates/mpv-ipc` (`MpvIpcClient`: `loadfile`, `seek`, `pause`, `volume`, `speed`, `time-pos`). Integration tests use a **fake Unix socket** (`UnixStream::pair`); **mpv** binary not required for those tests.
- **Prompt 355:** On `<video>` **`error`**, the UI invokes `start_mpv_lesson_handoff` with the same **`local_media_path`** (not the asset URL). Rust spawns **`mpv`** on **`PATH`** with **`--force-window=embed`**, **`--wid`** (host window handle), **`--geometry`**, **`--osc=no`**, and **`--input-ipc-server`**, then **`loadfile`** over JSON IPC. If **`mpv`** is missing, one sentence is shown and the lesson stays not **done**. Successful webview playback never starts **mpv**. `stop_mpv_lesson_handoff` runs when leaving the player.
- **Prompt 356:** Play, scrubber, time, speed, volume, mute, theater, and zen sit on **`player-bar--overlay`** at the bottom of **`player-frame`** (the video area). Pointer movement reveals the bar; **`PLAYER_THEATER_CONTROLS_HIDE_MS`** idle hides it in normal, theater, and zen modes. Capture, mark complete, PiP, and related actions stay in **`player-actions`** below the frame.

---
