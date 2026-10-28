# Domain Docs

How the engineering skills should consume this repo's domain documentation when exploring the codebase.

## Before exploring, read these

- **`docs/domain.md`**: Lessonbinder entities, invariants, and ubiquitous language (canonical for this product).
- **`GLOSSARY.md`** at the repo root, if it exists (lazy additions from `/domain-modeling`).
- **`docs/adr/`**: read ADRs that touch the area you're about to work in.

If `GLOSSARY.md` or `docs/adr/` do not exist yet, proceed silently; do not block work.

## File structure (this repo)

Single-context:

```
/
├── docs/
│   ├── domain.md          ← entities & invariants
│   ├── charter.md
│   ├── SPEC.md
│   └── adr/               ← created when ADRs are needed
└── crates/                ← from prompt 11 onward
```

## Use the glossary's vocabulary

When naming domain concepts (issues, tests, APIs), use terms from `docs/domain.md` (`Lesson`, `Progress`, `DerivedStatus`, `broken`, etc.). Do not invent synonyms.

## Flag ADR conflicts

If output contradicts an existing ADR, surface it explicitly rather than silently overriding.
