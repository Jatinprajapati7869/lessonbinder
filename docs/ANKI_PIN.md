# Anki (`rslib`) path pin

Lessonbinder links **`anki`**, **`anki_io`**, and **`anki_proto`** from a **local checkout** of Anki’s `rslib` tree (AGPL). These crates are **not published** to crates.io; `anki-bridge` uses `path = …` dependencies only. **Do not vendor** Anki source into the Lessonbinder repository.

**Source of truth:** [`crates/anki-bridge/Cargo.toml`](../crates/anki-bridge/Cargo.toml) `[dependencies]`.

| Crate        | Path on this machine                           |
| ------------ | ---------------------------------------------- |
| `anki`       | `/home/xingzi/Documents/anki-main/rslib`       |
| `anki_io`    | `/home/xingzi/Documents/anki-main/rslib/io`    |
| `anki_proto` | `/home/xingzi/Documents/anki-main/rslib/proto` |

| Field               | Value                                                                                                                                                                                                                                               |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Anki release string | `26.09.3` (from `anki-main/.version` at pin time)                                                                                                                                                                                                   |
| Git revision        | _unavailable_ — `anki-main` is not a git checkout on this machine                                                                                                                                                                                   |
| FTL translations    | Shallow clones of [anki-core-i18n](https://github.com/ankitects/anki-core-i18n) and [anki-desktop-ftl](https://github.com/ankitects/anki-desktop-ftl) under `anki-main/ftl/*-repo/` (required for `anki_i18n` build when not using `git submodule`) |

When `anki-main` is a git repo, record:

```bash
git -C /home/xingzi/Documents/anki-main rev-parse HEAD
```

Collection schema version for new profiles: **18** (`anki-bridge::COLLECTION_SCHEMA_VERSION`).
