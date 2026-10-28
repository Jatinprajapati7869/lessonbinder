# Lessonbinder prompt progress

Check off when the matching prompt in `auravault-cursor-prompts.md` (under `~/Documents/`) is complete.

## Planning (Documents + repo bootstrap)

- [x] 01 — Workspace boundary
- [x] 02 — Product charter
- [x] 03 — Grill the remaining decisions
- [x] 04 — Domain model
- [x] 05 — SPEC.md
- [x] 06 — Architecture
- [x] 07 — Anti-slop design contract
- [x] 08 — Tickets
- [x] 09 — Create the repo
- [x] 10 — Skills and hooks for this repo

## Implementation

- [x] 11 — Anki bridge opens a collection
- [ ] 12 — Note type and decks
- [x] 13 — vault.db
- [x] 14 — Progress state machine
- [x] 15 — Folder import
- [x] 16 — Content index
- [x] 17 — Resume position
- [x] 18 — Manual completion
- [x] 19 — Derived progress read model
- [x] 20 — Order proposals
- [x] 21 — Roadmap model
- [x] 22 — Remaining time
- [x] 23 — Flowchart graph
- [x] 24 — Playlist course
- [x] 25 — Playlist tracking
- [x] 26 — Unified library
- [x] 27 — Bridge between vault and Anki ids
- [x] 28 — OCR port
- [x] 29 — Selection to card draft
- [x] 30 — Write the Anki note
- [x] 31 — Due counts
- [x] 32 — Launch Anki
- [x] 33 — Transcription port
- [x] 34 — Architecture guard

**Prompt 34 — `cargo test --workspace` (2026-10-27):**

```text
cargo test: 65 passed (29 suites, 0.30s)
```

- [x] 35 — App shell
- [x] 36 — Visual foundations
- [x] 37 — Library screen

**Prompt 37 — manual:** Native folder/JSON pickers use `@tauri-apps/plugin-dialog` and only run inside the Tauri webview; Vitest uses injected `pickFolder` / `pickPlaylistJson` paths (no headless picker).

- [x] 38 — Course and player
- [x] 39 — Roadmap screen
- [x] 40 — OCR capture
- [x] 41 — Broken and offline states
- [ ] 42 — Order suggestion stub
- [x] 43 — Keyboard and preview
- [x] 44 — Impeccable audit
- [ ] 45 — Impeccable polish
- [x] 46 — React doctor

**Prompt 46 — `npx react-doctor@latest --verbose .` (ui/, 2026-10-28):**

```text
React Doctor — ui
Score: 88 / 100 Great
4 issues
Maintainability: 4 warnings

⚠ React function has high control-flow complexity ×2
  react-doctor/no-high-complexity-react-function
  src/screens/CaptureCardPanel.tsx:38
  src/screens/RoadmapScreen.tsx:12

⚠ Large component is hard to read and change ×2
  react-doctor/no-giant-component
  src/screens/CaptureCardPanel.tsx:38
  src/screens/RoadmapScreen.tsx:12

Share: https://react.doctor/share?p=ui&s=88&w=4&f=2
```

Fixes applied: split command hooks out of provider `.tsx` files; roadmap SVG `role="group"` (nested interactive); `cropPng` → ref; `refreshDetail` deps; stable player space listener via ref + effect. Remaining warnings left (component splits would be broad refactors).

- [ ] 47 — Deslop
- [ ] 48 — Review
- [ ] 49 — Repair the review
- [ ] 50 — Record what is true

## v1.1 (prompts 51–100)

Start only after 12 and 40–50 are checked. Full text: `~/Documents/auravault-cursor-prompts.md`.

- [x] 51 — v1.1 scope (`docs/SCOPE-v1.1.md`)
- [x] 52 — SPEC v1.1 (`docs/SPEC-v1.1.md`)
- [ ] 53 — Next card
- [ ] 54 — Answer a card
- [ ] 55 — Review session bounds
- [ ] 56 — Render card content
- [ ] 57 — Media from the collection
- [ ] 58 — Review UI
- [x] 59 — Study doc
- [ ] 60 — Subtitle cues on a lesson
- [ ] 61 — Player cues
- [ ] 62 — Cue to card draft
- [ ] 63 — Watched ranges
- [ ] 64 — Player reports ranges
- [ ] 65 — Ignore file
- [ ] 66 — Rescan
- [ ] 67 — Rescan UI
- [ ] 68 — Lesson notes
- [ ] 69 — Notes UI
- [ ] 70 — PDF position
- [ ] 71 — Markdown reader
- [ ] 72 — Search index
- [ ] 73 — Search UI
- [ ] 74 — Sections
- [ ] 75 — Section headings
- [ ] 76 — Continue
- [ ] 77 — Continue on the library
- [ ] 78 — Prerequisite editor
- [ ] 79 — Export SVG
- [ ] 80 — Relink a broken step
- [ ] 81 — Archive a course
- [ ] 82 — Archive UI
- [ ] 83 — Settings model
- [ ] 84 — Settings screen
- [ ] 85 — Transcription adapter
- [ ] 86 — Transcribe action
- [ ] 87 — Backup
- [ ] 88 — Restore
- [ ] 89 — Failure log
- [ ] 90 — Fixture scale
- [ ] 91 — First-run library
- [ ] 92 — Linux bundle
- [ ] 93 — Bundle smoke
- [x] 94 — Audit new screens (`docs/DESIGN_AUDIT_v1.1.md`)
- [x] 95 — Polish the audit (token/export/search/title fixes per audit)
- [x] 96 — Doctor and deslop

