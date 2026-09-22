#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
TMP="$(mktemp -d)"
cleanup() {
  rc=$?
  if [[ $rc -ne 0 && -d "$TMP/checkout/target/fr201-runner" ]]; then
    rm -rf "$ROOT/target/fr201-isolated-failure"
    mkdir -p "$ROOT/target/fr201-isolated-failure"
    cp -a "$TMP/checkout/target/fr201-runner" "$ROOT/target/fr201-isolated-failure/runner"
    [[ ! -d "$TMP/artifacts" ]] || cp -a "$TMP/artifacts" "$ROOT/target/fr201-isolated-failure/"
  fi
  git -C "$ROOT" worktree remove --force "$TMP/checkout" >/dev/null 2>&1 || true
  rm -rf "$TMP"
  exit "$rc"
}
trap cleanup EXIT

git -C "$ROOT" worktree add --detach "$TMP/checkout" HEAD >/dev/null
git -C "$ROOT" diff --binary HEAD > "$TMP/change.patch"
git -C "$TMP/checkout" apply "$TMP/change.patch"
git -C "$ROOT" ls-files --others --exclude-standard -z |
  tar -C "$ROOT" --null -T - -cf "$TMP/untracked.tar"
tar -C "$TMP/checkout" -xf "$TMP/untracked.tar"

export CARGO_TARGET_DIR="$TMP/cargo-target"
export BITLOOM_FR201_ARTIFACT_ROOT="$TMP/artifacts/backends"
export BITLOOM_FR201_FORMAL_ROOT="$TMP/artifacts/formal"
export BITLOOM_FR201_ISOLATED=1
cd "$TMP/checkout"
python3 _agile-output/test-artifacts/129-3-build-runner.py \
  --skip-isolated \
  --evidence "$TMP/isolated-results.json"
MAIN_RESULTS="$ROOT/_agile-output/test-artifacts/129-3-latest-results.json"
MERGED_RESULTS="$TMP/merged-results.json"
jq --slurpfile isolated "$TMP/isolated-results.json" \
  '.isolated_replay = $isolated[0].isolated_replay' \
  "$MAIN_RESULTS" > "$MERGED_RESULTS"
mv "$MERGED_RESULTS" "$MAIN_RESULTS"
echo "Story129.3 isolated replay PASS target=$CARGO_TARGET_DIR"
