# Lessonbinder scope — v1.5

**Status:** Planning (prompt 284).
**Baseline:** Shipped v1.4 behavior and verification are in [`SCOPE-v1.4.md`](SCOPE-v1.4.md), [`V1.4.md`](V1.4.md), and [`LEARNINGS.md`](LEARNINGS.md). v1.5 is a **sitting and week-tools pack** on top of v1.4: one vault integration test for the first-course walkthrough, documented AppImage retry status, tighter cancel semantics before lesson writes, file-manager shortcuts, a Settings shortcut list, and a capped local command-error log—not new product areas.

v1.5 does **not** change manual `done` semantics, the two-database rule, or Anki grading.

---

## In scope (v1.5 only)

### First-course sitting test

- **`vault_core::first_course_sitting::first_course_sitting`** exercises a temp folder end-to-end: import, reorder, resume, vault note, manual **done**, **100% watched without done**, and rescan after adding a file.
- [`SITTING.md`](SITTING.md) maps **Import**, **Resume**, and **A note** checklist rows to that test (plus existing slice tests); human-only rows stay human-only ([`FIRST-COURSE.md`](FIRST-COURSE.md) does not claim they pass in CI).

### AppImage retry (documented, not published)

- Record **257–259** on the reference machine: **`patchelf`** missing, bundler **not run** in **258**, **no launch** without a real bundle file ([`PACKAGE.md`](PACKAGE.md), [`README.md`](../README.md) **Install**, [`FIRST-COURSE.md`](FIRST-COURSE.md) § Boot).
- Retry instructions stay local build only; do **not** claim a published, signed, or store AppImage from this repository.

### Cancel before upsert

- Folder import/rescan cancel is honored **before each lesson row write**—no half-written lesson row after cancel (`vault_core::import_cancel::cancel_before_lesson_upsert_leaves_no_half_written_row`).
- v1.4 cancel-between-entries behavior and library copy for cancelled import **remain**; this pack closes the mid-write row case in docs and tests ([`V1.4.md`](V1.4.md) row updated in prompt **264**).

### Show folder

- **Show folder** on a **folder course** screen opens the stored course root via the system file manager (`open_course_folder_in_file_manager`, `course_folder_path_for_open`).
- Control appears **only** on the course screen (not on library course rows); **disabled** while a folder rescan is in progress; opener failures map to **`opener_open_path_failed`** with one user sentence.

### Show the lesson file

- **Show lesson file** on the **player** opens the lesson media’s directory in the file manager (`open_lesson_file_directory_in_file_manager`, path resolution with symlink escape checks).
- Same opener failure copy as **Show folder** when the OS cannot open the path.

### Settings shortcut list

- **Settings** shows player and review shortcuts derived from the existing keyboard modules (`implementedShortcuts.ts`: seek step, mark complete, exit theater/zen, review grade digits).
- List stays read-only documentation of wired keys—not a remapping UI.

### Last twenty command errors

- Unexpected command failures append to **`command-errors.log`** under app data (redacted context; not uploaded).
- Log keeps at most **20** lines after each append (`COMMAND_LOG_MAX_LINES`); regression test **`log_command_error_keeps_last_twenty_newest_lines`**.

### Docs and regression rows (252–284)

- README mentions **Show folder**, **Show lesson file**, and the Settings shortcut list; **Install** stays truthful from prompt **260**.
- Pack regression covers sitting test, cancel-before-upsert, opener paths, shortcuts, cue seek local-only, phone-home audit, show-folder gating, manual done, deslop, and react-doctor score row—see [`PROGRESS.md`](PROGRESS.md) **252–284**.

---

## Out of scope (v1.5)

| Area                                                    | Notes                                                                                           |
| ------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| **Accounts** and auth                                   | —                                                                                               |
| **Cloud sync** of library, notes, stats, or progress    | —                                                                                               |
| **Kit illustrations** / FlutKit or ProKit icon packs    | Waiting list unchanged                                                                          |
| **Code signing** / notarization / **published release** | Local unsigned Linux AppImage build only when bundling succeeds; see [`PACKAGE.md`](PACKAGE.md) |
| **Windows** and **macOS** shipping targets              | Linux-focused repo; no installer claims for other OSes                                          |
| **Auto-`done`** from watch percent or thresholds        | `done` remains manual ([`charter.md`](charter.md))                                              |

Also still out of scope: mobile clients, playlist media download, and downloading Whisper weights.

**UI kits:** FlutKit and ProKit **icon** sets remain on the waiting list—this pack does **not** import or ship those illustration assets. Licensed **Google Sans** and **Andina** font files under `ui/src/theme/fonts/` stay typography-only ([`FONTS.md`](FONTS.md)).

---

## Relationship to prompts 252–284

Implementation checklist: [`PROGRESS.md`](PROGRESS.md) rows **252–284** (sitting test and docs, AppImage retry honesty, cancel-before-upsert, show folder / show lesson file, Settings shortcuts, command log cap, v1.5 README and this scope document). Row **284** records approval of this scope document.

## Success criteria (planning)

- **`first_course_sitting`** proves the scripted first-course import/resume/note/done/rescan story without faking human-only checklist rows.
- Docs honestly describe **AppImage build and launch status** on the reference machine without claiming a published artifact.
- Users can open **course** and **lesson** paths in the file manager from the right screens, with clear failure copy and scan-time gating for **Show folder**.
- **Settings** lists shortcuts that match the wired keyboard handlers.
- Command failures leave a **local, capped** error log for support—not cloud telemetry.
