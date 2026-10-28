# AuraVault product charter (v1)

AuraVault is a **course vault** desktop app: organize local and online learning material, track honest lesson progress, visualize learning paths as a flowchart, and connect study to **Anki** (`rslib`) for spaced repetition. It is **not** Anki Desktop, **not** AulaVault, and **not** a second SRS.

**Sources for this charter:** `/home/xingzi/Documents/auravault-cursor-prompts.md`, workspace boundary in `auravault-workspace.md`. The competitor’s public marketing site and roadmap docs were **not used** as sources here. Progress and roadmap rules below align with the prompt pack and a lesson-level / derived-status / manual-roadmap model—without copying the competitor’s name, license server, or UI.

---

## Lesson progress status enum

Canonical lesson state (stored on each lesson; never a separate “roadmap checkbox”):

| Enum value    | Meaning                                                                                      |
| ------------- | -------------------------------------------------------------------------------------------- |
| `not_started` | No meaningful engagement yet (no resume position, no manual complete).                       |
| `in_progress` | Playback started, resume position &gt; 0, or user marked incomplete from `done`.             |
| `done`        | User or watch rules mark the lesson complete (manual tick allowed).                          |
| `broken`      | Source missing or invalid (file gone, playlist item unavailable, etc.)—not “user is behind”. |

**Derived progress:** Course status and roadmap step status are **computed** from lesson states (same rollup rules as courses). Roadmap nodes do not carry independent progress flags.

---

## In scope (v1)

1. **Local course import** — Point at a folder; index **real files**: video, audio, PDF, SRT/VTT, and user notes files.
2. **Content understanding (best effort)** — Beyond filenames: duration, embedded titles, subtitle cues, PDF text when cheap to extract, user-authored notes. If extraction fails or is skipped, the field stays **unknown**; do not invent metadata.
3. **Learning order** — A **saved sequence** of lessons. **Suggestions** (e.g. from deterministic rules or an optional local model) are **proposals only**; the user accepts or rejects. **Never reorder silently.**
4. **Roadmap** — A **graph** of courses (nodes and directed edges) the user can view as a **flowchart** (real graph UI, not a decorative image).
5. **Lesson progress** — Use the enum above. **Course** and **roadmap** progress **derive from lessons**, matching the rule that roadmap status is **not** a separate checkbox.
6. **Manual completion** — User can mark a lesson **complete** or **incomplete** explicitly.
7. **Offline video** — Playback from local files with **resume position** persisted per lesson.
8. **Online playlist** — A course may be sourced from a **playlist**; track **per-item watched** state (our data; playback via embedded page or user-supplied local file per later architecture decisions).
9. **OCR → Anki** — User selects text from a **video frame** (OCR); selection creates an **Anki note** through **`rslib`** (card body lives in the Anki collection).
10. **Review in Anki** — Scheduling stays in Anki (`rslib` on the **dedicated AuraVault-owned profile**). v1 shows **due counts** and creates cards from OCR; review UX ships **with the app** (Decision 16), not “install Anki Desktop first.” **No custom FSRS** and no parallel scheduler in AuraVault.

---

## Out of scope (v1)

- User **accounts** and authentication services
- **Payments** and licensing servers
- **Cloud sync** of library or progress
- **Mobile** apps (iOS/Android)
- **AI chat tutor** or conversational coaching UI
- Automatic **“course complete”** that ignores unwatched or incomplete lessons (rollup must reflect lesson truth)

---

## Relationship to other docs

| Document                 | Role                                                                                |
| ------------------------ | ----------------------------------------------------------------------------------- |
| `auravault-workspace.md` | Where repos live; AGPL; non-goals (trademark, license server, Qt fork, second SRS). |
| `auravault-domain.md`    | Entities and invariants (prompt 04).                                                |
| `auravault-SPEC.md`      | Testable scenarios (prompt 05).                                                     |

## Decisions

Settled in prompt 03 (grill). Each line is one binding v1 choice unless a later charter revision explicitly supersedes it.

1. **OCR engine** — **Local only.** v1 uses **Tesseract** (on-device; bundled or system dependency per platform). **Cloud OCR is forbidden** in v1. A local ONNX model is out of scope for v1 unless we reopen this decision after Tesseract quality testing.

