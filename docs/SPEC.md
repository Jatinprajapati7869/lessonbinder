# Lessonbinder SPEC (v1)

**Sources:** `lessonbinder-charter.md`, `lessonbinder-domain.md`
**Status:** Pre-repo; scenarios target `vault-core` and `anki-bridge` crates (see Architecture prompt). No feature below is outside the charter.

---

## Problem Statement

Learners keep course material in local folders (videos, PDFs, subtitles, notes) but lack a single offline app that (1) indexes real files honestly, (2) tracks per-lesson watch progress and manual completion without lying about “course done”, (3) shows learning paths as a course graph, and (4) connects highlights to **Anki** scheduling without maintaining a second SRS or corrupting their everyday Anki profile.

## Solution

Lessonbinder imports a folder as a **Course** of **Lessons**, extracts cheap metadata, persists **Progress** (resume, watch metrics, manual `done`), derives **Course** / **RoadmapStep** status, exposes **roadmap graph** data (nodes + edges), captures **OCR** selections into the Lessonbinder **Anki** profile via `rslib`, and surfaces **due counts** plus an in-app **study** path on the shipped Anki stack (Decision 16—not a separate Anki Desktop install).

## User Stories

1. As a learner, I want to import a local course folder, so that every video/audio/PDF/note file becomes a trackable lesson without duplicate rows on re-import.
2. As a learner, I want subtitle files paired to videos by filename stem, so that cues appear on the lesson without becoming separate lessons.
3. As a learner, I want duration and subtitle cue counts when extractors succeed, so that I see facts—not invented summaries.
4. As a learner, I want unknown metadata labeled unknown, so that the app never fabricates titles or durations.
5. As a learner, I want a proposed lesson order after import (default: relative path), so that I can accept or reject before it becomes my saved sequence.
6. As a learner, I want to reorder lessons only through explicit actions, so that the app never silently changes my learning order.
7. As a learner, I want to mark a lesson complete or incomplete manually, so that `done` reflects my intent (watch % does not auto-complete in v1).
8. As a learner, I want resume position saved per lesson, so that I continue where I left off.
9. As a learner, I want watched percentage from max position reached, so that progress resembles YouTube-style tracking.
10. As a learner, I want course status derived from lessons, so that I cannot tick a roadmap or course “done” independently of lessons.
11. As a learner, I want a roadmap as a graph of courses, so that I see prerequisites/order as nodes and edges in a flowchart view.
12. As a learner, I want roadmap step status derived from the linked course, so that there is no separate roadmap checkbox.
13. As a learner, I want missing files to mark lessons broken, so that “broken” means missing media—not being behind.
14. As a learner, I want to select text on a video frame via local OCR, so that it becomes an Anki note in the Lessonbinder profile.
15. As a learner, I want due card counts from the Lessonbinder Anki collection, so that I know how much review is waiting without a custom scheduler in Lessonbinder.
16. As a learner, I want to start study from Lessonbinder using the bundled Anki stack, so that I do not install Anki Desktop separately for v1.
17. As a learner, I want deleting a course to break vault links to Anki notes without deleting those notes, so that my cards remain in the collection.
18. As a learner, I want a playlist-backed course (later slice) with per-item watch tracking, so that online lists behave like local lessons without a term-breaking downloader.
19. As a learner, I want one local SQLite database for library state, so that progress survives restarts without cloud sync.

---

## Testable scenarios

Each row is acceptance criteria. **Evidence** is the test name or command expected once the product repo exists (prompt pack naming).

### Folder import

