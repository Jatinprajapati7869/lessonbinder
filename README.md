# Lessonbinder

Lessonbinder is an offline desktop app (Tauri, React, and TypeScript) for course folders you keep on disk. You import those paths, track lesson progress yourself, and review flashcards in a private Anki collection the app opens in-process through Anki’s Rust library (`rslib`). There is no account or cloud sync. Development and release bundling in this repository target Linux (AppImage via `tauri.conf.json`); it does not claim Windows, macOS, or a signed installer from here.

## What it does

- Import a folder as a course of lessons (video, audio, PDF, markdown). Re-import and rescan can add lessons or mark missing files broken while keeping accepted order; `.lessonbinderignore` can exclude paths. You can **cancel** an in-progress folder import or rescan; lessons already written before cancel stay in the library.
- Only one folder import or rescan runs at a time—a second scan waits until the first finishes (the app rejects overlapping attempts with a short message).
- Drop a folder onto the library to import it (same checks as **Add folder**; risky roots are rejected).
- Pair `.srt` or `.vtt` files to videos by filename stem instead of importing subtitles as separate lessons.
- Read duration and similar metadata when tools such as ffprobe are on `PATH`; otherwise store unknown values with a reason.
- Resume video or document position, record watched intervals for a percentage display, and mark lessons complete or incomplete yourself.
- Move lessons up or down within a course; accept or reject order proposals where the UI exposes them.
- Show roadmaps whose step status is derived from linked course progress.
- Create playlist courses from JSON or pasted lines (`title`, tab, URL) without fetching media.
- Search the library locally: lesson titles, paths, subtitle cues, and vault lesson-note text. Opening a hit can seek the player when the match has a timestamp.
- Export lesson or course notes to Markdown from vault data only (does not read or delete Anki cards).
- Capture a region on a paused video frame with the Tesseract CLI, then commit a card into the Lessonbinder Anki profile. Vault state lives in `vault.db`; the collection lives under `anki-profile/` in app data (`dev.lessonbinder.app` on Linux).
- Study due Lessonbinder cards in the app (Again / Hard / Good / Easy). Open the same profile in Anki Desktop when you configure that fallback.
- Keep per-lesson notes in the vault; save a note to Anki only after you confirm. List linked cards on a lesson and unlink without deleting the note.
- Optionally attach a local audio clip to a card when ffmpeg and the capture flow are available.
- Show ffprobe-derived chapters when present; offer “Up next” in course order and course-scoped review when cards are due.
- Use player **theater** and **zen** modes to hide chrome for focused playback. Picture-in-picture is supported in code but the control stays hidden when the embedded WebView does not expose PiP (common on Linux builds).
- Choose **light** or **dark** theme in Settings (stored in `vault.db`). UI text uses bundled **Google Sans**; the shell wordmark uses bundled **Andina** (local files — see [`docs/FONTS.md`](docs/FONTS.md)).
- Show a **study streak** on the library for consecutive days with qualifying study activity (not opening a course alone).
- Transcribe locally when Settings points at a Whisper model file and the CLI is available.
- Back up `vault.db` and the `anki-profile/` tree to a folder you pick in Settings.

## What it does not do

- User accounts
- Cloud sync
- Downloading playlist or course media from the network
- Marking a lesson done from watch percentage alone
- Downloading Whisper model weights
- Mobile clients

## Install

Build a **Linux AppImage on your own machine** from this repository. Images are **unsigned**; the bundled app version in `tauri.conf.json` is **0.0.0**. Builds are for **local** installation and testing only—not a published or signed release from the project.

From the repository root, with UI dependencies installed (`npm ci --prefix ui`):

```bash
npm run tauri:build:appimage
```

When bundling succeeds, the file appears under `target/release/bundle/appimage/*.AppImage`. Host dependencies, runtime tools on `PATH`, and AGPL distribution notes are in [`docs/PACKAGE.md`](docs/PACKAGE.md).

**258 (2026-10-28):** `npm run tauri:build:appimage` was **not run** on the reference machine because **`patchelf`** is not installed (`command -v patchelf` fails). Install with `sudo pacman -S patchelf` before retrying. Bundling did not finish; there is no AppImage path until a build completes and `target/release/bundle/appimage/*.AppImage` exists. Details: [`docs/PACKAGE.md`](docs/PACKAGE.md).

## Develop

Install UI dependencies once (`cd ui && npm install`, as in [`docs/FIRST-TEST.md`](docs/FIRST-TEST.md)), then from the repository root:

```bash
npm run tauri:dev
```

Isolated app data on Linux uses `XDG_DATA_HOME` and related variables; see [`docs/FIRST-TEST.md`](docs/FIRST-TEST.md). AppImage build steps, bundling limits, and runtime tools on `PATH` are in [`docs/PACKAGE.md`](docs/PACKAGE.md). The local `anki` / `rslib` checkout path is in [`docs/ANKI_PIN.md`](docs/ANKI_PIN.md).

## Test

```bash
cargo test --workspace
cd ui && npm test
```

The manual launch, fixture, and safety checklist is in [`docs/FIRST-TEST.md`](docs/FIRST-TEST.md).

## License

This project is AGPL-3.0-or-later (workspace `Cargo.toml`). Binaries link the `anki` crate from Anki’s `rslib`, which is AGPL as well; distributing a build requires complying with both licenses, including source obligations for the linked library.
