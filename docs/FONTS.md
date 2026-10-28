# Bundled fonts (Lessonbinder)

Google Sans and Andina ship as local files under `ui/src/theme/fonts/` and load through `@font-face` rules in `ui/src/theme/tokens.css`. The app does not request fonts from Google Fonts or any other network URL.

These typefaces are licensed to the project owner for use in Lessonbinder.

## Files and weights

| File                     | CSS `font-family` | Weight | Use                                |
| ------------------------ | ----------------- | ------ | ---------------------------------- |
| `googlesans-regular.ttf` | Google Sans       | 400    | UI body and controls (prompt 154+) |
| `googlesans-medium.ttf`  | Google Sans       | 500    | Emphasis, section titles           |
| `googlesans-bold.ttf`    | Google Sans       | 700    | Screen titles, strong labels       |
| `andina.ttf`             | Andina            | 400    | Shell wordmark only (prompt 154+)  |

Source copies (owner-licensed) live under `Documents/ProKit Flutter v7.7.0/prokit-flutter_v7.7.0/fonts/`; the repo names above are the canonical paths in the tree.

## Build verification (prompt 194)

**Claim:** the UI build does not reference `fonts.googleapis.com` or `fonts.gstatic.com`, and all four `.ttf` files are bundled with the frontend (not fetched at runtime).

### Automated checks

| Check                                                                                             | Where                                                              |
| ------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| No Google Fonts hosts in `tokens.css`                                                             | `ui/theme.test.ts` (`declares font family tokens`)                 |
| Four source fonts + four `@font-face` rules + no Google hosts in `ui/src` and `index.html`        | `ui/fontsBuild.test.ts`                                            |
| When `ui/dist/` exists: at least four `.ttf` under `dist/assets/`, no Google hosts in asset files | `ui/fontsBuild.test.ts` (skips dist assertions if no build output) |

Run:

```bash
cd ui && npm test -- --run theme.test.ts fontsBuild.test.ts
```

### Production bundle (Vite)

Tauri points at the Vite output (`src-tauri/tauri.conf.json` → `build.frontendDist`: `../ui/dist`). There is no separate Tauri resource list for fonts; they ship inside the web bundle.

After `npx vite build` (from `ui/`):

1. **Four font files** appear as hashed assets, for example:
   - `dist/assets/andina-*.ttf`
   - `dist/assets/googlesans-bold-*.ttf`
   - `dist/assets/googlesans-medium-*.ttf`
   - `dist/assets/googlesans-regular-*.ttf`
2. **No Google Fonts URLs** in the build tree:

```bash
cd ui && npx vite build
rg 'fonts\.googleapis\.com|fonts\.gstatic\.com' dist/
# (no matches — exit code 1 from ripgrep means none found)
```

CSS in `dist/assets/index-*.css` references those hashed `.ttf` paths via Vite’s asset URLs, not external hosts.

### Source scan (optional manual)

```bash
rg 'fonts\.googleapis\.com|fonts\.gstatic\.com' ui/src ui/index.html
# no matches
```

## Missing font at build time (prompt 195)

Before Vite bundles the UI, `buildStart` in `ui/vite.config.ts` calls `assertBundledFontFilesPresent` (`ui/build/assertBundledFonts.ts`). If any licensed `.ttf` under `ui/src/theme/fonts/` is absent, **dev and production builds throw** with a message like:

```text
Missing licensed font file: googlesans-regular.ttf
```

There is **no** fallback to Google Fonts or any other network URL.

Automated checks:

```bash
cd ui && npm test -- --run fontsBuild.test.ts
```

The `missing bundled font` cases in that file assert the error text and run `vite build` once with a font temporarily renamed away.

## Release bundle scan (prompt 247, 2026-10-28)

**AppImage from prompt 218:** Not built on this machine — `target/release/bundle/appimage/*.AppImage` is absent (see [`PACKAGE.md`](PACKAGE.md) § AppImage build). **No AppImage to search; that absence is not a pass.**

**`ui/dist`:** A local Vite output exists under `ui/dist/`, but it was **not** produced by a completed prompt **218** AppImage bundle. Searched anyway for network font hosts in the web bundle:

```bash
cd /home/xingzi/Documents/lessonbinder
rg 'fonts\.googleapis\.com|fonts\.gstatic\.com' ui/dist/
```

**Result:** no matches (ripgrep exit code **1** — empty). This records the current `ui/dist/` tree only; it does **not** certify an AppImage shipped without Google Fonts URLs.