| ID   | Actor  | Trigger                                                            | Result                                                                         | Failure                                       | Evidence                                                      |
| ---- | ------ | ------------------------------------------------------------------ | ------------------------------------------------------------------------------ | --------------------------------------------- | ------------------------------------------------------------- |
| F-01 | System | `import_folder(root)` on fixture with `a.mp4`, `b.mkv`, `notes.md` | One `Course`, three `Lesson` rows, stable `lesson_stable_key` by relative path | Hidden file or `node_modules` creates lessons | `vault_core::import::creates_lessons_for_eligible_extensions` |
| F-02 | System | `import_folder` twice on same path                                 | Same `CourseId` and `LessonId`s; lesson count unchanged                        | Second import duplicates lessons              | `vault_core::import::idempotent_reimport`                     |
| F-03 | System | Fixture with `lecture.mp4` + `lecture.srt`                         | One lesson for video; `subtitle_file_id` set; no lesson for `.srt`             | `.srt` appears as its own lesson              | `vault_core::import::attaches_subtitle_by_stem`               |
| F-04 | System | Re-import after deleting `a.mp4` from disk                         | Lesson for `a.mp4` → `Progress.status == broken`                               | Status stays `done` or `not_started`          | `vault_core::import::missing_file_marks_lesson_broken`        |
| F-05 | System | Re-import after adding `new.webm`                                  | New lesson appears; existing lessons unchanged                                 | New file ignored                              | `vault_core::import::adds_new_files_on_rescan`                |

### Content index (extractors)

| ID   | Actor  | Trigger                                      | Result                                                                | Failure                              | Evidence                                                      |
| ---- | ------ | -------------------------------------------- | --------------------------------------------------------------------- | ------------------------------------ | ------------------------------------------------------------- |
| C-01 | System | Index video lesson; ffprobe available in env | `duration_ms` is `known`                                              | Invented duration when ffprobe fails | `vault_core::extract::video_duration_when_ffprobe_present`    |
| C-02 | System | Index video; ffprobe absent                  | `duration_ms` is `unknown` with reason                                | Fake numeric duration                | `vault_core::extract::video_duration_unknown_without_ffprobe` |
| C-03 | System | Index lesson with `.srt` fixture             | `TranscriptSegment` count &gt; 0; first/last timestamps match file    | Empty segments with non-empty file   | `vault_core::extract::srt_cue_count_and_bounds`               |
| C-04 | System | Index `readme.md`                            | `heading` known or unknown with reason; no summary paragraph invented | LLM-style summary stored             | `vault_core::extract::markdown_first_heading_or_unknown`      |
| C-05 | System | Index PDF without page tool                  | `page_count` unknown with reason                                      | Arbitrary page count                 | `vault_core::extract::pdf_pages_unknown_when_tool_missing`    |

### Accept / reject order proposal

v1 proposals are **deterministic** (relative path order). No ML (Decision 8). ML proposals are out of scope until charter Decision 3 is exercised in a later release.

| ID   | Actor   | Trigger                                                    | Result                                                                       | Failure                                   | Evidence                                                 |
| ---- | ------- | ---------------------------------------------------------- | ---------------------------------------------------------------------------- | ----------------------------------------- | -------------------------------------------------------- |
| O-01 | Learner | After import, system offers `OrderProposal` (sorted paths) | `proposal.sort_indices` differs from current only when user had custom order | Proposal applied without user action      | `vault_core::order::proposal_not_applied_without_accept` |
| O-02 | Learner | `accept_order_proposal(proposal_id)`                       | `Lesson.sort_index` matches proposal; persisted                              | Partial apply or silent reorder on import | `vault_core::order::accept_applies_proposed_indices`     |
| O-03 | Learner | `reject_order_proposal(proposal_id)`                       | `sort_index` unchanged from pre-proposal state                               | Indices change on reject                  | `vault_core::order::reject_leaves_order_unchanged`       |
| O-04 | Learner | Manual `reorder_lessons([ids...])`                         | New `sort_index` sequence; no automatic follow-up reorder                    | Background job re-sorts by path           | `vault_core::order::manual_reorder_persists`             |

### Lesson tick (manual complete / incomplete)

