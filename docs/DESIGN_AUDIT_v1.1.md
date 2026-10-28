# Design audit v1.1

**Reference:** [`docs/DESIGN.md`](DESIGN.md) (v1 light theme, Operate-mode surfaces).
**Scope:** In-app **Review**, **Settings**, library **search**, lesson **notes**, and **roadmap SVG export** (UI + command path).
**Method:** Static review of implementation files (no visual restyle in this pass).
**Date:** 2026-10-28.
**Remediation:** 2026-10-28 (follow-ups 1–5 below; see **Deferred (product)**).

## Summary

| Area           | Overall | Notes                                                                      |
| -------------- | ------- | -------------------------------------------------------------------------- |
| Review         | Aligned | `--danger` on Again/errors; hint copy shortened (2026-10-28).              |
| Settings       | Aligned | Duplicate in-screen title removed; title from `AppShell` only.             |
| Search         | Aligned | DESIGN tokens + `--radius-sm`; arrow-key navigation on results.            |
| Notes          | Aligned | Seek timestamps use `--text-secondary`; delete confirm deferred (product). |
| Roadmap export | Aligned | Export SVG uses DESIGN hex + IBM Plex Sans stack.                          |

---

## Review

**Job (inferred):** In-app Anki queue—show front, reveal back, grade; optional **Open in Anki Desktop** fallback.

### Files

| Role          | Path                                                                                                                      |
| ------------- | ------------------------------------------------------------------------------------------------------------------------- |
| Screen        | `ui/src/screens/ReviewScreen.tsx`                                                                                         |
| Styles        | `ui/src/theme/review.css`                                                                                                 |
| Commands      | `ui/src/api/reviewCommands.ts`                                                                                            |
| Context       | `ui/src/review/ReviewCommandsContext.tsx`, `ui/src/review/useReviewCommands.ts`, `ui/src/review/reviewCommandsContext.ts` |
| Types         | `ui/src/api/reviewTypes.ts`                                                                                               |
| Shell / route | `ui/src/App.tsx` (route `review`, title **Review** in `AppShell`)                                                         |
| Global chrome | `ui/src/shell/AppShell.tsx`, `ui/src/theme/shell.css` (`.study-button`)                                                   |
| Tests         | `ui/Review.test.tsx`                                                                                                      |
| Backend       | `src-tauri/src/study_commands.rs` (review RPCs; not re-audited here)                                                      |

### Aligned with DESIGN.md

- **Operate tone:** Plain copy (“The review queue is empty.”, load/answer errors without “Oops!”).
- **Typography / spacing:** Uses `--text-sm` / `--text-lg`, `--space-*`, `--radius-sm` in `review.css`.
- **Primary actions:** **Show answer**, empty-state **Back to Library**, and shell **Study** use `--accent` / `.study-button` (32px min-height, `--radius-sm`).
- **Secondary:** **Open in Anki Desktop** uses elevated surface + border (secondary pattern).
- **Motion:** Button transitions only via shell `.study-button` (`--motion-button` 120ms); no forbidden page-load motion.
- **Accessibility:** `role="status"` / `role="alert"` on hints and errors; grade group `aria-label`; card sections labeled Front/Back.
- **Screen title:** Single title via top bar (`App.tsx` → `screenTitle` **Review**); `ReviewScreen` does not add a second `h1`.

### Gaps / risks

| Issue                        | Severity | Detail                                                                                                                   |
| ---------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------ |
| Hard-coded reds for “Again”  | Minor    | `.review-grade--again` and `.review-error` use `#c62828` instead of `--danger` (`#B91C1C` in `ui/src/theme/tokens.css`). |
| Accent on “Good” border only | OK       | Matches “accent for primary actions”; other grades stay neutral—acceptable.                                              |
| No status text on grades     | Minor    | DESIGN asks status not by color alone; buttons are labeled Again/Hard/Good/Easy (text present).                          |
| Lucide icons                 | N/A      | No icons on this screen; DESIGN allows text-only controls.                                                               |
| Desktop hint length          | Minor    | Hint mentions “full desktop experience”—slightly marketing-flavored vs shortest instructional line in DESIGN examples.   |

---

## Settings

**Job (inferred):** Edit vault paths (Tesseract, transcription model, Anki binary), show profile path, backup/restore.

### Files

