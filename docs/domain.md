# AuraVault domain model (v1)

Derived from `/home/xingzi/Documents/auravault-charter.md`. Describes **entities, identity, fields, and invariants** only—no UI framework, no SQL schema.

---

## Ubiquitous language

| Term               | Meaning                                                                                                    |
| ------------------ | ---------------------------------------------------------------------------------------------------------- |
| **Lesson**         | Smallest unit of learning progress; backed by one playable/readable **source** (file or playlist item).    |
| **Course**         | Ordered collection of lessons (usually one imported folder).                                               |
| **Progress**       | Watch/resume and **status** for one lesson; never stored on course or roadmap directly.                    |
| **Derived status** | Course or roadmap-step status **computed** from lessons (or linked course)—not user-editable.              |
| **Broken**         | Source is missing or course was removed; **not** “learner is behind schedule”.                             |
| **Learning order** | Saved `sort_index` sequence on lessons; changes only by explicit user action (or accepted proposal later). |

---

## Value types

### `LessonStatus` (stored on `Progress`)

| Value         | Meaning                                                                            |
| ------------- | ---------------------------------------------------------------------------------- |
| `not_started` | No playback yet and `resume_position_ms == 0`; not manually complete.              |
| `in_progress` | Playback started, `resume_position_ms > 0`, or user marked incomplete from `done`. |
| `done`        | User manually marked complete (v1: **no** auto-`done` from watch %).               |
| `broken`      | Source file or playlist item unavailable; see recovery rules below.                |

### `DerivedStatus` (computed only)

Same four values. Used for **course rollup** and **roadmap step** (via linked course). Rollup rule (charter Decision 13):

1. `not_started` if every lesson is `not_started`.
2. `done` if every lesson is `done`.
3. `broken` if any lesson is `broken` **and** no lesson is `in_progress` or `done`.
4. Otherwise `in_progress`.

### `LinkStatus` (`AnkiCardLink`)

| Value    | Meaning                                                                 |
| -------- | ----------------------------------------------------------------------- |
| `active` | Vault still recognizes the course/lesson context for this link.         |
| `broken` | Course deleted (or lesson removed) in vault; Anki note may still exist. |

### `CourseSourceKind`

| Value      | v1 usage                                                    |
| ---------- | ----------------------------------------------------------- |
| `folder`   | Local directory import (primary v1).                        |
| `playlist` | Online playlist metadata (charter-ready; after folder MVP). |

### `LessonSourceKind`

| Value           | Exactly one per lesson             |
| --------------- | ---------------------------------- |
| `local_file`    | Lesson backed by a `SourceFile`.   |
| `playlist_item` | Lesson backed by a `PlaylistItem`. |

### `SourceFileKind`

`video` | `audio` | `pdf` | `text` | `subtitle` (subtitle files are indexed but normally **attached** to a video lesson, not promoted to lessons).

### `ExtractorOutcome`

Per metadata field: `known(value)` | `unknown(reason)` — never invented summaries (charter).

---

## Entities

### Library

**Identity:** `LibraryId` (stable UUID; one primary library per install in v1).

**Role:** Root aggregate for all vault content on this machine.

| Field          | Type        | Notes               |
| -------------- | ----------- | ------------------- |
| `id`           | `LibraryId` | Primary key.        |
| `display_name` | string      | e.g. `"AuraVault"`. |
| `created_at`   | timestamp   |                     |
| `updated_at`   | timestamp   |                     |

**Relationships:** Owns many `Course`, many `Roadmap`. Does not own Anki collection data (separate Anki profile).

---

### Course

**Identity:** `CourseId` (UUID). Re-import of the same folder **reuses** the same `CourseId` when root path matches (idempotent import).

**Role:** One trackable course—typically one imported folder (AulaVault-like).

| Field             | Type               | Notes                                                                    |
| ----------------- | ------------------ | ------------------------------------------------------------------------ |
| `id`              | `CourseId`         |                                                                          |
| `library_id`      | `LibraryId`        |                                                                          |
| `title`           | string             | User-editable; default from folder name.                                 |
| `source_kind`     | `CourseSourceKind` | v1: `folder`.                                                            |
| `root_path`       | path               | Absolute path to imported folder; required when `source_kind == folder`. |
| `playlist_url`    | string?            | When `source_kind == playlist` (later).                                  |
| `imported_at`     | timestamp          |                                                                          |
| `last_scanned_at` | timestamp          | Last filesystem scan / re-import.                                        |