| ID   | Actor   | Trigger                                             | Result                                          | Failure                                      | Evidence                                                    |
| ---- | ------- | --------------------------------------------------- | ----------------------------------------------- | -------------------------------------------- | ----------------------------------------------------------- |
| T-01 | Learner | `mark_lesson_done(lesson_id)`                       | `Progress.status == done`; `marked_done_at` set | Watch % alone sets `done`                    | `vault_core::progress::manual_done`                         |
| T-02 | Learner | `mark_lesson_incomplete` from `done`, resume &gt; 0 | `status == in_progress`                         | Returns to `not_started` while resume &gt; 0 | `vault_core::progress::incomplete_from_done_with_resume`    |
| T-03 | Learner | `mark_lesson_incomplete` from `done`, resume == 0   | `status == not_started`                         | Stays `done`                                 | `vault_core::progress::incomplete_from_done_without_resume` |
| T-04 | System  | High `watched_percent`, no manual tick              | `status` remains not `done`                     | Auto-`done` at threshold                     | `vault_core::progress::watch_percent_does_not_auto_done`    |

### Derived course progress

| ID   | Actor  | Trigger                                    | Result                           | Failure                   | Evidence                                             |
| ---- | ------ | ------------------------------------------ | -------------------------------- | ------------------------- | ---------------------------------------------------- |
| D-01 | System | All lessons `not_started`                  | `course_progress == not_started` | Any other derived value   | `vault_core::rollup::course_all_not_started`         |
| D-02 | System | All lessons `done`                         | `course_progress == done`        | Partial done shows `done` | `vault_core::rollup::course_all_done`                |
| D-03 | System | Mix of `in_progress` and `not_started`     | `course_progress == in_progress` | `not_started`             | `vault_core::rollup::course_mixed_active`            |
| D-04 | System | Any `broken`, none `in_progress` or `done` | `course_progress == broken`      | `in_progress`             | `vault_core::rollup::course_broken_only`             |
| D-05 | System | Any `broken` with one `in_progress`        | `course_progress == in_progress` | `broken`                  | `vault_core::rollup::course_broken_with_in_progress` |

### Roadmap flowchart data (nodes + edges)

| ID   | Actor   | Trigger                                                 | Result                                                                  | Failure                      | Evidence                                                |
| ---- | ------- | ------------------------------------------------------- | ----------------------------------------------------------------------- | ---------------------------- | ------------------------------------------------------- |
| R-01 | Learner | `create_roadmap` + add steps for courses A, B; edge A→B | `roadmap_graph` returns nodes `{A,B}` and edges `[{A→B}]`               | Bitmap/image URL only        | `vault_core::roadmap::graph_nodes_and_edges`            |
| R-02 | System  | Course A derived `in_progress`, step links A            | Step derived status `in_progress`; no stored step checkbox              | Independent step `done` flag | `vault_core::roadmap::step_status_equals_course_rollup` |
| R-03 | Learner | Remove step from roadmap                                | Graph omits node; edges touching it removed or rejected                 | Dangling edge ids            | `vault_core::roadmap::remove_step_updates_edges`        |
| R-04 | System  | Roadmap with two steps, courses rollup mixed            | Roadmap derived status follows same rollup rule on step course statuses | Manual roadmap-only status   | `vault_core::roadmap::roadmap_derived_rollup`           |

### Playlist course (charter-ready; ship after folder MVP—Decision 6)

| ID   | Actor   | Trigger                                      | Result                                                                          | Failure                 | Evidence                                                                 |
| ---- | ------- | -------------------------------------------- | ------------------------------------------------------------------------------- | ----------------------- | ------------------------------------------------------------------------ |
| P-01 | Learner | `import_playlist(url)` with stubbed metadata | `Course.source_kind == playlist`; `PlaylistItem` rows with url, title, duration | Downloads media file    | `vault_core::playlist::stores_metadata_only` **`#[ignore]` until slice** |
| P-02 | Learner | Set watch position on playlist lesson        | Same `Progress` fields as local lesson                                          | Separate progress model | `vault_core::playlist::progress_parity_with_local` **`#[ignore]`**       |
| P-03 | Learner | Map `local_override_path` on item            | Playback resolves override; tracking unchanged                                  | Built-in ripper invoked | `vault_core::playlist::local_override_path` **`#[ignore]`**              |