| Role          | Path                                                                                                                                  |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| Screen        | `ui/src/screens/SettingsScreen.tsx`                                                                                                   |
| Styles        | `ui/src/theme/settings.css`                                                                                                           |
| Commands      | `ui/src/api/settingsCommands.ts`                                                                                                      |
| Context       | `ui/src/settings/SettingsCommandsContext.tsx`, `ui/src/settings/useSettingsCommands.ts`, `ui/src/settings/settingsCommandsContext.ts` |
| Types         | `ui/src/api/settingsTypes.ts`                                                                                                         |
| Shared errors | `ui/src/api/invokeError.ts`                                                                                                           |
| Shell / route | `ui/src/App.tsx` (route `settings`)                                                                                                   |
| Tests         | `ui/Settings.test.tsx`, `ui/src/test/settingsCommandStubs.ts`                                                                         |
| Backend       | `src-tauri/src/settings_commands.rs`, `src-tauri/src/backup_commands.rs`                                                              |

### Aligned with DESIGN.md

- **Form layout:** Narrow column (`max-width: 36rem`), labels `--text-sm` secondary, inputs `--bg-elevated`, `--border`, `--radius-sm`.
- **Copy:** Sentence case, instructional hints (“Transcription is disabled.”), restore confirm states consequence plainly.
- **Buttons:** **Save** and destructive confirm **Restore backup** use `.study-button` (primary); **Backup** / **Restore** / **Cancel** use `.library-button-secondary`.
- **Danger:** Errors use `.library-error` → `--danger` in `library.css`.
- **Tabular path:** Anki profile path uses `font-variant-numeric: tabular-nums` in `settings.css`.

### Gaps / risks

| Issue                           | Severity | Detail                                                                                                                                                                                        |
| ------------------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Duplicate **Settings** title    | Minor    | `AppShell` top bar (`ui/src/App.tsx`) and `SettingsScreen` `h1.screen-title` both show **Settings** (`ui/src/screens/SettingsScreen.tsx`). DESIGN: one screen title per view (global chrome). |
| Primary button count on restore | OK       | Confirm dialog: secondary Cancel + primary Restore—matches destructive confirm pattern.                                                                                                       |
| No Lucide                       | N/A      | Text-only controls.                                                                                                                                                                           |

---

## Library search

**Job (inferred):** Find lessons by title, path, or subtitle text; open lesson (and cue time when applicable).

### Files

| Role                  | Path                                                                                   |
| --------------------- | -------------------------------------------------------------------------------------- |
| Search field (parent) | `ui/src/screens/LibraryScreen.tsx` (`library-search` block)                            |
| Results list          | `ui/src/library/LibrarySearchResults.tsx`                                              |
| Debounce              | `ui/src/library/useDebouncedValue.ts` (`LIBRARY_SEARCH_DEBOUNCE_MS` in results module) |
| Styles                | `ui/src/theme/library.css` (`.library-search*`)                                        |
| Commands              | `ui/src/api/libraryCommands.ts` (`searchLibrary`)                                      |
| Types                 | `ui/src/api/librarySearchTypes.ts`                                                     |
| Shell integration     | Opens lesson via `ui/src/App.tsx` `handleOpenLessonFromSearch`                         |
| Tests                 | `ui/LibrarySearch.test.tsx`                                                            |
| Backend               | `src-tauri/src/library_commands.rs` → `vault-core` `search.rs`                         |

### Aligned with DESIGN.md

- **Copy:** Label **Search library**, placeholder describes searchable fields; empty **No matches.**—short, no exclamation.
- **Density:** Results as stacked rows, secondary metadata (`course_title`, `snippet`) in `--text-sm` muted colors.
- **No marketing / hero / emoji / illustrations** in search UI.
- **Operate:** Search sits on library table view (not a separate marketing page).

### Gaps / risks

| Issue                        | Severity  | Detail                                                                                                                                                                                                                                                                                                                                  |
| ---------------------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Undefined CSS variables      | **Major** | `.library-search__input` / results use `--border-subtle`, `--radius-md`, `--surface-raised`, `--surface-hover` in `ui/src/theme/library.css` but these are **not** declared in `ui/src/theme/tokens.css` (only `--border`, `--radius-sm`, `--bg-elevated`, etc.). Browsers treat invalid vars as unset—visual drift from DESIGN tokens. |
| Radius                       | Minor     | DESIGN: `--radius-sm` (4px) on inputs; search uses `--radius-md` (undefined).                                                                                                                                                                                                                                                           |
| Keyboard navigation          | Minor     | DESIGN: lists navigable with arrows; course table uses `ui/src/keyboard/tableListNavigation.ts`; **search results are buttons in a `<ul>` without arrow-key handler**.                                                                                                                                                                  |
| Hit title `font-weight: 600` | OK        | Section-level emphasis within result row; not accent color on headings.                                                                                                                                                                                                                                                                 |
| Lucide                       | N/A       | No search icon in field.                                                                                                                                                                                                                                                                                                                |

