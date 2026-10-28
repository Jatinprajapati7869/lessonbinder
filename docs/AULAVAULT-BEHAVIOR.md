# Observable behavior — competitor `aulavault` binary

**Artifact:** `/tmp/aulavault-re/squashfs-root/usr/bin/aulavault` (extracted AppImage root)
**Method:** `strings` on the ELF binary. **REA was not used** (not connected in this session).
**No decompilation:** command names and user-facing error strings only; no recovered source or asset bytes.

**Product hints in binary:** bundle id `com.coursevault.app`, marketing strings `coursevault` / `aulavault`, Pro-license gating on some features, bundled Whisper with **Hugging Face download URLs** for `ggml-tiny/base/small` weights, and LLM-style **flashcard generation** from transcripts (`src/flashcards.rs` path string only).

This document compares those IPC surfaces to **Lessonbinder** [`docs/SPEC.md`](SPEC.md) and [`docs/SCOPE-v1.1.md`](SCOPE-v1.1.md). Lessonbinder has **no accounts, cloud sync, or automatic course understanding**.

---

## Library and files

| Command                     | Implied behavior                                                                          | Lessonbinder equivalent                                                                                                       |
| --------------------------- | ----------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `scan_course_folder`        | Import or index a folder as a course (likely with progress events).                       | **Yes** — `import_course_folder` / folder import (`SPEC` F-01–F-05); v1.1 rescan (`SPEC-v1.1` I-01–I-03) via `rescan_course`. |
| `cancel_scan_course_folder` | Abort an in-flight folder scan.                                                           | **No** — Lessonbinder rescan/import is synchronous from the UI’s perspective; no cancel command.                              |
| `check_risky_import_path`   | Warn or block dangerous import paths (binary strings mention scanning `/` / system root). | **Partial** — charter/SPEC honest import and `.lessonbinderignore` (v1.1 I-04–I-06); no dedicated “risky path” IPC name.      |
| `check_missing_paths`       | Detect lessons whose media paths no longer exist on disk.                                 | **Partial** — broken-lesson semantics on rescan/import (`SPEC` F-04, v1.1 I-02); not exposed under this name.                 |
| `pick_course_folders`       | Native dialog to choose one or more course directories.                                   | **Partial** — library import picks a folder (`import_course_folder`); no multi-folder picker in SPEC.                         |
| `open_directory_fallback`   | Open a directory in the OS file manager when a path cannot be shown in-app.               | **No** in SPEC — opener plugin may exist for URLs; not a documented v1/v1.1 requirement.                                      |
| `allow_asset_paths`         | Register or validate extra filesystem roots for webview/asset access.                     | **No** — Lessonbinder serves lesson media via controlled paths/commands, not competitor-style asset allowlists.               |
| `lesson_resources`          | List or resolve ancillary files tied to a lesson (attachments, sidecars).                 | **Partial** — subtitle sidecars indexed at import (`SPEC` F-03, C-03); no separate `lesson_resources` API.                    |
| `list_lesson_resources`     | Enumerate lesson-linked files for the UI.                                                 | **Partial** — course detail / lesson rows expose paths; not this command name.                                                |
| `lesson_subtitles`          | Fetch subtitle/cue data for a lesson.                                                     | **Yes** — `list_lesson_cues` / player cues (`SPEC-v1.1` S-01, S-02).                                                          |
| `list_lesson_subtitles`     | Same seam, list-oriented naming.                                                          | **Yes** — same as `lesson_subtitles` / `list_lesson_cues`.                                                                    |
| `open_lesson_resource`      | Open a lesson file or attachment with the system opener.                                  | **Partial** — lesson playback reads local media; external open not a SPEC scenario.                                           |
| `pick_thumbnail_image`      | Choose an image file for a course/lesson thumbnail.                                       | **No** — not in SPEC or SCOPE-v1.1.                                                                                           |
| `capture_video_frame`       | Grab the current video frame for downstream use (OCR/cards).                              | **Yes** — frame capture → OCR card draft/commit (`SPEC` user story 14; `build_capture_draft` / `commit_capture_card`).        |
| `read_text_via_dialog`      | Load text from a user-picked file via dialog.                                             | **No** — not in SPEC (markdown/PDF lessons use import, not ad-hoc file read IPC).                                             |
| `save_text_via_dialog`      | Save text to a user-chosen path via dialog.                                               | **No** — not in SPEC.                                                                                                         |

