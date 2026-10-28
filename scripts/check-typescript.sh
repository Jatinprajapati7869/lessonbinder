#!/usr/bin/env bash
set -euo pipefail

mode="${1:-lint}"

has_ts() {
  find ui -type f \( -name '*.ts' -o -name '*.tsx' \) 2>/dev/null | grep -q .
}

case "$mode" in
  lint)
    if ! has_ts; then
      echo "check-typescript: skip (no ui TypeScript sources)"
      exit 0
    fi
    npx eslint "ui/**/*.{ts,tsx}"
    ;;
  format)
    npx prettier --check .
    ;;
  *)
    echo "check-typescript: unknown mode $mode" >&2
    exit 1
    ;;
esac