**Prompt 96 — `npx react-doctor@latest --verbose .` (ui/, 2026-10-28, v1.1 pass):**

```text
React Doctor — ui
Score: 68 / 100 Needs work
19 issues
Maintainability: 9 warnings
Accessibility: 3 warnings
Performance: 3 warnings
Bugs: 3 warnings
Security: 1 warning

⚠ Non-component export in component file ×2
  src/library/LibraryContinueRow.tsx:8

⚠ React function has high control-flow complexity ×3
  src/screens/CaptureCardPanel.tsx:37
  src/screens/ReviewScreen.tsx:32
  src/screens/RoadmapScreen.tsx:13

⚠ Large component is hard to read and change ×4
  src/screens/CaptureCardPanel.tsx:37
  src/screens/CourseScreen.tsx:30
  src/screens/PlayerScreen.tsx:90
  src/screens/RoadmapScreen.tsx:13

⚠ Custom modal instead of dialog ×3
  src/screens/CourseScreen.tsx:309
  src/screens/SettingsScreen.tsx:206
  src/screens/SubtitleCueCardPanel.tsx:77

⚠ State initializer runs on every render
  src/screens/DocumentLessonScreen.tsx:35

⚠ fetch Response consumed without status check
  src/screens/DocumentLessonScreen.tsx:67

⚠ State adjusted after a prop changes
  src/screens/DocumentLessonScreen.tsx:92

⚠ Unguarded numeric parse of an input value
  src/screens/DocumentLessonScreen.tsx:155

⚠ iframe missing sandbox attribute
  src/screens/DocumentLessonScreen.tsx:163

⚠ createObjectURL without revokeObjectURL
  src/screens/ReviewScreen.tsx:70

⚠ State only used in handlers
  src/screens/SettingsScreen.tsx:22

Share: https://react.doctor/share?p=ui&s=68&w=19&f=9
```

Deslop (v1.1 delta): dropped redundant export-SVG and `unexpected_command_failure` doc comments. No `--scope changed` (repo has no commits yet; full `ui/` scan). Remaining doctor warnings left (component splits, `<dialog>`, DocumentLesson hardening) — same class as prompt 46; ReviewScreen added to complexity list.

- [ ] 97 — Review v1.1
- [ ] 98 — Repair v1.1
- [ ] 99 — Keyboard on review and capture
- [ ] 100 — Freeze v1.1

## REA behavior check (prompts 101–110)

Optional. Start only after 50. Do not decompile AulaVault into this repo. Full text: `~/Documents/auravault-cursor-prompts.md`.

- [ ] 101 — Command inventory
- [ ] 102 — What we will build from that list
- [ ] 103 — Risky path guard
- [ ] 104 — Guard in the library UI
- [ ] 105 — Cancel a scan
- [ ] 106 — Lesson resources
- [ ] 107 — Resources on the lesson screen
- [ ] 108 — Frame thumbnail
- [ ] 109 — Thumbnail in the library
- [ ] 110 — Stop the REA track

## Study loop (prompts 111–130)

Start here. 101–110 are already in the tree. Full text: `~/Documents/auravault-cursor-prompts.md`.

- [x] 111 — Scope (`docs/SCOPE-v1.2.md`)
- [x] 112 — Lesson reorder
- [x] 113 — Reorder controls
- [x] 114 — Skip foreign due cards
- [x] 115 — Empty queue copy
- [x] 116 — Links for a lesson
- [x] 117 — Cards on the lesson
- [x] 118 — Unlink
- [x] 119 — Note to card draft
- [x] 120 — Audio clip port
- [x] 121 — Clip on the card
- [x] 122 — Chapters
- [x] 123 — Up next
- [x] 124 — Review this course
- [x] 125 — Paste a playlist
- [x] 126 — Reorder survives rescan in the UI
- [ ] 127 — Foreign-card regression names
- [ ] 128 — README stack line
- [x] 129 — Tests for this slice
  - `cargo test --workspace`: **193 passed**
  - `cd ui && npm test`: **93 passed** (38 files)
- [ ] 130 — Freeze the slice

## First test (prompts 131–140)

No new features. Full text: `~/Documents/auravault-cursor-prompts.md`.

- [ ] 131 — What we will test
- [x] 132 — Automated tests
  - `cargo test --workspace`: **pass** — 193 passed; 0 failed
  - `cd ui && npm test`: **pass** — Test Files 38 passed (38); Tests 93 passed (93)
  - `npm run typecheck`: **pass** — `tsc -p ui/tsconfig.json --noEmit` (exit 0)
  - `npm run lint`: **pass** — ESLint: No issues found (exit 0). Root `eslint .` was failing on generated paths (`target/`, `ui/dist/`, `.venv/`); `eslint.config.js` now ignores those and lints `ui/**` + `scripts/**` only (not `anki-main`).
- [ ] 133 — Deslop, keep behavior
- [ ] 134 — Slop scan
- [x] 135 — Doctor, blockers only
  - `cd ui && npx react-doctor@latest . --verbose`: **62 / 100** (was 60 before fix). Fixed 2 errors: `no-ref-current-in-render` in `ReviewScreen.tsx` (keyboard handler refs synced in `useEffect` instead of render). Left complexity, giant components, custom modals, iframe sandbox, and fetch-status warnings unchanged.
- [x] 136 — Dev launch
  - Recorded under [`docs/FIRST-TEST.md`](FIRST-TEST.md) § Launch (`npm run tauri:dev`, temp `XDG_DATA_HOME`, library empty state confirmed).
