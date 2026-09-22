#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
export_dir="${BITLOOM_REPLAY_EXPORT_DIR:-$root/_agile-output/test-artifacts}"
work_root="$(mktemp -d "${TMPDIR:-/tmp}/bitloom-129-2-replay.XXXXXX")"
checkout="$work_root/checkout"
patch="$work_root/working-tree.patch"
log="$work_root/replay.log"
manifest="$work_root/replay-manifest.txt"

git -C "$root" diff --binary HEAD >"$patch"
git clone --no-hardlinks --quiet "$root" "$checkout"
if [[ -s "$patch" ]]; then
  git -C "$checkout" apply "$patch"
fi

mkdir -p "$work_root/cargo-target" "$work_root/rtl" "$export_dir"
{
  printf 'utc_start=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  printf 'source_root=%s\n' "$root"
  printf 'isolated_checkout=%s\n' "$checkout"
  printf 'head=%s\n' "$(git -C "$checkout" rev-parse HEAD)"
  printf 'working_tree_patch_sha256=%s\n' "$(sha256sum "$patch" | cut -d' ' -f1)"
  printf 'cargo_target_dir=%s\n' "$work_root/cargo-target"
  printf 'rtl_artifact_root=%s\n' "$work_root/rtl"
  printf 'rustc=%s\n' "$(rustc --version)"
  printf 'cargo=%s\n' "$(cargo --version)"
  printf 'python=%s\n' "$(python3 --version 2>&1)"
  printf 'iverilog=%s\n' "$(iverilog -V 2>&1 | head -n1)"
  printf 'vvp=%s\n' "$(vvp -V 2>&1 | head -n1)"
  printf 'yosys=%s\n' "$(yosys -V 2>&1 | head -n1)"
  printf '%s\n' 'source_sha256:'
  find "$checkout/crates/bitloom/tests/fr198_peripheral_system.rs" \
    "$checkout/crates/bitloom/tests/fr198_peripheral_system" \
    "$checkout/crates/bitloom-prelude/src/ip/axi_lite_csr.rs" \
    "$checkout/crates/bitloom-prelude/src/ip/csr_decoder.rs" \
    "$checkout/crates/bitloom-prelude/src/ip/gpio/csr.rs" \
    "$checkout/crates/bitloom-prelude/src/ip/irq.rs" \
    "$checkout/crates/bitloom-prelude/src/ip/timer.rs" \
    "$checkout/crates/bitloom-prelude/src/ip/uart_csr.rs" \
    "$checkout/_agile-output/test-artifacts/129-2-automate.py" \
    "$checkout/Cargo.toml" \
    "$checkout/rust-toolchain.toml" \
    -type f -print0 | sort -z | xargs -0 sha256sum
} >"$manifest"

set +e
(
  cd "$checkout"
  CARGO_TARGET_DIR="$work_root/cargo-target" \
    BITLOOM_FR198_ARTIFACT_ROOT="$work_root/rtl" \
    python3 _agile-output/test-artifacts/129-2-automate.py
) 2>&1 | tee "$log"
status=${PIPESTATUS[0]}
set -e

printf 'utc_end=%s\nexit_code=%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$status" >>"$manifest"
cp "$manifest" "$export_dir/129-2-isolated-replay-manifest.txt"
cp "$log" "$export_dir/129-2-isolated-replay.log"
if [[ -f "$checkout/_agile-output/test-artifacts/129-2-latest-results.json" ]]; then
  cp "$checkout/_agile-output/test-artifacts/129-2-latest-results.json" "$export_dir/129-2-latest-results.json"
fi
if [[ -f "$checkout/_agile-output/test-artifacts/129-2-behavior-evidence.tar.gz" ]]; then
  cp "$checkout/_agile-output/test-artifacts/129-2-behavior-evidence.tar.gz" "$export_dir/129-2-behavior-evidence.tar.gz"
fi

if [[ "$status" -ne 0 ]]; then
  printf 'Replay failed; isolated workspace preserved at %s\n' "$work_root" >&2
  exit "$status"
fi

rm -rf "$work_root"
printf 'ISOLATED REPLAY PASS: manifest and evidence exported to %s\n' "$export_dir"
