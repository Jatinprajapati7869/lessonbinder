# AuraVault implementation tickets

**Sources:** `auravault-SPEC.md`, `auravault-ARCHITECTURE.md`, `auravault-cursor-prompts.md` (prompts **11–42**).
**Format:** One ticket per prompt; vertical slices in dependency order. No accounts, cloud sync, or custom in-app reviewer (no embedded FSRS UI).

**Tracker:** Pre-repo; publish to GitHub issues after prompt 09 if desired. **Blocked by** lists prompt numbers.

---

## Dependency overview

```text
11 → 12 → 13 → 14 → 15 → 16 → 17 → 18 → 19 → 20 → 21 → 22 → 23 → 24 → 25 → 26
                                                              ↓
27 → 28 → 29 → 30 → 31 → 32 → 33 → 34 → 35 → 36 → 37 → 38 → 39 → 40 → 41 → 42
```

---

## T11 — Anki bridge opens a collection

**Prompt:** 11
**Blocked by:** None (after repo scaffold prompt 10).

**What:** `anki-bridge` opens the AuraVault profile via `CollectionBuilder`, creates `collection.anki2` + media paths, reopens without raw SQL in our crate.

**Done when:** `cargo test -p anki-bridge` passes; `docs/ANKI_PIN.md` records `anki-main` git rev; `docs/PROGRESS.md` marks 11.

**Test to add:** Temp-profile test: build collection → reopen → read schema version through `anki` APIs only (`anki_bridge::profile::open_and_schema_version` or equivalent).

**Not in this ticket:** Note types, decks, due counts, UI, `vault-core`, OCR.

---

## T12 — Note type and decks

**Prompt:** 12
**Blocked by:** T11.

**What:** `AuraVaultClip` notetype (fields: Course, Lesson, Front, Back, Source, Timestamp, Screenshot). Decks `AuraVault::<course title>` only; `add_clip_note` / `get_note`.

**Done when:** `cargo test -p anki-bridge` covers notetype, deck prefix guard, roundtrip note fields.

**Test to add:** `anki_bridge::clip::creates_note_in_prefixed_deck`; `anki_bridge::clip::rejects_deck_outside_prefix`.

**Not in this ticket:** OCR, screenshots on disk, vault links, UI.

---

## T13 — vault.db schema

**Prompt:** 13
**Blocked by:** T12 (sequencing per prompt pack; no functional dep on Anki).

**What:** `vault-core` SQLite migrations for domain entities; course + two lessons; unique source path; default `not_started`.

**Done when:** `cargo test -p vault-core` passes migrations + insert; `vault-core` has **no** `anki` dependency.

**Test to add:** `vault_core::db::migrate_fresh`; `vault_core::db::course_unique_source_path`; `vault_core::db::lesson_defaults_not_started`.

**Not in this ticket:** Import, progress FSM, Tauri, extractors.

---

## T14 — Progress state machine

**Prompt:** 14
**Blocked by:** T13.

**What:** Lesson transitions + illegal-edge errors; `derive_status` for course rollup (SPEC D-01–D-05).

**Done when:** Pure transition and rollup tests pass without filesystem or Anki.

**Test to add:** `vault_core::progress::legal_transitions`; `vault_core::progress::illegal_transitions_rejected`; `vault_core::rollup::course_status_cases`.

**Not in this ticket:** Resume persistence, import, UI, playlist.

---

## T15 — Folder import

**Prompt:** 15
**Blocked by:** T14.

**What:** Import directory → one course; extensions per SPEC; stem-matched subtitles; idempotent re-import (F-01–F-05).

**Done when:** Fixture-dir tests pass; no duplicate lessons on second import.

**Test to add:** `vault_core::import::eligible_extensions`; `vault_core::import::subtitle_by_stem`; `vault_core::import::idempotent_reimport`.

**Not in this ticket:** ffprobe metadata, order proposals, UI picker.

---

## T16 — Content index

**Prompt:** 16
**Blocked by:** T15.

**What:** Extractors: duration (ffprobe when present), SRT/VTT cues, PDF pages or unknown, md heading; store extractor id + error, never invented summary (C-01–C-05).

**Done when:** Lesson rows show `known` or `unknown(reason)`; ffprobe tests skip cleanly when missing.

**Test to add:** `vault_core::extract::srt_fixture`; `vault_core::extract::markdown_heading`; `vault_core::extract::unknown_without_ffprobe`.

**Not in this ticket:** Player, Whisper, LLM summaries.

---

## T17 — Resume position

**Prompt:** 17
**Blocked by:** T16.

