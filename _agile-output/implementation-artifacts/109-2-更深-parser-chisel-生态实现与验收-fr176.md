---
title: '109.2 更深 Parser/Chisel 生态实现与验收（FR176）'
type: 'feature'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: 'b1c7665 Story 109.1: Epic 109 NFR14 risk record for Parser/Chisel ecosystem (FR176).'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr176-deeper-parser-chisel-ecosystem.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic109-deeper-parser-chisel-ecosystem-fr176.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR165 and FR170 alone do not close FR176 ecosystem deepen.

**Approach:** Combined gate: FR165 style-lint + FR170 parser-head + FR176 emit/check markers; AD-27 revise; CI + ATDD.

## Boundaries & Constraints

**Always:** ≠ FR170/165/138/130 alone; AD-27 revise; FORCE_MISSING non-zero; Bitloom.

**Never:** Claim FR177 closed; silent-Ok; AD-9 unpaired bump.

</frozen-after-approval>

## Story

As a 工具链/验证工程师,
I want 超出 FR170/FR165 的组合 Parser/Chisel 生态产品路径,
So that Phase 21 FR176 可宣称关闭。

## Acceptance Criteria

1. Combined just/CI gate + emit/check + docs/ATDD
2. FR170/165/138/130 closes remain valid (NFR78)
3. FORCE_MISSING / missing markers → non-zero; AD-27 revised

## Tasks / Subtasks

- [x] T1: emit/check FR176 + script + just + CI + docs
- [x] T2: AD-27 revise (NFR80)
- [x] T3: ATDD
- [x] T4: sprint 109.2 done；109.3 ready-for-dev
- [x] T5: code-review + automation-summary

## Testing

- `cargo test -p bitloom --test fr176_deeper_parser_chisel_ecosystem`
- `just chisel-ecosystem-deepen-check`（或 FORCE_MISSING 负向）
- 回归：`cargo fmt --all && just test`
