# AuraVault design system (v1)

**Product:** Desktop study vault for **long local courses**—library, honest progress, roadmaps, offline playback, Anki capture. **Not** a marketing site.

**Stack:** React + TypeScript in Tauri (`auravault-ARCHITECTURE.md`). Implement UI from this document only; do not invent a second aesthetic in later prompts.

**Rejected defaults (do not use):** purple gradients, glassmorphism, hero sections, three-column feature grids, testimonials, emoji icons, Inter/Roboto as the voice, cream+terracotta “AI landing” palettes, mint-tinted app backgrounds, playful kids’ education fonts (ui-ux-pro-max “Hero + Testimonials” output is explicitly out of scope).

---

## Design intent

AuraVault should feel like **a well-made reference app**: calm surfaces, tight rows, legible type at small sizes, color only where progress and primary actions need attention. The subject matter is **hours of course material**—the UI stays out of the way and reports state clearly (`not_started`, `in_progress`, `done`, `broken`).

**One bold choice:** a single **teal** accent on an otherwise **stone-neutral** canvas—evokes “continue / on track” without startup-purple or achievement-orange gamification.

---

## Typography

**One family everywhere:** **[IBM Plex Sans](https://fonts.google.com/specimen/IBM+Sans)** (weights 400, 500, 600 only).

- Why: dense UI readability, distinct from Inter/Roboto/system UI, serious study-tool tone, strong at 12–14px table text.
- No separate display face. Headings use **weight + size**, not a second typeface.
- **Tabular figures** (`font-feature-settings: "tss" on` or `font-variant-numeric: tabular-nums`) for durations, due counts, percentages.

### Type scale (rem, base 16px)

| Token         | Size             | Weight | Use                                   |
| ------------- | ---------------- | ------ | ------------------------------------- |
| `--text-xs`   | 0.75rem (12px)   | 400    | Metadata, timestamps, table secondary |
| `--text-sm`   | 0.8125rem (13px) | 400    | Default list rows, lesson titles      |
| `--text-base` | 0.875rem (14px)  | 400    | Body, player chrome labels            |
| `--text-md`   | 1rem (16px)      | 500    | Section titles inside a screen        |
| `--text-lg`   | 1.125rem (18px)  | 600    | Screen title (one per view)           |

- **Line-height:** 1.45 for body/list; 1.25 for single-line rows.
- **Max line length** for prose notes: ~72ch; lists and tables use full pane width.
- **Case:** sentence case everywhere. No all-caps labels. No eyebrow kicker lines above titles.

### Font loading (webview)

```css
@import url("https://fonts.googleapis.com/css2?family=IBM+Plex+Sans:wght@400;500;600&display=swap");

:root {
  font-family: "IBM Plex Sans", system-ui, sans-serif;
}
```

---

## Color tokens

Neutral surfaces; **one hue** (`--accent`) for **progress indicators** and **primary buttons** only. Destructive actions use `--danger` (not accent).

### Light theme (v1 default)

| Token              | Hex       | Role                                                                |
| ------------------ | --------- | ------------------------------------------------------------------- |
| `--bg-app`         | `#F4F4F3` | App chrome background                                               |
| `--bg-surface`     | `#FAFAF9` | Panels, sidebars                                                    |
| `--bg-elevated`    | `#FFFFFF` | Active row, popovers, player control bar                            |
| `--border`         | `#E5E4E2` | Dividers, table rules, input outlines                               |
| `--border-strong`  | `#D6D4D1` | Focused control outline (with ring)                                 |
| `--text-primary`   | `#1C1917` | Primary text                                                        |
| `--text-secondary` | `#57534E` | Secondary labels, paths                                             |
| `--text-muted`     | `#78716C` | Disabled, hints                                                     |
| `--accent`         | `#0D6E6A` | **Only:** progress fill, primary button bg, active lesson indicator |
| `--accent-hover`   | `#0A5C58` | Primary button hover                                                |
| `--accent-subtle`  | `#E6F2F1` | Progress track background, selected row wash (low saturation)       |
| `--on-accent`      | `#FFFFFF` | Text on primary button                                              |
| `--progress-track` | `#E7E5E4` | Unfilled progress bar                                               |
| `--status-broken`  | `#B45309` | `broken` lesson/course (amber, not red panic)                       |
| `--status-done`    | `#44403C` | `done` checkmarks (neutral, not green confetti)                     |
| `--danger`         | `#B91C1C` | Irreversible delete confirm only                                    |
| `--focus-ring`     | `#0D6E6A` | 2px outline offset 2px                                              |

**Rules**

- Do **not** use `--accent` for decorative icons, headings, or borders.
- Status colors: `in_progress` uses `--accent`; `not_started` uses `--text-muted`; `done` / `broken` as above.
- No gradients. No translucent “glass” panels. Shadows: **none** by default; optional `0 1px 0 var(--border)` on elevated bar only.

Dark theme is **out of v1** unless a later prompt adds paired tokens; do not ship half a dark mode.

---

## Spacing scale

**Base unit:** 4px. Dense dashboard rhythm (`--density` high).

| Token       | px  |
| ----------- | --- |
| `--space-1` | 4   |
| `--space-2` | 8   |
| `--space-3` | 12  |
| `--space-4` | 16  |
| `--space-5` | 24  |
| `--space-6` | 32  |

**Layout**

- App padding: `--space-4` horizontal; `--space-3` vertical in lists.
- List row height: **36px** min (40px if touch target needed on hybrid builds).
- Split panes: **library list | detail** with a **1px** `--border` divider; no card grid.
- Player: video **maximizes**; controls in a fixed **40px** bar (`--bg-elevated`).

**Radius:** `--radius-sm: 4px` on inputs and buttons only; tables and panes are **square**.

---

## Motion

**Rule: almost none.**

| Allowed                                          | Duration             | Notes                                           |
| ------------------------------------------------ | -------------------- | ----------------------------------------------- |
| User-triggered: button press, toggle, panel open | **120ms** `ease-out` | Opacity or background only—no slide-up sections |
| Progress bar width update                        | **80ms** linear      | Tied to playback position                       |
| Focus ring                                       | instant              | Never animate focus away                        |

**Forbidden:** page-load fades, staggered list reveals, parallax, hover scale on rows, GSAP scroll triggers, carousel auto-rotate.

```css
@media (prefers-reduced-motion: reduce) {
  * {
    animation-duration: 0.01ms !important;
    transition-duration: 0.01ms !important;
  }
}
```

---

## Iconography

- **Lucide** SVG icons, 16px inline / 20px toolbar, stroke 1.5, color `currentColor` (usually `--text-secondary`).
- No emoji. No illustrated empty-state mascots.

---

## Global chrome

- **Top bar (fixed, 48px):** screen title (left), global due count + **Study** primary button (right). Due count is numeric tabular; **Study** uses `--accent` fill.
- **No** global marketing nav, no hero, no footer links.
- **Keyboard:** visible focus rings; lists navigable with arrows; Enter opens lesson; Esc backs one level.

---

## Screens (one job each)

### Library

**Job:** Show every imported **course** with derived status and let the user **add a folder** or open one course.

**Layout:** Single primary table (title, lesson count, derived status, last engaged). Optional narrow sidebar for filters later—v1 table only.

**Empty state:** One sentence: “Import a folder on your disk or a playlist JSON file to add courses.” **Two actions:** **Add folder** (primary), **Add playlist JSON** (secondary). No illustration, sample course, or onboarding copy.

---

### Course

**Job:** Show this course’s **ordered lesson list**, per-lesson progress and resume, and entry to **Player** or reorder.

**Layout:** Header: course title + derived status + thin course progress bar (`--accent` on `--progress-track`). Lesson table: order #, title, duration, watched %, status, resume hint.

**Empty state:** “This course has no lessons.” “Check that the folder contains supported files (mp4, mkv, pdf, …).” **Single action:** **Rescan folder**.

---

### Player

**Job:** Play local media with **resume**, show **watched %**, and let the user **mark complete** or **capture a card**.

**Layout:** Video (or audio + poster) dominant; bottom bar: time, scrubber (accent fill for watched portion), play/pause, **Mark complete**, **Capture card**. No theater-mode marketing chrome.

**Empty state:** (edge) “No playable file for this lesson.” “The file may have moved or been deleted.” **Single action:** **Mark as broken** / **Rescan course** (whichever matches domain state).

---

### Roadmap

**Job:** Display the **course graph** (nodes + directed edges) and each node’s **derived** status from its course.

**Layout:** Canvas/SVG flowchart; nodes = courses (label + status dot); edges = arrows. Side panel lists steps when a node is selected. No raster “roadmap image.”

**Empty state:** “No roadmap yet.” “Connect courses in the order you want to study them.” **Single action:** **Create roadmap**.

---

### Card capture

**Job:** Select a **region on the current frame**, run **local OCR**, preview text, and **save an Anki note** on the AuraVault profile.

**Layout:** Frozen frame + draggable selection rectangle; right column: OCR text (editable), lesson/course context, timestamp. Primary **Save card**; secondary Cancel.

**Empty state:** “No frame selected.” “Pause the video and drag to select text on screen.” **Single action:** **Select region** (or focus canvas).

---

## Components (minimal set)

| Component            | Spec                                                                                          |
| -------------------- | --------------------------------------------------------------------------------------------- |
| **Button primary**   | `--accent` bg, `--on-accent` text, `--radius-sm`, height 32px, padding `--space-3` horizontal |
| **Button secondary** | `--bg-elevated`, `--border` 1px, `--text-primary`                                             |
| **Progress bar**     | height 4px, track `--progress-track`, fill `--accent`; course/lesson variants same token      |
| **Status pill**      | 12px text, no uppercase; dot + word: Not started / In progress / Done / Broken                |
| **Data table**       | zebra optional off; row hover `--accent-subtle` at 50% opacity wash                           |
| **Toast**            | bottom-left, 1 line, auto-dismiss 4s; no slide animation (fade 120ms)                         |

---

## Copy tone

Plain, instructional, sentence case. Errors state what failed and what to do next. No apologies, no “Oops!”, no exclamation marks in empty states.

**Examples**

- Import failed: “Could not read that folder. Check permissions and try again.”
- Anki missing (if launch fails): “Anki is not installed. Install Anki or check Settings for the executable path.”

---

## Accessibility (non-negotiable)

- Body text contrast ≥ **4.5:1** on `--bg-surface` / `--bg-app`.
- Focus visible on all interactive elements (`--focus-ring`).
- Progress and status: **never color alone**—always text or icon label.
- `prefers-reduced-motion` honored (see Motion).

---

## Implementation checklist (for UI prompts)

- [ ] CSS variables from **Color tokens** and **Spacing scale** in one `theme.css`
- [ ] IBM Plex Sans only
- [ ] Accent only on progress + primary actions
- [ ] Five screens match **one job** sentences
- [ ] Every empty state: instructional copy + primary action (library first-run: **two** import actions)
- [ ] No hero, grid features, testimonials, glass, purple gradient

---

## Reference alignment

- Domain statuses: `auravault-domain.md` (`LessonStatus`, `DerivedStatus`).
- Architecture: Tauri webview, no marketing routes.
