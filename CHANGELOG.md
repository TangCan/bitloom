# Changelog

All notable changes to the **Bitloom** (`bitloom`) publish surface are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
From **1.0.0**, in-surface breaking changes require a **major** bump
([`docs/semver-1-0-policy.md`](docs/semver-1-0-policy.md) / FR143). Pre-1.0 history
followed Cargo **0.x** compatibility rules.

## [Unreleased]

## [1.0.0] - 2026-09-11

### Added

- Phase 17 API stability gate: public surface (`docs/public-api-1-0-surface.md`), SemVer 1.0 policy, `just semver-check` / CI FR144 gate, FR145-skip hygiene decision
- SemVer **1.0.0** promise for in-surface crates: `bitloom`, `bitloom-prelude`, `bitloom-macro`, `bitloom-sim`

### Changed

- Workspace package version **0.1.2 → 1.0.0** (lockstep for publishable Bitloom crates)
- Post-1.0 default `cargo-semver-checks` release-type **minor** (override via `BITLOOM_SEMVER_RELEASE_TYPE`)

### Notes

- Out-of-promise crates (`bitloom-hir` / `bitloom-builder` / `bitloom-vlog`) may share the workspace version but are **not** covered by the 1.0 SemVer stability promise (Q2)
- **NFR59** product deepen items remain deferred (NFR63); 1.0 ≠ empty backlog
- crates.io upload: see `docs/fr146-bitloom-1-0-0-release.md` (dry-run + manual checklist)

## [0.1.2] - 2026-08-20

### Added

- True-standalone path: `cargo install bitloom` → `cargo bitloom new` → `cargo bitloom build` without cloning the monorepo
- Host shim resolves `--package` via `cargo metadata` and pins `bitloom-vlog` / `bitloom-hir` from crates.io outside the monorepo
- MVP library family on crates.io: `bitloom-{hir,builder,macro,prelude,vlog}` (lockstep 0.1.2)
- Optional `bitloom-sim` 0.1.2 for `cargo add bitloom-sim --dev` cycle-accurate tick

### Changed

- Design crates depend on **`bitloom-prelude`** (not CLI `bitloom`); README quick start is install-first

## [0.1.1] - 2026-08-19

### Added

- Crate metadata: `repository` / `homepage` / `documentation` → https://github.com/TangCan/bitloom

## [0.1.0] - 2026-08-19

### Added

- Initial crates.io registration: [`bitloom`](https://crates.io/crates/bitloom) 0.1.0
- Public brand **Bitloom**; binary `cargo-bitloom` (`cargo bitloom`)
- Architecture AD-2 publish identity locked to `bitloom`
- Maturity contract: SECURITY.md, SemVer 0.x policy, CI on rustc 1.97.1