- [x] 137 — Fixture pass
  - [`docs/FIRST-TEST.md`](FIRST-TEST.md) § Fixture pass; `cargo test -p vault-core --test first_test_fixture`.
- [x] 138 — Anki pass
  - [`docs/FIRST-TEST.md`](FIRST-TEST.md) § Anki pass; `cargo test -p anki-bridge first_test_anki_pass_walkthrough` + UI Review/Capture tests.
- [x] 139 — Safety pass
  - [`docs/FIRST-TEST.md`](FIRST-TEST.md) § Safety pass (five invariant tests, all pass).
- [x] 140 — Ready or blocked
  - [`docs/FIRST-TEST.md`](FIRST-TEST.md): **READY**; verification counts 195 + 93 tests.

## Rename to Lessonbinder (prompts 141–150)

Product name is Lessonbinder. Full text: `~/Documents/auravault-cursor-prompts.md`. Keep `vault-core` and `vault.db`. Keep the competitor name AulaVault.

- [x] 141 — Write the map
  - [`docs/RENAME.md`](RENAME.md): spelling map, keep list, app-data note.
- [x] 142 — Words a person sees
  - README, `ui/index.html`, `tauri.conf.json` titles, UI headings/aria-labels/capture copy; Vitest label assertions updated.
- [x] 143 — Packages and the binary
  - `lessonbinder` / `lessonbinder_lib` / `lessonbinder-tauri`; root `package.json`; Rust `use lessonbinder_tauri`.
- [x] 144 — Anki deck and note type
  - `Lessonbinder::`, `LessonbinderClip`, constants/functions/errors; default library name; no Anki migrator (see `docs/RENAME.md`).
- [x] 145 — Ignore file and playlist scheme
  - `.lessonbinderignore` module; `lessonbinder://` pasted playlist; tests and `docs/FIRST-TEST.md`.
- [x] 146 — Docs inside the repo
  - `docs/`, `AGENTS.md`, and product comments → Lessonbinder; `AULAVAULT-BEHAVIOR.md` keeps competitor `aulavault`; `ANKI_PIN.md` path unchanged.
- [x] 147 — Planning copies in Documents
  - Renamed `~/Documents/lessonbinder-{charter,domain,SPEC,ARCHITECTURE,DESIGN,TICKETS,workspace}.md`; product name updated; prompt pack header unchanged for 01–140.
- [x] 148 — Leftovers and tests
  - `rg` cleanup; `LessonbinderApp` / `LessonbinderProfilePaths`; identifier `dev.lessonbinder.app`; tests 195 + 93 + typecheck pass; leftovers in `docs/RENAME.md`.
- [x] 149 — Folder and GitHub
  - Checkout moved to `/home/xingzi/Documents/lessonbinder`; GitHub repo + About updated (see `docs/RENAME.md`).
- [x] 150 — Name is on the window

## README (prompt 151)

Can run after 142. Full text: `~/Documents/auravault-cursor-prompts.md`.

- [x] 151 — README

## Study app (prompts 152–201)

Repo is `/home/xingzi/Documents/lessonbinder`. Full text: `~/Documents/auravault-cursor-prompts.md`. Google Sans for UI text. Andina for the wordmark.

