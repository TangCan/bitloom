---
title: '114.2 更深 Style Guide / linter 实现与验收（FR181）'
type: 'feature'
created: '2026-09-14'
status: 'done'
route: 'oneshot'
baseline_commit: '14efac6 Story 114.1: Epic 114 NFR14 risk record for Style/linter deepen (FR181).'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr181-deeper-style-guide-linter.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic114-deeper-style-guide-linter-fr181.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Product only had FR176 ecosystem pack; FR181 needs deeper Style Guide / linter.

**Approach:** Add wartremover + fatal-warnings deepen layer; revise AD-27; FORCE_MISSING non-zero; chain FR176 gate.

## Boundaries & Constraints

**Always:** ≠ FR176 alone; AD-27 revise; Bitloom.

**Never:** Claim full Style Guide 全家桶; claim FR182–184 closed; silent-Ok under FORCE_MISSING.

</frozen-after-approval>

## Story

As a 工具链维护者,
I want 产品路径交付超出 FR176 的 Style Guide / linter 加深,
So that NFR81 Style leftover 可勾选关闭。

## Acceptance Criteria

1. style-linter deepen path + ATDD/CI
2. AD-27 revised (NFR85); FR176 closes still valid (NFR83)
3. Wrong shape / FORCE_MISSING → non-zero readable failure

## Tasks / Subtasks

- [x] T1: API + script + just + CI + docs/fr181-*
- [x] T2: AD-27 revise
- [x] T3: ATDD `fr181_deeper_style_guide_linter`
- [x] T4: sprint 114.2 done；114.3 ready-for-dev
- [x] T5: code-review + automation-summary

## Testing

- `cargo test -p bitloom --test fr181_deeper_style_guide_linter`
- 回归：`cargo clean && cargo fmt --all && just test`