---

## Whisper / subtitles pipeline

| Command                     | Implied behavior                                                             | Lessonbinder equivalent                                                                                                                                                              |
| --------------------------- | ---------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `whisper_download_model`    | Download Whisper weights from the network (binary embeds Hugging Face URLs). | **Do not implement** — v1.1 allows **optional local** transcription only when the user sets a **model path**; SCOPE-v1.1 explicitly excludes Lessonbinder-initiated weight download. |
| `whisper_model_status`      | Report whether a model is present, downloading, or ready.                    | **Partial** — `get_transcription_model_path` + engine availability (`SPEC-v1.1` T-01–T-02); no bundled downloader status.                                                            |
| `whisper_cancel_transcribe` | Stop an in-progress Whisper run.                                             | **Partial** — `transcribe_lesson` exists (`SPEC-v1.1` T-02); **no cancel** command in Lessonbinder today.                                                                            |
| `whisper_cancel_download`   | Abort a model download.                                                      | **Do not implement** (pairs with `whisper_download_model`).                                                                                                                          |
| `whisper_delete_model`      | Remove a downloaded model from disk.                                         | **Do not implement** — Lessonbinder does not manage downloaded model stores.                                                                                                         |
| `generate_subtitles`        | Run speech-to-text to create subtitle/cue data for a lesson.                 | **Partial** — `transcribe_lesson` + `TranscriptionEngine` (`SPEC-v1.1` T-\*); user-supplied binary/model, not competitor’s integrated downloader.                                    |

---

## Flashcards

