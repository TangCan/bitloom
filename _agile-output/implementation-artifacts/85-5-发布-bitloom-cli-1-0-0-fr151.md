---
title: '85.5 发布 bitloom CLI 1.0.0（FR151）'
type: 'feature'
created: '2026-09-11'
status: 'done'
route: 'oneshot'
baseline_commit: '4673939 Story 85.4: Adopt FR152(b) so bitloom-lsp does not block CLI publish.'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr151-bitloom-cli-publish.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Bitloom CLI is not yet on crates.io despite library 1.0.0 and firrtl/viz publishability.

**Approach:** With FR149–152(b) satisfied, publish `bitloom` 1.0.0; document `cargo install bitloom`; ATDD locks versioned deps + dry-run; do not expand FR142.

## Boundaries & Constraints

**Always:** versioned deps; dry-run; try live publish; Bitloom brand.
**Never:** expand FR142; clear NFR59; Epic 86 honesty (→ 86); start before 85.2–85.4.

</frozen-after-approval>

## Story

As a 用户/维护者,
I want 从 crates.io 安装 Bitloom CLI,
So that `cargo install bitloom` / `cargo bitloom` 可用。

## Acceptance Criteria

1. version 依赖齐；`cargo publish -p bitloom` 成功（1.0.0）
2. `cargo install bitloom` 文档路径可验证
3. 不扩大 FR142 表面
4. ATDD/清单勾选 FR151

## Tasks / Subtasks

- [x] docs/fr151 + ATDD
- [x] dry-run + live publish
- [x] sprint / review / just test / commit