- [x] 152 — Scope
- [x] 153 — Bundle the fonts
- [x] 154 — Type tokens
- [x] 155 — Library look
- [x] 156 — Course look
- [x] 157 — Player look
- [x] 158 — Review, settings, roadmap
- [x] 159 — Drop a folder
- [x] 160 — Search notes
- [x] 161 — Seek from search
- [x] 162 — Export one lesson
- [x] 163 — Export the course
- [x] 164 — Theater
- [x] 165 — Picture-in-picture
- [x] 166 — Zen
- [x] 167 — Dark theme
- [x] 168 — Streak rules
- [x] 169 — Streak number
- [x] 170 — Heatmap
- [x] 171 — Weekly report
- [x] 172 — Notes save on blur
- [x] 173 — Playback speed
- [x] 174 — Volume
- [x] 175 — Last lesson
- [x] 176 — Backup
- [x] 177 — Rescan
- [x] 178 — Done stays manual
- [x] 179 — Escape
- [x] 180 — Drop a dangerous path
- [x] 181 — Export stays out of Anki
- [x] 182 — Search stays local
- [x] 183 — Dark player
- [x] 184 — Wordmark test
- [x] 185 — Heatmap text
- [x] 186 — Report copy
- [x] 187 — Drop on the empty library — Rust: `validate_import_folder_path_guard_blocks_drop_on_risky_root`. UI: `LibraryEmpty.test.tsx` stubs a dropped path via `libraryFolderDropIntoLibrary` (Vitest does not emit Tauri drag events).
- [x] 188 — Settings survive restart
- [x] 189 — Streak does not count a visit
- [x] 190 — Five-second seek — `ArrowLeft` / `ArrowRight` seek ±5s on player focus (scrubber `step` 5000 ms + `playerSeekKeyboard.ts`; no separate shortcut).
- [x] 191 — Mark complete from the keyboard — `D` on player focus calls `markLessonComplete` (same as button); `aria-label` `Mark complete (D)`; `playerCompleteKeyboard.ts` + `Player.test.tsx` / `playerCompleteKeyboard.test.ts`; watch interval still does not complete.
- [x] 192 — Archive keeps cards — `archive_keeps_lessons_progress_roadmap_status_and_anki_links` in `crates/vault-core/tests/archive_course.rs` (Anki link + `anki_note_id` unchanged after `archive_course`; UI copy: `Course.test.tsx` `archive confirm keeps Anki notes line and calls command`).
- [x] 193 — Playlist does not download — `parse_playlist_lines_accepts_two_tab_separated_lines_and_skips_blanks` + `import_playlist_from_pasted_lines_creates_playlist_course_without_http` in `crates/vault-core/tests/paste_playlist.rs` (title/tab/URL only; metadata import, no HTTP). No on-disk lesson media for playlist rows: `playlist_lesson_has_no_local_clip_source` in `crates/vault-core/tests/audio_clip.rs` (same `playlist_item` path as paste).
- [x] 194 — Fonts are local in the build — `ui/fontsBuild.test.ts` + `ui/theme.test.ts`; four `.ttf` in Vite `ui/dist/assets/`; Tauri `frontendDist` only (`docs/FONTS.md` records `rg` / `vite build` check; no `fonts.googleapis.com` / `fonts.gstatic.com`).
- [x] 195 — Missing font file — `ui/build/assertBundledFonts.ts` + Vite `buildStart` in `vite.config.ts`; build fails with `Missing licensed font file: <name>`; `fontsBuild.test.ts`; no network fallback.
- [x] 196 — Deslop this pack — shared `playerKeyboardGuard`; one player keydown listener; inlined Vite font check; trimmed prompt-number test titles and redundant font-build helpers.
- [x] 197 — Doctor — `npx react-doctor@latest . --verbose` in `ui/`: **64 / 100** (was 54); fixed pack crash — missing `SettingsCommandsProvider` import in `App.tsx`. Left complexity / giant-component / dialog warnings unchanged.
- [x] 198 — README — documents bundled Google Sans / Andina, drag-and-drop import, local note search, Markdown export, theater / zen, dark theme, study streak; PiP omitted when WebView lacks support (one clause).
- [x] 199 — Checklist — [`docs/V1.3.md`](V1.3.md) from [`SCOPE-v1.3.md`](SCOPE-v1.3.md): **28 pass**, **2 unproven** (look pass per screen; Settings theme radios E2E).
- [x] 200 — Tests — `cargo test --workspace`: **219 passed** (70 suites); `cd ui && npm test`: **157 passed** (51 files); `npm run typecheck`: **pass**.
- [x] 201 — Freeze — [`docs/FIRST-TEST.md`](FIRST-TEST.md) § Study app (v1.3) manual steps; [`docs/V1.3.md`](V1.3.md) footer **READY** + test counts (219 / 157); no release tag.

## First course and cancel (prompts 202–251)

Repo: `/home/xingzi/Documents/lessonbinder`. Full text: `~/Documents/auravault-cursor-prompts.md`. Prompt 202 commits the finished v1.3 tree. After that, every second prompt commits and pushes that pair.

