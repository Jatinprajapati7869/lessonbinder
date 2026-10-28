# AuraVault scope — v1.2

**Status:** Planning (prompt 111).
**Baseline:** v1.1 scope and shipped behavior are in [`SCOPE-v1.1.md`](SCOPE-v1.1.md), [`V1.1.md`](V1.1.md), and [`LEARNINGS.md`](LEARNINGS.md). This slice is the **study loop**: deliberate lesson order, a reviewer queue that does not stall on foreign cards, and lessons that own their Anki links end-to-end.

v1.2 does **not** add accounts, sync, mobile, chat, auto-completion, bundled model download, or drag-and-drop library UX.

---

## In scope (v1.2 only)

### Lesson order — explicit reorder

- The user can **move lessons up and down** within a course (explicit reorder controls).
- **Rescan** and **re-import** must **not** undo an accepted order: existing lessons keep their user order; only **new** files from disk append at the end (same spirit as v1 order proposals, but v1.2 adds direct reorder without requiring a proposal workflow for every nudge).

### In-app review — skip foreign due cards

- **`next_review_card`** (and the in-app review session built on it) **skips cards that are due but not in AuraVault decks**, instead of returning an **empty queue** while AuraVault-linked cards are still due elsewhere in the collection.

### Lesson ↔ Anki links

- A **lesson lists its Anki links** (vault `anki_card_link` rows tied to that lesson).
- **Unlink** removes the **vault row only** and **leaves the Anki note** in the collection (no delete-note side effect).

### Vault note → card (confirmed)

- A **vault lesson-note** may become an **Anki card draft** only **after the user confirms** (no silent auto-commit from note text).

### Optional audio clip on a card

- From a **local media lesson**, the user may attach an **optional audio clip** for a chosen **time range**; the clip file is stored in the **Anki media folder** (collection media), not as a substitute for vault progress or lesson files.

### Chapters via ffprobe

- When **`ffprobe`** is available **and** the media file exposes chapters, show them on the player; **click** seeks to the chapter start.
- When chapters are missing or ffprobe is unavailable, **unknown stays unknown** (no invented chapter list).

### Up next

- **Up next** on the course (or player) means the **next lesson in this course’s accepted order** (after the current lesson), not a global heuristic across courses.

### Review this course

- **Review this course** limits the in-app queue to **notes linked from lessons in that course** (scoped review, not whole-collection due sweep).

### Paste playlist

- A **playlist course** can be created from **pasted lines**: `title<TAB>url` per line.
- **No download** of video or audio; metadata and URLs only (same charter line as v1 playlist courses).

---

## Out of scope (v1.2)

| Area                                             | Notes                                |
| ------------------------------------------------ | ------------------------------------ |
| **Accounts** and auth                            | —                                    |
| **Cloud** sync or hosted library                 | —                                    |
| **Mobile** apps                                  | —                                    |
| **Chat** / AI tutor UI                           | —                                    |
| **Auto-`done`** from watch percent or thresholds | `done` remains manual                |
| **Model download** (Whisper or other weights)    | Local path only when user configures |
| **Drag-and-drop** library or course reorder UX   | Explicit up/down only in v1.2        |

Also still out of scope: a second SRS, custom FSRS in AuraVault, playlist rippers, and renaming the existing OCR note type.

---

## Relationship to prompts 112–130

Implementation checklist: [`PROGRESS.md`](PROGRESS.md) rows **112–130** (reorder, foreign-card skip, links/unlink, note→card confirm, audio clip, chapters, up next, review-this-course, paste playlist, regression names, README, tests, freeze). Row **111** records approval of this scope document.

## Success criteria (planning)

- Learners can **fix lesson order** without losing it on the next rescan.
- In-app **Study** and **Review this course** behave predictably when the Anki profile contains non-AuraVault due cards.
- Each lesson is a hub for **linked cards** and **confirmed** promotion from vault notes, without breaking the two-database rule ([`LEARNINGS.md`](LEARNINGS.md)).
