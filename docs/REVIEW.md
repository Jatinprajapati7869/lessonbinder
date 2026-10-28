# AuraVault review (repo snapshot)

**Repo:** `/home/xingzi/Documents/auravault` (not `anki-main`).
**Baseline:** No meaningful `git` history on `master` yet; findings are from static review of the working tree vs `docs/SPEC.md`, `docs/ARCHITECTURE.md`, and `docs/DESIGN_AUDIT.md`.
**Scope requested:** bugs, SPEC mismatches, Anki collection safety, `vault-core` ↔ `anki`, progress rollups, OCR network calls.
**No fixes applied in this review.**

**Severity:** **P0** = data loss / wrong collection / charter violation · **P1** = SPEC acceptance gap or serious bug · **P2** = partial implementation / test gap · **P3** = maintainability / polish

---

## Standards (coding & architecture)

| Sev | Finding                                                                                                                                                      | Location                                                                  |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------- |
| P3  | `anki-bridge` pins `anki` via absolute path ` /home/xingzi/Documents/anki-main/rslib` — breaks CI/other machines unless path exists.                         | `crates/anki-bridge/Cargo.toml`                                           |
| P3  | Command-context split into 8 small modules (`*CommandsContext.ts` + `*commandsContext.ts` + `use*.ts`) — correct for react-doctor, adds navigation overhead. | `ui/src/capture/`, `ui/src/course/`, `ui/src/library/`, `ui/src/roadmap/` |
| P3  | `CaptureCardPanel` and `RoadmapScreen` flagged as high complexity / large components (react-doctor 88/100).                                                  | `ui/src/screens/CaptureCardPanel.tsx`, `ui/src/screens/RoadmapScreen.tsx` |
| P3  | UI loads IBM Plex from Google Fonts at runtime (network). Acceptable per `docs/DESIGN.md` but conflicts with strict offline-only OCR story.                  | `ui/src/theme/tokens.css`                                                 |

**Passes (standards / architecture):**

- `vault-core` does **not** declare an `anki` dependency; guarded by `crates/vault-core/tests/architecture.rs` and `scripts/vault-core-must-not-depend-on-anki.sh`.
- `anki-bridge` is the only crate linking `anki` / `rslib` (`crates/anki-bridge/Cargo.toml`, `crates/anki-bridge/src/lib.rs`).
- Rollup logic is centralized in `crates/vault-core/src/rollup.rs` with tests in `crates/vault-core/tests/rollup.rs` (matches SPEC “single function” intent).

---

## Spec (requirements vs implementation)

### Study & Anki profile (SPEC S-01–S-03, user story 16)

| Sev    | Finding                                                                                                                                                                                                                 | Spec / story                                  | Location                                                                                                                                                                                                      |
| ------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **P0** | **Study launches external Anki Desktop** (`anki` on PATH, `-b`/`-p` profile args), not in-process `rslib` review. SPEC S-02 explicitly expects the opposite failure mode (“Requires external `anki` binary” = failure). | `docs/SPEC.md` S-01–S-03; charter Decision 16 | `src-tauri/src/commands.rs` (`launch_anki_profile`), `crates/tauri/src/study.rs`, `ui/src/shell/AppShell.tsx`, `ui/src/api/tauri.ts` (`launchAnkiProfile`), `docs/ARCHITECTURE.md` (documents Desktop launch) |
| **P1** | No `start_study_session()` / in-process review API or tests named in SPEC (`uses_auravault_profile_only`, `in_process_review_entry`, etc.).                                                                             | `docs/SPEC.md` S-01–S-03                      | `crates/anki-bridge/src/study.rs` (due counts only); `crates/anki-bridge/tests/study.rs`                                                                                                                      |

### Anki card links & course delete (SPEC A-03, user story 17)

| Sev    | Finding                                                                                                                                                                                                                                                            | Spec                                                                                          | Location                                                                                                                                                                           |
| ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **P1** | **Course delete does not set `link_status = broken`.** `delete_course` only `DELETE FROM course`. Lessons cascade-delete; `anki_card_link` rows **CASCADE** with lesson and are **removed**, not marked broken. SPEC wants links broken, notes kept in collection. | `docs/SPEC.md` A-03; evidence `vault_core::course_delete::breaks_links_not_notes` **missing** | `crates/vault-core/src/db.rs` (`delete_course`), `crates/vault-core/src/roadmap.rs` (`delete_course`), `crates/vault-core/migrations/007_anki_card_link.sql` (`ON DELETE CASCADE`) |
| **P2** | No `course_delete` module or test file referenced by SPEC.                                                                                                                                                                                                         | `docs/SPEC.md` A-03                                                                           | `crates/vault-core/tests/` (no `course_delete.rs`)                                                                                                                                 |
| **P2** | No UI/Tauri surface to delete a course; domain API exists for roadmap tests only.                                                                                                                                                                                  | User story 17 (workflow)                                                                      | `crates/vault-core/src/roadmap.rs`; no `src-tauri` / `ui` delete command                                                                                                           |