**What:** Persist `resume_position_ms`; &gt;0 → `in_progress`; never auto-`done`; reject negative (V-01–V-03).

**Done when:** Round-trip resume tests pass in `vault-core` only.

**Test to add:** `vault_core::progress::resume_sets_in_progress`; `vault_core::progress::resume_round_trip`; `vault_core::progress::resume_does_not_mark_done`.

**Not in this ticket:** Player UI, `max_position_reached_ms` / watched-percent (T25 playlist watch; local progress bar in T38), manual complete (T18).

---

## T18 — Manual completion

**Prompt:** 18
**Blocked by:** T17.

**What:** `mark_complete` / `mark_incomplete`; broken cannot complete (T-01–T-03).

**Done when:** Command tests pass including broken rejection.

**Test to add:** `vault_core::progress::manual_done`; `vault_core::progress::incomplete_from_done_with_resume`; `vault_core::progress::broken_cannot_complete`.

**Not in this ticket:** Auto-done from watch %, UI buttons.

---

## T19 — Derived progress read model

**Prompt:** 19
**Blocked by:** T18.

**What:** `course_progress`, `library_summary`; refresh marks missing files `broken` and updates rollup (D-\* + file refresh).

**Done when:** Three-lesson fixture tests cover summary counts and broken refresh.

**Test to add:** `vault_core::read_model::library_summary_by_status`; `vault_core::read_model::refresh_marks_broken`.

**Not in this ticket:** Roadmap, remaining time, UI.

---

## T20 — Order proposals

**Prompt:** 20
**Blocked by:** T19.

**What:** Stored lesson order; `propose_order` / `accept` / `reject`; invalid proposal errors (O-01–O-04).

**Done when:** Proposal never applies without accept; reject leaves order unchanged.

**Test to add:** `vault_core::order::accept_applies`; `vault_core::order::reject_unchanged`; `vault_core::order::invalid_proposal_errors`.

**Not in this ticket:** UI Accept/Reject, deterministic “Suggest order” (T42), ML.

---

## T21 — Roadmap model

**Prompt:** 21
**Blocked by:** T20.

**What:** Roadmap steps → course; reorder/remove; step status = course progress; missing course → broken step; roadmap rollup (R-02, R-04).

**Done when:** Model tests pass without graph UI.

**Test to add:** `vault_core::roadmap::step_status_from_course`; `vault_core::roadmap::rollup_matches_course_rules`.

**Not in this ticket:** `roadmap_graph` edges UI, flowchart SVG.

---

## T22 — Remaining time

**Prompt:** 22
**Blocked by:** T21.

**What:** Sum non-done lesson durations; exclude unknowns into `unknown_lessons` count; done course → 0.

**Done when:** Mixed fixture test passes without defaulting unknown lengths.

**Test to add:** `vault_core::time::remaining_excludes_unknown`; `vault_core::time::done_course_zero`.

**Not in this ticket:** UI formatting, playlist-specific logic beyond shared durations.

---

## T23 — Flowchart graph data

**Prompt:** 23
**Blocked by:** T22.

**What:** `roadmap_graph(roadmap_id)` → nodes + directed edges; prerequisite edges; reject cycles (R-01, R-03).

**Done when:** Graph API tests return structured nodes/edges, not images.

**Test to add:** `vault_core::roadmap::graph_nodes_and_edges`; `vault_core::roadmap::rejects_prerequisite_cycle`.

**Not in this ticket:** SVG/canvas rendering (T39), export PNG.

---

## T24 — Playlist course import

**Prompt:** 24
**Blocked by:** T23.

**What:** Course from local JSON fixture playlist; lessons per item; refresh adds/broken rules (P-01, SPEC playlist).

**Done when:** Fixture JSON import tests pass; no network.

**Test to add:** `vault_core::playlist::import_from_fixture`; `vault_core::playlist::refresh_marks_missing_broken`.

**Not in this ticket:** YouTube fetch, embedded player, UI JSON picker (T37).

---

## T25 — Playlist tracking

**Prompt:** 25
**Blocked by:** T24.

**What:** `set_watched` on playlist lessons; resume/status parity without local file (P-02).

**Done when:** Two-item fixture: one done → course `in_progress`.

**Test to add:** `vault_core::playlist::watched_updates_progress`; `vault_core::playlist::rollup_in_progress_partial_done`.

**Not in this ticket:** Local video player, OCR.

---

## T26 — Unified library list

**Prompt:** 26
**Blocked by:** T25.

**What:** `list_courses` merges folder + playlist courses; filter by derived status; remaining time fields.

