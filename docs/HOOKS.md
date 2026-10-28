# Git hooks

## Husky + pre-commit

- **Husky** runs on `git commit`.
- **pre-commit** runs generic checks, **detect-secrets**, **rustfmt/clippy** (when `Cargo.toml` exists), **Prettier**, and **ESLint** (when `ui/**/*.ts` exists).

## Setup

```bash
npm install
python -m venv .venv
.venv/bin/pip install pre-commit detect-secrets
.venv/bin/detect-secrets scan > .secrets.baseline   # first time only
```

Husky owns `core.hooksPath`; do **not** run `pre-commit install` (it conflicts). Commits run `.husky/pre-commit`, which calls `pre-commit run`.

## Dry run (all files)

```bash
pre-commit run --all-files
```

## Staged-only (what commit runs)

```bash
npx lint-staged
pre-commit run
```

Rust and TypeScript checks **skip gracefully** until prompt 11 (crates) and prompt 35 (ui) add sources.
