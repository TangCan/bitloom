# Code review — Story 13.3

**Verdict:** approve

## Findings
- Host shim uses crates.io `bitloom-vlog` / `bitloom-hir` at `CARGO_PKG_VERSION` when not in monorepo (`crates/rhdl-vlog` absent).
- Monorepo auto path backends avoid duplicate `bitloom_hir` (registry vs path design); overrides `BITLOOM_FORCE_REGISTRY` / `BITLOOM_DEV_PATH` documented in code.
- `--package` resolved via `cargo metadata` (FR51); no longer assumes only `examples/<name>`.
- AD-14 preserved: in-process elaborate + emit in generated host main.

## Must not regress
- Standalone host Cargo.toml must not contain `crates/rhdl-vlog` path
- Version pin must track CLI package version
