# Lessonbinder scope — v1.1

**Status:** Planning (prompt 51).
**Baseline:** Shipped v1 behavior and gaps are summarized in [`V1.md`](V1.md). Charter intent for study and subtitles is in [`charter.md`](charter.md) (Decisions 9, 14, 16); current v1 study behavior is documented in [`STUDY.md`](STUDY.md).

v1.1 closes the largest v1 honesty gaps **without** expanding into accounts, sync, mobile, or a second SRS. It does **not** rename the existing OCR note type (`LessonbinderClip` in code; charter wording “Lessonbinder OCR” unchanged for v1.1).

---

## In scope (v1.1 only)

### Study — in-app review (primary)

- **Default:** Review cards in **Lessonbinder decks** (`Lessonbinder::…`) through **`rslib`** in-process: show the next due card and grade with **Again / Hard / Good / Easy**.
- **Profile:** Same dedicated Lessonbinder Anki profile as today (`anki-profile/` under app data). No reading the user’s everyday Anki profile.
- **Fallback:** Launch **Anki Desktop** remains supported when in-app review cannot run (e.g. missing capability), but it is **not** the default “Study” path. Update [`STUDY.md`](STUDY.md) when this ships.
- **Not in v1.1:** Custom FSRS, scheduler logic, or review UX duplicated outside Anki’s grading APIs; full Anki Desktop feature parity.

### Subtitles on the player

- **Show** ingested **SRT/VTT** cues on the lesson player (data already extracted at index time; see v1 C-03).
- **Click a cue** → seek playback to that timestamp.
- **Cue → card draft:** A subtitle line can seed an OCR-style **card draft** (same commit path as frame capture; note type unchanged).
- **Not in v1.1:** Whisper, on-device transcription, or generated subtitles (charter Decision 9 unchanged for automatic pipelines).

### Watch tracking — interval union

- **Persist watched ranges:** Union of **played intervals** on the timeline (not only `max_position_reached_ms`).
- **`watched_percent`:** When lesson `duration_ms` is known, compute from the **union length / duration** (charter Decision 14 evolution).
- **`done`:** Stays **manual only**; watch metrics do not auto-complete lessons.

### Folder rescan and ignore rules

- **Rescan** an already-imported folder: pick up new files, mark removed files **broken**, keep idempotent lesson ids where paths match.
- **`.lessonbinderignore`:** Honor ignore patterns at import/rescan (same spirit as hidden paths / `node_modules` exclusions in v1).

### Lesson notes (vault-only)

- **User-typed notes** per lesson, stored in **`vault.db`**.
- **Separate from Anki notes** (capture/OCR cards remain in the collection only).

### PDF and markdown position

- **Remember position** for PDF and markdown lessons (page, scroll offset, or equivalent persisted field)—resume where the user left off, analogous to video resume.

### Search

- **Search** across lesson **titles**, file **paths**, and **subtitle cue text** (indexed from ingested sidecars).
- UI to run search and open a lesson from a hit (scope of prompts 72–73).

### Optional local transcription (user-supplied model)

- **Only when** the user configures a **local model path** on disk.
- Adapter plugs into the existing **`TranscriptionEngine`** port ([`TRANSCRIPTION.md`](TRANSCRIPTION.md)).
- **No** Lessonbinder-initiated **download** of model weights over the network.
- **Not** default-on; no Whisper requirement for v1.1 ship criteria.

---

## Out of scope (v1.1)

Explicitly **not** part of v1.1 (unchanged from charter / v1):

| Area                                             | Notes                                        |
| ------------------------------------------------ | -------------------------------------------- |
| User **accounts** and auth                       | —                                            |
| **Cloud sync** of library or progress            | —                                            |
| **Mobile** apps                                  | —                                            |
| **AI chat tutor** / coaching UI                  | —                                            |
| **Auto-`done`** from watch percent or thresholds | `done` remains manual                        |
| **Playlist downloaders** / term-breaking rippers | Metadata-only playlist courses stay as in v1 |
| **Renaming** the existing OCR **note type**      | Keep `LessonbinderClip` / current fields        |

Also still out of scope: custom FSRS in Lessonbinder, cloud OCR, bundled weight downloads, and “AI understands the course” summarization.

---

## Relationship to prompts 52–100

Implementation checklist: [`PROGRESS.md`](PROGRESS.md) rows **52–100** (SPEC v1.1, study stack, subtitles, ranges, rescan, notes, readers, search, transcription adapter, backup/bundle, polish). Row **51** records approval of this scope document.

## Success criteria (planning)

- [`V1.md`](V1.md) study rows **S-01–S-02** can move toward **pass** with in-app review as primary.
- Watch and subtitle scenarios gain SPEC v1.1 rows (prompt 52) with named tests.
- No regression to v1 guarantees: two databases, proposal-not-silent-reorder, derived course/roadmap status ([`LEARNINGS.md`](LEARNINGS.md)).