- [x] 202 — Land v1.3 — commit `Ship the v1.3 study loop…` on `master`; migrations 019–024, bundled fonts, study-loop UI + docs.
- [x] 203 — First course checklist — [`docs/FIRST-COURSE.md`](FIRST-COURSE.md): one real folder; import → resume → note → card → review → theme → quit → relaunch → Continue.
- [x] 204 — Theme radios — `Settings.test.tsx` dark → save → `applyUiTheme`; light → save again (settings command stubs).
- [x] 205 — Look of each screen — `ui/screenShellLook.test.tsx` (Library, Course, Player, Review, Settings, Roadmap in `AppShell` + wordmark / Andina wiring).
- [x] 206 — Boot — [`docs/FIRST-COURSE.md`](FIRST-COURSE.md) § Boot: temp `XDG_DATA_HOME`, Library confirmed, no startup crash (2026-10-28).
- [x] 207 — One blocker — no blocker ([`docs/FIRST-COURSE.md`](FIRST-COURSE.md) § Boot: crash none; no dead control named).
- [x] 208 — Profile in use — `AnkiProfileInUse.test.tsx` shows `study_bar_state` lock sentence; `anki-bridge` `open_profile_while_lock_held_returns_in_use_message`.
- [x] 209 — Cancel design — [`docs/CANCEL-SCAN.md`](CANCEL-SCAN.md): cancel between files, partial course OK, no Anki on import, Escape ≠ cancel.
- [x] 210 — Import yields — `import_folder_with_cancel` checks cancel during scan and before each lesson upsert; `#[tauri::command(async)]` on `import_course_folder`; `import_cancel.rs` keeps first lesson on mid-import cancel.
- [x] 211 — Cancel control — `LibraryFolderScan` while `scanningFolder`; `Cancel import` calls `cancelImportFolder`; hidden when idle (`Library.test.tsx`).
- [x] 212 — Rescan yields — `rescan_with_cancel` + `#[tauri::command(async)]` on `rescan_course`; same `cancel_import_course_folder` flag; `LibraryFolderScan` **Cancel rescan** on course screen; `rescan_cancel.rs` + `Course.test.tsx`.
- [x] 213 — One import path — `importCancellableFolderAtPath` for Add folder + drop; `validate_import_folder_path` before scan; `libraryCancellableFolderImport.test.ts`.
- [x] 214 — Cancel is not completion — `import_cancel.rs`: cancel after first of two files leaves that lesson `not_started`, not `done` or `broken`.
- [x] 215 — Count — `FolderScanProgress` + `folder_scan_progress` event; library/course scan UI shows real `n of m` (`folderScanProgress.ts`, `import_progress.rs`).
- [x] 216 — One scan at a time — `folder_scan_guard` rejects overlapping import/rescan with `folder_scan_in_progress`; first scan keeps the guard until done (`folder_scan_guard.rs` tests + `libraryImportError.test.ts`).
- [x] 217 — Fonts the bundle will ship
- [x] 218 — Build the AppImage
- [x] 219 — Launch the image
- [x] 220 — Install section
- [x] 221 — Identity of the bundle
- [x] 222 — Empty folder
- [x] 223 — File disappears
- [x] 224 — Media symlink
- [x] 225 — Delete keeps the Anki note
- [x] 226 — OCR stays on the machine
- [x] 227 — Cancel copy
- [x] 228 — Escape is not cancel
- [x] 229 — Cancelled rescan keeps work — `rescan_cancel.rs`: `cancel_rescan_leaves_lesson_order_and_notes_unchanged` (cancel on second scan entry keeps explicit `reorder_lessons` order and lesson notes; no third lesson).
- [x] 230 — Import again after cancel — `import_cancel.rs`: `reimport_after_cancelled_partial_import_finishes_without_duplicating_first_lesson`.
- [x] 231 — Ignore file still applies — `import_ignore.rs`: `import_skips_secret_mp4_and_keeps_lecture` also uses `import_folder_with_cancel`.
- [x] 232 — Risky root — `risky_import_path.rs`: `validate_import_folder_path_guard_blocks_drop_on_risky_root` (`import_folder_with_cancel` on `/` returns `RiskyImportPath` before scan).
- [x] 233 — Done stays manual — `watched.rs`: `union_full_watch_does_not_auto_done`; `import_progress.rs`: `cancellable_import_finish_does_not_mark_lessons_done` (finished `import_folder_with_cancel` does not mark done).
- [x] 234 — Backup still round-trips — `crates/tauri/tests/restore.rs`: `backup_restore_round_trip_preserves_study_days_and_pack_settings` (passes unchanged after cancellable import work).
- [x] 235 — Notes and search — `lesson_note_export.rs`: `markdown_export_reads_vault_notes_only_and_does_not_delete_cards`; `search.rs`: `matches_lesson_note_text`, `search_stays_local_lesson_note_body_returned_and_missing_note_not`.
- [x] 236 — First-course page — [`docs/FIRST-COURSE.md`](FIRST-COURSE.md): **Cancel import** checklist row; AppImage omitted in one line (218 produced no bundle).
- [x] 237 — Deslop this pack — trimmed redundant test asserts, import doc comment, cancellable-import JSDoc, stale CANCEL-SCAN “next step”, verbose Vitest title.
- [x] 238 — Doctor — `npx react-doctor@latest --verbose .` in `ui/`: **64 / 100** (34 issues). No crash, dead Cancel, or missing label on pack-added controls; left complexity / giant-component / dialog / iframe / fetch warnings unchanged.

**Prompt 238 — `npx react-doctor@latest --verbose .` (ui/, 2026-10-28):**

```text
React Doctor — ui
Score: 64 / 100 Needs work
34 issues
Accessibility: 6 warnings
Bugs: 7 warnings
Security: 2 warnings
Maintainability: 16 warnings
Performance: 3 warnings

Share: https://react.doctor/share?p=ui&s=64&w=34&f=16
```

- [x] 239 — README — cancel import + one scan at a time; **Install** from 220 unchanged; no published release / Windows / macOS claims.
- [x] 240 — Scope — [`docs/SCOPE-v1.4.md`](SCOPE-v1.4.md): cancellable import/rescan, scan count, empty-folder refusal, AppImage attempt, profile-in-use sentence; out-of-scope table unchanged on accounts/cloud/kits/signing/auto-done.
- [x] 241 — Checklist — [`docs/V1.4.md`](V1.4.md) from [`SCOPE-v1.4.md`](SCOPE-v1.4.md): 28 pass / 1 unproven (mid-row cancel granularity).
- [x] 242 — Tests — `cargo test --workspace`: **232 passed** (74 suites); `cd ui && npm test`: **175 passed** (55 files); `npm run typecheck`: **pass**.

**Prompt 242 — verification (2026-10-28):**

```bash
cargo test --workspace   # 232 passed (74 suites); 0 failed
cd ui && npm test       # 55 files; 175 passed
npm run typecheck       # ok
```

- [x] 243 — Study doc — [`docs/STUDY.md`](STUDY.md): in-app default, Anki Desktop fallback; no “install Anki before Study” requirement; scheduler wording unchanged.
- [x] 244 — Pin path — [`docs/ANKI_PIN.md`](ANKI_PIN.md) matches `crates/anki-bridge/Cargo.toml` path deps; documents unpublished crates; no vendoring.
- [x] 245 — Command failures — failed import: `Library.test.tsx` — `shows one sentence when import rejects a risky folder path`; cancelled import: `libraryImportError.test.ts` — `maps import_cancelled to one library sentence` (library `role="alert"` via same `libraryImportErrorMessage` path as risky import); locked profile: `AnkiProfileInUse.test.tsx` — `shows the profile lock sentence when study bar state fails`.
- [x] 246 — Continue after relaunch — [`FIRST-COURSE.md`](FIRST-COURSE.md) **Continue** row: resume seek, no auto-play; `LibraryContinue.test.tsx` — `shows course, lesson, and watched percent and opens at resume position`; `Player.test.tsx` — `seeks to initialSeekMs when the lesson loads (search hit navigation)` (`play` not called).
- [x] 247 — No network font in the image — [`docs/FONTS.md`](FONTS.md) § Release bundle scan: no AppImage from 218 (not a pass); `rg` on existing `ui/dist/` → no Google Font hosts.
- [x] 248 — Unsigned, on purpose — [`docs/PACKAGE.md`](PACKAGE.md): unsigned AppImage, `createUpdaterArtifacts: false`, `version: 0.0.0`; no published-artifact claims.
- [x] 249 — Waiting list — [`SCOPE-v1.4.md`](SCOPE-v1.4.md): FlutKit/ProKit icons unused; Google Sans + Andina `.ttf` shipped from kit fonts only.
- [x] 250 — Tests again — `cargo test --workspace`: **232 passed** (74 suites); `cd ui && npm test`: **175 passed** (55 files).