**Derived (not stored):** `derived_status: DerivedStatus` from child lessons’ `Progress.status`.

**Import rules (folder, Decision 12):** Lesson-eligible extensions under `root_path`; ignore hidden segments and `node_modules`. Subtitle files attach by stem; re-import adds new files, marks missing lesson sources `broken`, does not duplicate lessons.

**Lifecycle:** Deleting a course does **not** delete Anki notes; sets related `AnkiCardLink.link_status` to `broken`.

---

### Lesson

**Identity:** `LessonId` (UUID). Stable across re-import via stable key: `(course_id, lesson_stable_key)` where `lesson_stable_key` is normally **relative path** from course root for file-backed lessons.

**Role:** One item in the learning sequence with exactly one content source.

| Field               | Type                        | Notes                                            |
| ------------------- | --------------------------- | ------------------------------------------------ |
| `id`                | `LessonId`                  |                                                  |
| `course_id`         | `CourseId`                  |                                                  |
| `lesson_stable_key` | string                      | Idempotent import key (e.g. relative path).      |
| `display_title`     | string                      | From extractor or filename; may be `unknown`.    |
| `sort_index`        | int                         | **Learning order** in course (user may reorder). |
| `source_kind`       | `LessonSourceKind`          |                                                  |
| `source_file_id`    | `SourceFileId`?             | Set when `local_file`.                           |
| `playlist_item_id`  | `PlaylistItemId`?           | Set when `playlist_item`.                        |
| `subtitle_file_id`  | `SourceFileId`?             | Optional attached `srt`/`vtt` (same stem).       |
| `duration_ms`       | `ExtractorOutcome<int>`?    | Video/audio; unknown allowed.                    |
| `page_count`        | `ExtractorOutcome<int>`?    | PDF.                                             |
| `heading`           | `ExtractorOutcome<string>`? | First heading for md/txt.                        |

**Invariant — one source:**

- If `source_kind == local_file` → `source_file_id` is set and `playlist_item_id` is null.
- If `source_kind == playlist_item` → `playlist_item_id` is set and `source_file_id` is null.
- Never both; never neither.

**Relationships:** Exactly one `Progress` (by `lesson_id`). Zero or many `TranscriptSegment` (when subtitles ingested). Zero or many `OcrSelection`, `AnkiCardLink`.

---

### SourceFile

**Identity:** `SourceFileId` (UUID). Stable key: `(course_id, relative_path)` across re-import.

**Role:** Indexed file on disk belonging to a course (lesson media, attached subtitle, or future sidecar).

| Field           | Type             | Notes                                             |
| --------------- | ---------------- | ------------------------------------------------- |
| `id`            | `SourceFileId`   |                                                   |
| `course_id`     | `CourseId`       |                                                   |
| `relative_path` | string           | From course `root_path`; POSIX-style for logic.   |
| `kind`          | `SourceFileKind` |                                                   |
| `extension`     | string           | Lowercase, no dot.                                |
| `size_bytes`    | int?             | Unknown if unreadable.                            |
| `last_seen_at`  | timestamp        | Updated on scan when file exists.                 |
| `missing_since` | timestamp?       | Set when absent on scan → drives lesson `broken`. |

**Invariant:** Path must resolve under course `root_path` at import time; broken lessons when file absent.

---

### PlaylistItem

**Identity:** `PlaylistItemId` (UUID). Stable key: `(course_id, provider_item_key)` when playlist courses exist.

**Role:** One entry in a playlist-backed course (charter Decision 2; **after** folder MVP).

