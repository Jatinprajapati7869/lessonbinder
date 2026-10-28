# Lessonbinder scope — v1.3

**Status:** Planning (prompt 152).
**Baseline:** Shipped v1.2 behavior and verification are in [`SCOPE-v1.2.md`](SCOPE-v1.2.md), [`V1.2.md`](V1.2.md), and [`LEARNINGS.md`](LEARNINGS.md). v1.2 kept **drag-and-drop** import out of scope; v1.3 adds it. v1.1 search covers lesson titles, paths, and subtitle cue text only—not vault lesson notes.

v1.3 is a **study-app polish pack**: bundled typography, a coordinated look pass on existing screens, richer notes discovery and export, player presentation modes, a dark theme, and local-only habit stats. It does **not** change the two-database rule, Anki grading model, or manual `done` semantics.

---

## In scope (v1.3 only)

### Typography (bundled, local)

- Ship **Google Sans** for UI copy and **Andina** for the wordmark in the app bundle (local files; no runtime fetch from Google Fonts or a CDN).
- Fonts load from packaged assets in release builds; missing font files fail with a clear error path (see prompts 194–195).

### Look pass — existing screens

- Refresh layout, spacing, type scale, and color tokens on screens that already exist: **Library**, **Course**, **Player**, **Review**, **Settings**, and **Roadmap**—no new product areas beyond what this scope lists elsewhere.

### Folder import — drag and drop

- On **Library** (including empty state), the user can **drop a folder** onto the window to start the same import flow as **Add folder** (local path only; no upload to a server).

### Search — lesson notes

- **Search** includes text stored in **vault lesson notes** (`vault.db`), in addition to existing title, path, and subtitle cue search.
- Opening a hit navigates to the lesson; seek-from-search behavior for time-bearing hits stays aligned with prompts 160–161.

### Notes — Markdown export

- **Export** vault lesson notes to **Markdown** files on disk: per **lesson** and per **course** (aggregated), without writing into the Anki collection or altering Anki notes.

### Player — presentation

- **Theater mode:** chrome hides or recedes so the video (or primary media pane) uses more of the window.
- **Picture-in-picture:** when the platform/WebView supports PiP for the active player, the user can pop video out while keeping Lessonbinder open.
- **Zen mode:** minimal player chrome for focused watching (distinct from theater where both may apply; exact layering is implementation detail).
- **Playback speed:** user-selectable rate for local media playback on the lesson player.
- **Reopen last lesson:** on launch or return to Library, offer or restore the **last opened lesson** from local app state (no cloud “continue watching”).

### Theme

- A **dark theme** (user-selectable or system-aligned—implementation chooses one coherent default in Settings) applied across existing UI, including player chrome where prompts require contrast checks.

### Local habit stats (no account)

- **Streak:** count consecutive **local** days with qualifying study activity; rules are defined in implementation prompts (what counts as a day, what does not count a mere app open).
- **52-week heatmap:** grid of activity intensity by day for the past year, derived from the same local event log as the streak.
- **Weekly report:** a read-only summary for the past seven days (counts or durations from local data only)—copy and metrics in prompts 186+.

---

## Out of scope (v1.3)

| Area                                                 | Notes                                                                                                                                                                                                                                                                                   |
| ---------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Accounts** and auth                                | —                                                                                                                                                                                                                                                                                       |
| **Payment** / subscriptions                          | —                                                                                                                                                                                                                                                                                       |
| **Cloud sync** of library, notes, stats, or progress | —                                                                                                                                                                                                                                                                                       |
| **Downloading models** (Whisper or other weights)    | Local path only when already configured                                                                                                                                                                                                                                                 |
| **Kit illustrations** / marketing art packs          | Typography and UI polish only                                                                                                                                                                                                                                                           |
| **Code signing** / notarization / store publish      | See [`PACKAGE.md`](PACKAGE.md)                                                                                                                                                                                                                                                          |
| **Mobile** clients                                   | Desktop Tauri only                                                                                                                                                                                                                                                                      |
| **Auto-`done`** from watch percent or thresholds     | `done` remains manual ([`charter.md`](charter.md)). **Verified by** `union_full_watch_does_not_auto_done` in [`crates/vault-core/tests/watched.rs`](../crates/vault-core/tests/watched.rs): full-span playback yields **100%** watched and status stays **`not_started`** (not `done`). |

Also still out of scope: a second SRS, custom FSRS in Lessonbinder, playlist media download, cloud OCR, and renaming the existing OCR note type.

---

## Relationship to prompts 153–201

Implementation checklist: [`PROGRESS.md`](PROGRESS.md) rows **153–201** (fonts, tokens, screen looks, drop import, note search/export, theater/PiP/zen, dark theme, streak/heatmap/report, playback speed, last lesson, regression and safety rows, README, tests, freeze). Row **152** records approval of this scope document.

## Success criteria (planning)

- The app reads as one product visually (bundled Google Sans + Andina wordmark) without network font loads.
- Users can **drop a folder** to import, **find text in lesson notes**, and **export notes to Markdown** locally.
- Player modes (theater, PiP where supported, zen), **speed**, and **last lesson** reopen improve daily use without changing Anki scheduling rules.
- **Dark theme**, **streak**, **heatmap**, and **weekly report** work entirely offline and never require an account.
