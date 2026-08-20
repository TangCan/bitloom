# Changelog

All notable changes to the **Bitloom** (`bitloom`) publish surface are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
with Cargo's **0.x** compatibility rules (leftmost non-zero component is the public API axis).

## [Unreleased]

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