2. **Playlist ingest & playback** — **Offline-first for initial delivery**, same product shape as AulaVault today: local course folders, real files, resume, and lesson progress are the priority. When playlist-backed courses ship, we store **URL, title, duration**, and **our** per-item watch/progress data only; we **do not** bundle a video downloader that ignores site terms. Playback is an **embedded provider page** or a **user-supplied local file** mapped to that item—not a built-in ripper. **Tracking is always AuraVault’s data.**

3. **Content understanding & lesson order** — **Deterministic extractors first** (duration, tags, sidecars, cheap PDF text, etc.). An **optional local model** may **propose** lesson order; the user **must accept or reject** the proposal. **Never reorder silently.**

4. **Anki profile** — **One Anki profile directory owned by AuraVault** (under app data), **not** the user’s everyday Anki profile, so vault/OCR work cannot corrupt their main collection. Review data and scheduling go through **`rslib`** on **that** profile. **How** the user reviews (in-app vs shipped Anki UI) is decided in Decision 16; profile isolation here is unchanged.

5. **Frontend stack** — **Tauri 2** desktop shell with **React + TypeScript** UI (cross-platform: Linux, Windows, macOS). This matches the ecosystem we target and allows **react-doctor** `/doctor` on the webview UI later. Not Svelte; not Anki Qt.

6. **v1 course source (folder → trackable course)** — v1 ships **local folder import** that becomes a **trackable course**: indexed lessons (files) with per-lesson progress, order, and resume—same product idea as AulaVault’s offline library, not a separate “playlist URL” flow in the first slice. **Online playlist URLs** stay charter-ready but **after** this folder-based trackable course works end-to-end.

7. **Watch tracking** — **YouTube-style engagement tracking** per video lesson: persist **resume position** and **how much of the timeline the user has actually watched** (not filename guesses). Playback moves `not_started` → `in_progress`; progress is visible in the UI. **`done`** remains **manual tick** in v1 unless we later add an explicit watch-percent threshold in SPEC (tracking data is stored either way).

8. **Lesson-order proposals (v1)** — **No local ML** for order in the first ship: deterministic extractors + **user-controlled order** (drag/reorder). Optional local model proposals stay a later release (Decision 3 unchanged).

9. **Subtitles (v1)** — **Ingest only**: existing **SRT/VTT** (and cheap embedded cues where extractors already handle them). **No** Whisper or on-device transcription pipeline in the first ship.

10. **Persistence (v1)** — **One local SQLite database** per install for library, courses, lessons, roadmaps, progress, and resume/watch stats; media stays on disk at user paths. **No cloud sync** in v1.

11. **`done` vs watch metrics (v1)** — **YouTube-style** resume and watched-% are stored and shown. **`done` is manual only** (user marks complete). Watch progress does **not** auto-flip `done` in v1.

12. **Folder import → lessons (AulaVault-like)** — One imported folder = one **course**. Lessons from media/docs: `mp4`, `mkv`, `webm`, `mp3`, `m4a`, `pdf`, `md`, `txt`; ignore hidden paths and `node_modules`. **`srt` / `vtt`** bind to the video with the **same stem**, not separate lessons. Default order = **relative path**; user may reorder. Re-import is **idempotent** (no duplicate lessons). Missing files on disk → lesson `broken`.

13. **Derived course & roadmap status** — Course rollup: `not_started` if all lessons `not_started`; `done` if all `done`; `broken` if any `broken` and none `in_progress` or `done`; else `in_progress`. Roadmap step status = linked **course** rollup (no separate roadmap checkbox).

14. **Watched % (v1 display)** — `watched_percent = min(100, max_position_reached_ms / duration_ms)` when duration is known; unknown duration → show unknown, not a fake %. Interval-union (full YouTube semantics) may come later.

15. **OCR → Anki note (v1)** — On the AuraVault profile: default deck **`AuraVault`**, note type **`AuraVault OCR`**: **Front** = OCR text; **Back** = lesson title, course name, timestamp (and optional source path). No deck picker on every capture in v1.

16. **Ship with Anki (not a separate install)** — AuraVault ships as **one product** with the **Anki review stack included** (dedicated profile + **`rslib`** collection in-process). Users are **not** required to install Anki Desktop separately for v1. Deeper Anki-powered features later (e.g. quiz flows) build on the same profile; full Anki Desktop parity or bundling details belong in **architecture**, not a second SRS in AuraVault.
