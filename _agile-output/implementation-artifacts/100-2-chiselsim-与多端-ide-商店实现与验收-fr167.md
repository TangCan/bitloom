---
title: '100.2 ChiselSim 与多端 IDE 商店实现与验收（FR167）'
type: 'feature'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: 'dcd7ba4 Story 103.1: Epic 103 NFR14 risk record for Chisel HEAD Parser (FR170).'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic100-chiselsim-ide-stores-fr167.md'
  - '{project-root}/docs/fr167-chiselsim-ide-stores.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR167 requires (a) ChiselSim coupling and (b) Open VSX/JetBrains multi-store beyond FR162/FR134.

**Approach:** Emit chiselsim.* + ide-stores.* product artifacts from `cargo bitloom wave`; token/root FORCE_MISSING hard-fail; publish script honesty.

## Boundaries & Constraints

**Always:** both (a)+(b); non-zero missing token/root; Bitloom; NFR73 preserve FR162/FR134.

**Never:** silent-Ok; claim Epic 100 closed (→ 100.3); design-crate ChiselSim runtime.

</frozen-after-approval>

## Story

As a 维护者,
I want 使用完整 ChiselSim 耦合与额外 IDE 商店多端产品路径,
So that FR167 (a)+(b) 可验收。

## Acceptance Criteria

1. (a) `--chiselsim` emits manifests + validates root / FORCE_MISSING
2. (b) `--ide-stores` + publish script require OVSX_PAT + JETBRAINS_TOKEN
3. ATDD green; FR162/FR134 still valid

## Tasks / Subtasks

- [x] T1: bitloom-viz FR167 APIs
- [x] T2: CLI `--chiselsim` / `--ide-stores`
- [x] T3: publish script + docs
- [x] T4: ATDD + review + sprint 100-2 done / 100-3 ready

## Testing

- `cargo test -p bitloom --test fr167_chiselsim_ide_stores`
- `cargo fmt --all && just test`
