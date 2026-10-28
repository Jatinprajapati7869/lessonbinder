# Packaging (Linux AppImage)

Lessonbinder is **not published** to an app store or update channel from this repository. Builds are for local distribution and testing only. **No code signing** is configured.

## Build command (AppImage)

From the repository root, with UI dependencies installed (`npm ci --prefix ui`):

```bash
npm run tauri:build:appimage
```

Equivalent:

```bash
cd src-tauri && npx @tauri-apps/cli@2 build --bundles appimage
```

Release artifacts:

| Output                            | Path                                        |
| --------------------------------- | ------------------------------------------- |
| Binary                            | `target/release/lessonbinder`               |
| AppImage (when bundling succeeds) | `target/release/bundle/appimage/*.AppImage` |

`src-tauri/tauri.conf.json` sets `bundle.targets` to **`["appimage"]`** on Linux (no `.deb` / `.rpm` from this config). Updater artifacts are disabled (`createUpdaterArtifacts: false`).

The workspace crate is **`publish = false`**; do not `cargo publish` these packages.

## Development (no bundle)

Does not require `patchelf` or AppImage tooling:

```bash
npm run tauri:dev
```

Equivalent: `cd src-tauri && npx @tauri-apps/cli@2 dev`

This runs the Vite UI dev server (`beforeDevCommand`) and the Rust app with hot reload. Same CLI as build; omit `--bundles`.

## Linux build host dependencies

### Arch Linux (pacman)

Typical Tauri 2 / WebKitGTK stack:

```bash
sudo pacman -S --needed base-devel curl wget file openssl pkgconf \
  webkit2gtk-4.1 librsvg libappindicator-gtk3
```

`libayatana-appindicator` is an alternative to `libappindicator-gtk3` on some distros; this machine has both installed.

### AppImage bundling

Tauri downloads **AppRun** from GitHub during the bundle step. A slow or blocked network can fail with a download **timeout** (observed here: `failed to bundle project: timeout: global` after the release binary already built).

**Missing on the reference machine (2026-10-28):**

| Package        | Role                                                                                                                                            |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| **`patchelf`** | Required to patch the release binary when assembling the AppImage. Install with `sudo pacman -S patchelf`, then re-run the build command above. |

Until `patchelf` is installed (and AppRun download succeeds), treat **`target/release/lessonbinder`** as the working local build output.

## Runtime dependencies (not inside the AppImage)

These are **system** tools Lessonbinder invokes or expects on `PATH`. They are **not** vendored in the bundle.

| Dependency                                               | Required?                     | Notes                                                                                                                                                                                                                             |
| -------------------------------------------------------- | ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Tesseract** (`tesseract` CLI)                          | **Yes** for frame OCR capture | Without it, capture returns a clear “not installed” error. Images are processed locally only.                                                                                                                                     |
| **whisper.cpp** (`whisper-cli` or path set in Settings)  | **Optional**                  | Local lesson transcription when `transcription_model_path` points at a model file. Empty path disables transcription.                                                                                                             |
| **Anki Desktop** (`anki` on `PATH`, or path in Settings) | **Optional**                  | In-app review uses the embedded collection via `anki-bridge`. **Study in Anki** launches external Anki Desktop against the Lessonbinder profile; if Anki is missing, that fallback is unavailable (in-app review may still work). |
| **ffprobe**                                              | Optional                      | Better video duration metadata when present.                                                                                                                                                                                      |

## AGPL source offer

Lessonbinder is licensed under **AGPL-3.0-or-later** (see workspace `Cargo.toml`). Binaries link **`anki` / `rslib`**, which is also AGPL.

If you distribute an Lessonbinder AppImage or binary to others, you must comply with the AGPL, including offering **complete corresponding source** for that build. A practical offer:

1. State that the program is AGPL-3.0-or-later.
2. Provide a written offer (for example in release notes or a `SOURCE-OFFER.txt` beside the AppImage) to deliver source on request for at least three years, or point to a public Git repository tag/commit that matches the distributed binary.
3. Include license texts (`AGPL-3.0`) and note that Anki/rslib source is available from [Anki’s repositories](https://github.com/ankitects/anki) at the version pinned in [`docs/ANKI_PIN.md`](ANKI_PIN.md).

This repository does **not** automate publishing or signing; maintainers are responsible for any distribution they make outside their own machine.

## Code signing

Not configured. Tauri bundle settings do not enable macOS notarization or Windows Authenticode. Linux AppImages from this project are unsigned.

## AppImage smoke test (post-build)

**Artifact check (2026-10-28):** Prompt 92 did **not** produce an AppImage on this machine.

- `target/release/bundle/appimage/` does not exist.
- No `*.AppImage` under the repository (`find` / glob over repo root).
- `target/release/lessonbinder` **does** exist (~28 MB release binary from a successful `cargo`/Tauri compile step before bundling failed).

Bundling failed earlier because **`patchelf`** was not installed and the AppRun download hit a **network timeout** (see above). There is nothing to run as an AppImage; the smoke test below was **not** executed.

**Command to use when an AppImage exists** (isolated app data via temp `HOME`; adjust the glob if the filename differs):

```bash
export APP_HOME="$(mktemp -d)"
export HOME="$APP_HOME"
trap 'rm -rf "$APP_HOME"' EXIT

APPIMAGE="$(ls -1 target/release/bundle/appimage/*.AppImage | head -1)"
chmod +x "$APPIMAGE"

"$APPIMAGE" &
APP_PID=$!
sleep 15
kill -TERM "$APP_PID" 2>/dev/null || true
wait "$APP_PID" 2>/dev/null || true
```

**Expected result when run manually:** process stays up for the sleep window; empty library copy and **Add folder** / **Add playlist JSON** visible; then exits after `kill`.

**Result on this machine:** **Not run** — no AppImage artifact.
