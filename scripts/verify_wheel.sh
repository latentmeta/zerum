#!/usr/bin/env bash
# Verify a maturin-built Zerum wheel installs and runs.
# Usage: ./scripts/verify_wheel.sh [dist_dir]
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIST_DIR="${1:-$ROOT/dist}"
VENV="$ROOT/.venv-packaging"

shopt -s nullglob
WHEELS=("$DIST_DIR"/zerum-*.whl)
if [[ ${#WHEELS[@]} -eq 0 ]]; then
  echo "No zerum-*.whl in $DIST_DIR — run: maturin build --release --locked --out dist" >&2
  exit 1
fi

python3 -m venv "$VENV"
# shellcheck disable=SC1091
source "$VENV/bin/activate"
pip install --upgrade pip
pip install "${WHEELS[0]}"

zerum --help
zerum list-checks | head
zerum check "$ROOT/tests/fixtures/clean_project"

set +e
zerum check "$ROOT/tests/fixtures/bad_project"
status=$?
set -e
if [[ "$status" -ne 1 ]]; then
  echo "expected exit 1 from bad_project, got $status" >&2
  exit 1
fi

echo "OK: wheel install and CLI smoke passed (${WHEELS[0]})"
