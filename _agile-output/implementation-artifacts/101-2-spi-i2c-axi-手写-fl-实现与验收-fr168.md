---
title: '101.2 SPI·I2C·AXI 手写 FL 实现与验收（FR168）'
type: 'feature'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: '82e8ff6 Story 100.3: Close Epic 100 FR167 with honesty documentation.'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic101-spi-i2c-axi-handwritten-fl-fr168.md'
  - '{project-root}/docs/fr168-spi-i2c-axi-handwritten-fl.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR168 requires handwritten FL for SpiMaster + I2cMaster + Axi4LiteSlave (all three).

**Approach:** Add three AbstractionView functionals + dual stimuli + verify_* APIs in bitloom-sim; ATDD Pass/Fail; docs.

## Boundaries & Constraints

**Always:** all three; ≠ FR163 alone; ≠ GeneratedFunctional alone; bitloom-prelude for design crates.

**Never:** claim Epic 101 closed (→ 101.3); silent-Ok wrong models.

</frozen-after-approval>

## Story

As a 设计者,
I want SPI、I2C、AXI 协议各自具备手写 FL / 双模型验收,
So that IP 诚实面覆盖 FR163 未交付的三条协议。

## Tasks / Subtasks

- [x] T1: SpiMasterFunctional + verify
- [x] T2: I2cMasterFunctional + verify
- [x] T3: Axi4LiteSlaveFunctional + verify
- [x] T4: docs + ATDD + review

## Testing

- `cargo test -p bitloom --test fr168_spi_i2c_axi_handwritten_fl`
- `cargo fmt --all && just test`
