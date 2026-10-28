# AuraVault prompt progress

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

- [ ] 141 — Write the map
- [ ] 142 — Words a person sees
- [ ] 143 — Packages and the binary
- [ ] 144 — Anki deck and note type
- [ ] 145 — Ignore file and playlist scheme
- [ ] 146 — Docs inside the repo
- [ ] 147 — Planning copies in Documents
- [ ] 148 — Leftovers and tests
- [ ] 149 — Folder and GitHub
- [ ] 150 — Name is on the window
