# AuraVault

Offline desktop app (**Tauri**, **React + TypeScript** UI) for local course folders: honest lesson progress, roadmap graphs, OCR capture into a **dedicated Anki collection**, and spaced-repetition study on that profile.

**v1.1 study:** **in-app review** (Again / Hard / Good / Easy via `rslib` in-process) is the default **Study** path. **Anki Desktop** remains an explicit fallback when you choose it or when in-app review cannot run.

**Optional local transcription:** when you configure a **local** whisper.cpp model path in Settings, the player can transcribe lessons on device. No bundled weights and no cloud transcription.

**Backup:** Settings can copy `vault.db` and the AuraVault `anki-profile/` tree to a folder you choose on disk (local only; not cloud sync).

**Not included:** accounts, cloud sync, automatic course understanding, playlist media download, or mobile. See [`docs/V1.md`](docs/V1.md) (v1 baseline) and [`docs/V1.1.md`](docs/V1.1.md) (v1.1 SPEC checklist).

**v1.2 study loop (in tree):** move lessons up/down within a course without losing order on rescan; in-app review skips due cards outside AuraVault decks; each lesson lists linked Anki cards (unlink vault-only), confirmed promote from vault notes and optional local audio clips on cards; player chapters when ffprobe provides them, **Up next** in course order, **Review this course** for a scoped queue, and **Paste playlist** (`title` + tab + `url` lines, metadata only).

## What v1 does

- Import a folder as a **course** of **lessons** (videos, audio, PDFs, markdown); re-import is idempotent.
- Pair `.srt` / `.vtt` to videos by filename stem (not separate lessons).
- Extract metadata when tools exist (ffprobe, parsers); otherwise store **unknown** with a reason—no invented summaries.
- **Progress:** resume position, max position reached, manual complete/incomplete; watch % is display-only (does not auto-complete).
- **Order proposals:** accept/reject explicit proposals; import alone does not reorder lessons.
- **Roadmaps:** graph of courses with derived step status from course rollups.
- **Playlist-shaped courses:** metadata import only (no downloader).
- **Capture:** local OCR (Tesseract) → note in the AuraVault Anki profile (`anki-bridge` + `rslib`).
- **Study:** due counts in-app; default **in-app review** grades cards in AuraVault decks; **Open in Anki Desktop** launches the external app on the same profile directory.

Two databases: `vault.db` (library/progress/links) and the Anki collection under `anki-profile/` in app data.

## License

This project is **AGPL-3.0-or-later**. Linking the `anki` crate (`rslib`) triggers the same obligations for distributed binaries. See workspace `Cargo.toml` and [`docs/ANKI_PIN.md`](docs/ANKI_PIN.md).

## Anki (`rslib`) pin

AuraVault depends on a local checkout of Anki’s Rust library—not a fork bundled in this repo. Path and version are documented in [`docs/ANKI_PIN.md`](docs/ANKI_PIN.md). Update `crates/anki-bridge/Cargo.toml` when you change the pin.

## OCR (Tesseract)

Capture uses the **Tesseract CLI** on your PATH (images stay on device). The `vault-core` crate gates the real adapter behind the **`tesseract` Cargo feature**; enable it on the app crate when building a capture-capable binary. Without Tesseract, capture commands return a clear “not installed” error.

Optional: **ffprobe** on PATH improves video duration metadata; tests skip or assert “unknown” when it is missing.

## Package (Linux AppImage)

Local AppImage builds and system dependencies are documented in [`docs/PACKAGE.md`](docs/PACKAGE.md). Use `npm run tauri:dev` for day-to-day development without bundling.

## Run / test

**Rust (domain + Anki bridge + shell):**

```bash
cargo test --workspace
```

Architecture guard: `vault-core` must not depend on `anki` (`crates/vault-core/tests/architecture.rs`, `scripts/vault-core-must-not-depend-on-anki.sh`).

**Web UI (Vitest):**

```bash
cd ui && npm install && npm test
```

**Desktop dev:**

```bash
cd ui && npm install
cargo tauri dev   # from repo root; Tauri CLI on PATH
```

Web assets are served in dev as documented in [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md). Implementation checklist: [`docs/PROGRESS.md`](docs/PROGRESS.md).

## How to test

**Desktop dev (from repo root, after `cd ui && npm install`):**

```bash
npm run tauri:dev
```

For an isolated app-data directory on Linux, set `XDG_DATA_HOME` (and optionally `XDG_CONFIG_HOME`, `XDG_CACHE_HOME`) to empty temp dirs before that command — see [`docs/FIRST-TEST.md`](docs/FIRST-TEST.md) § Launch.

**Automated checks:**

```bash
cargo test --workspace
cd ui && npm test
npm run typecheck && npm run lint   # from repo root
```

**Optional tools** (features degrade gracefully when missing): **ffmpeg** / **ffprobe** (clip metadata, chapters), **tesseract** (frame OCR capture; build with the `tesseract` feature on the app crate), **whisper-cli** + a local model path (Settings transcription), **anki** on PATH (Open in Anki Desktop fallback).

**Manual first-test checklist:** [`docs/FIRST-TEST.md`](docs/FIRST-TEST.md) (fixture under `/tmp/auravault-first-test`, study/capture/safety rows).

## Hooks

Agent skill wiring: `docs/agents/`. Git hooks: `docs/HOOKS.md` (`npm install`, `.venv` + `pre-commit run --all-files`).
