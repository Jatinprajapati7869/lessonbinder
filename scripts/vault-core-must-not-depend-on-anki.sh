#!/usr/bin/env bash
# Fails if crates/vault-core/Cargo.toml declares an `anki` dependency.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
manifest="$root/crates/vault-core/Cargo.toml"

for section in dependencies dev-dependencies build-dependencies; do
  if awk -v section="$section" '
    /^\[/ { in_section = ($0 == "[" section "]") }
    in_section && /^[[:space:]]*anki[[:space:]]*=/ { found = 1 }
    END { exit (found ? 0 : 1) }
  ' "$manifest"; then
    echo "vault-core [$section] must not list anki (see docs/ARCHITECTURE.md)" >&2
    exit 1
  fi
done

echo "vault-core: no anki dependency"