| Command               | Implied behavior                                                                                                      | Lessonbinder equivalent                                                                                                                                        |
| --------------------- | --------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `generate_flashcards` | LLM batch-generates Q/A cards from a transcript (binary prompt: “study flashcards from a video lesson's transcript”). | **No** — Lessonbinder uses **manual** OCR capture and **subtitle cue → card draft** (`SPEC-v1.1` S-04); no automatic flashcard synthesis from full transcript. |
| `ollama_list_models`  | List local Ollama models (likely used to pick an LLM for flashcards).                                                 | **No** — not in SPEC or SCOPE-v1.1; no Ollama integration.                                                                                                     |

---

## License

| Command              | Implied behavior                                               | Lessonbinder equivalent                                                     |
| -------------------- | -------------------------------------------------------------- | --------------------------------------------------------------------------- |
| `license_status`     | Return activation state for a Pro/commercial license.          | **Do not implement** — no accounts or license server in charter/SPEC/SCOPE. |
| `activate_license`   | Activate with a license key (`license.json` string in binary). | **Do not implement**                                                        |
| `deactivate_license` | Deactivate / unbind license on this machine.                   | **Do not implement**                                                        |
| `check_in_license`   | Online or periodic license validation (“check in”).            | **Do not implement**                                                        |

---

## Required names — classification index

| Name                        | Group      | Lessonbinder stance          |
| --------------------------- | ---------- | ---------------------------- |
| `scan_course_folder`        | library    | Equivalent (import/rescan)   |
| `cancel_scan_course_folder` | library    | No equivalent                |
| `check_risky_import_path`   | library    | Partial (honest import only) |
| `lesson_resources`          | library    | Partial                      |
| `lesson_subtitles`          | library    | Equivalent (cues)            |
| `open_lesson_resource`      | library    | Partial                      |
| `pick_thumbnail_image`      | library    | No equivalent                |
| `capture_video_frame`       | library    | Equivalent (capture/OCR)     |
| `generate_flashcards`       | flashcards | No equivalent (do not copy)  |
| `whisper_download_model`    | whisper    | **Do not implement**         |
| `whisper_model_status`      | whisper    | Partial                      |
| `whisper_cancel_transcribe` | whisper    | Partial (no cancel)          |
| `license_status`            | license    | **Do not implement**         |
| `deactivate_license`        | license    | **Do not implement**         |

---

## Other observable IPC (not in the required list)

Included for completeness from the same `strings` pass; same grouping rules apply.

| Command                   | Group      | Note                                 |
| ------------------------- | ---------- | ------------------------------------ |
| `check_missing_paths`     | library    | See `check_missing_paths` row above. |
| `list_lesson_resources`   | library    | See `lesson_resources`.              |
| `list_lesson_subtitles`   | library    | See `lesson_subtitles`.              |
| `pick_course_folders`     | library    | Folder picker for import.            |
| `open_directory_fallback` | library    | OS folder reveal.                    |
| `allow_asset_paths`       | library    | Tauri asset scope.                   |
| `whisper_cancel_download` | whisper    | **Do not implement**                 |
| `whisper_delete_model`    | whisper    | **Do not implement**                 |
| `generate_subtitles`      | whisper    | See transcription port.              |
| `activate_license`        | license    | **Do not implement**                 |
| `check_in_license`        | license    | **Do not implement**                 |
| `ollama_list_models`      | flashcards | **Do not implement**                 |

---

## Limits of this analysis

- Only **symbol/string** evidence; typos in the table (e.g. concatenated Rust symbol names) were disambiguated by matching the user’s known-name list and repeated substrings in the binary.
- Tauri **plugin** commands (`plugin:dialog|…`, `plugin:shell|open`, SQL plugin) are present but omitted here; they are framework plumbing, not app domain commands.
- Behavior beyond name + embedded error/prompt strings is **inference**, not execution trace.

---

## Build

**Out of scope for this gap list (explicit):**

- **`generate_flashcards`** — Already covered by Lessonbinder’s **cue → card draft → Anki commit** path (implementation prompts **62** cue-to-card draft, **29** selection to card draft, **30** write the Anki note). We do not add LLM batch flashcards from full transcripts.
- **Whisper model download** (`whisper_download_model`, `whisper_cancel_download`, `whisper_delete_model`) — **Refused.** v1.1 transcription is optional and **local-only** (user-supplied model path); see [`SCOPE-v1.1.md`](SCOPE-v1.1.md).
- **Licensing** (`license_status`, `activate_license`, `deactivate_license`, `check_in_license`) — **Refused.** No accounts, activation server, or Pro gate in charter/SPEC.

**Gaps worth building (competitor parity without copying the above):**

1. **Risky import guard before a folder scan**
   Block or confirm before indexing paths that are unsafe or misleading (e.g. filesystem root, huge system trees), aligned with honest import in SPEC.
   **Done when:** Import/rescan refuses or requires explicit confirmation for configured risky paths, with a test or SPEC row naming the behavior.

2. **Cancel an in-progress folder scan**
   Allow the user to abort a long-running folder import/rescan without leaving the library half-updated.
   **Done when:** A cancel command or equivalent stops the active scan, UI returns to a stable state, and partial work is defined (rollback or commit rules documented and tested).

3. **Extra lesson resources (PDF, SRT, images next to a video) that are not separate lessons**
   Surface sidecar files tied to a video lesson (subtitles already indexed; extend to other neighbors) without promoting each file to its own lesson row.
   **Done when:** Lesson detail or a dedicated command lists linked resources for a video lesson, SRT/VTT remain sidecars not lessons (F-03), and at least one non-subtitle type (e.g. PDF or image) is listed or openable per lesson.

4. **Capture a local video frame at the current time and store it as that lesson’s thumbnail**
   Persist a poster image derived from playback position, distinct from OCR capture cards.
   **Done when:** User can save current frame as the lesson thumbnail, thumbnail survives reload and appears in course/lesson UI, stored under app data or vault conventions (not the Anki media folder unless explicitly chosen).
