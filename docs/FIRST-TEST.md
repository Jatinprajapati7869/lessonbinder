# AuraVault — first manual test checklist

Use this in a **running dev build** (`cargo tauri dev` from the repo root after `cd ui && npm install`). Keep sample media on disk: a small folder with at least one `.mp4` (and optionally a matching `.srt`), plus a PDF or `.md` if you want document lessons.

**Out of scope for this checklist:** accounts, cloud sync, downloading playlist or course videos, auto-`done` from watch %, or bundled Whisper weights.

**Optional tools (only where noted):** `tesseract`, `ffmpeg`, `ffprobe`, local `whisper` model + CLI, `anki` on PATH for Desktop fallback—not required for every row.

---

## Launch

AuraVault has **no** dedicated app-data env var. On Linux, Tauri stores data under `{XDG_DATA_HOME}/{identifier}/` (`identifier` = `dev.auravault.app` from `src-tauri/tauri.conf.json`). For an isolated first-run profile, point `XDG_DATA_HOME` at an empty temp directory.

**Command (repo root):**

```bash
export XDG_DATA_HOME="$(mktemp -d)"
export XDG_CONFIG_HOME="$(mktemp -d)"
export XDG_CACHE_HOME="$(mktemp -d)"
cd /home/xingzi/Documents/auravault && npm run tauri:dev
```

**Data directory used (2026-10-28 dev launch):** `/tmp/tmp.9Gr4oH9z3U/dev.auravault.app/` (`vault.db` created on first open; sibling `anki-profile/` created when study/Anki paths initialize).

**Result:** Window title **AuraVault**; **Library** route with side nav (Library / Roadmap / Settings), screen title **Library**, empty-library copy and **Add folder** / **Add playlist JSON** / **Paste playlist** — not a blank webview and not the Vite/React starter template.

**Crash / startup failure text:** none (process exited cleanly after manual quit once the library was visible).

**Note:** Vite may log `[Unhandled rejection] NotSupportedError` from `PlayerScreen.tsx` if the user triggers **play/pause** on a lesson without a decodable local file; that is unrelated to library startup.

---

## Fixture pass

**Fixture directory:** `/tmp/auravault-first-test` (local only; not committed).

**Seed (re-run anytime):**

```bash
FIX=/tmp/auravault-first-test
rm -rf "$FIX" && mkdir -p "$FIX"
if command -v ffmpeg >/dev/null; then
  ffmpeg -y -f lavfi -i color=c=black:s=320x240:d=1 -pix_fmt yuv420p "$FIX/alpha.mp4"
else
  : > "$FIX/alpha.mp4"
fi
: > "$FIX/private-beta.mp4"
cat > "$FIX/alpha.srt" <<'EOF'
1
00:00:00,000 --> 00:00:02,000
First cue

2
00:00:02,000 --> 00:00:04,000
Second cue
EOF
cat > "$FIX/notes.md" <<'EOF'
# Fixture notes

Short markdown lesson for first-test.
EOF
cat > "$FIX/.auravaultignore" <<'EOF'
# Second video is out of scope for this course import.
private-beta.mp4
EOF
```

**Verification (2026-10-28):** GUI not driven; same `vault_core` calls as Tauri (`import_folder`, `mark_complete`, `reorder_lessons`, `rescan`, `course_detail`):

```bash
cargo test -p vault-core --test first_test_fixture
```

| Step                                                                                                    | Result   | Notes                                                                                                 |
| ------------------------------------------------------------------------------------------------------- | -------- | ----------------------------------------------------------------------------------------------------- |
| Fixture on disk (`alpha.mp4`, `alpha.srt`, `notes.md`, `.auravaultignore`, excluded `private-beta.mp4`) | **pass** | `alpha.mp4` generated with **ffmpeg** when available (~1s silent 320×240).                            |
| Import folder                                                                                           | **pass** | Two lessons (`alpha.mp4`, `notes.md`); `private-beta.mp4` skipped; `.srt` paired, not a third lesson. |
| Mark first lesson (`alpha`) complete                                                                    | **pass** | `LessonStatus::Done` on first import order id.                                                        |
| Move first lesson down (swap with `notes`)                                                              | **pass** | `reorder_lessons` → order `[notes, alpha]` by title.                                                  |
| Rescan folder (no file changes)                                                                         | **pass** | `learning_order` unchanged after `rescan`; course detail titles still `notes` then `alpha`.           |

---

## Anki pass

**Profile:** isolated temp AuraVault `anki-profile/` (same pattern as dev: empty collection under a fresh directory—not your daily Anki profile). Reuse fixture **`/tmp/auravault-first-test`** for the lesson row.

**Verification (2026-10-28):**

```bash
cargo test -p anki-bridge first_test_anki_pass_walkthrough
cd ui && npm test -- Review.test.tsx CaptureCard.test.tsx
```

