---
title: '112.2 浮动 CIRCT git HEAD 实现与验收（FR179）'
type: 'feature'
created: '2026-09-14'
status: 'done'
route: 'oneshot'
baseline_commit: 'b042dab Story 112.1: Epic 112 NFR14 risk record for floating CIRCT HEAD (FR179).'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr179-floating-circt-git-head.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic112-floating-circt-git-head-fr179.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Product only had AD-9 paired pin and FR174 document-pinned unpaired **1.156.0**; FR179 needs a floating CIRCT HEAD **track** beyond FR174.

**Approach:** Document-pin floating-track firtool-**1.159.0**; `circt-floating-git-head-check` + CI; revise AD-9 (NFR85); FORCE_MISSING non-zero; unbounded live tip still NFR86.

## Boundaries & Constraints

**Always:** ≠ FR174 alone; ≠ AD-9 product pin as floating HEAD; AD revise before ready; Bitloom.

**Never:** Claim unbounded live tip without pin; claim FR180–184 closed; silent-Ok under FORCE_MISSING.

</frozen-after-approval>

## Story

As a 维护者/互操作消费者,
I want 使用文档钉死的浮动 CIRCT HEAD track 产品路径,
So that 不再仅停留在 FR174 @ 1.156.0。

## Acceptance Criteria

1. Document-pinned floating-track firtool-1.159.0 path + ATDD/CI
2. AD-9 revised (NFR85); FR174/FR173 closes still valid (NFR83)
3. Missing / FORCE_MISSING → non-zero readable failure

## Tasks / Subtasks

- [x] T1: script + just + CI + docs/fr179-*
- [x] T2: AD-9 revise (+ FR174 leftover pointer honesty)
- [x] T3: ATDD `fr179_floating_circt_git_head`
- [x] T4: sprint 112.2 done；112.3 ready-for-dev
- [x] T5: code-review + automation-summary

## Testing

- `cargo test -p bitloom --test fr179_floating_circt_git_head`
- 回归：`cargo clean && cargo fmt --all && just test`
