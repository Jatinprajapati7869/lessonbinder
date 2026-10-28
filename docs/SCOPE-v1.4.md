# Lessonbinder scope — v1.4

**Status:** Planning (prompt 240).
**Baseline:** Shipped v1.3 behavior and verification are in [`SCOPE-v1.3.md`](SCOPE-v1.3.md), [`V1.3.md`](V1.3.md), and [`LEARNINGS.md`](LEARNINGS.md). v1.4 is a **folder-scan and packaging honesty pack** on top of v1.3: cancellable import/rescan, visible progress, stricter import guards, clearer Anki profile locking, and documented Linux AppImage build attempts—not new product areas.

v1.4 does **not** change manual `done` semantics, the two-database rule, or Anki grading.

---

## In scope (v1.4 only)

### Cancellable folder import

- **Add folder** and drag-and-drop use a **cancellable** backend import (`import_folder_with_cancel`): the user can choose **Cancel import** while a scan or lesson upsert is in progress.
- Cancel is honored **between** directory entries and **before** each lesson write—not mid-row for a single lesson.
- Lessons already written before cancel **stay** in the library; cancel does **not** mark lessons **done** or **broken**.
- Cancelled import surfaces one library sentence: **Folder import was cancelled; lessons already added stay in the library.**
- **Escape** does **not** cancel folder import (player theater/zen only). Design note: [`CANCEL-SCAN.md`](CANCEL-SCAN.md).

### Cancellable folder rescan

- Course **rescan** uses the same cancel granularity as import (between scan entries and before each vault write).
- Cancelled rescan keeps explicit lesson order and vault lesson notes unchanged for lessons already on disk.

### Folder scan progress (**the count**)

- During folder import and rescan, the UI shows **real progress**: lesson files **seen** vs **planned** (`n of m`), driven by `FolderScanProgress` from the backend—not a fake indeterminate bar only.

### One folder scan at a time

- Only **one** folder import or rescan runs at a time. A second attempt while the first is active is **rejected** with a short message until the first scan finishes (`folder_scan_in_progress`).

### Empty-folder refusal

- Importing a folder with **no eligible lesson files** fails with a clear **No lesson files.** message (`ImportError::NoLessonFiles` / `no_lesson_files`) instead of creating an empty course.

### Risky path and ignore rules (cancellable paths)

- The cancellable importer still **rejects risky roots** (e.g. filesystem `/`) **before** walking entries.
- **`.lessonbinderignore`** applies on cancellable import/rescan the same as on the non-cancellable path.

### Re-import after cancelled partial import

- Importing the **same folder again** after cancel **finishes** the course without duplicating lessons already written before cancel.

### AppImage attempt (documented, not shipped)

- Record an **on-machine AppImage build attempt** (`npm run tauri:build:appimage`), host gaps (**`patchelf`** missing on the reference machine), and bundler limits in [`PACKAGE.md`](PACKAGE.md) and [`README.md`](../README.md) **Install**.
- Do **not** claim a published, signed, or store release; do **not** claim Windows or macOS installers from this repo.
- [`FIRST-COURSE.md`](FIRST-COURSE.md) omits AppImage launch steps when no bundle file exists, with a one-line reason.

### Profile-in-use sentence

- When the Lessonbinder Anki collection is locked (another process or operation holds the profile), opening the profile fails with a **user-visible sentence**: **Lessonbinder Anki profile is in use (close Anki or wait for another operation to finish)**—surfaced in the study bar / due-count path when `open_profile` cannot acquire the lock.

### Docs and regression rows

- First-course walkthrough includes optional **Cancel import**; README states cancel and one-scan-at-a-time behavior.
- Pack regression tests cover cancel, ignore, risky root, done-not-auto, backup round-trip, notes export/search, and related vault behavior (see [`PROGRESS.md`](PROGRESS.md) rows **203–240**).

---

## Out of scope (v1.4)

| Area                                                    | Notes                                                                                           |
| ------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| **Accounts** and auth                                   | —                                                                                               |
| **Cloud sync** of library, notes, stats, or progress    | —                                                                                               |
| **Kit illustrations** / FlutKit or ProKit icon packs    | Waiting list unchanged                                                                          |
| **Code signing** / notarization / **published release** | Local unsigned Linux AppImage build only when bundling succeeds; see [`PACKAGE.md`](PACKAGE.md) |
| **Auto-`done`** from watch percent or thresholds        | `done` remains manual ([`charter.md`](charter.md))                                              |

Also still out of scope: Windows and macOS shipping targets from this repository, mobile clients, playlist media download, and downloading Whisper weights.

---

## Relationship to prompts 203–240

Implementation checklist: [`PROGRESS.md`](PROGRESS.md) rows **203–240** (first-course doc, cancellable import/rescan, scan guard and progress, empty folder, AppImage/docs, profile lock copy, README, scope, deslop, doctor, tests). Row **240** records approval of this scope document.

## Success criteria (planning)

- Users can **cancel** long folder imports and rescans without losing partial work or getting false **done**/**broken** states.
- Library and course UI show **honest scan counts** and refuse overlapping scans.
- Empty folders and risky roots fail with **clear copy** before vault damage.
- Docs honestly describe **AppImage build status** on the reference machine without overstating release readiness.
- Profile lock errors read as **actionable sentences**, not opaque SQLite failures.