**Prompt 250 — verification (2026-10-28):**

```bash
cargo test --workspace   # 232 passed (74 suites); 0 failed
cd ui && npm test       # 55 files; 175 passed
```

- [x] 251 — Freeze — [`docs/V1.4.md`](V1.4.md) ends **READY** (242 + cancel-import test names); prompt 250 counts pasted; no release tag.

## Sitting and week tools (prompts 252–301)

Repo: `/home/xingzi/Documents/lessonbinder`. Full text: `~/Documents/auravault-cursor-prompts.md`. Commit and push on 252 when the tree is dirty, then on every even prompt through 300, and on 301.

- [x] 252 — Clean tree — **clean** on `origin/master` for v1.4 (`9eecaea`). Local dirty file is unrelated: `docs/PROGRESS.md` adds the prompts **252–301** section (+55 lines)—not committed (do not mix with freeze).
- [x] 253 — What the sitting already proves — `docs/SITTING.md` maps each `FIRST-COURSE.md` checklist row to test names or **human-only**; no product code.
- [x] 254 — One sitting test — `vault_core::first_course_sitting::first_course_sitting` (temp folder: import, resume, note, manual done, rescan; watch percent ≠ done).
- [x] 255 — Checklist matches the test — `docs/SITTING.md` Import, Resume, and A note rows point at `first_course_sitting`; human-only rows unchanged.
- [x] 256 — Do not fake the human rows — `docs/FIRST-COURSE.md` points at `SITTING.md` and says not to check human-only rows pass.
- [x] 257 — patchelf, no install — **`patchelf` missing** (`command -v patchelf` empty, 2026-10-28); left existing `sudo pacman -S patchelf` line in [`docs/PACKAGE.md`](PACKAGE.md); stopped (no install).
- [x] 258 — One bundle attempt — **not run** (`patchelf` missing; bundler not invoked).
- [x] 259 — Launch only a real image — no AppImage from **258**; one sentence in [`docs/FIRST-COURSE.md`](FIRST-COURSE.md) § Boot → [`docs/PACKAGE.md`](PACKAGE.md) (no launch invented).
- [x] 260 — Install section stays true — [`README.md`](../README.md) **Install** matches **258** (bundler not run; **`patchelf`**); still **unsigned**, **0.0.0**.
- [x] 261 — Bundle identity — no AppImage to inspect `.desktop` **`Name=`**; [`src-tauri/tauri.conf.json`](../src-tauri/tauri.conf.json) unchanged: `productName` **Lessonbinder**, `version` **0.0.0**, `createUpdaterArtifacts` **false**.
- [x] 262 — Fonts in that build — **no pass**; no **`ui/dist` from 258** ([`docs/FONTS.md`](FONTS.md) § Prompt 262).
- [x] 263 — Cancel before the row write — `vault_core::import_cancel::cancel_before_lesson_upsert_leaves_no_half_written_row`.
- [x] 264 — Close that row — [`docs/V1.4.md`](V1.4.md) mid-write cancel row **pass** with `cancel_before_lesson_upsert_leaves_no_half_written_row`.
- [x] 265 — Show the course folder — `open_course_folder_in_file_manager` uses **`tauri-plugin-opener`** `OpenerExt::open_path`; `vault_core::course_folder::course_folder_path_for_open` (stored root + risky-path check only).
- [x] 266 — Show folder control — `Course.test.tsx` — `Show folder calls open command with course id and stubbed opener receives course root`; accessible name **Show folder**.
- [x] 267 — Show the lesson file — `lesson_file_directory_for_open` + `open_lesson_file_directory_in_file_manager`; player **Show lesson file**; `lesson_file_directory_open::rejects_lesson_media_that_escapes_course_root_via_symlink`.
- [x] 268 — Opener failure — `opener_open_path_failed` command error; course + player use `invokeErrorMessage`; `Course.test.tsx` / `Player.test.tsx` / `courseFolderOpenError.test.ts`.
- [x] 269 — Shortcut list — `ui/src/keyboard/implementedShortcuts.ts` reads `playerSeekKeyboard`, `playerCompleteKeyboard`, `playerModeKeyboard`, `reviewKeyboard`; `implementedShortcuts.test.ts`.
- [x] 270 — Shortcuts in Settings — `SettingsKeyboardShortcutsList`; `Settings.test.tsx` — `renders keyboard shortcut rows from the implemented list`; `assertImplementedShortcutsMatchKeyModules` in `implementedShortcuts.test.ts`.
- [x] 271 — Last errors — already persisted: **`command-errors.log`** under app data (`lessonbinder_tauri::command_log::COMMAND_LOG_FILE`, `log_command_error`); not stderr-only; nothing uploaded.
- [x] 272 — Twenty lines — `command_log.rs` `COMMAND_LOG_MAX_LINES` (20); `crates/tauri/tests/command_log.rs` — `log_command_error_keeps_last_twenty_newest_lines`; library risky import sentence unchanged (`Library.test.tsx`).
- [x] 273 — Cues that already exist — player lists sidecar cues via `PlayerCueList` (`listLessonCues`); **`PlayerCues.test.tsx`**; cue activate covered in **`Player.test.tsx`** below.
- [x] 274 — Cue seek stays local — **`ui/Player.test.tsx`** — **`clicking a cue seeks local playback and saves resume without opening a URL or marking complete`**.
- [x] 275 — Source offer — [`SOURCE-OFFER.txt`](../SOURCE-OFFER.txt) at repo root (AGPL via `anki` link; source = build commit).
- [x] 276 — No analytics host — `docs/SITTING.md` § Prompt 276 ripgrep audit; removed fixture `example.com` URL + block off-localhost `fetch`; `noPhoneHomeHosts.test.ts`.
- [x] 277 — Show folder only with a course — `LibraryEmpty.test.tsx` — empty library has no **Show folder**; `Library.test.tsx` — `course row list does not show Show folder`; `Course.test.tsx` — `folder course screen shows Show folder`.
- [x] 278 — Show folder waits for the scan — `CourseScreen` disables **Show folder** when `rescanningFolder`; `Course.test.tsx` — cancel rescan test asserts disabled **Show folder** + enabled **Cancel rescan**; `Library.test.tsx` — import in flight has no **Show folder**, **Cancel import** enabled.
- [x] 279 — Complete stays manual — **D → done:** `ui/Player.test.tsx` — **`d key marks complete and exposes the shortcut on the control`**; `ui/playerCompleteKeyboard.test.ts`. **Full watch % → not done:** `vault_core::watched::union_full_watch_does_not_auto_done`; `ui/Player.test.tsx` — **`records watched playback on interval without marking complete`**, **`shows full watched percent without marking complete`**; `vault_core::first_course_sitting::first_course_sitting`. No new auto-`done` path in prompts 252–279.
- [x] 280 — Cancel still wins — `Library.test.tsx` — `cancel folder scan calls command layer while import is in flight`; `Course.test.tsx` — `cancel rescan calls command layer while rescan is in flight` (Show folder / shortcut list unchanged).
- [x] 281 — Deslop
- [x] 282 — Doctor — `npx react-doctor@latest --verbose` on `ui/`: **64/100** (36 issues). No crash, dead Show folder/Shortcuts control, or missing pack control label; complexity warnings left as-is.
- [x] 283 — README — [`README.md`](../README.md) mentions **Show folder**, **Show lesson file**, and Settings **keyboard shortcuts**; **Install** unchanged from **260** (no published AppImage claim).
- [x] 284 — Scope — [`docs/SCOPE-v1.5.md`](SCOPE-v1.5.md): sitting test, AppImage retry docs, cancel-before-upsert, show folder, show lesson file, shortcut list, last-20 errors; out table on accounts/cloud/kits/signing/Windows/macOS/auto-done.
- [x] 285 — Checklist — [`docs/V1.5.md`](V1.5.md) from [`SCOPE-v1.5.md`](SCOPE-v1.5.md); AppImage launch row **unproven** → [`PACKAGE.md`](PACKAGE.md) (259 did not launch); other rows **pass** with test or SITTING.md evidence.
- [x] 286 — Tests — `cargo test --workspace`: **241 passed** (77 suites); `cd ui && npm test`: **192 passed** (59 files); `npm run typecheck`: **pass**.