### Resume position & watch metrics

| ID   | Actor   | Trigger                                               | Result                                                         | Failure                                        | Evidence                                                  |
| ---- | ------- | ----------------------------------------------------- | -------------------------------------------------------------- | ---------------------------------------------- | --------------------------------------------------------- |
| V-01 | Learner | `set_resume_position(lesson_id, ms)` with ms &gt; 0   | Persisted ms; `not_started` → `in_progress`                    | Negative ms accepted                           | `vault_core::progress::resume_sets_in_progress`           |
| V-02 | Learner | Reopen lesson                                         | Same resume ms returned                                        | Reset to 0                                     | `vault_core::progress::resume_round_trip`                 |
| V-03 | System  | `set_resume_position` only                            | `status` not `done`                                            | Resume marks done                              | `vault_core::progress::resume_does_not_mark_done`         |
| V-04 | System  | `max_position_reached_ms` updated on playback reports | `watched_percent` computed per Decision 14 when duration known | Percent when duration unknown shown as unknown | `vault_core::progress::watched_percent_from_max_position` |

### OCR selection → Anki note

| ID   | Actor   | Trigger                                             | Result                                                                                          | Failure                            | Evidence                                            |
| ---- | ------- | --------------------------------------------------- | ----------------------------------------------------------------------------------------------- | ---------------------------------- | --------------------------------------------------- |
| A-01 | Learner | OCR capture on frame at `t_ms` with local Tesseract | `OcrSelection` stored; `AnkiCardLink` with note in deck `Lessonbinder`, type `Lessonbinder OCR` | Cloud OCR endpoint called          | `anki_bridge::ocr::creates_note_local_tesseract`    |
| A-02 | System  | Note created                                        | Front = OCR text; Back contains lesson title, course name, timestamp                            | Card body stored only in vault SQL | `anki_bridge::ocr::card_fields_shape`               |
| A-03 | Learner | Delete course                                       | `AnkiCardLink.link_status == broken`; note id still in collection                               | Note deleted from collection       | `vault_core::course_delete::breaks_links_not_notes` |

### Due-count read

| ID   | Actor  | Trigger                                    | Result                                       | Failure                   | Evidence                                            |
| ---- | ------ | ------------------------------------------ | -------------------------------------------- | ------------------------- | --------------------------------------------------- |
| K-01 | System | Open Lessonbinder profile with N due cards | `due_counts()` returns N matching collection | Custom FSRS in vault-core | `anki_bridge::study::due_counts_matches_collection` |
| K-02 | System | Empty new profile                          | Due counts zero                              | Panic or wrong profile    | `anki_bridge::study::due_counts_empty_profile`      |

### Study session (shipped Anki stack—not separate Desktop install)

Charter Decision 16 replaces “install Anki Desktop first.” v1 **study** opens review through the **bundled `rslib` path** on the Lessonbinder profile (UI shell TBD in architecture).

| ID   | Actor   | Trigger                                | Result                                                | Failure                                 | Evidence                                              |
| ---- | ------- | -------------------------------------- | ----------------------------------------------------- | --------------------------------------- | ----------------------------------------------------- |
| S-01 | Learner | `start_study_session()`                | Session uses Lessonbinder Anki profile path only      | Reads user’s default Anki profile       | `anki_bridge::study::uses_lessonbinder_profile_only`  |
| S-02 | Learner | `start_study_session()` with due cards | At least one card reachable via `rslib` review APIs   | Requires external `anki` binary on PATH | `anki_bridge::study::in_process_review_entry`         |
| S-03 | System  | Profile missing / corrupt              | Structured error; no silent fallback to other profile | Opens wrong collection                  | `anki_bridge::study::errors_when_profile_unavailable` |

---

## Implementation Decisions

