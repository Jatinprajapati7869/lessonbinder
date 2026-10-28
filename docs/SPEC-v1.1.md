# Lessonbinder SPEC (v1.1)

**Scope:** [`SCOPE-v1.1.md`](SCOPE-v1.1.md) only.
**Baseline:** v1 scenarios that already **pass** in [`V1.md`](V1.md) are not repeated here (e.g. subtitle **indexing** C-03, folder import F-01–F-03, due counts K-01–K-02).

Each row is acceptance criteria. **Evidence** is the expected Rust integration test name (`cargo test`) or UI test name where the seam is the webview.

**Primary seams:** `anki-bridge` (in-process review, collection profile), `vault-core` (progress, import/rescan, notes, search index, watch ranges, document position, transcription port). UI scenarios cite Vitest when behavior is presentation-only.

---

## Study — in-app review (`R-`)

| ID   | Trigger                                                            | Result                                                                          | Failure                                            | Evidence                                                                                     |
| ---- | ------------------------------------------------------------------ | ------------------------------------------------------------------------------- | -------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| R-01 | `start_review_session()` on temp Lessonbinder profile              | Session opens **only** `{app_data}/anki-profile/` collection; no home Anki path | Default or fallback profile used                   | `anki_bridge::study::uses_lessonbinder_profile_only`                                         |
| R-02 | Session started; at least one due card in `Lessonbinder::` deck    | `next_review_card()` returns a card id in that deck subtree                     | Card from unrelated deck or empty while dues exist | `anki_bridge::study::next_card_from_lessonbinder_decks`                                      |
| R-03 | `answer_card(card_id, Good)` (and Again / Hard / Easy)             | Card scheduled via `rslib`; session can advance to next due                     | Grade no-op, panic, or writes outside collection   | `anki_bridge::study::answer_grades_persist_in_collection`                                    |
| R-04 | No cards due in Lessonbinder subtree                               | `next_review_card()` returns end-of-queue (not error)                           | Panic or infinite repeat of same card              | `anki_bridge::study::empty_queue_ends_session`                                               |
| R-05 | Profile directory missing or collection cannot open                | Structured error; no silent open of another profile                             | Wrong collection or empty success                  | `anki_bridge::study::errors_when_profile_unavailable`                                        |
| R-06 | Primary **Study** action in shell (integration / manual checklist) | In-app review surface opens by default                                          | Spawns Anki Desktop without user choosing fallback | `ui::StudyScreen.test.tsx::opens_in_app_review_by_default`                                   |
| R-07 | User chooses explicit **Study in Anki** fallback                   | `anki` binary spawned with Lessonbinder `-b`/`-p` args                          | Wrong profile args or required when in-app works   | `lessonbinder_tauri::study::launch_command_uses_profile_dir` (existing); manual UI checklist |

---

## Subtitles on the player (`S-`)

| ID   | Trigger                                                    | Result                                                                       | Failure                                 | Evidence                                                      |
| ---- | ---------------------------------------------------------- | ---------------------------------------------------------------------------- | --------------------------------------- | ------------------------------------------------------------- |
| S-01 | Load lesson with indexed SRT/VTT sidecar                   | Player read model includes ordered cues `{start_ms, end_ms, text}`           | Empty list despite C-03 index data      | `vault_core::read_model::lesson_subtitle_cues_for_player`     |
| S-02 | Lesson with no subtitle file                               | Cue list empty; playback still works                                         | Error state blocking player             | `vault_core::read_model::lesson_without_subtitles_empty_cues` |
| S-03 | `seek_to_cue(lesson_id, cue_start_ms)` (command or domain) | Persisted resume / player position equals cue start                          | Seek ignored or wrong ms                | `vault_core::progress::seek_to_subtitle_cue`                  |
| S-04 | `card_draft_from_subtitle_cue(lesson_id, cue_id)`          | `CardDraft` front = cue text; timestamp = cue start; same commit path as OCR | Separate note type or missing timestamp | `vault_core::card::draft_from_subtitle_cue`                   |
| S-05 | User activates cue in player UI                            | Video seeks to cue start (smoke)                                             | Click no-op                             | `ui::Player.test.tsx::clicking_cue_seeks_playback`            |

---

## Watch tracking — interval union (`W-`)