```bash
cargo test --workspace   # 241 passed (77 suites); 0 failed
cd ui && npm test       # 59 files; 192 passed
npm run typecheck       # tsc -p ui/tsconfig.json --noEmit; pass
```

- [x] 287 — Icons stay waiting — [`SCOPE-v1.5.md`](SCOPE-v1.5.md) out-of-scope + UI kits paragraph: FlutKit/ProKit **icons** unused; only kit files in app are four **Google Sans** + **Andina** `.ttf` under `ui/src/theme/fonts/` (no icons copied).
- [x] 288 — No other platforms — [`README.md`](../README.md) + [`PACKAGE.md`](PACKAGE.md): Linux-only bundling copy; removed macOS/Windows/signing name-drops that read like platform claims.
- [x] 289 — Note and search — `vault_core::lesson_note_export::markdown_export_reads_vault_notes_only_and_does_not_delete_cards`; `vault_core::search::search_stays_local_lesson_note_body_returned_and_missing_note_not` (both pass; no pack regression).
- [x] 290 — Backup — `lessonbinder_tauri::restore::backup_restore_round_trip_preserves_study_days_and_pack_settings` (pass; streak, theme, speed, volume unchanged).
- [x] 291 — Profile lock — `ui/AnkiProfileInUse.test.tsx` — `shows the profile lock sentence when study bar state fails`; sentence matches `anki_bridge` — **Lessonbinder Anki profile is in use (close Anki or wait for another operation to finish)**.
- [x] 292 — Missing file — `vault_core::missing_media_on_resume::missing_file_on_resume_marks_broken_not_done` (pass; **Broken**, resume kept, not **done**).
- [x] 293 — Human rows — [`SITTING.md`](SITTING.md) ends with **Human-only rows (still open)**; OCR, Whisper, and **your course folder** import stay **not pass**.
- [x] 294 — Counts again — `cargo test --workspace`: **241 passed** (77 suites); `cd ui && npm test`: **192 passed** (59 files).