| Field                 | Type             | Notes                                    |
| --------------------- | ---------------- | ---------------------------------------- |
| `id`                  | `PlaylistItemId` |                                          |
| `course_id`           | `CourseId`       | Parent playlist course.                  |
| `ordinal`             | int              | Order in playlist.                       |
| `url`                 | string           | Provider URL (metadata only; no ripper). |
| `title`               | string           | Stored title; may be unknown.            |
| `duration_ms`         | int?             | Unknown allowed.                         |
| `local_override_path` | path?            | User-mapped file for playback.           |
| `provider_item_key`   | string?          | Stable id from provider when known.      |

**Relationships:** At most one `Lesson` may reference this item (`playlist_item_id`).

---

### Progress

**Identity:** `lesson_id: LessonId` (1:1 with `Lesson`; `Progress` has no separate public id in v1).

**Role:** All **lesson-level** state: status, resume, watch metrics. Course and roadmap never store parallel status fields.

| Field                     | Type           | Notes                                  |
| ------------------------- | -------------- | -------------------------------------- |
| `lesson_id`               | `LessonId`     | Primary key.                           |
| `status`                  | `LessonStatus` | Includes `broken` when source missing. |
| `resume_position_ms`      | int            | ≥ 0; persisted across sessions.        |
| `max_position_reached_ms` | int            | For watched % (Decision 14).           |
| `marked_done_at`          | timestamp?     | Set when user marks `done`.            |
| `last_engaged_at`         | timestamp?     | Last playback or manual status change. |

**Derived (not stored):** `watched_percent` = `min(100, max_position_reached_ms / duration_ms * 100)` when lesson `duration_ms` is known; else unknown.

**Allowed transitions (v1):**

| From             | To            | Trigger                                                                  |
| ---------------- | ------------- | ------------------------------------------------------------------------ |
| `not_started`    | `in_progress` | Playback starts or `resume_position_ms > 0`.                             |
| any non-`broken` | `done`        | User marks complete.                                                     |
| `done`           | `in_progress` | User marks incomplete (if `resume_position_ms > 0`, else `not_started`). |
| any              | `broken`      | Source file/item missing on scan.                                        |
| `broken`         | `not_started` | User action after source restored; **do not** guess prior status.        |

Setting resume **does not** set `done`. Watch % **does not** auto-set `done` (Decision 11).

---

### Roadmap

**Identity:** `RoadmapId` (UUID).

**Role:** Named learning path as a **directed graph** of courses (flowchart view).

| Field         | Type        | Notes |
| ------------- | ----------- | ----- |
| `id`          | `RoadmapId` |       |
| `library_id`  | `LibraryId` |       |
| `title`       | string      |       |
| `description` | string?     |       |
| `created_at`  | timestamp   |       |
| `updated_at`  | timestamp   |       |

**Graph structure (part of aggregate):**

| Field   | Type                                                                 | Notes                                                             |
| ------- | -------------------------------------------------------------------- | ----------------------------------------------------------------- |
| `edges` | list of `{ from_step_id: RoadmapStepId, to_step_id: RoadmapStepId }` | Directed edges for flowchart; no duplicate step ids in edge list. |

**Derived (not stored):** Overall roadmap `DerivedStatus` from all referenced courses using the **same rollup** as multi-lesson courses (treat each step’s course as one “lesson” in the rollup)—equivalently: rollup over steps where each step’s status is its course’s `derived_status`.

---

### RoadmapStep

**Identity:** `RoadmapStepId` (UUID).

**Role:** One **node** in a roadmap graph; points at a **course** (not a lesson).

| Field        | Type            | Notes                                                                         |
| ------------ | --------------- | ----------------------------------------------------------------------------- |
| `id`         | `RoadmapStepId` |                                                                               |
| `roadmap_id` | `RoadmapId`     |                                                                               |
| `course_id`  | `CourseId`      | Referenced course.                                                            |
| `label`      | string?         | Optional display override on the node.                                        |
| `sort_hint`  | int?            | Optional layout hint only; **not** learning order (graph edges define order). |

**Derived (not stored):** `derived_status: DerivedStatus` = `course.derived_status`.

**Invariant:** No separate `Progress` or checkbox on the step—only the linked course’s lesson progress matters.

---

### TranscriptSegment

**Identity:** `TranscriptSegmentId` (UUID) or composite `(lesson_id, sequence_index)`.

**Role:** One subtitle cue ingested from SRT/VTT (Decision 9: ingest only).

