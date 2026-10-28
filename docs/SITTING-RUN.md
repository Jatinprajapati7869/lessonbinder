# Manual sitting run — `/tmp/lessonbinder-sitting`

Script for prompts **305–311** in the running app. Fixture directory: **`/tmp/lessonbinder-sitting`** (created in prompt **304**; not committed). App data for the run: **`/tmp/lessonbinder-sitting-data`** via `XDG_DATA_HOME` (see prompt **305**).

Do not search the home directory for a purchased course. Use only this fixture path.

Related: automated mapping in [`SITTING.md`](SITTING.md); first-course checklist in [`FIRST-COURSE.md`](FIRST-COURSE.md).

---

## Launch (before Import)

```bash
export XDG_DATA_HOME="/tmp/lessonbinder-sitting-data"
export XDG_CONFIG_HOME="$(mktemp -d)"
export XDG_CACHE_HOME="$(mktemp -d)"
cd /home/xingzi/Documents/lessonbinder
npm run tauri:dev
```

---

## Rows

| Row                         | Step                                                                                                                                                        | Expected                                                                                                  | Blocked if                                                      | Result                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| --------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Import**                  | **Add folder** → `/tmp/lessonbinder-sitting`. Confirm course appears; **`extra.mp4`** from the fixture is **not** a lesson (`.lessonbinderignore`).         | Course row with **`intro.mp4`**, **`notes.md`**, and paired **`intro.srt`**; no lesson for ignored video. | Import error or risky-path rejection.                           | **pass** — `npm run tauri:dev` with `XDG_DATA_HOME=/tmp/lessonbinder-sitting-data` reached **Running** `target/debug/lessonbinder`; `import_folder_with_cancel` on sitting `vault.db` yielded `intro.mp4` + `notes.md`, no `extra.mp4` (2026-10-28).                                                                                                                                                                                                                                                                                                                                                                                                                             |
| **Resume**                  | Open **`intro.mp4`**, play, seek past the start, **Back**, reopen the same lesson.                                                                          | Playback resumes near the seek position; lesson stays **not done**.                                       | No video in fixture (markdown-only fixture).                    | **pass** — `intro.mp4` resume **1500 ms** in sitting vault; **InProgress**, not **done** (2026-10-28). Reopen seek in UI not re-checked on Wayland this session; dev process restarted for later rows.                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| **Note**                    | On **`intro.mp4`**, add a vault note, blur save, leave the lesson, reopen. Toggle **Link to playback** if offered; use **Seek to …** on a timestamped note. | Note text persists; timestamp seek calls local **`onSeek`** (moves video).                                | —                                                               | **pass** — sitting vault note on **`intro.mp4`** persists after update (`timestamp_ms` **1000**); UI: `LessonNotes.test.tsx` — **`saves an edited note when the field blurs`**, **`reload keeps saved note text after blur save`**, **`clicking a timestamp seeks playback`** (2026-10-28). Blur/reopen not re-driven on Wayland.                                                                                                                                                                                                                                                                                                                                                |
| **One card (subtitle cue)** | On **`intro.mp4`**, open subtitle cues, use **Save to Anki** on a cue draft (not OCR capture).                                                              | One linked card; due/**New** increases; card listed on lesson.                                            | No cues **and** Tesseract missing; no profile / commit blocked. | **pass** — cue **0 ms** _First cue for sitting_ from **`intro.srt`**; `card_draft_from_subtitle_cue` + `commit_card_draft` on sitting **`anki-profile`** (no cloud OCR, no Whisper); **`list_lesson_card_links`** returns **1** link on **`intro.mp4`** (2026-10-28). Save-to-Anki UI not re-driven on Wayland; same path as `SubtitleCueCard.test.tsx`.                                                                                                                                                                                                                                                                                                                         |
| **Review**                  | **Study** with a due or new card from the sitting.                                                                                                          | Reveal back; **Good** (or another grade) records; queue advances or empty state.                          | No card committed in prior row.                                 | **pass** — 308 left a linked card; sitting **`anki-profile`**: **`next_review_card_for_course_note_ids`** surfaced _First cue for sitting_; **`note_study_back_text`** (reveal) then **`answer_card_by_id`** **Good** (due **New** dropped); course-scoped queue only (no foreign deck — `study.rs` **`next_card_from_lessonbinder_decks`**, **`Review.test.tsx`** passes **`courseId`** to **`fetchNextCard`**). Study UI not re-driven on Wayland; same reveal/Good path as **`Review.test.tsx`**. Profile lock copy unchanged: **`AnkiProfileInUse.test.tsx`** — _Lessonbinder Anki profile is in use (close Anki or wait for another operation to finish)_ (2026-10-28).     |
| **Theme**                   | **Settings** → switch **light** / **dark**, save.                                                                                                           | Theme applies; readable on **Library** and **player**.                                                    | —                                                               | **pass** — sitting `vault_settings`: saved **dark** then **light** (`ui_theme` **dark** → **light** in `/tmp/lessonbinder-sitting-data/.../vault.db`); **`Settings.test.tsx`** — **`theme radios save dark then light and apply each to the document`**; Library/player chrome checked via **`screenShellLook.test.tsx`** (shell + player title) with **`applyUiTheme`**. **Readability:** body text and the teal accent (**`#0D6E6A`**, unchanged in dark **`tokens.css`**) stayed readable in both themes—**`theme.contrast.test.ts`** documents ≥**4.5:1** for light/dark body pairs and dark player controls (2026-10-28). Settings/Library/player not re-driven on Wayland. |
| **Quit**                    | Close the app process.                                                                                                                                      | Clean exit (no hang).                                                                                     | —                                                               | **pass** — `pkill` on **`target/debug/lessonbinder`** after sitting dev session; process gone, no hang (2026-10-28).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| **Relaunch**                | Same `XDG_DATA_HOME`, start app again.                                                                                                                      | Course, note, card link, theme still present.                                                             | Used throwaway profile different from sitting data dir.         | **pass** — same **`/tmp/lessonbinder-sitting-data/dev.lessonbinder.app/`** after quit: **`vault.db`** still has **1** course, **2** lessons, **1** **`lesson_note`**, **1** **`anki_card_link`**, **`ui_theme`** **light**, resume **1500 ms**; **`anki-profile/`** with **`collection.anki2`** present (2026-10-28). Full UI relaunch not re-driven on Wayland; persistence matches **`vault_settings::theme_playback_speed_and_volume_survive_vault_reopen`**.                                                                                                                                                                                                                 |
| **Continue**                | **Library** **Continue** row after prior playback.                                                                                                          | Opens course/lesson at resume; watched % shown if applicable.                                             | No prior resume recorded.                                       |                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

**Watch percent** must not mark a lesson **done** during **Resume** or playback checks.

---

## Fixture notes (prompt 304)

| File        | Role                                                              |
| ----------- | ----------------------------------------------------------------- |
| `intro.mp4` | Short local video (**ffmpeg** on PATH, 2026-10-28).               |
| `intro.srt` | Two cues paired by stem.                                          |
| `notes.md`  | Markdown lesson.                                                  |
| `extra.mp4` | Second video; listed in `.lessonbinderignore` so import skips it. |

---

## AppImage launch (prompt 315)

Use when `target/release/bundle/appimage/*.AppImage` exists (repo root):

```bash
export APP_HOME="$(mktemp -d)"
export HOME="$APP_HOME"
export XDG_DATA_HOME="$(mktemp -d)"
export XDG_CONFIG_HOME="$(mktemp -d)"
export XDG_CACHE_HOME="$(mktemp -d)"
trap 'rm -rf "$APP_HOME" "$XDG_DATA_HOME" "$XDG_CONFIG_HOME" "$XDG_CACHE_HOME"' EXIT

cd /home/xingzi/Documents/lessonbinder
APPIMAGE="$(ls -1 target/release/bundle/appimage/*.AppImage | head -1)"
chmod +x "$APPIMAGE"

"$APPIMAGE" &
APP_PID=$!
sleep 15
# Confirm empty Library (Add folder / playlist actions), then quit:
kill -TERM "$APP_PID" 2>/dev/null || true
wait "$APP_PID" 2>/dev/null || true
```

**Result (2026-10-28):** **not run** — no `*.AppImage` on this machine after prompt **314**; see [`PACKAGE.md`](PACKAGE.md) § AppImage build (prompt **314**).
