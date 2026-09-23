#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
MAIN_RESULTS="${1:-$ROOT/_agile-output/test-artifacts/129-3-latest-results.json}"
TMP="$(mktemp -d)"
cleanup() {
  rc=$?
  if [[ $rc -ne 0 ]]; then
    failure="$ROOT/_agile-output/test-artifacts/129-3-raw/failure-$(date +%s)-$$"
    mkdir -p "$failure"
    if [[ -d "$TMP/artifacts" ]]; then
      tar -C "$TMP" --exclude=target --exclude='*.class' --exclude='*.jar' --exclude=simulation -cf "$failure/artifacts.tar" artifacts
    fi
    [[ ! -f "$TMP/change.patch" ]] || cp "$TMP/change.patch" "$failure/checkout.patch"
    [[ ! -f "$TMP/isolated-results.json" ]] || cp "$TMP/isolated-results.json" "$failure/"
  fi
  git -C "$ROOT" worktree remove --force "$TMP/checkout" >/dev/null 2>&1 || true
  rm -rf "$TMP"
  exit "$rc"
}
trap cleanup EXIT

git -C "$ROOT" worktree add --detach "$TMP/checkout" HEAD >/dev/null
git -C "$ROOT" diff --binary HEAD > "$TMP/change.patch"
if [[ -s "$TMP/change.patch" ]]; then
  git -C "$TMP/checkout" apply "$TMP/change.patch"
fi
git -C "$ROOT" ls-files --others --exclude-standard -z -- . ':(exclude)_agile-output/test-artifacts/129-3-raw/**' |
  tar -C "$ROOT" --null -T - -cf "$TMP/untracked.tar"
tar -C "$TMP/checkout" -xf "$TMP/untracked.tar"

export CARGO_TARGET_DIR="$TMP/cargo-target"
export BITLOOM_FR201_ARTIFACT_ROOT="$TMP/artifacts/backends"
export BITLOOM_FR201_FORMAL_ROOT="$TMP/artifacts/formal"
export BITLOOM_FR201_EVIDENCE_ROOT="$ROOT"
printf '%s\n' "$TMP/checkout" > "$TMP/child-token"
ISOLATED_RESULTS="$ROOT/_agile-output/test-artifacts/129-3-raw/isolated-$(date +%s)-$$.json"
cd "$TMP/checkout"
python3 _agile-output/test-artifacts/129-3-build-runner.py \
  --isolated-child "$TMP/child-token" --evidence "$ISOLATED_RESULTS"
python3 - "$ROOT" "$MAIN_RESULTS" "$ISOLATED_RESULTS" <<'PY'
import json
import os
from pathlib import Path
import sys
root, main, child = map(Path, sys.argv[1:])
sys.path.insert(0, str(root / '_agile-output/test-artifacts'))
from fr201_evidence import validate
value = json.loads(main.read_text())
validate(json.loads(child.read_text()), root, complete=False)
value['isolated_replay'] = {'exit_code': 0, 'used_main_target': False,
                            'used_hidden_tmp_rtl': False, 'evidence': os.path.relpath(child, root)}
validate(value, root, complete=True)
main.write_text(json.dumps(value, indent=2) + '\n')
PY
echo "Story129.3 isolated replay PASS evidence=$ISOLATED_RESULTS"
