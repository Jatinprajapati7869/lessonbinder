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
- [ ] 213 — One import path
- [ ] 214 — Cancel is not completion
- [ ] 215 — Count
- [ ] 216 — One scan at a time
- [ ] 217 — Fonts the bundle will ship
- [ ] 218 — Build the AppImage
- [ ] 219 — Launch the image
- [ ] 220 — Install section
- [ ] 221 — Identity of the bundle
- [ ] 222 — Empty folder
- [ ] 223 — File disappears
- [ ] 224 — Media symlink
- [ ] 225 — Delete keeps the Anki note
- [ ] 226 — OCR stays on the machine
- [ ] 227 — Cancel copy
- [ ] 228 — Escape is not cancel
- [ ] 229 — Cancelled rescan keeps work
- [ ] 230 — Import again after cancel
- [ ] 231 — Ignore file still applies
- [ ] 232 — Risky root
- [ ] 233 — Done stays manual
- [ ] 234 — Backup still round-trips
- [ ] 235 — Notes and search
- [ ] 236 — First-course page
- [ ] 237 — Deslop
- [ ] 238 — Doctor
- [ ] 239 — README
- [ ] 240 — Scope
- [ ] 241 — Checklist
- [ ] 242 — Tests
- [ ] 243 — Study doc
- [ ] 244 — Pin path
- [ ] 245 — Command failures
- [ ] 246 — Continue after relaunch
- [ ] 247 — No network font in the image
- [ ] 248 — Unsigned, on purpose
- [ ] 249 — Waiting list
- [ ] 250 — Tests again
- [ ] 251 — Freeze