- **Primary test seam:** `vault-core` — pure domain: import, extractors, order proposals, `Progress` transitions, rollups, `roadmap_graph`. No `anki` crate dependency in `vault-core`.
- **Secondary seam:** `anki-bridge` — only crate linking `anki` / `rslib`: note creation, due counts, study session entry, dedicated profile path under app data.
- **Persistence:** Single SQLite database per install (Decision 10); media paths on disk.
- **Order proposals:** Deterministic generator (e.g. relative path) produces a proposal record; accept/reject commands mutate `Lesson.sort_index`; import alone does not apply proposals (O-01).
- **Progress state machine:** Matches `lessonbinder-domain.md` transition table; illegal edges rejected with error.
- **Rollup:** Single function `derive_status(Vec<LessonStatus>) -> DerivedStatus` used for courses and roadmap aggregates.
- **Roadmap API:** `roadmap_graph(roadmap_id) -> { nodes: [{ step_id, course_id, label, derived_status }], edges: [{ from, to }] }` — data for SVG/canvas, not a raster asset.
- **OCR:** Tesseract only (Decision 1); forbidden HTTP OCR hosts in v1 builds (lint or integration guard).
- **Playlist:** Domain types present; importer tests ignored until post–folder-MVP slice.
- **Anki:** One profile directory owned by Lessonbinder; card bodies only in collection; `AnkiCardLink` in vault DB.

### Progress + rollup (decision-rich snippet from domain)

```text
LessonStatus: not_started | in_progress | done | broken

derive_status(lessons):
  if all not_started -> not_started
  if all done -> done
  if any broken and none in_progress or done -> broken
  else -> in_progress
```

---

## Testing Decisions

- **Good tests** assert **observable outcomes**: entity counts, ids, status enums, graph shape, note ids, due integers—not private SQL or internal caches.
- **Fixtures:** Temp directories under `vault-core/tests/fixtures/` for import/extract; in-memory or temp-file SQLite for integration tests.
- **ffprobe:** Tests that need it use `#[cfg(feature = "ffprobe-integration")]` or env guard and skip cleanly when missing (prompt 16 pattern).
- **anki-bridge:** Use temporary Anki collection path per test; never touch user home Anki profile.
- **Commands (target repo):**
  - `cargo test -p vault-core`
  - `cargo test -p anki-bridge`
  - `cargo test --workspace` before merge

**Testing seam check (to-spec):** Prefer **one** primary seam—`vault-core` domain API—with `anki-bridge` as the only Anki touchpoint. Confirm in chat if you want e2e Tauri tests added later (out of v1 SPEC scope).

---

## Out of Scope

- Accounts, payments, cloud sync, mobile apps, AI chat tutor (charter).
- Whisper / generated subtitles (Decision 9); ingest SRT/VTT only.
- Local ML order proposals (Decision 8).
- Cloud OCR (Decision 1).
- Bundled video downloaders that violate site terms (Decision 2).
- Custom FSRS or parallel scheduler in Lessonbinder.
- Auto-`done` from watch percentage (Decision 11).
- AulaVault trademark, license server, Anki Qt fork, second SRS (`lessonbinder-workspace.md`).
- UI polish adjectives without acceptance rows (e.g. “intuitive”, “AI organizes the course”).
- Requiring a **separate** Anki Desktop install (Decision 16)—superseded by shipped `rslib` study path (S-01–S-03).

---

## Further Notes

- **Issue tracker:** Product repo lives under `/home/xingzi/Documents/` (see prompt 09); this SPEC is file-based until prompt 09. No `ready-for-agent` label applied.
- **Prompt vs charter:** Scenarios use **study session / rslib** instead of “open Anki Desktop” where Decision 16 conflicts with older prompt wording.
- **AGPL:** Distributing builds that link `anki` triggers AGPL obligations (`lessonbinder-workspace.md`).
- **Glossary:** Use terms from `lessonbinder-domain.md` (`Lesson`, `Progress`, `DerivedStatus`, `broken`, etc.) in all future docs and tests.