### OCR → Anki (SPEC A-01–A-02)

| Sev    | Finding                                                                                                                     | Spec                                                           | Location                                                                      |
| ------ | --------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| **P2** | Notetype name is `AuraVaultClip`, not `AuraVault OCR` in SPEC table. Likely wording drift; fields/deck prefix match intent. | `docs/SPEC.md` A-01                                            | `crates/anki-bridge/src/clip.rs` (`AURAVAULT_CLIP_NOTETYPE`)                  |
| **P2** | SPEC evidence tests under `anki_bridge::ocr::*` **not present**; flow is `commit_card_draft` + `vault_core::ocr`.           | `docs/SPEC.md` A-01–A-02                                       | `crates/anki-bridge/src/commit.rs`, `crates/anki-bridge/tests/integration.rs` |
| **P3** | OCR path is local Tesseract CLI only in production builds with `tesseract` feature; no HTTP OCR in code reviewed.           | `docs/SPEC.md` Implementation Decisions (“forbidden HTTP OCR”) | `crates/vault-core/src/ocr.rs`, `src-tauri/src/card_commands.rs`              |
| **P3** | No repo script/CI guard for “forbidden HTTP OCR hosts” (SPEC mentions lint guard).                                          | `docs/SPEC.md` line ~156                                       | `scripts/` (no OCR network lint found)                                        |

### Order proposals (SPEC O-01, T42)

| Sev    | Finding                                                                                                                                                                        | Spec                                                      | Location                                                                                                   |
| ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| **P1** | **T42 not shipped:** no deterministic “suggest order” (natural sort + subtitle cue count), no `docs/ORDER.md`, no UI “Suggest order”. `propose_order` / accept / reject exist. | `docs/TICKETS.md` T42; `docs/PROGRESS.md` (42 open); O-01 | `crates/vault-core/src/order.rs`; `ui/src/screens/CourseScreen.tsx` (panel only when proposal already set) |
| **P2** | Import sorts lessons by **lexicographic** `relative_path`, not natural sort; v1 suggest was meant to differ for `10` vs `2` style names.                                       | O-01 / T42 design                                         | `crates/vault-core/src/import.rs` (~line 90)                                                               |
| **P2** | No automatic order **proposal** offered after import (O-01 “system offers”); user must get proposal via other means.                                                           | `docs/SPEC.md` O-01                                       | `crates/vault-core/src/import.rs` (no `propose_order` on import)                                           |

### Progress & rollups (SPEC D-01–D-05, V-04)

| Sev    | Finding                                                                                                                                                                   | Spec                     | Location                                                                                                   |
| ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------ | ---------------------------------------------------------------------------------------------------------- |
| **P3** | **Rollup implementation matches SPEC snippet** (empty → not_started, all done → done, broken-only → broken, else in_progress).                                            | `docs/SPEC.md` D-01–D-05 | `crates/vault-core/src/rollup.rs`, `crates/vault-core/tests/rollup.rs`                                     |
| **P3** | `max_position_reached_ms` updated on resume save in domain; player persists resume on interval. Watched % in UI uses stored max, not live scrub position until next save. | V-04                     | `crates/vault-core/src/progress_persist.rs`; `ui/src/screens/PlayerScreen.tsx` (`RESUME_SAVE_INTERVAL_MS`) |

### Other SPEC evidence gaps (not necessarily bugs)

| Sev | Finding                                                                                                        | Location                                                            |
| --- | -------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| P2  | Many SPEC `Evidence` test names are not implemented as named tests (playlist P-01–P-03 ignored as documented). | `docs/SPEC.md`; `crates/vault-core/tests/playlist.rs` (`#[ignore]`) |
| P2  | `docs/DESIGN_AUDIT.md` items still open (player chrome, toast, Lucide, list keyboard partial, etc.).           | `docs/DESIGN_AUDIT.md`                                              |

---

## Anki collection safety

