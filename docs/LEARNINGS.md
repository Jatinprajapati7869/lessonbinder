# AuraVault — engineering learnings

Short notes that are easy to miss when reading only the final code.

## Two databases

- **`vault.db`** holds courses, lessons, progress, roadmaps, and `anki_card_link` rows (`anki_note_id` + link status). `vault-core` never imports the `anki` crate.
- **`anki-profile/`** under app data holds the real Anki **collection** (notes, cards, scheduling). All writes go through **`anki-bridge`** → `rslib`.

Deleting a course must **break** vault links (`link_status = broken`, row kept) without deleting collection notes (migration `008`, `course_delete::breaks_links_not_notes`).

## Proposal ≠ silent reorder

Import establishes lesson rows and default path order; changing `sort_index` through an **order proposal** still requires an explicit **accept** (`accept_proposal`). `propose_order` + `reject_proposal` tests lock this in—re-import alone must not apply a new order (O-01/O-02). v1 still lacks automatic “offer proposal after import” (T42).

**v1.2 explicit reorder** is different: `reorder_lessons` writes `sort_index` immediately and does **not** create or accept an `order_proposal` row (`reorder_lessons_swaps_two_lessons_without_proposal`).

## Lesson ↔ Anki links

**Unlink** (`unlink_card`) deletes the vault `anki_card_link` row only; it does **not** delete the note in the Anki collection (`unlink_card_keeps_note_in_temp_profile`).

## Derived progress

Lesson `Progress.status` is stored; **course** and **roadmap step** status are **derived** via `rollup::derive_course_status` / roadmap helpers—no separate “course done” checkbox. Roadmap steps reflect linked course rollups; missing course → step **broken**.

## In-app review uses `rslib`, not a private scheduler

AuraVault does **not** reimplement FSRS or card queues. `anki-bridge::study` sets the current deck to the AuraVault root and calls Anki’s public scheduler (`get_queued_cards` / `answer_card`) on the **dedicated** `anki-profile/` collection. That keeps grading, intervals, and collection format aligned with Anki Desktop on the same profile. A bespoke scheduler would fork behavior and AGPL surface area without buying a second source of truth.

## Watch ranges ≠ `done`

v1.1 stores **unioned playback intervals** and derives `watched_percent` when `duration_ms` is known. `record_playback` and resume ticks update ranges and `in_progress`; they **do not** call `apply_mark_complete`. **`done` stays manual** (charter / SPEC W-05) so engagement metrics cannot silently complete a lesson the learner has not explicitly marked finished.

## Competitor REA notes vs AuraVault implementation

[`docs/AULAVAULT-BEHAVIOR.md`](AULAVAULT-BEHAVIOR.md) records **named commands and error strings** observed from a competitor AppImage binary (strings only—no decompiled source or extracted assets in this repo). The four **Build** gaps we chose to close are **our** designs and tests: risky folder import guard, cancellable folder scan, video-attached lesson resources (not duplicate lesson rows), and lesson thumbnails from the current playback frame. We **did not** build their license/activation server, Pro gating, or automatic Whisper weight download; v1.1 transcription stays **local path only** when the user configures a model.
