---
title: '91.2 MemRead 完整生成实现与验收（FR159）'
type: 'feat'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: '2ee0c3a Story 91.1: Epic 91 NFR14 risk record for FR159 MemRead.'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic91-memread-full-emit-fr159.md'
  - '{project-root}/docs/fr112-generated-functional-memread-equiv.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR112 closed in-process MemRead≡tick while `generate_functional_sim` still stubbed emitted `MemRead` as `0`.

**Approach:** Emit real MemRead/MemWrite + SyncReadMem latency-1 in the functional-sim crate; docs + ATDD; FR112 alone ≠ FR159.

## Boundaries & Constraints

**Always:** SyncReadMem latency-1 aligned with GeneratedFunctional/Sim::tick; Bitloom brand.
**Never:** stub-alone or FR112-alone close; multi-bank / full arbitration expand.

</frozen-after-approval>

## Story

As a 验证工程师,
I want 发出的功能仿真 crate 完整实现 MemRead（含 SyncReadMem latency-1）,
So that 不必只靠 in-process GeneratedFunctional 证明双模型。

## Tasks / Subtasks

- [x] T1: `generate.rs` emit MemRead / MemWrite / pending
- [x] T2: `docs/fr159-*` + FR112 honesty update
- [x] T3: ATDD `fr159_memread_full_emit`
- [x] T4: sprint 91-2 done / 91-3 ready
- [x] T5: code-review + automation-summary
