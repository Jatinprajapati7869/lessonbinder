#!/usr/bin/env bash
set -euo pipefail

mode="${1:-fmt}"

if [[ ! -f Cargo.toml ]]; then
  echo "check-rust: skip (no Cargo.toml)"
  exit 0
fi

case "$mode" in
  fmt)
    cargo fmt --all --check
    ;;
  clippy)
    cargo clippy --all-targets --all-features --workspace -- -D warnings
    ;;
  *)
    echo "check-rust: unknown mode $mode" >&2
    exit 1
    ;;
esac
