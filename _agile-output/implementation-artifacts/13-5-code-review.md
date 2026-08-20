# Code review — Story 13.5

**Verdict:** approve

## Findings
- ATDD `standalone_empty_dir` runs `new` + `build` in a fresh temp dir; asserts non-empty `.v` and registry-pinned host (no `crates/rhdl-vlog` path).
- Relies on published `bitloom-*` 0.1.2 matching CLI version; needs network for crates.io on cold cache.
- Fixed `new` hint: `--out-dir out` (not `{name}/out`) under `--manifest-dir {name}`.

## Must not regress
- Empty-dir path must not require git clone of TangCan/bitloom
- Host backends outside monorepo stay version-pinned crates.io
