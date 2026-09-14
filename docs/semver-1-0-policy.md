# Bitloom SemVer 1.0 policy (FR143)

> **Contract:** Phase 17 / **FR143** / Epic 81.  
> **Surface:** [`docs/public-api-1-0-surface.md`](public-api-1-0-surface.md) (FR142).  
> **Legacy 0.x:** [`docs/semver-0x-policy.md`](semver-0x-policy.md) (NFR15) remains historical for the 0.x era.

## Relationship to NFR15 / 0.x

- During maturity closeout, the toolchain stayed on **0.x** and closing a sprint was **not** a reason to ship 1.0 (**NFR15** / `semver-0x-policy.md`).
- **This FR authorizes** promoting the **FR142 in-surface** crates to SemVer **1.0.0** once FR141–145 (incl. FR145-skip) are closed and Epic 83 / FR146 executes the release.
- Closing a sprint or backlog is **still not** an automatic major bump.

## In-surface breaking changes → major

Once **1.0.0** is published for an in-surface crate:

- Removing or renaming a documented public API / CLI verb → **major**.
- Tightening documented accepted inputs in a way that breaks callers → **major**.
- Expanding in-surface is allowed in **minor** / **patch** only when additive and documented.
- **FR183 / Epic 116:** documented promote of `bitloom-firrtl` interop subset into in-surface is an **additive** expand → **minor** on the next `bitloom-firrtl` publish after the surface revision (see [`docs/public-api-1-0-surface.md`](public-api-1-0-surface.md) / [`docs/fr183-explicit-fr142-api-expand.md`](fr183-explicit-fr142-api-expand.md)). Silent expand remains forbidden.

Out-of-promise crates (`bitloom-hir` / `bitloom-builder` / `bitloom-vlog`) may continue to change without a Bitloom 1.0 major **unless** they break the prelude/sim in-surface contract.

## Deprecation window

- Prefer a documented deprecation in a minor release before removal in the next major.
- Emergency security fixes may skip deprecation with an explicit CHANGELOG note.

## MSRV (Q5)

- Current MSRV: **Rust 1.97.1** (`rust-version` in workspace `Cargo.toml`).
- Default: **keep** MSRV for 1.0.
- Raising MSRV requires an explicit policy/doc update and is treated as a **minor**-incompatible change under Cargo rules (document + bump accordingly).

## CI gate (FR144)

- Local: `just semver-check` → `scripts/semver-check.sh`
- CI required job: `semver-check` (no `continue-on-error`)
- Tool: **`cargo-semver-checks`** against FR142 **library** surface crates (`bitloom-prelude`, `bitloom-sim`)
- `bitloom` CLI verbs and `bitloom-macro` are in-surface via documentation; they are **not** fully covered by rustdoc-based semver-checks (macro has no conventional lib API; CLI is binary-first)
- **Pre-1.0.0:** default `--release-type major` so known crates.io drift may be absorbed by the upcoming 1.0 major; tool must still run; missing tool → **non-zero**
- **After 1.0.0 is on crates.io (FR153):** default `--release-type minor` for workspace version ≥1.0.0 (the temporary 1.0.0 / `BITLOOM_SEMVER_ASSUME_PUBLISHED` special-case was **removed** once library + CLI 1.0.0 were published)
- Override anytime with `BITLOOM_SEMVER_RELEASE_TYPE`
- Missing tool or policy-violating breakage → **non-zero** with readable diagnostics；`BITLOOM_SEMVER_FORCE_MISSING=1` forces the missing-tool path for ATDD

## Brand

- Public product name **Bitloom**; crates.io / CLI **`bitloom`** / `bitloom-*`.
- Never publish `rhdl` / `rhdl-bits` as the product name.
