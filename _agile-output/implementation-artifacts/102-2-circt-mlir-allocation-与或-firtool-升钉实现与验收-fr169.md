---
title: '102.2 CIRCT/MLIR allocation 与/或 firtool 升钉实现与验收（FR169）'
type: 'feature'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: '796c91c Story 101.3: Close Epic 101 FR168 with honesty documentation.'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic102-circt-mlir-firtool-fr169.md'
  - '{project-root}/docs/fr169-circt-mlir-allocation.md'
warnings: []
deferred:
  - 'Option (B) firtool bump / AD-9 revise — deferred (NFR76); no Chisel formal pairing in this story'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR169 requires CIRCT/MLIR allocation and/or paired firtool bump beyond FR164 sim.

**Approach:** Deliver NFR14 option **(A)** multi-lower at AD-9 `firtool-1.155.0`: `--ir-fir` + `--ir-hw` + Verilog via `circt-external-alloc-check`; required CI job; ATDD. Option **(B)** not selected.

## Boundaries & Constraints

**Always:** ≠ FR164 alone; ≠ FR137 alone; ≠ FR129 alone; AD-9 pin; non-zero FORCE_MISSING.

**Never:** claim Epic 102 closed (→ 102.3); PATH-random firtool; unpaired CIRCT HEAD bump.

</frozen-after-approval>

## Story

As a 维护者,
I want 超出 FR164 的 CIRCT/MLIR allocation 或配对后 firtool 升钉产品路径,
So that 工具链加深可勾选。

## Tasks / Subtasks

- [x] T1: `scripts/circt-external-alloc-check.sh` + fixture
- [x] T2: `just circt-external-alloc-check` + CI `circt-external-alloc`
- [x] T3: `docs/fr169-circt-mlir-allocation.md` + ATDD
- [x] T4: review + regression

## Testing

- `cargo test -p bitloom --test fr169_circt_mlir_allocation`
- `just circt-external-alloc-check`
- `cargo fmt --all && just test`
