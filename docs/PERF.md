# Performance budgets (local reference)

AuraVault keeps list and library search responsive for large folder courses. Budgets are enforced in `vault-core` integration tests (`tests/perf_large_course.rs`), not in the UI.

## Reference machine

| Field    | Value                                                                |
| -------- | -------------------------------------------------------------------- |
| Recorded | 2026-10-28                                                           |
| OS       | Linux 7.2.11-arch1-1 (x86_64)                                        |
| Test     | `cargo test -p vault-core five_hundred_lesson -- --nocapture`        |
| Dataset  | 500 `local_file` lessons in one folder course (in-memory `vault.db`) |

## Observed timings (debug build)

| Operation                             | Observed | Budget (test constant) |
| ------------------------------------- | -------: | ---------------------: |
| `list_courses` (library list)         |     3 ms |                 250 ms |
| `course_detail` (course lesson list)  |     6 ms |                 250 ms |
| `search` (`marker-0250`, title match) |     1 ms |                 150 ms |

Budgets are intentionally loose versus observed times so debug builds and slower laptops stay green; tighten only after re-measuring on this machine.

## Query notes

- Library **search** filters lesson titles and media paths in SQL (`lessons_matching_title_or_path`), searches transcription cues in SQL, and only reads on-disk subtitle sidecars for lessons that have a `subtitle_file_id`.
- Do not mask slow queries with UI caching; fix the vault query instead.