| Step                                                                | Result   | Notes                                                                                                                                                                                                                                                                                                                                          |
| ------------------------------------------------------------------- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Study** with empty queue → empty state                            | **pass** | Fresh profile: `next_review_card` = none, `due_summary` all zero (`first_test_anki_pass_walkthrough`). UI: `Review.test.tsx` shows **The review queue is empty.** when `fetchNextCard` is null and due counts are zero.                                                                                                                        |
| **Capture** when Tesseract missing                                  | **pass** | `TesseractOcrEngine::without_tesseract()` → `OcrUnavailable`; no Anki note created (`due_summary` unchanged). UI: `CaptureCard.test.tsx` alert **Tesseract is not installed… Images are not sent anywhere else.** (matches `capture_ocr_status` / `build_capture_draft` error mapping). OCR is local CLI only in `vault-core`; no upload path. |
| Create one note (subtitle cue draft), **Unlink**, Anki note remains | **pass** | `card_draft_from_subtitle_cue` + `commit_card_draft` with cue **First cue** on fixture `alpha` lesson; `unlink_card` clears vault link; `get_note` still returns front **First cue**.                                                                                                                                                          |
| Top-bar due counts from AuraVault profile                           | **pass** | After commit, `due_summary` → **1** new (not learning/review). App loads counts via `study_bar_state` → `anki_bridge::due_summary` on `{app_data}/anki-profile/` (`fetchStudyBarState` in UI); `zeroDue` is only used when that command fails—not a normal empty-queue guess.                                                                  |

---

## Library import

| Step                                                                                                                                                        | Expected result                                                                                                                                | Blocked if |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| Open **Library**. Choose **Import folder** (or **Add folder** on an empty library) and select a folder that contains supported files (e.g. `.mp4`, `.pdf`). | A new **course** row appears; opening it lists **lessons** derived from files (subtitles paired by stem, not separate video+subtitle lessons). | —          |
| Leave the app, add another eligible file under the same folder, return to the course and use **Rescan folder**.                                             | New file appears as a new lesson; existing lessons and progress remain.                                                                        | —          |
| (Optional) **Add playlist JSON** and pick a valid playlist definition file.                                                                                 | A **playlist** course appears; lessons show URLs, not local playback paths.                                                                    | —          |

---

## Lesson order

| Step                                                                                              | Expected result                                                                                                  | Blocked if                              |
| ------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- | --------------------------------------- |
| Open a **folder** course with at least two lessons. On the first lesson row, click **Move down**. | The two lessons **swap** order in the table; **Move up** / **Move down** hide on first/last rows as appropriate. | Fewer than two lessons.                 |
| Click **Rescan folder** without changing files on disk.                                           | Lesson **order stays** as after the move (not reverted to filename order).                                       | Playlist course (no **Rescan folder**). |

---

## Player resume

| Step                                                                                                         | Expected result                                                                               | Blocked if                                                                                   |
| ------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| From a course, open a **local video** lesson. Play or seek partway through, then go **Back** to the course.  | **Watched** / resume hints update; reopening the same lesson resumes near the prior position. | Lesson is playlist-only (no local file) or broken/missing file.                              |
| (Optional) If the file has chapters and **ffprobe** is installed, open the chapter list and click a chapter. | Playback seeks to that chapter start.                                                         | **ffprobe** missing (chapters may be absent or unknown; app must not invent a chapter list). |

---

## Manual complete

| Step                                                                                          | Expected result                                                                                                   | Blocked if |
| --------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- | ---------- |
| On the course table or inside the player, use **Mark complete** on a lesson that is not done. | Status shows **done**; course rollup updates.                                                                     | —          |
| Use **Mark incomplete** on the same lesson.                                                   | Status returns to not started or in progress; **done** is not re-applied automatically after watching to the end. | —          |

---

## Roadmap

| Step                                                                                                         | Expected result                                                                                  | Blocked if                 |
| ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------ | -------------------------- |
| Side nav → **Roadmap**. Create or open a roadmap and **add** at least one step linked to an imported course. | Graph shows nodes; step status reflects linked course progress (derived, not a manual checkbox). | No courses in library yet. |
| Add a prerequisite edge between two steps (if the UI offers it), then remove it.                             | Graph refreshes; cycle errors show as a clear message if you try an invalid edge.                | Fewer than two steps.      |

---

## OCR capture

| Step                                                                                                                                                                                                              | Expected result                                                                                 | Blocked if                                                                                                                              |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| Open a **local video** lesson. Pause on a frame with readable text. Click **Capture card**, drag a region on the frame, run OCR, edit front/back if needed, confirm **Save card** (or equivalent commit control). | Card commits into the AuraVault **Anki profile**; capture panel closes without a generic crash. | **Tesseract** not installed or app built without OCR support (capture shows install hint / error). Non-video lesson (no frame capture). |

---

## In-app review

| Step                                                                                                                                             | Expected result                                                                                    | Blocked if                                                              |
| ------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| After at least one AuraVault card exists (e.g. from OCR capture above), note **New / Learning / Review** counts in the top bar. Click **Study**. | In-app **review** opens: front shown, **Show answer**, then Again / Hard / Good / Easy as offered. | No due cards in AuraVault decks (counts all zero and nothing captured). |
| Grade one card **Good** (or another offered button).                                                                                             | Next card loads or empty-queue message appears; top-bar due counts change.                         | —                                                                       |
| (Optional) With **Anki Desktop** installed and path set in Settings, from review fallback use **Open in Anki Desktop**.                          | External Anki opens on the same **AuraVault profile** directory shown in Settings.                 | **`anki`** not on PATH and Settings binary path wrong.                  |

