# Lessonbinder scope — v1.6

**Status:** Planning (prompt 334).
**Baseline:** Shipped v1.5 behavior and verification are in [`SCOPE-v1.5.md`](SCOPE-v1.5.md), [`V1.5.md`](V1.5.md), and [`LEARNINGS.md`](LEARNINGS.md). v1.6 is a **sitting-in-the-app, bundle retry, and outline-icons pack** on top of v1.5: documented human sitting results on a dev fixture, one honest AppImage build attempt after **`patchelf`** was present, six licensed FlutKit **outline** PNGs copied locally (five wired in the UI), and packaging docs for **Windows** / **macOS** reference commands with **signing explicitly left off**—not new product areas.

v1.6 does **not** change manual `done` semantics, the two-database rule, or Anki grading.

---

## In scope (v1.6 only)

### Sitting run (in the app)

- Manual **sitting** on fixture **`/tmp/lessonbinder-sitting`** with isolated app data (`XDG_DATA_HOME`), scripted in [`SITTING-RUN.md`](SITTING-RUN.md) and mapped to first-course rows in [`SITTING.md`](SITTING.md).
- **Pass** rows recorded for prompts **305–311** on the reference machine (**2026-10-28**): import, resume, note, one subtitle-cue card, review, theme, quit, and relaunch. **Continue** and purchased-folder import stay **not sat** / human-only where documented.
- README **Test** section links the sitting result docs without claiming automated CI runs every human checklist row.

### AppImage retry (documented, not published)

- **`patchelf`** present on the reference machine (**313**: `/usr/bin/patchelf`).
- One **`npm run tauri:build:appimage`** run (**314**): release binary built (**34 158 352** B); bundling failed with **`timeout: global`** downloading AppRun; **no** `target/release/bundle/appimage/*.AppImage`.
- **315** documents the launch command for when an artifact exists; launch **not run** on this machine. [`PACKAGE.md`](PACKAGE.md) and [`README.md`](../README.md) **Install** stay aligned: **unsigned**, **`0.0.0`**, **not published** from this repository—do **not** claim a signed build or a store-ready AppImage from this tree.

### Named outline icons (copied files only)

- Six FlutKit **outline** PNGs copied into **`ui/src/theme/icons/`** with the same filenames as the kit ([`ICONS.md`](ICONS.md)):
  - `file-tray-full-outline.png` — **Show folder**
  - `search-outline.png` — library **Search library** field
  - `setting-outline.png` — side-nav **Settings**
  - `reader-outline.png` — **Show lesson file** and **Notes** heading
  - `list-outline.png` — **Lessons** heading
  - `school-outline.png` — **copied but unused** in the UI (no **School** control)
- Icons load from repo paths only (Vite imports); accessible names for wired controls stay unchanged (tests in **325–326**). **Kit illustrations** and the rest of the FlutKit tree remain **out of scope** (see below).

### Windows and macOS (documentation only)

- [`PACKAGE.md`](PACKAGE.md) § **Platform scope**: this repository **builds on Linux** (`bundle.targets` **`["appimage"]`** only in `tauri.conf.json`); **no** Windows or macOS bundles produced here and **no** project-hosted download for other OSes.
- Reference **Tauri 2** commands for maintainers on native hosts: **`msi`** (Windows) and **`dmg`** (macOS)—**unrun** on the Linux reference machine (**328**). README **Install** does **not** claim Windows or macOS binaries.

### Signing left off

- [`PACKAGE.md`](PACKAGE.md) § **Code signing** (**330**): **not configured** for **Linux**, **Windows**, or **macOS** in this repository—unsigned local bundles only; no certificates, notarization, or signing identities added to config.

### Docs and regression rows (302–334)

- Pack checklist: [`PROGRESS.md`](PROGRESS.md) rows **302–334** (sitting script and runs, bundle **314**, icon copy and wiring **319–326**, platform and signing docs **327–330**, deslop/doctor **331–332**, README **333**, this scope document **334**).
- v1.5 features (show folder, show lesson file, Settings shortcuts, command log cap, `first_course_sitting`, cancel-before-upsert) **remain** in scope as shipped baseline; v1.6 adds verification and packaging honesty on top.

---

## Out of scope (v1.6)

| Area                                                                    | Notes                                                      |
| ----------------------------------------------------------------------- | ---------------------------------------------------------- |
| **Accounts** and auth                                                   | No login or identity in this pack                          |
| **Cloud sync** of library, notes, stats, or progress                    | Local vault and profile only                               |
| **Kit illustrations** / non-outline FlutKit or ProKit assets            | Outline PNGs listed above only; no illustration imports    |
| **A certificate** / code signing / notarization / **published release** | Signing explicitly **off**; see [`PACKAGE.md`](PACKAGE.md) |
| **Auto-`done`** from watch percent or thresholds                        | `done` remains manual ([`charter.md`](charter.md))         |

Also still out of scope: mobile clients, playlist media download, downloading Whisper weights, and shipping **Windows** or **macOS** **installers** from this repository (docs may name build commands; artifacts are not claimed).

**Typography:** Bundled **Google Sans** and **Andina** under `ui/src/theme/fonts/` stay font-only ([`FONTS.md`](FONTS.md)).

---

## Relationship to prompts 302–334

Implementation and verification checklist: [`PROGRESS.md`](PROGRESS.md) rows **302–334**. Row **334** records approval of this scope document. Checklist freeze for v1.6 continues at prompt **335** ([`V1.6.md`](V1.6.md) from this doc).

## Success criteria (planning)

- [`SITTING-RUN.md`](SITTING-RUN.md) and [`SITTING.md`](SITTING.md) honestly record which first-course rows were **sat** in the app on the dev fixture versus human-only or **not run** (AppImage launch, purchased folder).
- [`PACKAGE.md`](PACKAGE.md) and README **Install** describe **314** bundle status without claiming a published, signed, or cross-platform binary from this repo.
- Copied **outline** icons match [`ICONS.md`](ICONS.md); wired controls keep prior accessible names; **`school-outline.png`** is not shown until a matching control exists.
- **Windows** / **macOS** appear only as **unrun reference commands** and platform-scope limits—not as shipped targets from this tree.
- **Signing** remains **not configured**; no certificate or release channel is implied.