**Done when:** Single DB fixture with both source kinds lists correctly.

**Test to add:** `vault_core::library::list_mixed_sources`; `vault_core::library::filter_by_status`.

**Not in this ticket:** Tauri commands, React Library screen.

---

## T27 — Vault ↔ Anki link record

**Prompt:** 27
**Blocked by:** T26.

**What:** `AnkiCardLink` rows in `vault.db`; uniqueness on `(lesson_id, timestamp_ms, front_hash)`; no Anki calls.

**Done when:** Roundtrip + duplicate-key tests in vault layer.

**Test to add:** `vault_core::anki_link::insert_roundtrip`; `vault_core::anki_link::duplicate_selection_rejected`.

**Not in this ticket:** Note creation, OCR, Tesseract.

---

## T28 — OCR port

**Prompt:** 28
**Blocked by:** T27.

**What:** `OcrEngine` trait; fake engine for tests; Tesseract behind feature; `ocr_unavailable` when missing (Decision 1).

**Done when:** Fake + unavailable path tested; no network.

**Test to add:** `vault_core::ocr::fake_returns_text`; `vault_core::ocr::tesseract_unavailable_error`.

**Not in this ticket:** Card drafts, Anki media copy, UI rectangle.

---

## T29 — Selection to card draft

**Prompt:** 29
**Blocked by:** T28.

**What:** Build `CardDraft` from lesson + timestamp + image bytes; OCR default front; reject empty front (A-prep).

**Done when:** Draft builder tests pass without Anki write.

**Test to add:** `vault_core::card::draft_from_ocr`; `vault_core::card::rejects_empty_front`.

**Not in this ticket:** `add_clip_note`, screenshot file on disk, UI editor.

---

## T30 — Write the Anki note

**Prompt:** 30
**Blocked by:** T29, T12, T27.

**What:** Commit draft → hash-named PNG in Anki media → `add_clip_note` → `AnkiCardLink`; idempotent second commit (A-01–A-03, S-30).

**Done when:** Temp profile + temp vault integration test passes.

**Test to add:** `integration::commit_card_creates_note_and_link`; `integration::duplicate_commit_same_note_id`.

**Not in this ticket:** Due counts, launch Anki, capture UI.

---

## T31 — Due counts

**Prompt:** 31
**Blocked by:** T30.

**What:** `due_summary()` for `AuraVault::` decks only (K-01, K-02).

**Done when:** Fresh collection zeros; after add note, assert real scheduler counts (document behavior in test name).

**Test to add:** `anki_bridge::study::due_summary_empty`; `anki_bridge::study::due_summary_after_new_note`.

**Not in this ticket:** Launching Anki binary, in-app review UI.

---

## T32 — Launch Anki command builder

**Prompt:** 32
**Blocked by:** T31.

**What:** Build argv for `anki` on AuraVault profile; typed error if binary missing; no GUI in CI (ARCHITECTURE review boundary).

**Done when:** Argument-vector unit tests pass; `docs/STUDY.md` documents v1 study in Anki Desktop.

**Test to add:** `tauri::study::anki_argv_for_profile`; `tauri::study::missing_binary_error`.

**Not in this ticket:** Spawning Anki in CI, custom reviewer, due count logic.

---

## T33 — Transcription port (disabled)

**Prompt:** 33
**Blocked by:** T32.

**What:** `TranscriptionEngine` trait; default `transcription_disabled`; no Whisper download; `docs/TRANSCRIPTION.md` stub.

**Done when:** Disabled engine does not touch lesson order/progress.

**Test to add:** `vault_core::transcription::disabled_is_noop`.

**Not in this ticket:** Whisper weights, subtitle generation, UI.

---

## T34 — Architecture guard

**Prompt:** 34
**Blocked by:** T33.

**What:** CI/test fails if `vault-core` depends on `anki`; `cargo test --workspace` documented in README; PROGRESS notes 34.

**Done when:** Guard passes; workspace tests green for our crates.

**Test to add:** `scripts::vault_core_must_not_depend_on_anki` (or `tests/architecture.rs`).

**Not in this ticket:** New features, UI.

---

## T35 — App shell

**Prompt:** 35
**Blocked by:** T34.

**What:** Tauri 2 + React TS shell: nav Library/Roadmap; top bar due counts + Study (disabled with error if no binary); open `vault.db` in app data.

**Done when:** `cargo test --workspace` + frontend unit test renders shell; no lorem/sample courses.

**Test to add:** `ui/shell.test.tsx` renders nav and due placeholder; `cargo test` for db open command.

