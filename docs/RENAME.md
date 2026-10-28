# Rename AuraVault → Lessonbinder

Product name is **Lessonbinder** (one word). Spoken: “lesson binder.” Prompts 141–150 apply this rename; prompts 01–140 still say AuraVault.

## Spelling map

Use only this table when renaming strings in code, config, and docs (later prompts).

| Old                                       | New                                             |
| ----------------------------------------- | ----------------------------------------------- |
| AuraVault                                 | Lessonbinder                                    |
| auravault                                 | lessonbinder                                    |
| auravault_lib                             | lessonbinder_lib                                |
| auravault-tauri / auravault_tauri         | lessonbinder-tauri / lessonbinder_tauri         |
| AuraVaultApp                              | LessonbinderApp                                 |
| AuraVaultProfilePaths                     | LessonbinderProfilePaths                        |
| AURAVAULT_CLIP_NOTETYPE / AuraVaultClip   | LESSONBINDER_CLIP_NOTETYPE / LessonbinderClip   |
| AURAVAULT_DECK_PREFIX / AuraVault::       | LESSONBINDER_DECK_PREFIX / Lessonbinder::       |
| ensure_auravault_clip_notetype            | ensure_lessonbinder_clip_notetype               |
| validate_auravault_deck_name              | validate_lessonbinder_deck_name                 |
| NoAuraVaultDeck / CardNotInAuraVaultDecks | NoLessonbinderDeck / CardNotInLessonbinderDecks |
| `.auravaultignore`                        | `.lessonbinderignore`                           |
| `dev.auravault.app`                       | `dev.lessonbinder.app`                          |
| `auravault://`                            | `lessonbinder://`                               |

## Keep list

These names stay as-is (database / vault layer, not the product brand):

- Crate **`vault-core`**
- File **`vault.db`**
- Directory **`anki-profile`**

Keep **`AulaVault`** wherever it means the other product (competitor), for example `docs/AULAVAULT-BEHAVIOR.md`, aulavault.com, licensing.aulavault.com.

Out of scope for this rename work:

- Do not edit `/home/xingzi/Documents/anki-main`.
- Do not hand-edit `src-tauri/gen/` (refresh via Tauri build).
- Do not rewrite the bodies of prompts 01–140 in the prompt pack.

## App data

Existing app-data under the old identifier is left in place; the new identifier starts a fresh directory.

## Anki collection (deck prefix and notetype)

New commits use **`Lessonbinder::`** decks and the **`LessonbinderClip`** notetype. There is **no migrator**: an Anki profile created before this rename keeps its existing **`AuraVault::`** deck names and **`AuraVaultClip`** notes on disk. In-app review and due counts only include cards under the **`Lessonbinder`** deck tree; rename decks and notetypes in Anki Desktop if you need old cards in the new namespace.

## Remaining `rg 'AuraVault|auravault'` (prompt 148)

Allowed: **AulaVault** (competitor), this file’s spelling map / pre-migration Anki names, **`src-tauri/gen/`** until the next Tauri build (no matches at audit time). Git history under `.git/` still mentions the old product name.

Intentional: external prompt pack filename unchanged (`auravault-cursor-prompts.md`):

```
docs/PROGRESS.md:Check off when the matching prompt in `auravault-cursor-prompts.md` (under `~/Documents/`) is complete.
docs/PROGRESS.md:Start only after 12 and 40–50 are checked. Full text: `~/Documents/auravault-cursor-prompts.md`.
docs/PROGRESS.md:Optional. Start only after 50. Do not decompile AulaVault into this repo. Full text: `~/Documents/auravault-cursor-prompts.md`.
docs/PROGRESS.md:Start here. 101–110 are already in the tree. Full text: `~/Documents/auravault-cursor-prompts.md`.
docs/PROGRESS.md:No new features. Full text: `~/Documents/auravault-cursor-prompts.md`.
docs/PROGRESS.md:Product name is Lessonbinder. Full text: `~/Documents/auravault-cursor-prompts.md`. Keep `vault-core` and `vault.db`. Keep the competitor name AulaVault.
docs/PROGRESS.md:Can run after 142. Full text: `~/Documents/auravault-cursor-prompts.md`.
docs/agents/issue-tracker.md:Until tickets are copied from `docs/TICKETS.md`, implementation order follows prompt numbers in `~/Documents/auravault-cursor-prompts.md` and `docs/PROGRESS.md`.
docs/charter.md:**Sources for this charter:** `/home/xingzi/Documents/auravault-cursor-prompts.md`, workspace boundary in `lessonbinder-workspace.md`. …
docs/TICKETS.md:**Sources:** `lessonbinder-SPEC.md`, `lessonbinder-ARCHITECTURE.md`, `auravault-cursor-prompts.md` (prompts **11–42**).
```

## Checkout folder and GitHub (prompt 149)

- **Local path:** `/home/xingzi/Documents/lessonbinder` (was `auravault`).
- **Remote:** `https://github.com/Jatinprajapati7869/lessonbinder`
- **About:** Offline desktop course tracker. Import a local folder, keep lessons in your order, mark progress yourself, and make Anki cards from the lesson.
- No absolute paths under `Documents/auravault` remained in the tree before the move (only `auravault-cursor-prompts.md` references).

## Prompt 150 — verification

- **README title:** Lessonbinder (`# Lessonbinder`)
- **`tauri.conf.json` `productName`:** Lessonbinder
- **`tauri.conf.json` window `title`:** Lessonbinder
- **`cargo test --workspace`:** pass (195 tests, 67 suites)
- **`cd ui && npm test`:** pass (93 tests)

READY
