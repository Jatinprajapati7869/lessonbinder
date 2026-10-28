# Design audit (prompt 44)

**Reference:** `docs/DESIGN.md`
**Scope:** UI built to date (`ui/src/screens/*`, `ui/src/shell/*`, `ui/src/theme/*`, `ui/src/App.tsx`, roadmap/capture components).
**Method:** Static review against DESIGN.md (no restyle, no browser run). Impeccable `context` launcher was unavailable in this environment.

---

## Audit health (estimated)

| #         | Dimension                | Score     | Key finding                                                                                      |
| --------- | ------------------------ | --------- | ------------------------------------------------------------------------------------------------ |
| 1         | Accessibility            | 2         | Keyboard list navigation and capture region selection gaps; Study control inert when enabled     |
| 2         | Performance              | 3         | No obvious layout thrash; video frame capture is on-demand                                       |
| 3         | Theming                  | 3         | Tokens centralized; literal px in roadmap SVG typography; translucent `color-mix` overlays       |
| 4         | Responsive               | 2         | Capture drag has no `pointercancel` / lost-capture handling; 36px controls vs 44px hybrid target |
| 5         | Implementation integrity | 2         | Several screen specs (player chrome, global chrome, components) partially implemented            |
| **Total** |                          | **12/20** | **Acceptable — significant alignment work**                                                      |

**Severity counts:** P0: 1 · P1: 8 · P2: 9 · P3: 4

---

## P0 — blocking

### Study in Anki has no action when enabled

- **Location:** `ui/src/shell/AppShell.tsx` (Study button ~L44–52)
- **Category:** Implementation integrity / Accessibility
- **Evidence:** `type="button"` with `disabled={studyDisabled}` but no `onClick` (or equivalent) to call launch/study flow described in `docs/ARCHITECTURE.md`.
- **Impact:** When Anki is available, the primary global action cannot be activated from the UI.
- **Recommendation:** Wire to the existing Tauri launch RPC and preserve disabled + status copy when unavailable.

---

## P1 — major (release / WCAG / DESIGN screen jobs)

### Global top bar layout does not match chrome spec

- **Location:** `ui/src/shell/AppShell.tsx` (~L28–56), `ui/src/theme/shell.css` (`.top-bar` ~L8–17)
- **Category:** Implementation integrity
- **Evidence:** DESIGN.md Global chrome: screen title (left), due counts + Study (right), fixed **48px** bar. Implementation places due counts on the left, Study on the right, and omits a screen title; `.top-bar` uses padding only (no 48px height).
- **Impact:** Persistent chrome diverges from the design contract; title lives only in `main-pane` per screen.

### Player control bar missing scrubber, time, and watched progress

- **Location:** `ui/src/screens/PlayerScreen.tsx` (~L222–245), `ui/src/theme/course.css` (`.player-bar` ~L121–137)
- **Category:** Implementation integrity
- **Evidence:** DESIGN.md Player layout: “time, scrubber (accent fill for watched portion), play/pause…”. Implementation shows play/pause, static “Resume at …” from stored resume, and action buttons—no scrubber, no current playback time, no watched-% bar in chrome.
- **Impact:** Player screen does not fulfill its documented job for progress feedback during playback.

### Course empty state missing single recovery action

- **Location:** `ui/src/screens/CourseScreen.tsx` (~L148–154)
- **Category:** Implementation integrity
- **Evidence:** DESIGN.md Course empty state specifies copy plus **single action: Rescan folder**. Block has two hint lines only; no button (contrast: `rescanCourse` exists on `CourseCommandPort` and is used from `PlayerScreen.tsx` ~L35–38).
- **Impact:** User with an empty course cannot rescan from the course screen.

### Library empty state offers two primary actions

- **Location:** `ui/src/screens/LibraryScreen.tsx` (~L71–94)
- **Category:** Implementation integrity
- **Evidence:** DESIGN.md Library empty state: **single action** “Import folder”. Empty UI also renders “Add playlist” (`library-button-secondary` ~L86–93).
- **Impact:** Violates empty-state pattern; secondary import path is valid product-wise but not per v1 DESIGN empty-state rule.

### List keyboard model incomplete (arrows / Enter / Esc)

