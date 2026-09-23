#!/usr/bin/env bash
# Verify package coverage and error propagation independently of registry/network.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SEMVER_TEST_DIR="$(mktemp -d)"
trap 'rm -rf "$SEMVER_TEST_DIR"' EXIT
export SEMVER_TEST_LOG="$SEMVER_TEST_DIR/calls"
cargo-semver-checks() {
  if [[ "$#" == 1 && "$1" == --version ]]; then
    echo 'cargo-semver-checks test double'
    return 0
  fi
  [[ "$#" == 5 && "$1" == check-release && "$2" == -p && "$4" == --release-type && "$5" == "$SEMVER_TEST_EXPECT_RELEASE" ]] || return 90
  printf '%s %s %s %s %s\n' "$@" >> "$SEMVER_TEST_LOG"
  [[ "$3" != "${SEMVER_TEST_FAIL:-}" ]]
}
export -f cargo-semver-checks
unset BITLOOM_SEMVER_RELEASE_TYPE SEMVER_TEST_FAIL BITLOOM_SEMVER_FORCE_MISSING
# Current workspace is post-1.0: an unset override must select minor.
for release_type in minor major; do
  export SEMVER_TEST_EXPECT_RELEASE="$release_type"
  if [[ "$release_type" == major ]]; then export BITLOOM_SEMVER_RELEASE_TYPE=major; fi
  : > "$SEMVER_TEST_LOG"
  bash "$ROOT/scripts/semver-check.sh" > "$SEMVER_TEST_DIR/output" 2>&1
  printf 'check-release -p %s --release-type %s\n' bitloom-prelude "$release_type" bitloom-sim "$release_type" bitloom-firrtl "$release_type" > "$SEMVER_TEST_DIR/expected"
  diff -u "$SEMVER_TEST_DIR/expected" "$SEMVER_TEST_LOG"
done
for pkg in bitloom-prelude bitloom-sim bitloom-firrtl; do
  export SEMVER_TEST_FAIL="$pkg"
  if bash "$ROOT/scripts/semver-check.sh" > "$SEMVER_TEST_DIR/output" 2>&1; then
    echo "failure for $pkg was swallowed" >&2
    exit 1
  fi
  grep -q "failed for -p ${pkg}" "$SEMVER_TEST_DIR/output"
done
if BITLOOM_SEMVER_FORCE_MISSING=1 bash "$ROOT/scripts/semver-check.sh" > "$SEMVER_TEST_DIR/output" 2>&1; then
  echo 'missing-tool failure was swallowed' >&2
  exit 1
fi
echo 'semver script package coverage and failure propagation: passed'
