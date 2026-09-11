#!/usr/bin/env bash
# FR144 — surface-crate SemVer breakage gate (cargo-semver-checks).
# Documented entry: `just semver-check` (not part of default `just test`).
#
# Library surface crates (FR142): bitloom-prelude, bitloom-sim.
# CLI (`bitloom`) + macro (`bitloom-macro`) are in-surface via docs; not fully
# covered by rustdoc-based semver-checks.
#
# Missing tool / policy-violating breakage → non-zero (never continue-on-error).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export PATH="${HOME}/.cargo/bin:${PATH}"

echo "semver-check: FR144 Bitloom 1.0 surface gate (cargo-semver-checks)"
echo "semver-check: policy=docs/semver-1-0-policy.md surface=docs/public-api-1-0-surface.md"

if [[ "${BITLOOM_SEMVER_FORCE_MISSING:-0}" == "1" ]]; then
  echo "error: cargo-semver-checks unavailable (BITLOOM_SEMVER_FORCE_MISSING=1)" >&2
  echo "error: install with: cargo install cargo-semver-checks --locked" >&2
  echo "error: refusing silent success" >&2
  exit 1
fi

SEMVER_BIN=""
if command -v cargo-semver-checks >/dev/null 2>&1; then
  SEMVER_BIN="cargo-semver-checks"
elif cargo semver-checks --version >/dev/null 2>&1; then
  SEMVER_BIN="cargo semver-checks"
else
  echo "error: cargo-semver-checks not found on PATH / as cargo subcommand" >&2
  echo "error: install with: cargo install cargo-semver-checks --locked" >&2
  echo "error: then re-run: just semver-check" >&2
  echo "error: refusing silent success" >&2
  exit 1
fi

# Pre-1.0.0: default major (absorb known crates.io drift into upcoming 1.0).
# Post-1.0.0 (published on crates.io, FR153): default minor.
# Override with BITLOOM_SEMVER_RELEASE_TYPE.
# Historical: BITLOOM_SEMVER_ASSUME_PUBLISHED=1 forced minor during first 1.0.0;
# the 1.0.0 special-case was removed after library+CLI 1.0.0 landed on crates.io.
detect_release_type() {
  if [[ -n "${BITLOOM_SEMVER_RELEASE_TYPE:-}" ]]; then
    echo "${BITLOOM_SEMVER_RELEASE_TYPE}"
    return
  fi
  local ver
  ver="$(
    grep -m1 '^version' Cargo.toml 2>/dev/null | sed -E 's/.*"([^"]+)".*/\1/' || true
  )"
  if [[ "$ver" =~ ^0\. ]]; then
    echo "major"
  else
    echo "minor"
  fi
}

RELEASE_TYPE="$(detect_release_type)"
echo "semver-check: release-type=${RELEASE_TYPE} (tool=${SEMVER_BIN})"
${SEMVER_BIN} --version 2>&1 | head -n 3 | sed 's/^/semver-check: /' || true

PACKAGES=(
  bitloom-prelude
  bitloom-sim
)

run_check() {
  local pkg="$1"
  echo "semver-check: checking -p ${pkg} --release-type ${RELEASE_TYPE}"
  # shellcheck disable=SC2086
  if ! ${SEMVER_BIN} check-release -p "$pkg" --release-type "${RELEASE_TYPE}"; then
    echo "error: cargo-semver-checks failed for -p ${pkg}" >&2
    exit 1
  fi
}

for pkg in "${PACKAGES[@]}"; do
  run_check "$pkg"
done

echo "semver-check: OK (FR144)"