| Field            | Type                  | Notes                                                |
| ---------------- | --------------------- | ---------------------------------------------------- |
| `id`             | `TranscriptSegmentId` |                                                      |
| `lesson_id`      | `LessonId`            | Video (or audio) lesson with attached subtitle file. |
| `sequence_index` | int                   | Order in file.                                       |
| `start_ms`       | int                   |                                                      |
| `end_ms`         | int                   |                                                      |
| `text`           | string                | Cue text.                                            |

**Invariant:** Segments belong to lessons that have a `subtitle_file_id`; no invented transcript text.

---

### OcrSelection

**Identity:** `OcrSelectionId` (UUID).

**Role:** Record of a **local Tesseract** capture from a video frame before/at card creation (Decision 1, 15).

| Field                | Type              | Notes                                |
| -------------------- | ----------------- | ------------------------------------ |
| `id`                 | `OcrSelectionId`  |                                      |
| `lesson_id`          | `LessonId`        |                                      |
| `video_timestamp_ms` | int               | Position in lesson media.            |
| `recognized_text`    | string            | OCR output; may be empty on failure. |
| `created_at`         | timestamp         |                                      |
| `anki_card_link_id`  | `AnkiCardLinkId`? | Set after note created.              |

**Invariant:** Cloud OCR forbidden in v1; failures stored explicitly, not hallucinated.

---

### AnkiCardLink

**Identity:** `AnkiCardLinkId` (UUID).

**Role:** Stable bridge from vault lesson context to Anki collection note (Decision 4, 15, 16).

| Field                | Type             | Notes                                       |
| -------------------- | ---------------- | ------------------------------------------- |
| `id`                 | `AnkiCardLinkId` |                                             |
| `lesson_id`          | `LessonId`       |                                             |
| `anki_note_id`       | int64            | Note id in AuraVault-owned Anki collection. |
| `video_timestamp_ms` | int              | Same sense as charter “timestamp”.          |
| `link_status`        | `LinkStatus`     |                                             |
| `created_at`         | timestamp        |                                             |

**Invariant:** **Card body lives only in the Anki collection** (deck `AuraVault`, type `AuraVault OCR` in v1). Vault stores ids and context, not card HTML.

**Invariant — course delete:** Deleting a course sets all its lessons’ `AnkiCardLink.link_status` to `broken`. **Does not** delete the Anki note.

**Optional back-reference:** `ocr_selection_id` if created from one `OcrSelection`.

---

## Global invariants (checklist)

1. **One source per lesson:** `local_file` **xor** `playlist_item`.
2. **Progress on lesson only:** `Progress` is 1:1 with `Lesson`; course and roadmap step status are **derived**.
3. **Broken means missing source**, not behind schedule.
4. **AnkiCardLink:** stores `anki_note_id` + `lesson_id` + `video_timestamp_ms`; body in Anki; links go `broken` on course delete; notes are not deleted by vault.
5. **Learning order:** `Lesson.sort_index` changes only by explicit user action (or future **accepted** proposal—not silent reorder).
6. **Re-import:** Same `lesson_stable_key` → same `LessonId`; new files → new lessons; vanished files → `broken`.
7. **Subtitles:** `srt`/`vtt` attach by stem; `TranscriptSegment` rows optional per ingested cues.
8. **No second SRS:** Scheduling fields live in Anki/`rslib` only; vault may read due counts, not implement FSRS.

---

## Entity relationship (conceptual)

```text
Library
 ├── Course ──< Lesson ── Progress (1:1)
 │              ├── SourceFile (lesson media)
 │              ├── SourceFile? (subtitle)
 │              ├── TranscriptSegment*
 │              ├── OcrSelection*
 │              └── AnkiCardLink*
 │              └── PlaylistItem? (playlist-backed lesson)
 └── Roadmap ──< RoadmapStep ──> Course (reference)
        └── edges between RoadmapSteps
```

---

## Out of model (v1)

- SQL tables, Tauri commands, React components.
- Anki `Card` / `Revlog` entities (owned by Anki collection).
- User accounts, payments, cloud sync.
- Whisper-generated transcripts (ingest existing subtitles only).
