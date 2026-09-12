---
title: '108.2 更广 CIRCT/MLIR/sim 实现与验收（FR175）'
type: 'feature'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: '14c9303 Story 108.1: Epic 108 NFR14 risk record for broader CIRCT/SV (FR175).'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr175-broader-circt-mlir-sim.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR169 only covers fir+hw multi-lower; FR175 needs SV / ir-verilog deepen.

**Approach:** `circt-external-sv-check` with `--ir-sv` + `--ir-verilog` @ AD-9 1.158.0; CI + ATDD + FORCE_MISSING.

## Boundaries & Constraints

**Always:** ≠ FR169 alone; AD-9 pin; Bitloom; non-zero on missing.

**Never:** Claim FR176–177 closed; claim Handshake dialect family cleared.

</frozen-after-approval>

## Story

As a 工具链/验证工程师,
I want 超出 FR169 钉死子集的 CIRCT/MLIR/sim 产品路径,
So that allocation/sim 加深面可宣称关闭。

## Acceptance Criteria

1. `--ir-sv` + `--ir-verilog` gate + docs/ATDD/CI
2. FR169/164/137 close still valid (NFR78)
3. FORCE_MISSING / version mismatch → non-zero

## Tasks / Subtasks

- [x] T1: script + just + CI + docs
- [x] T2: ATDD
- [x] T3: sprint 108.2 done；108.3 ready-for-dev
- [x] T4: code-review + automation-summary

## Testing

- `cargo test -p bitloom --test fr175_broader_circt_mlir_sim`
- 回归：`cargo fmt --all && just test`