| ID   | Trigger                                                                | Result                                         | Failure                             | Evidence                                                         |
| ---- | ---------------------------------------------------------------------- | ---------------------------------------------- | ----------------------------------- | ---------------------------------------------------------------- |
| W-01 | `record_watched_range(lesson_id, start_ms, end_ms)` twice on same span | Stored union length equals single interval     | Duplicate storage inflates percent  | `vault_core::progress::watched_range_idempotent`                 |
| W-02 | Two overlapping played ranges on same lesson                           | Union is merged span; length ≤ duration        | Double-count overlap                | `vault_core::progress::watched_range_union_merges_overlap`       |
| W-03 | Union length 30s, `duration_ms` 60s                                    | `watched_percent` == 50 (rounded per UI rule)  | Uses `max_position_reached_ms` only | `vault_core::progress::watched_percent_from_union`               |
| W-04 | `duration_ms` unknown                                                  | `watched_percent` unknown; ranges still stored | Invented percent                    | `vault_core::progress::watched_percent_unknown_without_duration` |
| W-05 | Union covers 100% of duration; status `not_started` or `in_progress`   | `Progress.status` unchanged (not `done`)       | Auto-`done` from watch metrics      | `vault_core::progress::union_full_watch_does_not_auto_done`      |
| W-06 | Playback position reports during play                                  | `resume_position_ms` and union both update     | Resume frozen while ranges grow     | `vault_core::progress::playback_updates_resume_and_ranges`       |

---

## Folder rescan and ignore (`I-`)

| ID   | Trigger                                                           | Result                                        | Failure                                 | Evidence                                                               |
| ---- | ----------------------------------------------------------------- | --------------------------------------------- | --------------------------------------- | ---------------------------------------------------------------------- |
| I-01 | `rescan_course_folder(course_id)` after adding `new.webm` on disk | New lesson row; existing lesson ids unchanged | Duplicate lessons or ids reused wrongly | `vault_core::import::rescan_adds_new_files`                            |
| I-02 | Rescan after deleting a lesson file from disk                     | That lesson `Progress.status == broken`       | Stays `not_started` / `done`            | `vault_core::import::rescan_missing_file_marks_broken`                 |
| I-03 | Rescan with no filesystem changes                                 | Lesson count and ids unchanged                | Spurious new rows                       | `vault_core::import::rescan_unchanged_is_idempotent`                   |
| I-04 | Import folder containing `.lessonbinderignore` listing `extras/`  | Files under `extras/` not lessons             | Ignored paths imported as lessons       | `vault_core::import::lessonbinderignore_excludes_paths`                |
| I-05 | Rescan after adding ignored path under existing ignore rules      | No lesson created for ignored file            | New lesson for ignored path             | `vault_core::import::rescan_honors_lessonbinderignore`                 |
| I-06 | Import with ignore file + normal media at root                    | Eligible root files still become lessons      | Everything ignored                      | `vault_core::import::lessonbinderignore_does_not_block_eligible_files` |

---

## Lesson notes — vault only (`N-`)

| ID   | Trigger                                 | Result                                                                     | Failure                            | Evidence                                               |
| ---- | --------------------------------------- | -------------------------------------------------------------------------- | ---------------------------------- | ------------------------------------------------------ |
| N-01 | `set_lesson_note(lesson_id, text)`      | Note persisted in `vault.db` only                                          | Written to Anki collection         | `vault_core::lesson_note::save_round_trip`             |
| N-02 | `get_lesson_note(lesson_id)` after save | Returns same text                                                          | Stale or cross-lesson bleed        | `vault_core::lesson_note::get_after_set`               |
| N-03 | `set_lesson_note(lesson_id, "")`        | Note cleared; no Anki side effect                                          | Orphan Anki note created           | `vault_core::lesson_note::clear_does_not_touch_anki`   |
| N-04 | Commit OCR card for same lesson         | `AnkiCardLink` + collection note; lesson note unchanged unless user edited | Lesson note overwritten by capture | `vault_core::lesson_note::independent_of_anki_capture` |

---

## PDF and markdown position (`D-`)

| ID   | Trigger                                                              | Result             | Failure                           | Evidence                                                         |
| ---- | -------------------------------------------------------------------- | ------------------ | --------------------------------- | ---------------------------------------------------------------- |
| D-01 | `set_document_position(lesson_id, pdf_page)` on PDF lesson           | Position persisted | Lost on reload                    | `vault_core::progress::pdf_position_round_trip`                  |
| D-02 | Open PDF lesson after save                                           | Same page restored | Resets to first page              | `vault_core::read_model::pdf_lesson_resumes_page`                |
| D-03 | `set_document_position(lesson_id, scroll_offset)` on markdown lesson | Offset persisted   | Lost on reload                    | `vault_core::progress::markdown_position_round_trip`             |
| D-04 | Markdown lesson with position; user marks `done` then incomplete     | Position retained  | Position cleared on status change | `vault_core::progress::document_position_survives_status_change` |

