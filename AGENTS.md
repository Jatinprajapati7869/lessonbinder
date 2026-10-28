# AuraVault agent rules

## Agent skills

### Issue tracker

Issues live as local markdown under `.scratch/`. See `docs/agents/issue-tracker.md`.

### Triage labels

Five canonical roles (`needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`). See `docs/agents/triage-labels.md`.

### Domain docs

Single-context; canonical vocabulary in `docs/domain.md`. See `docs/agents/domain.md`.

---

- Read **`docs/SPEC.md`** before coding or changing behavior.
- **Product code never goes in `anki-main`.** Upstream Anki is read-only reference; path pin documented in `docs/ANKI_PIN.md` once prompt 11 lands.
- **UI follows `docs/DESIGN.md`** (tokens, screens, empty states).
- **Anki access only through `crates/anki-bridge`.** `vault-core` must not depend on the `anki` crate.
- Planning docs: `docs/charter.md`, `docs/domain.md`, `docs/ARCHITECTURE.md`, `docs/TICKETS.md`, `docs/PROGRESS.md`.