```bash
cargo test --workspace   # 241 passed (77 suites); 0 failed
cd ui && npm test       # 59 files; 192 passed
```

- [x] 295 — Checklist evidence — [`V1.5.md`](V1.5.md) pass rows cite named tests or pack doc lines (**293** SITTING human-only, **264** cancel-before-upsert, exact `LibraryEmpty` test); none demoted to **unproven**.
- [x] 296 — Unsigned — [`PACKAGE.md`](PACKAGE.md) **Bundle identity** + prompt **248** paragraph: **unsigned**, `createUpdaterArtifacts` **false**, `version` **0.0.0**; **not published** from this repo.
- [x] 297 — Typecheck — `npm run typecheck` (`tsc -p ui/tsconfig.json --noEmit`): **pass**.
- [x] 298 — Lint — `npm run lint` (`eslint .`): **pass**; no fixes in pack-touched files.
- [x] 299 — First course page — [`FIRST-COURSE.md`](FIRST-COURSE.md) links [`SITTING.md`](SITTING.md) (intro + Related); **Do not check those rows pass** for human-only steps.
- [x] 300 — Tests before freeze — `cargo test --workspace`: **241 passed** (77 suites); `cd ui && npm test`: **192 passed** (59 files).

```bash
cargo test --workspace   # 241 passed (77 suites); 0 failed
cd ui && npm test       # 59 files; 192 passed
```

- [x] 301 — Freeze — [`V1.5.md`](V1.5.md) ends **READY** (300 counts; `first_course_sitting`, **Show folder**, Settings shortcuts named; AppImage launch **unproven** → [`PACKAGE.md`](PACKAGE.md)); no release tag.

## Pending plan (prompts 302–351)

Repo: `/home/xingzi/Documents/lessonbinder`. Full text: `~/Documents/auravault-cursor-prompts.md`. Sitting in the app, one AppImage try, a few outline icons, and docs for Windows, macOS, and signing. Commit on 302 when dirty, then every even prompt through 350, and on 351.

- [x] 302 — Clean tree — **clean** on `origin/master` for v1.5 freeze (`ebf3b5a`); no empty commit. Local `PROGRESS.md` **302–351** section tracks the pending plan (not the freeze commit).
- [x] 303 — Sitting script — [`docs/SITTING-RUN.md`](SITTING-RUN.md): import, resume, note, subtitle-cue card, review, theme, quit, relaunch, Continue; fixture `/tmp/lessonbinder-sitting`.
- [x] 304 — Fixture — `/tmp/lessonbinder-sitting`: `intro.mp4` + `intro.srt` (two cues), `notes.md`, `extra.mp4` + `.lessonbinderignore` (ffmpeg on PATH); fixture not committed.
- [x] 305 — Import in the app — [`SITTING-RUN.md`](SITTING-RUN.md) **Import** **pass** (sitting profile + fixture; `extra.mp4` ignored).
- [x] 306 — Resume in the app — [`SITTING-RUN.md`](SITTING-RUN.md) **Resume** **pass** (1500 ms resume; not **done**); commit 305–306.
- [x] 307 — Note in the app — [`SITTING-RUN.md`](SITTING-RUN.md) **Note** **pass** (vault persist + timestamp seek via `LessonNotes.test.tsx`).
- [x] 308 — One card in the app — [`SITTING-RUN.md`](SITTING-RUN.md) **One card (subtitle cue)** **pass** (sitting vault link + subtitle draft commit; no OCR/Whisper).
- [x] 309 — Review in the app — [`SITTING-RUN.md`](SITTING-RUN.md) **Review** **pass** (course-scoped reveal + Good; profile-in-use sentence via `AnkiProfileInUse.test.tsx`).
- [x] 310 — Theme in the app — [`SITTING-RUN.md`](SITTING-RUN.md) **Theme** **pass** (dark/light save + contrast sentence); commit 309–310.
- [ ] 311 — Quit and relaunch
- [ ] 312 — Sitting page
- [ ] 313 — patchelf again
- [ ] 314 — Bundle once
- [ ] 315 — Launch the image
- [ ] 316 — Install stays true
- [ ] 317 — Image identity
- [ ] 318 — Fonts in the image build
- [ ] 319 — Which icons
- [ ] 320 — Copy the outlines
- [ ] 321 — Show folder icon
- [ ] 322 — Search icon
- [ ] 323 — Settings icon
- [ ] 324 — Reader and list
- [ ] 325 — Names survive the icons
- [ ] 326 — Icons are local
- [ ] 327 — Other operating systems
- [ ] 328 — Commands for those machines
- [ ] 329 — Linux target stays
- [ ] 330 — Signing stays off
- [ ] 331 — Deslop
- [ ] 332 — Doctor
- [ ] 333 — README
- [ ] 334 — Scope
- [ ] 335 — Checklist
- [ ] 336 — Tests
- [ ] 337 — Cancel still works
- [ ] 338 — Show folder still opens the root
- [ ] 339 — Shortcuts still match
- [ ] 340 — Backup
- [ ] 341 — Note and card boundaries
- [ ] 342 — OCR still local
- [ ] 343 — No extra kit files
- [ ] 344 — Human rows that failed
- [ ] 345 — Typecheck
- [ ] 346 — Lint
- [ ] 347 — Unsigned
- [ ] 348 — Icons paragraph
- [ ] 349 — Checklist evidence
- [ ] 350 — Tests before freeze
- [ ] 351 — Freeze