- **Location:** `ui/src/App.tsx` (navigation ~L140–183); `ui/src/screens/LibraryScreen.tsx` (course links ~L141–147); `ui/src/screens/CourseScreen.tsx` (lesson open ~L192–199)
- **Category:** Accessibility
- **Evidence:** DESIGN.md Global chrome Keyboard: “lists navigable with arrows; Enter opens lesson; Esc backs one level.” No arrow-key handlers on tables; `Escape` closes capture (`CaptureCardPanel.tsx` ~L104–114) but does not back from course/player at app level.
- **Impact:** Keyboard-first study workflow is partial outside player space/capture.

### Capture region selection is pointer-only

- **Location:** `ui/src/screens/CaptureCardPanel.tsx` (~L244–251), `ui/src/theme/capture.css` (`.capture-frame` ~L46–56)
- **Category:** Accessibility
- **Evidence:** Selection uses `onPointerDown` / `Move` / `Up` on a non-focusable `div`; DESIGN.md Card capture empty state allows “Select region (or focus canvas)”. No `tabIndex`, no keyboard alternative, no “Select region” control.
- **Impact:** Capture flow is not fully keyboard-operable.

### Capture dialog does not trap focus

- **Location:** `ui/src/screens/CaptureCardPanel.tsx` (~L200–219, dialog ~L206–211)
- **Category:** Accessibility
- **Evidence:** `role="dialog"` / `aria-modal="true"` with initial focus on Cancel (~L104–105) but no focus trap; Tab can move focus to player controls behind the dialog.
- **Impact:** Modal semantics do not match focus behavior (WCAG 2.4.3).

### Video element has no accessible name

- **Location:** `ui/src/screens/PlayerScreen.tsx` (~L213–221)
- **Category:** Accessibility
- **Evidence:** `<video controls={false}>` without `aria-label` / `title` tied to lesson title.
- **Impact:** Screen readers get an unnamed media element.

---

## P2 — minor

### Status presentation is text-only (no status pill dot)

- **Location:** `ui/src/theme/library.css` (`.library-status*` ~L104–118); `ui/src/screens/LibraryScreen.tsx` (~L151–153); `ui/src/screens/CourseScreen.tsx` (~L208–213)
- **Category:** Implementation integrity / Accessibility
- **Evidence:** DESIGN.md Components **Status pill**: “dot + word”. Implementation uses colored text spans only.
- **Impact:** Status relies more heavily on color (DESIGN also requires text labels—which are present).

### Data table row hover wash not implemented

- **Location:** `ui/src/theme/library.css` (`.library-table` ~L62–74)
- **Category:** Implementation integrity
- **Evidence:** DESIGN.md Data table: row hover `--accent-subtle` at 50% opacity. No `:hover` rule on `tr` / `td`.

### Translucent overlays via `color-mix`

- **Location:** `ui/src/theme/capture.css` (~L74, ~L82); `ui/src/theme/roadmap.css` (~L188)
- **Category:** Theming / Implementation integrity
- **Evidence:** DESIGN.md: “No translucent ‘glass’ panels.” `color-mix(..., transparent)` used for frame hint and selection wash and roadmap step hover.

### Roadmap SVG typography uses literal px, not type tokens

- **Location:** `ui/src/theme/roadmap.css` (~L75–89, ~L104–106)
- **Category:** Theming
- **Evidence:** `font-size: 12px | 11px | 10px | 8px` instead of `--text-xs` / scale tokens from DESIGN.md Type scale.

### Theme split across imports (checklist item open)

- **Location:** `ui/src/index.css` (~L1–7); `docs/DESIGN.md` Implementation checklist (~L238)
- **Category:** Implementation integrity
- **Evidence:** Checklist expects “CSS variables … in one `theme.css`”; repo uses `tokens.css` + per-surface stylesheets.

### Lucide iconography not used

- **Location:** `ui/package.json` (no `lucide-react`); no imports under `ui/src/`
- **Category:** Implementation integrity
- **Evidence:** DESIGN.md Iconography specifies Lucide SVGs for toolbar/inline icons.

### Toast component not implemented

- **Location:** `docs/DESIGN.md` Components table (~L212); no toast styles/components under `ui/src/`
- **Category:** Implementation integrity
- **Evidence:** Spec defines bottom-left toast; no matching UI.