---

## Lesson notes

**Job (inferred):** Per-lesson scratch notes (vault DB), optional timestamp link during playback.

### Files

| Role         | Path                                                                         |
| ------------ | ---------------------------------------------------------------------------- |
| Component    | `ui/src/lesson/LessonNotesColumn.tsx`                                        |
| Styles       | `ui/src/theme/course.css` (`.lesson-notes*`)                                 |
| Commands     | `ui/src/api/courseCommands.ts` (list/create/update/delete lesson notes)      |
| Host screens | `ui/src/screens/PlayerScreen.tsx`, `ui/src/screens/DocumentLessonScreen.tsx` |
| Tests        | `ui/LessonNotes.test.tsx`, `ui/src/test/lessonNoteCommandStubs.ts`           |
| Backend      | `src-tauri/src/course_commands.rs` → `vault-core` `lesson_note.rs`           |

### Aligned with DESIGN.md

- **Operate panel:** Narrow column (`flex: 0 0 14rem`), border `--border`, `--bg-elevated`, compact padding.
- **Typography:** Title `--text-md` weight 500; body `--text-sm`; timestamps `--text-xs` with tabular nums.
- **Buttons:** **Add note** / **Save note** follow secondary/primary library button classes; **Delete** is secondary (not accent—destructive confirm not implemented inline).
- **Empty:** **No notes yet.**—single line, no mascot; **Add note** always visible in header (action available).
- **Prose:** `white-space: pre-wrap`; no artificial 72ch cap in column (DESIGN allows full pane width in side panels).

### Gaps / risks

| Issue                  | Severity | Detail                                                                                                                                                               |
| ---------------------- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Accent on seek links   | Minor    | `.lesson-notes__timestamp` uses `color: var(--accent)` for underline links. DESIGN limits accent to progress + **primary buttons**; this is decorative link styling. |
| Empty state pattern    | Minor    | DESIGN empty states often pair one sentence + one button; here empty text + header **Add note** (acceptable Operate pattern).                                        |
| Delete without confirm | Product  | Not a visual DESIGN violation; irreversible delete uses secondary button only (no `--danger` until a confirm step exists).                                           |

---

## Roadmap SVG export (path)

**Job (inferred):** Export the **vector** course graph (not a raster roadmap image)—DESIGN allows SVG flowchart; export is a file save of that graph.

### Files (end-to-end)

| Layer                       | Path                                                                                                                               |
| --------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| UI trigger                  | `ui/src/screens/RoadmapScreen.tsx` — `handleExportSvg`, button **Export SVG** (`library-button-secondary`, disabled when no steps) |
| Command port                | `ui/src/api/roadmapCommands.ts` — `exportRoadmapGraphSvg` builds SVG string, invokes Tauri                                         |
| SVG builder (export)        | `ui/src/roadmap/roadmapGraphSvg.ts` — `buildRoadmapGraphSvg`                                                                       |
| On-screen graph (reference) | `ui/src/roadmap/RoadmapGraph.tsx`, `ui/src/theme/roadmap.css`                                                                      |
| Tauri command               | `src-tauri/src/roadmap_commands.rs` — `export_roadmap_graph_svg` (save dialog, write file)                                         |
| Registration                | `src-tauri/src/lib.rs` — `export_roadmap_graph_svg` in invoke handler                                                              |
| Tests                       | `ui/roadmapGraphSvg.test.ts`, roadmap UI tests under `ui/Roadmap*.test.tsx`                                                        |

### Aligned with DESIGN.md

- **No raster export:** SVG only; matches “Canvas/SVG flowchart” and “No raster roadmap image.”
- **UI control:** Secondary **Export SVG** in roadmap header meta row—not accent-filled decoration.
- **Flow:** User picks path via native save dialog; cancel is silent success (no error toast).

### Gaps / risks

