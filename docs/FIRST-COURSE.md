# Lessonbinder — first course walkthrough

Walk through **one real course folder** on your machine (not a playlist URL course). Use a directory you are allowed to import: at least one **local video or document** lesson, and for the card step either a **paired subtitle file** (same stem as the video, `.srt` or `.vtt`) or a frame suitable for **Capture card**. Do the rows **in order** on the same app profile so quit, relaunch, and **Continue** prove persistence.

Run a dev or packaged build the way you normally open Lessonbinder. Prefer your **usual app data location** (not a throwaway temp profile) so relaunch reads the same library. For a **throwaway dev profile** (boot check only), use the command in **Boot** below.

**Optional tools** (only where a row says so): **Tesseract** (OCR capture), **ffmpeg** / **ffprobe** (media tooling on `PATH`), a local **Whisper** model path plus CLI (transcription only), **Anki Desktop** on `PATH` when using the external fallback—not required for in-app **Study**.

---

## Boot

Isolated dev launch before walking the checklist on a **persistent** profile. Same pattern as [`FIRST-TEST.md`](FIRST-TEST.md) § Launch.

**Command (repo root, 2026-10-28):**

```bash
export XDG_DATA_HOME="$(mktemp -d)"
export XDG_CONFIG_HOME="$(mktemp -d)"
export XDG_CACHE_HOME="$(mktemp -d)"
npm run tauri:dev
```

**Data directory used:** `/tmp/tmp.OuWd4XEug0/dev.lessonbinder.app/` (`vault.db` and `anki-profile/` created on first open).

**Result:** **pass** — `Finished` / `Running …/target/debug/lessonbinder` in the dev log; window title **Lessonbinder**; **Library** route (empty-library **Add folder** / playlist actions), side nav (Library / Roadmap / Settings), screen title **Library** — not a blank webview and not the Vite + React starter template (`ui/index.html` title is **Lessonbinder**; app mounts `main.tsx`).

**Crash / startup failure text:** none. Process quit cleanly after manual stop once the library was up.

**Non-fatal console noise:** Vite logged repeated React duplicate-key warnings (`1970-01-01`) from `LibraryStudyHeatmap` on the library screen; startup was not blocked.

---

## AppImage launch (prompt 219, 2026-10-28)

Prompt **218** did not finish bundling; there is no `target/release/bundle/appimage/*.AppImage` to run. **No launch command was executed** and the library was not confirmed inside an AppImage.

**Missing system package:** **`patchelf`** is not installed on this host (`command -v patchelf` fails). Install with `sudo pacman -S patchelf` before retrying. Do not claim an AppImage exists until a bundle completes and `target/release/bundle/appimage/*.AppImage` is present.

(See also [`docs/PACKAGE.md`](PACKAGE.md) § AppImage build (prompt 218).)

---

## Checklist

| Step                                                                                                                                                                                                                                   | Expected                                                                                                                                                                                                | Blocked if                                                                                                                                                                                                                                                                                                               |
| -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Import** — On **Library**, use **Add folder** (or drag the folder onto the window if your desktop supports it) and choose your course directory.                                                                                     | A **course** row appears. Opening it lists **lessons** from supported files; sidecar subtitles count as cues on the video lesson, not as a separate lesson. Progress starts empty.                      | Import rejected (empty folder, unsupported-only contents, or a blocked risky root such as your home directory).                                                                                                                                                                                                          |
| **Resume** — Open one **local** lesson with playable media. Start playback, seek or watch partway through, go **Back** to the course, then open the **same lesson** again.                                                             | Playback resumes near where you left off; watched / resume hints on the course or player reflect the session (exact percent may read **Unknown** when duration was never learned).                      | No local decodable file (playlist-only lesson, missing file, or broken lesson row). **ffprobe** missing only blocks reliable duration and chapter metadata—not basic resume when the file plays.                                                                                                                         |
| **A note** — On that lesson, open the **Notes** column. Add text and leave the field (blur) or save explicitly.                                                                                                                        | The note appears in the list and stays after you navigate away and return. Notes live in **vault.db** only until you promote one to Anki.                                                               | —                                                                                                                                                                                                                                                                                                                        |
| **One card** — Create exactly **one** Lessonbinder card linked to this lesson: either confirm **Save to Anki** on a **subtitle cue** draft, confirm **Save to Anki** from a saved lesson note, or complete **Capture card** after OCR. | **Linked cards** on the player lists one row; top-bar **New** (or due) count increases by one. Unlink is not part of this walkthrough.                                                                  | **No subtitle cues** and you skip note promotion → use **Capture card** → blocked if **Tesseract** is not installed. **ffmpeg** missing only blocks attaching an **audio clip** to the card, not a text-only commit. **Whisper** not applicable. In-app collection failure (profile cannot open) blocks any card commit. |
| **A review** — From the top bar, check **New / Learning / Review**, then click **Study**.                                                                                                                                              | In-app **review** shows the card front, **Show answer**, then grade with **Again / Hard / Good / Easy** (or the subset offered). After one grade, the queue advances or shows the empty-queue sentence. | Step **One card** not done (nothing due). **anki** on `PATH` matters only for **Open in Anki Desktop** from review—not for in-app **Study**.                                                                                                                                                                             |
| **Theme switch** — **Settings** → choose **Dark** or **Light** (whichever you are not using) → **Save**. Visit **Library** and open the **player** again.                                                                              | Surfaces match the saved theme; controls stay readable (including player chrome). Theme persists without re-importing the course.                                                                       | —                                                                                                                                                                                                                                                                                                                        |
| **Quit** — Close the Lessonbinder window (or exit the app from your desktop environment).                                                                                                                                              | Process ends; no need to export or back up for this checklist.                                                                                                                                          | —                                                                                                                                                                                                                                                                                                                        |
| **Relaunch** — Start Lessonbinder again using the **same app data** as before. Open **Library**.                                                                                                                                       | Your imported **course**, lesson progress, vault **note**, Anki link, theme choice, and study history are still present.                                                                                | You used a disposable profile (e.g. fresh temp `XDG_DATA_HOME`) and deleted it between runs.                                                                                                                                                                                                                             |
| **Continue** — On **Library**, use the **Continue** row (course title, lesson title, watched hint).                                                                                                                                    | The app opens that **lesson** at the saved **resume position** from your earlier session—not the start of the file.                                                                                     | No prior in-progress lesson with a stored resume (you never reached **Resume**, or you cleared progress). Same media blocks as **Resume**.                                                                                                                                                                               |

---

## Done when

All nine rows match **Expected**, or you record an explicit **Blocked if** reason (missing optional tool), not a crash, silent wrong import, or empty **Study** when a card was committed.

Related: automated and slice checklists in [`FIRST-TEST.md`](FIRST-TEST.md); v1.3 product rows in [`V1.3.md`](V1.3.md).
