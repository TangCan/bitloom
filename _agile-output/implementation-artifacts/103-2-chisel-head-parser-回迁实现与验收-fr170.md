---
title: '103.2 Chisel HEAD Parser 回迁实现与验收（FR170）'
type: 'feature'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: 'bd999e8 Story 102.3: Close Epic 102 FR169 with honesty documentation.'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic103-chisel-head-parser-fr170.md'
  - '{project-root}/docs/fr170-chisel-head-parser.md'
warnings: []
deferred:
  - 'Unpaired CIRCT HEAD / firtool bump beyond AD-9 — NFR76 (Epic 102 option B deferred)'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR170 requires Chisel HEAD / update-mainline Parser beyond FR138/FR165, with AD-27 revise.

**Approach:** Document-pinned update-mainline path: `BitloomFirrtlParser.parseUpdateMainline` on FIRRTL 6.0.0 @ Chisel 7.14.0 ↔ firtool-1.155.0; AD-27 revise; AD-9 unchanged; ATDD/CI.

## Boundaries & Constraints

**Always:** ≠ FR138 alone; ≠ FR165 alone; ≠ FR130 alone; AD-27 revise before ready; FORCE_MISSING non-zero.

**Never:** claim Epic 103 closed (→ 103.3); PATH-random firtool; unpaired HEAD bump without AD-9.

</frozen-after-approval>

## Story

As a 维护者/Chisel 消费者,
I want 相对现行钉死对的 HEAD（或更新主线）Parser 产品路径,
So that Parser 生态可跟上游主线回迁。

## Tasks / Subtasks

- [x] T1: fixture + `parser-head-migration-check.sh` + Scala façade
- [x] T2: AD-27 revise + docs/fr170
- [x] T3: Just + CI + ATDD
- [x] T4: review + regression

## Testing

- `cargo test -p bitloom --test fr170_chisel_head_parser`
- `just parser-head-migration-check`
- `cargo fmt --all && just test`