| Issue                       | Severity  | Detail                                                                                                                                                                                                                                                                                                                                                     |
| --------------------------- | --------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Export palette ≠ app tokens | **Major** | `roadmapGraphSvg.ts` hard-codes Tailwind-like slate hex (`#f8fafc`, `#cbd5e1`, `#0f172a`, `#64748b`, `#334155`) and `font-family="system-ui, sans-serif"`. In-app graph uses `ui/src/theme/roadmap.css` + DESIGN tokens (`--bg-surface`, `--text-primary`, status classes). **Exported files will not match on-screen stone/teal world or IBM Plex Sans.** |
| Status in export            | Minor     | Export text uses `formatStatus()` words; on-screen graph also uses status **dot** classes—export is text-only (acceptable if status words present).                                                                                                                                                                                                        |
| Duplicate graph logic       | Minor     | Layout constants duplicated between `RoadmapGraph.tsx` and `roadmapGraphSvg.ts` (maintenance drift risk).                                                                                                                                                                                                                                                  |
| Roadmap screen tokens       | Minor     | `roadmap.css` also references undefined `--border-subtle`, `--radius-md`, `--surface-sunken` (same class of issue as search).                                                                                                                                                                                                                              |

---

## Cross-cutting (scoped surfaces)

| Topic                    | Finding                                                                                 | Paths                                                   |
| ------------------------ | --------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| Theme tokens             | Canonical literals in `ui/src/theme/tokens.css`; imported via `ui/src/theme/theme.css`. |                                                         |
| IBM Plex Sans            | Loaded in `tokens.css`; body uses `--font-family`. Export SVG does **not**.             | `tokens.css`, `roadmapGraphSvg.ts`                      |
| Lucide                   | Not used on audited surfaces.                                                           | —                                                       |
| Global chrome            | 48px top bar, due counts tabular, **Study** primary—matches DESIGN.                     | `ui/src/shell/AppShell.tsx`, `ui/src/theme/shell.css`   |
| `prefers-reduced-motion` | Global rule in `ui/src/theme/base.css`.                                                 |                                                         |
| Contrast tests           | Automated checks exist.                                                                 | `ui/theme.contrast.test.ts`, `ui/src/theme/contrast.ts` |

---

## Recommended follow-ups

| #   | Item                                                                                                                                                                            | Status    |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------- |
| 1   | Replace undefined CSS aliases with DESIGN tokens in `ui/src/theme/library.css` (search + continue row), `ui/src/theme/roadmap.css`, `ui/src/theme/course.css` (section heading) | **Fixed** |
| 2   | Roadmap export SVG uses DESIGN hex + IBM Plex Sans in `ui/src/roadmap/roadmapGraphSvg.ts`                                                                                       | **Fixed** |
| 3   | Review `.review-error` / `.review-grade--again` use `--danger` in `ui/src/theme/review.css`                                                                                     | **Fixed** |
| 4   | Remove duplicate `h1.screen-title` where it matched shell title: `SettingsScreen`, `LibraryScreen` (all branches), `RoadmapScreen` (empty state)                                | **Fixed** |
| 5   | Arrow-key navigation on search results in `ui/src/library/LibrarySearchResults.tsx`                                                                                             | **Fixed** |

**Also fixed (minor gaps in scope):** Lesson note seek timestamps use `--text-secondary` (`ui/src/theme/course.css`). Review Anki fallback hint shortened (`ui/src/screens/ReviewScreen.tsx`).

## Deferred (product / out of fix pass)

| Item                                                                                             | Note                                                                                                               |
| ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| Lesson note **Delete** without confirm                                                           | Product safety; needs confirm dialog or undo—not a token/copy fix.                                                 |
| Roadmap **detail** still uses in-content `h1` with roadmap title while top bar shows **Roadmap** | Showing the active roadmap name in `AppShell` (`ui/src/App.tsx`) would be a product/navigation change; left as-is. |
| Duplicate layout between `RoadmapGraph.tsx` and `roadmapGraphSvg.ts`                             | Maintenance refactor only; export now matches token colors.                                                        |
| Lucide icons on audited surfaces                                                                 | Not required when controls are text-only.                                                                          |

---

## Out of scope (this audit)

- Library first-run empty state, Course, Player, Capture (other DESIGN screens).
- Dark theme (explicitly out of v1 in DESIGN.md).
- Runtime visual screenshots or a11y tooling scores.
