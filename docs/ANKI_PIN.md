# Anki (`rslib`) path pin

Lessonbinder links the **`anki`** crate from a local checkout (AGPL). Do not vendor Anki source into this repo.

| Field               | Value                                                                                                                                                                                                                                               |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Path                | `/home/xingzi/Documents/anki-main/rslib`                                                                                                                                                                                                            |
| Package name        | `anki`                                                                                                                                                                                                                                              |
| Cargo dependency    | `anki = { path = "/home/xingzi/Documents/anki-main/rslib", package = "anki" }`                                                                                                                                                                      |
| Anki release string | `26.09.3` (from `anki-main/.version` at pin time)                                                                                                                                                                                                   |
| Git revision        | _unavailable_ — `anki-main` is not a git checkout on this machine                                                                                                                                                                                   |
| FTL translations    | Shallow clones of [anki-core-i18n](https://github.com/ankitects/anki-core-i18n) and [anki-desktop-ftl](https://github.com/ankitects/anki-desktop-ftl) under `anki-main/ftl/*-repo/` (required for `anki_i18n` build when not using `git submodule`) |

When `anki-main` is a git repo, record:

```bash
git -C /home/xingzi/Documents/anki-main rev-parse HEAD
```

Collection schema version for new profiles: **18** (`anki-bridge::COLLECTION_SCHEMA_VERSION`).
