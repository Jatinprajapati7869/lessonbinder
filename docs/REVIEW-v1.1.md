# Code review — AuraVault v1.1

**Date:** 2026-10-28
**Scope:** v1.1 implementation on the current working tree (`/home/xingzi/Documents/auravault`). There is **no git baseline** yet (`No commits yet on master`); this review treats the staged v1.1 work as a whole against [`docs/SPEC-v1.1.md`](SPEC-v1.1.md) and [`docs/SCOPE-v1.1.md`](SCOPE-v1.1.md).
**Instruction:** Findings only — **no fixes** in this pass.

**Validation run:** `cargo test -p anki-bridge -p vault-core -p auravault-tauri` — **143 passed** (this turn).

---

## Executive summary (priority risks)

| Risk                                           | Verdict                                                                                                    | Severity           |
| ---------------------------------------------- | ---------------------------------------------------------------------------------------------------------- | ------------------ |
| Scheduler answers affecting the **wrong deck** | **Mostly mitigated** for grades; **gap** on “empty queue” when a non–AuraVault card sits at scheduler head | Medium             |
| **Media path escape**                          | **Mitigated** in `read_media` + test; UI trusts note field filename (expected)                             | Low–medium         |
| **Transcription network**                      | **No HTTP in app code**; **T-03 evidence test missing**                                                    | Low (doc gap)      |
| **Silent reorder**                             | **Course order:** proposal + post-transcription guard; **roadmap** reorder is explicit UI                  | Low                |
| **Auto-`done` from watch %**                   | **Implementation looks correct**; **W-05 named test missing**                                              | Low (coverage gap) |
| **Backup deleting live profile**               | **Backup into live profile refused**; restore is destructive-by-design with aside copy                     | Low                |

---

## Priority deep dive

### 1. Scheduler / wrong deck (`R-02`, `R-03`, `R-04`)

**What works**

- `answer_card` re-fetches the queue head, requires `card_id` match (`CardNotAtQueueHead`), and rejects cards outside `AuraVault::` decks (`CardNotInAuraVaultDecks`). Covered by `crates/anki-bridge/tests/study.rs` (`answer_card_rejects_wrong_queue_head`, `review_session_rejects_card_outside_auravault_prefix`).
- `ReviewSession::answer` calls `ensure_card_in_auravault_decks` before grading.
- UI sends the `card_id` from `review_next_card` and the server re-validates queue head on answer (`ui/src/screens/ReviewScreen.tsx`, `src-tauri/src/study_commands.rs`).

**Gap (spec R-02) — fixed**

- `next_review_card` scans the scoped queue via `get_queued_cards` and **skips** due cards outside AuraVault decks (`next_auravault_review_card` in `crates/anki-bridge/src/study.rs`); foreign dues are not answered or buried.
- **Evidence:** `anki_bridge::study::next_card_from_auravault_decks` (`crates/anki-bridge/tests/study.rs`) — foreign **Other** deck new card plus AuraVault clip; `next_review_card` returns the AuraVault front and the foreign card stays on the **New** queue.

**Judgement:** Wrong-deck **grading** is well guarded; R-02 **empty queue while AuraVault dues exist** is covered by the named test above.

### 2. Media path escape

**Implementation:** `crates/anki-bridge/src/media.rs` — rejects empty, `..`, non-normal path components; `join` + `canonicalize` + `starts_with(media_root)` before read.

**Tests:** `read_media_rejects_path_escape` (`../outside.png`); commit round-trip in `read_media_returns_png_written_by_commit`.

**Residual notes**

- No test for **symlink** under `collection.media` pointing outside the media root (canonicalize + `starts_with` likely blocks; worth an explicit test).
- `review_read_media` accepts any `filename` string from the client; path rules still apply. Screenshot name comes from Anki note field (`study_commands.rs`); a malicious or hand-edited collection could store odd names — validation is the right layer.

### 3. Transcription network (`T-03`, `T-04`)

**Implementation:** `vault-core` uses `std::process::Command` to run local `whisper-cli` with a **user-supplied model path** (`transcription.rs`, `whisper_cpp.rs`). No `reqwest`/HTTP in transcription code.

**Gaps**

- SPEC **`vault_core::transcription::no_network_weight_download`** — **not present** in the repo (no test name match). Contract is implicit only.
- SPEC **`failure_leaves_lesson_unchanged`** — **not present**. Happy-path guard exists in `transcribe_with_cues` (order/progress equality after success). No test for **failed** `run_whisper_cli` or for **order check failure after** `replace_lesson_transcription_cues` (would leave cues written while returning error — edge case for T-04 “no partial segments” intent).

### 4. Silent reorder

**Course learning order**

- Changes go through `accept_proposal` → `apply_learning_order` (`order.rs`). Import/rescan updates `sort_index` from scan order (deterministic), not from ML proposals.
- Transcription: `transcribe_with_cues` compares `learning_order` and `progress` before/after cue replace; mismatch → `TranscriptionError::Db(...)`. Tested in `fake_engine_writes_three_transcription_cues_without_changing_order_or_progress`.

**Roadmap**

- `RoadmapScreen` reorder is **explicit** user action (`reorderRoadmapSteps`) — not the “proposal ≠ silent reorder” lesson-order rule; document as intentional if roadmap is in v1.1 scope.

**No finding** of silent lesson reorder on transcription success path in reviewed code.

### 5. Auto-`done` from watch percent (`W-05`)

**Implementation:** `record_playback` only merges watched ranges (`progress_persist.rs`); doc comment states it does not change `done`. `record_playback_position` uses `apply_playback_starts` but not `apply_mark_complete`.

