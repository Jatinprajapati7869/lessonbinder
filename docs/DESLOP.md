# Deslop log

Edits remove unused client glue, one-off helper modules, and scaffold-era skip messages. No screens, commands, tests, or migrations removed.

## Deleted paths

| Path                                        | Notes                                                                                        |
| ------------------------------------------- | -------------------------------------------------------------------------------------------- |
| `ui/src/lesson/formatLessonResourceKind.ts` | Single caller; `lessonResourceKindLabel` lives in `ui/src/player/PlayerLessonResources.tsx`. |
| `ui/src/library/libraryVisibility.ts`       | Type moved to `LibraryVisibility` in `ui/src/api/libraryTypes.ts`.                           |

## Removed without deleting files

- `fetchVaultDbInfo` and `VaultDbInfo` in `ui/src/api/tauri.ts` / `ui/src/api/types.ts` (unused UI wrapper; `vault_db_info` Tauri command unchanged).
- Vite starter comment in `ui/vite.config.ts`.
- Progress-prompt placeholders in `scripts/check-rust.sh`, `scripts/check-typescript.sh`, `scripts/typecheck-if-ui.mjs`.

## Search terms (`ui/`, `crates/`, `src-tauri/`, `README.md`)

Scanned for: `lorem`, `placeholder`, `TODO implement`, `welcome to your journey`, `Svelte`, `aulavault.com`, `licensing.aulavault.com`, `coming soon`, `dummy text`.

Nothing user-visible or false removed (no matches for lorem, TODO implement, journey copy, Svelte stack claim, competitor URLs, coming soon, or dummy text). Remaining substring hits are intentional UX or test fixtures:

| File                                           | Line | Match                                 |
| ---------------------------------------------- | ---- | ------------------------------------- |
| `ui/src/screens/LibraryScreen.tsx`             | 133  | `placeholder=` (search field hint)    |
| `ui/src/library/LibraryPastePlaylistPanel.tsx` | 46   | `placeholder=` (paste format hint)    |
| `ui/src/theme/course.css`                      | 300  | `.lesson-markdown__image-placeholder` |
| `ui/src/lesson/lessonMarkdown.tsx`             | 10   | `renderImagePlaceholder`              |
| `ui/src/lesson/lessonMarkdown.tsx`             | 14   | `markdown-image-placeholder`          |
| `ui/src/lesson/lessonMarkdown.tsx`             | 15   | `markdown-image-placeholder`          |
| `ui/src/lesson/lessonMarkdown.tsx`             | 63   | `renderImagePlaceholder`              |
| `crates/vault-core/tests/extract.rs`           | 40   | `dummy.pdf` (fixture path)            |
| `crates/vault-core/tests/fixtures/dummy.pdf`   | 2    | `dummy` (minimal PDF bytes)           |
