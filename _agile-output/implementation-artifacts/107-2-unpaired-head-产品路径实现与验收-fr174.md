---
title: '107.2 unpaired HEAD 产品路径实现与验收（FR174）'
type: 'feature'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: '2ab050d Story 107.1: Epic 107 NFR14 risk record for unpaired HEAD (FR174).'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr174-unpaired-head.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic107-unpaired-head-fr174.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Product only had paired AD-9 firtool / FR170 update-mainline; FR174 needs an unpaired HEAD/mainline channel.

**Approach:** Document-pin unpaired firtool-1.156.0; `circt-unpaired-head-check` + CI; revise AD-9/AD-27; FORCE_MISSING non-zero.

## Boundaries & Constraints

**Always:** ≠ FR170 alone; AD revise before ready; ≠ AD-9 product pin as HEAD; Bitloom.

**Never:** Claim floating git HEAD beyond 1.156.0; claim FR175–177 closed.

</frozen-after-approval>

## Story

As a 维护者/互操作消费者,
I want 使用 unpaired CIRCT/Chisel HEAD（或文档钉死未配对主线）产品路径,
So that 不再仅停留在 FR170 @ paired pin 同钉面。

## Acceptance Criteria

1. Document-pinned unpaired firtool-1.156.0 path + ATDD/CI
2. AD-9 / AD-27 revised (NFR80); FR170 close still valid (NFR78)
3. Missing / FORCE_MISSING → non-zero readable failure

## Tasks / Subtasks

- [x] T1: script + just + CI + docs/fr174-*
- [x] T2: AD-9 / AD-27 revise
- [x] T3: ATDD `fr174_unpaired_head`
- [x] T4: sprint 107.2 done；107.3 ready-for-dev
- [x] T5: code-review + automation-summary

## Testing

- `cargo test -p bitloom --test fr174_unpaired_head`
- 回归：`cargo fmt --all && just test`