**Not in this ticket:** DESIGN tokens full pass (T36), import flows, player.

---

## T36 — Visual foundations

**Prompt:** 36
**Blocked by:** T35.

**What:** Implement `auravault-DESIGN.md` tokens on shell only; strip Vite/Tauri starter chrome.

**Done when:** Running app uses IBM Plex + stone palette + accent rules; verification noted in PROGRESS.

**Test to add:** `ui/theme.test.ts` CSS variables match DESIGN tokens (or snapshot token module).

**Not in this ticket:** Library/Course screens, new features.

---

## T37 — Library screen

**Prompt:** 37
**Blocked by:** T36.

**What:** Import folder + playlist JSON; list courses with status, remaining time, unknown count; open course (DESIGN Library empty state).

**Done when:** Component tests with fake invoke layer; no bundled demo data.

**Test to add:** `ui/Library.test.tsx` lists courses from mock commands; import triggers command.

**Not in this ticket:** Course player, roadmap graph, OCR.

---

## T38 — Course and player

**Prompt:** 38
**Blocked by:** T37.

**What:** Lesson list, complete/incomplete, local video player with resume save; proposal Accept/Reject; playlist rows show URL + watched toggle.

**Done when:** Fake-layer tests for status + resume; DESIGN Course/Player patterns.

**Test to add:** `ui/Course.test.tsx` mark complete; `ui/Player.test.tsx` resume invoke interval.

**Not in this ticket:** Roadmap SVG, OCR dialog, Suggest order button (T42).

---

## T39 — Roadmap screen

**Prompt:** 39
**Blocked by:** T38.

**What:** CRUD roadmap/steps; render `roadmap_graph` as SVG/canvas; derived status + remaining time on nodes.

**Done when:** Graph test with two nodes, one edge; empty state single action per DESIGN.

**Test to add:** `ui/RoadmapGraph.test.tsx` renders nodes/edges from fixture graph.

**Not in this ticket:** Card capture, order suggestion stub.

---

## T40 — OCR capture

**Prompt:** 40
**Blocked by:** T39.

**What:** Player “Capture card”: frame, rectangle, local OCR, edit front/back, commit T30 path, duplicate handling; tesseract missing message.

**Done when:** Fake OCR tests for editor + duplicate; no network upload.

**Test to add:** `ui/CaptureCard.test.tsx` duplicate shows existing; `ui/CaptureCard.test.tsx` ocr_unavailable message.

**Not in this ticket:** Keyboard polish (T43), broken-file states (T41).

---

## T41 — Broken and offline states

**Prompt:** 41
**Blocked by:** T40.

**What:** Missing file → broken row + player message; missing Anki binary → Study explains, counts still work; DB open failure → shell error, no fake library.

**Done when:** Three state tests pass; one message + one recovery action each.

**Test to add:** `ui/BrokenLesson.test.tsx`; `ui/MissingAnki.test.tsx`; `ui/DbOpenFailure.test.tsx`.

**Not in this ticket:** Toast libraries, accounts, sync.

---

## T42 — Order suggestion stub

**Prompt:** 42
**Blocked by:** T41.

**What:** “Suggest order” → deterministic proposal (natural filename, then subtitle cue count); uses T20 API; Accept required; `docs/ORDER.md` for future local model.

**Done when:** Fixture unsorted names produce proposal; live order unchanged until Accept.

**Test to add:** `vault_core::order::deterministic_suggest_natural_sort`; `ui/Course.test.tsx` suggest creates proposal not live order.

**Not in this ticket:** ML model, cloud APIs, auto-accept.

---

## Explicitly excluded (no tickets)

- User accounts, authentication, payments
- Cloud sync of library or progress
- Custom in-app SRS reviewer / FSRS implementation
- AI chat tutor, Whisper transcription pipeline (port only in T33)
- Marketing site patterns (hero, testimonials) per DESIGN

---

## `/to-tickets` — granularity check

Presented for approval before issue export:

| Question       | Default answer in this file                                                               |
| -------------- | ----------------------------------------------------------------------------------------- |
| Granularity    | One prompt = one ticket (32 slices); each ends with testable evidence                     |
| Blocking edges | Linear chain 11→42 matches prompt pack “Continue: after N”                                |
| Merge/split    | Do not merge 11–12 (Anki vs vault DB are separate seams); UI split 35–42 stays per-screen |

To publish as local issues after repo exists: copy each **T11–T42** section to `docs/scratch/v1/issues/NN-*.md` or create GitHub issues with native dependencies.
