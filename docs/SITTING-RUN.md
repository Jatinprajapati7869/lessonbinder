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

| Row                         | Step                                                                                                                                                | Expected                                                                                                  | Blocked if                                              | Result |
| --------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- | ------ |
| **Import**                  | **Add folder** → `/tmp/lessonbinder-sitting`. Confirm course appears; **`extra.mp4`** from the fixture is **not** a lesson (`.lessonbinderignore`). | Course row with **`intro.mp4`**, **`notes.md`**, and paired **`intro.srt`**; no lesson for ignored video. | Import error or risky-path rejection.                   |        |
| **Resume**                  | Open **`intro.mp4`**, play, seek past the start, **Back**, reopen the same lesson.                                                                  | Playback resumes near the seek position; lesson stays **not done**.                                       | No video in fixture (markdown-only fixture).            |        |
| **Note**                    | Open a lesson, add or edit a vault note, blur save, reopen lesson.                                                                                  | Note text persists.                                                                                       | —                                                       |        |
| **One card (subtitle cue)** | On **`intro.mp4`**, open subtitle cues, use **Save to Anki** on a cue draft (not OCR capture).                                                      | One linked card; due/**New** increases; card listed on lesson.                                            | No cues / no profile / commit blocked.                  |        |
| **Review**                  | **Study** with a due or new card from the sitting.                                                                                                  | Reveal back; **Good** (or another grade) records; queue advances or empty state.                          | No card committed in prior row.                         |        |
| **Theme**                   | **Settings** → switch **light** / **dark**, save.                                                                                                   | Theme applies; readable on **Library** and **player**.                                                    | —                                                       |        |
| **Quit**                    | Close the app process.                                                                                                                              | Clean exit (no hang).                                                                                     | —                                                       |        |
| **Relaunch**                | Same `XDG_DATA_HOME`, start app again.                                                                                                              | Course, note, card link, theme still present.                                                             | Used throwaway profile different from sitting data dir. |        |
| **Continue**                | **Library** **Continue** row after prior playback.                                                                                                  | Opens course/lesson at resume; watched % shown if applicable.                                             | No prior resume recorded.                               |        |

**Watch percent** must not mark a lesson **done** during **Resume** or playback checks.

---

## Fixture notes (prompt 304)

| File        | Role                                                              |
| ----------- | ----------------------------------------------------------------- |
| `intro.mp4` | Short local video (**ffmpeg** on PATH, 2026-10-28).               |
| `intro.srt` | Two cues paired by stem.                                          |
| `notes.md`  | Markdown lesson.                                                  |
| `extra.mp4` | Second video; listed in `.lessonbinderignore` so import skips it. |