---

## Cards on a lesson

| Step                                                                                                                                     | Expected result                                                                   | Blocked if                         |
| ---------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | ---------------------------------- |
| On the **player** for a lesson where you saved a capture (or other linked card), scroll to the **linked cards** list.                    | At least one link shows with preview text / timestamp consistent with the commit. | No card linked to this lesson yet. |
| (Optional) Add a **lesson note**, promote it to a card only after explicit **Save to Anki** (or equivalent confirm)—not on typing alone. | New link appears after confirm; note remains in vault if you cancel.              | —                                  |

---

## Unlink

| Step                                                                         | Expected result                                                                                                             | Blocked if              |
| ---------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| On the player linked-cards list, choose **Unlink** (or equivalent), confirm. | Link **disappears** from the lesson list; the note remains in the Anki collection (still reviewable globally if still due). | No links on the lesson. |

---

## Paste playlist

| Step                                                                                                                                              | Expected result                                                                                   | Blocked if |
| ------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- | ---------- |
| **Library** → **Paste playlist**. Enter a **course title** and two or more lines `Lesson title<TAB>https://example.com/watch/…`, then **Create**. | New **playlist** course appears; lessons show titles and URLs; **no** video files are downloaded. | —          |
| Paste a line **without** a tab character and try **Create**.                                                                                      | Error names the bad line; no partial course left in a broken state.                               | —          |

---

## Backup

| Step                                                | Expected result                                                                                                      | Blocked if                     |
| --------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- | ------------------------------ |
| **Settings** → **Backup**, choose a folder on disk. | Success message with a backup path; folder contains copied **vault.db** and **anki-profile** tree (local copy only). | Cannot write to chosen folder. |

---

## Settings

| Step                                                                                         | Expected result                                                                                             | Blocked if                                                                                              |
| -------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| Open **Settings**. Confirm **Anki profile directory** shows a path under app data.           | Read-only path is non-empty after first run.                                                                | —                                                                                                       |
| Change **Tesseract language** or **Anki desktop binary**, click **Save**.                    | **Settings saved** status; values persist after reload.                                                     | —                                                                                                       |
| Set **Transcription model path** to a valid local Whisper model file (optional).             | Hint reads transcription **enabled**; player transcribe control becomes usable where implemented.           | **`whisper`** CLI / model path missing (transcription stays disabled; not required for other sections). |
| (Optional) **Restore** from a backup folder you created earlier—only on a throwaway profile. | Confirm dialog; after **Restore backup**, library matches backup; prior live data moved aside per app copy. | No backup folder; avoid on your only copy of real data.                                                 |

---

## Safety pass

Automated proofs (2026-10-28). Run:

```bash
cargo test -p vault-core rejects_filesystem_root
cargo test -p vault-core vault_core_must_not_depend_on_anki
cargo test -p vault-core no_network_weight_download
cargo test -p anki-bridge unlink_card_keeps_note_in_temp_profile
cargo test -p anki-bridge read_media_rejects_path_escape
```

| Invariant                                                     | Test                                                                                 | Result   |
| ------------------------------------------------------------- | ------------------------------------------------------------------------------------ | -------- |
| Import of **`/`** is rejected                                 | `rejects_filesystem_root` (`crates/vault-core/tests/risky_import_path.rs`)           | **pass** |
| **`vault-core`** does not depend on the **`anki`** crate      | `vault_core_must_not_depend_on_anki` (`crates/vault-core/tests/architecture.rs`)     | **pass** |
| **Unlink** does not delete the Anki note                      | `unlink_card_keeps_note_in_temp_profile` (`crates/anki-bridge/tests/integration.rs`) | **pass** |
| **Transcription** does not download weights                   | `no_network_weight_download` (`crates/vault-core/tests/transcription.rs`)            | **pass** |
| **`read_media`** rejects a path that escapes the media folder | `read_media_rejects_path_escape` (`crates/anki-bridge/tests/media.rs`)               | **pass** |

---

## Quick pass criteria

You have exercised the v1 / v1.1 / v1.2 surface described in [`README.md`](../README.md), [`V1.md`](V1.md), [`V1.1.md`](V1.1.md), and [`V1.2.md`](V1.2.md) when every section above completes as expected (or is explicitly **blocked** for a documented missing tool, not a crash or silent wrong behavior).

**Status:** **READY** — dev launch shows Library (§ Launch); fixture import / reorder / rescan (§ Fixture pass); automated gate prompt **132** and safety prompt **139** passed (see [`docs/PROGRESS.md`](PROGRESS.md)).

---

## Verification counts (2026-10-28)

```bash
cargo test --workspace   # 195 passed; 0 failed
cd ui && npm test       # 38 files; 93 passed
```