**Tests:** `record_playback_does_not_mark_done` (lesson already `Done` stays `Done` after full-span playback). **Missing:** SPEC test `union_full_watch_does_not_auto_done` — e.g. `not_started`/`in_progress` lesson, union to 100% of `duration_ms`, assert status unchanged.

**UI:** No `mark_complete` tied to `watched_percent` in `ui/` (display only in continue row).

### 6. Backup vs live profile

**Backup:** `backup_to` refuses when `destination_parent` normalizes to the live `anki-profile` path or lies inside it (`destination_targets_live_profile`). Test: `backup_to_refuses_live_anki_profile_directory`.

**Restore:** Copies live `vault.db` + profile to `auravault-aside-{ts}` before overwriting live data (`restore_from`). Test: `restore_from_replaces_live_data_and_preserves_lesson_and_note_id`.

**Residual:** Restore still `remove_dir_all` on live profile before copying backup profile — failure **after** removal leaves live profile empty until retry, though aside should hold the pre-restore copy. Not the same as “backup deletes live,” but worth a hardening note for a follow-up (atomic swap).

---

## Standards

_Sources:_ `AGENTS.md` (SPEC before behavior, anki-bridge-only Anki access, DESIGN for UI), `docs/SPEC-v1.1.md` evidence column, Fowler smell baseline (judgement calls).

| Finding                                 | Type                           | Detail                                                                                                                                                                                                            |
| --------------------------------------- | ------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Missing SPEC-named tests                | Hard vs spec                   | Several v1.1 evidence functions are named in SPEC but absent as tests (`next_card_from_auravault_decks`, `union_full_watch_does_not_auto_done`, `no_network_weight_download`, `failure_leaves_lesson_unchanged`). |
| `transcribe_with_cues` write-then-check | Judgement (Shotgun / ordering) | Cues are committed before order/progress checks; unlikely to fail in practice, but T-04 “no partial writes” is not airtight.                                                                                      |
| `ReviewScreen.tsx` size/complexity      | Judgement                      | react-doctor (prompt 96) flagged high complexity; not a standards violation but maintainability debt.                                                                                                             |
| Duplicated profile open per command     | Judgement (Duplicated Code)    | Each `study_commands` handler opens profile independently — acceptable for v1.1, could consolidate later.                                                                                                         |

**Standards axis:** **4 findings** (1 spec-evidence gap cluster, 3 judgement). **Worst:** missing automated evidence for security- and scheduler-critical SPEC rows.

---

## Spec

| ID                        | Status                   | Notes                                                                   |
| ------------------------- | ------------------------ | ----------------------------------------------------------------------- |
| R-02                      | **Partial**              | See scheduler head skip → `None` when non-AuraVault card is queue head. |
| R-03                      | **Met**                  | Answers persist via `col.answer_card`; tests present.                   |
| R-04                      | **Met** with R-02 caveat | Empty when truly no dues; false empty possible per R-02.                |
| W-05                      | **Likely met**           | Behavior consistent; **evidence test missing**.                         |
| T-03                      | **Likely met**           | No in-app network; **evidence test missing**.                           |
| T-04                      | **Partial**              | Success-path guards tested; failure / partial-write paths not.          |
| Backup (scope / PROGRESS) | **Met**                  | Refuse backup destination = live profile; restore aside pattern tested. |

**Scope creep (acceptable):** Local whisper transcription ships in v1.1 though early SCOPE text mentioned “not in v1.1”; SPEC `T-*` rows now govern.

**Spec axis:** **2 partial** (R-02, T-04), **2 likely-met-without-tests** (W-05, T-03). **Worst:** R-02 false empty queue with dues still in AuraVault decks.

---

## Thermo-nuclear code quality

1. **Code-judo candidate — `next_review_card`:** Loop or skip until an AuraVault card or true queue end, **or** return a dedicated error instead of `None`, would align implementation with R-02 in one place and remove a class of “works in clean profile only” behavior.
2. **Transcription transaction:** Run order/progress checks inside the same DB transaction as cue replace, or write cues only after checks on a dry-run — simplifies T-04 story.
3. **`ReviewScreen`:** Split load/answer/media into hooks or subcomponents to address complexity without behavior change.
4. **File size:** No file reviewed here crossed 1k lines due to v1.1 alone; `db.rs` remains a concentration risk for future features.

**Approval bar:** **Not approved** for merge on scheduler queue semantics (R-02) until tested or fixed; other priority areas are acceptable with test backfill.

---

## Compound engineering verdict

| Item                         | Result                                                                                                    |
| ---------------------------- | --------------------------------------------------------------------------------------------------------- |
| **Intent**                   | Ship v1.1 gaps: in-app review, watch union, rescan, notes, search, transcription adapter, backup, polish. |
| **Correctness**              | Core paths sound; **scheduler queue head** and **SPEC test names** are the main holes.                    |
| **Security**                 | Media read path hardened; review IPC relies on bridge validation.                                         |
| **Recommended before ship**  | (1) R-02 test + behavior, (2) W-05 / T-03 / T-04 tests per SPEC names, (3) optional symlink media test.   |
| **Out of scope this review** | Full UI/e2e matrix, AppImage smoke (documented absent in `PACKAGE.md`).                                   |

---

## One-line summary

**Standards:** 4 findings — worst: SPEC evidence tests not wired for scheduler, watch, and transcription network contract. **Spec:** 2 partial (R-02, T-04) — worst: `next_review_card` can return empty while AuraVault dues exist. **Thermo-nuclear:** fix queue draining in `next_review_card` and tighten transcription DB atomicity before calling v1.1 done.
