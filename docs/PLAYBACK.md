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

## Dark gaps (prompt 357)

**Method:** Ripgrep over `ui/src` for `#…`, `rgb(`, `rgba(`, `hsl(`, and for `background` / `color` values that are not `var(--…)` theme tokens (2026-10-28). **Excluded** (canonical palette, not UI gaps): `ui/src/theme/tokens.css`, `ui/src/theme/tokens.ts`, `ui/src/theme/contrast.ts`.

**Player bar** (`ui/src/theme/course.css`; overlay bar in `ui/src/screens/PlayerScreen.tsx`)

| Selector                                       | Property      | Value (not a theme token)                                                                                 |
| ---------------------------------------------- | ------------- | --------------------------------------------------------------------------------------------------------- |
| `.player-frame .player-video`, `.player-video` | `background`  | `var(--text-primary)` (surface uses a **text** token; in dark theme the letterbox reads as a light panel) |
| `.player-bar--overlay`                         | `background`  | `color-mix(in srgb, var(--bg-elevated) 88%, transparent)`                                                 |
| `.player-bar`, `.player-bar--overlay`          | (uses tokens) | Bar chrome itself uses `--bg-elevated` / borders; no literal hex                                          |

**Library** (`ui/src/theme/library.css`)

| Selector                  | Property     | Value (not a theme token)                                                    |
| ------------------------- | ------------ | ---------------------------------------------------------------------------- |
| `.library-course-link`    | `background` | `none`                                                                       |
| `.library-continue-row`   | `color`      | `inherit`                                                                    |
| `.library-search__input`  | `background` | `transparent`                                                                |
| `.library-search__hit`    | `color`      | `inherit`                                                                    |
| `.library-status::before` | `background` | `currentColor` (dot follows status `color`, which is tokenized on modifiers) |

**Course** (`ui/src/theme/course.css`; course table / document lesson share this sheet)

| Selector                                       | Property               | Value (not a theme token)                                          |
| ---------------------------------------------- | ---------------------- | ------------------------------------------------------------------ |
| `.lesson-notes__timestamp`                     | `background`           | `none`                                                             |
| `.player-chapter`                              | `background`, `border` | `transparent`                                                      |
| `.player-cue`                                  | `background`, `border` | `transparent` / `transparent`                                      |
| `.player-cue--active`                          | `background`           | `color-mix(in srgb, var(--accent) 12%, transparent)`               |
| `.player-resources__item[aria-pressed="true"]` | `background`           | `color-mix(in srgb, var(--accent-subtle) 50%, var(--bg-elevated))` |
| `.player-cards__row[aria-pressed="true"]`      | `background`           | `color-mix(in srgb, var(--accent-subtle) 50%, var(--bg-elevated))` |
| `.document-lesson__text`                       | `color`                | _(unset — inherits from parent, not an explicit `--text-*`)_       |
| `.document-lesson__page-input`                 | `background`, `color`  | _(unset — browser default field colors)_                           |
| `.player-chapter__title`                       | `color`                | _(unset)_                                                          |
| `.library-continue-row__course`                | `color`                | _(unset; parent uses `color: inherit`)_                            |

**Review** (`ui/src/theme/review.css`)

| Selector | Property | Value (not a theme token)                                    |
| -------- | -------- | ------------------------------------------------------------ |
| —        | —        | **No entries** — `color` / `background` use `var(--…)` only. |

**Settings** (`ui/src/theme/settings.css`)

| Selector | Property | Value (not a theme token)                                    |
| -------- | -------- | ------------------------------------------------------------ |
| —        | —        | **No entries** — `color` / `background` use `var(--…)` only. |

**Roadmap** (`ui/src/theme/roadmap.css`, `ui/src/roadmap/roadmapGraphSvg.ts`)

| File / selector                                                                                                                                                                                                       | Property            | Value (not a theme token)                                                                             |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------- | ----------------------------------------------------------------------------------------------------- |
| `roadmapGraphSvg.ts` — `EXPORT_BG_SURFACE`, `EXPORT_BG_ELEVATED`, `EXPORT_BORDER`, `EXPORT_TEXT_PRIMARY`, `EXPORT_TEXT_SECONDARY`, `EXPORT_TEXT_MUTED`, `EXPORT_ACCENT`, `EXPORT_STATUS_BROKEN`, `EXPORT_STATUS_DONE` | SVG fills / strokes | Literal light-theme hex (e.g. `#fafaf9`, `#ffffff`, `#1c1917`) — export ignores `[data-theme="dark"]` |
| `.roadmap-step-list__item`                                                                                                                                                                                            | `background`        | `transparent`                                                                                         |
| `.roadmap-node__title`                                                                                                                                                                                                | `fill`              | `var(--text-primary)` _(token OK)_; **`font-size: 12px`** is px, not a color gap                      |
| `.roadmap-node__status`, `.roadmap-node__remaining`                                                                                                                                                                   | `fill`              | tokens OK; **`font-size: 11px`** is px, not a color gap                                               |

Prompt **358** replaced the rows above with `var(--…)` theme tokens (and roadmap SVG export uses `themeColors.ts` / active `data-theme`). **`theme.contrast.test.ts`** still asserts dark body text ≥ 4.5:1 on app, surface, and elevated backgrounds.

- **Prompt 358:** CSS gaps from § **Dark gaps** use existing light/dark tokens only (`course.css`, `library.css`, `roadmap.css`); `roadmapGraphSvg.ts` reads `DESIGN_*_COLOR_TOKENS` via `themeColors.ts` (matches `[data-theme="dark"]`). No second palette.
- **Prompt 359:** In **`[data-theme="dark"]`**, **`player-frame`**, video letterbox, and **`player-bar--overlay`** use **`--bg-surface`** (not **`--bg-elevated`**); theater/zen **`player-screen`** uses **`--bg-app`** and **`player-main`** uses **`--bg-surface`** for idle fill. Light theme unchanged (stone **`--bg-surface`** / **`--bg-elevated`** on the bar).
- **Prompt 360:** Tests in **`ui/PlayerMpvHandoff.test.tsx`**, **`ui/darkThemePageBackground.test.tsx`**, and **`ui/theme.test.ts`** (dark player bar); **`cargo test --workspace`** and **`npm test`**.

---