| Sev    | Finding                                                                                                                                                                                                                                                                              | Location                                                                                                                     |
| ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| **P0** | **Concurrent collection access:** `study_bar_state` opens the AuraVault profile via `open_profile` while **Study in Anki** can spawn a second process (`launch_anki_profile`) on the **same** profile directory. Risk: SQLite lock errors, failed commits, or collection corruption. | `src-tauri/src/commands.rs` (`study_bar_state`, `launch_anki_profile`), `crates/anki-bridge/src/profile.rs` (`open_profile`) |
| **P1** | `commit_capture_card` opens collection, writes note + media, no documented single-writer lock or “Anki is open” guard.                                                                                                                                                               | `src-tauri/src/card_commands.rs` (`commit_capture_card`)                                                                     |
| **P2** | Duplicate capture: idempotent by `(lesson_id, timestamp_ms, front_hash)` before write — good; race if two commits in parallel not addressed.                                                                                                                                         | `crates/anki-bridge/src/commit.rs`, `src-tauri/src/card_commands.rs`                                                         |
| **P3** | Clip notes scoped to `AuraVault::<course>` decks; validation prevents nested `::` in deck names — reduces cross-deck pollution.                                                                                                                                                      | `crates/anki-bridge/src/clip.rs` (`validate_auravault_deck_name`, `add_clip_note`)                                           |
| **P3** | Due counts limited to `AuraVault` deck subtree — does not read default/user profile.                                                                                                                                                                                                 | `crates/anki-bridge/src/study.rs` (`due_summary`, `auravault_root_deck`)                                                     |

---

## vault-core importing anki

| Sev | Finding                                                                                                                                                           | Location                                                             |
| --- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| —   | **No violation:** `vault-core` stores `anki_note_id` and link metadata only; no `anki` crate import.                                                              | `crates/vault-core/Cargo.toml`, `crates/vault-core/src/anki_link.rs` |
| P3  | **Reverse dependency:** `anki-bridge` depends on `vault-core` for DB links during commit (`commit_card_draft`). Intended seam but couples bridge to vault schema. | `crates/anki-bridge/Cargo.toml`, `crates/anki-bridge/src/commit.rs`  |

---

## OCR network calls

| Sev | Finding                                                                                        | Location                                                                |
| --- | ---------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| —   | **No HTTP OCR** in `vault-core` OCR port; Tesseract uses local `Command` and temp files only.  | `crates/vault-core/src/ocr.rs`                                          |
| P3  | PNG bytes stay on device through Tauri commands; UI copy states images are not sent elsewhere. | `src-tauri/src/card_commands.rs`, `ui/src/screens/CaptureCardPanel.tsx` |

---

## Thermo-nuclear (maintainability / structure)

| Sev | Finding                                                                                                                                                              | Location                                                                  |
| --- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| P2  | **Shotgun surgery for command ports:** four parallel context/hook triplets instead of one shared pattern — tradeoff for lint rules vs file count.                    | `ui/src/*/use*Commands.ts`, `ui/src/*/*CommandsContext.tsx`               |
| P2  | **Capture + roadmap screens** are the main complexity debt; splitting would reduce risk before more features land.                                                   | `ui/src/screens/CaptureCardPanel.tsx`, `ui/src/screens/RoadmapScreen.tsx` |
| P3  | **Study flow split across three docs** (SPEC in-process vs ARCHITECTURE Desktop vs implemented launch) — readers will implement the wrong thing without reconciling. | `docs/SPEC.md`, `docs/ARCHITECTURE.md`, `src-tauri/src/commands.rs`       |

---

## Summary

| Axis            | Worst issue                                                                                    |
| --------------- | ---------------------------------------------------------------------------------------------- |
| **Standards**   | Portable build blocked by absolute `anki` path in `anki-bridge` (P3).                          |
| **Spec**        | Study uses external Anki Desktop while SPEC S-01–S-02 describe in-process `rslib` review (P0). |
| **Safety**      | Same profile opened from app and external Anki concurrently (P0).                              |
| **Spec (data)** | Course delete CASCADE-deletes `anki_card_link` rows instead of marking `broken` per A-03 (P1). |

**Counts (approx.):** P0 ×2 · P1 ×5 · P2 ×10 · P3 ×12

---

## Suggested follow-ups (not done here)

1. Reconcile SPEC S-* with ARCHITECTURE or implement `start_study_session` in `anki-bridge` and wire UI.
2. On course delete: break links (`UPDATE anki_card_link SET link_status = 'broken'`) before cascade, or change FK strategy; add `breaks_links_not_notes` test.
3. Ship T42 suggest order + `docs/ORDER.md`.
4. Serialize or detect exclusive access to `anki-profile` before `open_profile` / launch.
5. Replace absolute `anki` path with workspace/git submodule policy documented in `docs/ANKI_PIN.md`.