### Order proposal lists may duplicate React keys

- **Location:** `ui/src/screens/OrderProposalPanel.tsx` (~L23–33)
- **Category:** Implementation integrity
- **Evidence:** `key={\`current-${title}\`}`/`proposed-${title}` collide when titles repeat.

### Capture drag lacks cancel / lost-capture cleanup

- **Location:** `ui/src/screens/CaptureCardPanel.tsx` (~L115–158)
- **Category:** Responsive
- **Evidence:** No `onPointerCancel` / `lostpointercapture` handler; drag state may stick after interrupted gestures (audit touch-interaction criteria).

### Roadmap empty-state error lacks alert semantics

- **Location:** `ui/src/screens/RoadmapScreen.tsx` (~L177)
- **Category:** Accessibility
- **Evidence:** Error rendered as `<p className="library-error">` without `role="alert"` (other screens use `role="alert"` e.g. `LibraryScreen.tsx` ~L96).

### `not_started` status color deviates from token rule

- **Location:** `ui/src/theme/library.css` (~L116–118)
- **Category:** Theming
- **Evidence:** DESIGN.md status colors: `not_started` uses `--text-muted`; CSS uses `--text-secondary` (comment cites contrast on `--bg-app`).

---

## P3 — polish

### Primary / secondary control height vs button spec

- **Location:** `ui/src/theme/shell.css` (`.study-button` ~L53–55); `ui/src/theme/library.css` (`.library-button-*` ~L15–17, ~L37–39)
- **Category:** Implementation integrity
- **Evidence:** DESIGN.md Button primary height **32px**; CSS uses `min-height: 36px` (DESIGN list row min 36px / 40px hybrid).

### Course progress bar missing accessible name

- **Location:** `ui/src/screens/CourseScreen.tsx` (~L123–134)
- **Category:** Accessibility
- **Evidence:** `role="progressbar"` with value min/max but no `aria-label` / `aria-labelledby`.

### Duplicate list keys in order proposal (data)

- **Same as P2** — listed once above.

---

## Positive findings (file-backed)

- Design tokens live in `ui/src/theme/tokens.css` with parity tests in `ui/theme.test.ts` and `ui/src/theme/tokens.ts`.
- Body-text contrast pairs are guarded in `ui/theme.contrast.test.ts` and `ui/src/theme/contrast.ts`.
- Global `:focus-visible` uses tokenized ring in `ui/src/theme/base.css` (~L32–35); accent-filled buttons add a visible outer ring in `ui/src/theme/shell.css` (~L77–83).
- `prefers-reduced-motion` block matches DESIGN.md in `ui/src/theme/base.css` (~L37–45).
- Player space play/pause and capture Escape + focus return: `ui/src/screens/PlayerScreen.tsx`, `ui/src/screens/CaptureCardPanel.tsx` (covered by `ui/Player.test.tsx`, `ui/CaptureCard.test.tsx`).
- Roadmap graph nodes are keyboard-activatable: `ui/src/roadmap/RoadmapGraph.tsx` (~L132–138).
- Shell styles avoid raw hex outside tokens: `ui/theme.test.ts` (~L55–59).

---

## Recommended commands (priority)

1. **P0 — `/impeccable harden`:** Wire `AppShell` Study action and error paths (`ui/src/shell/AppShell.tsx`).
2. **P1 — `/impeccable layout`:** Player bar scrubber/time/watched chrome (`PlayerScreen.tsx`, `course.css`).
3. **P1 — `/impeccable onboard`:** Empty states — course rescan, library single-action pattern (`CourseScreen.tsx`, `LibraryScreen.tsx`).
4. **P1 — `/impeccable adapt`:** Capture keyboard region + focus trap (`CaptureCardPanel.tsx`).
5. **P1 — `/impeccable clarify`:** Global Esc back navigation (`App.tsx`) and list arrow behavior.
6. **P2 — `/impeccable document` or `/impeccable extract`:** Status pill, toast, Lucide — close checklist gaps vs `DESIGN.md`.
7. **Final — `/impeccable polish`:** After fixes, pass for token/typography nits (`roadmap.css`, `color-mix` overlays).

Re-run `/impeccable audit` after substantive fixes to refresh scores.