---

## Search (`Q-`)

| ID   | Trigger                                           | Result                                     | Failure                  | Evidence                                        |
| ---- | ------------------------------------------------- | ------------------------------------------ | ------------------------ | ----------------------------------------------- |
| Q-01 | `search_library("intro")` matches lesson title    | Hit includes `lesson_id` and course        | Miss despite title match | `vault_core::search::matches_lesson_title`      |
| Q-02 | Query matches relative path substring             | Hit includes correct lesson                | Path not indexed         | `vault_core::search::matches_relative_path`     |
| Q-03 | Query matches subtitle cue text (indexed sidecar) | Hit includes lesson with cue snippet       | Cue text not searchable  | `vault_core::search::matches_subtitle_cue_text` |
| Q-04 | Empty or whitespace query                         | No spurious hits (or defined empty result) | Returns entire library   | `vault_core::search::empty_query_behavior`      |
| Q-05 | User opens hit from search UI                     | Navigates to course/player for that lesson | Wrong lesson             | `ui::Search.test.tsx::opens_lesson_from_hit`    |

---

## Optional local transcription (`T-`)

| ID   | Trigger                                                | Result                                                             | Failure                                  | Evidence                                                     |
| ---- | ------------------------------------------------------ | ------------------------------------------------------------------ | ---------------------------------------- | ------------------------------------------------------------ |
| T-01 | No local model path configured                         | `transcribe_lesson` returns `transcription_disabled`; no DB writes | Partial segments written                 | `vault_core::transcription::disabled_without_model_path`     |
| T-02 | User sets valid local model path; fake adapter in test | Transcription succeeds; segments stored on lesson                  | Order or progress changed before success | `vault_core::transcription::writes_segments_when_configured` |
| T-03 | Transcription adapter invoked                          | No HTTP(S) fetch of weights (guard or fake adapter contract)       | Network download attempted               | `vault_core::transcription::no_network_weight_download`      |
| T-04 | Transcription fails mid-run                            | Lesson order and progress unchanged; error surfaced                | Silent reorder or `done` flip            | `vault_core::transcription::failure_leaves_lesson_unchanged` |

---

## Implementation decisions

- **Review:** New `anki-bridge` study session API wraps `rslib` card queue and answer paths; Lessonbinder profile lock matches existing `open_profile` / `GuardedProfile` behavior. UI default route is in-app review; Desktop launch remains `lessonbinder-tauri::study` fallback.
- **Subtitles:** Cues served from existing extractor/index data; player commands reuse `CardDraft` / `commit_card_draft` without new note types.
- **Watch union:** Persist normalized intervals on `Progress` (or child table); `watched_percent` derived at read time when `duration_ms` known; `done` rules unchanged from v1.
- **Rescan:** Explicit `rescan_course_folder` on known `Course` source path; `.lessonbinderignore` parsed with gitignore-style rules alongside existing hidden/`node_modules` skips.
- **Lesson notes:** New vault entity or column keyed by `lesson_id`; never synced to Anki.
- **Document position:** Typed fields per `SourceFileKind` (page vs scroll offset).
- **Search:** SQLite FTS or normalized index table rebuilt on import/rescan/index; queries return lesson + course ids only.
- **Transcription:** `TranscriptionEngine` implementation selected when settings contain model path; Whisper/cpp adapter is user-supplied binary path only.

## Testing decisions

- Assert **observable outcomes**: status enums, ids, cue lists, search hits, scheduling state after grades—not private SQL.
- **Fixtures:** Reuse `vault-core/tests/fixtures/` for SRT and import trees; temp Anki profiles for `anki-bridge` review tests.
- **Commands:** `cargo test -p vault-core`, `cargo test -p anki-bridge`, `cargo test --workspace`; `cd ui && npm test` for player/search/review shell cases marked above.

## Out of scope (v1.1)

Same as [`SCOPE-v1.1.md`](SCOPE-v1.1.md): accounts, sync, mobile, chat tutor, auto-`done` from watch %, playlist downloaders, renaming OCR note type, Whisper-by-default, cloud OCR, custom FSRS, bundled model downloads.

## Further notes

- When v1.1 ships, update [`STUDY.md`](STUDY.md), [`V1.md`](V1.md) successor checklist, and charter Decision 16 wording vs shipped default (in-app primary).
- Prompt **52** in [`PROGRESS.md`](PROGRESS.md) tracks adoption of this file.
