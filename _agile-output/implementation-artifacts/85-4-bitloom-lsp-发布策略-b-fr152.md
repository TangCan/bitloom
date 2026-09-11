---
title: '85.4 bitloom-lsp 发布策略 (b)（FR152）'
type: 'chore'
created: '2026-09-11'
status: 'done'
route: 'oneshot'
baseline_commit: '7d322f1 Story 85.3: Publish bitloom-viz 1.0.0 as crates.io visualization package.'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr152-bitloom-lsp-publish-policy.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** path-only `bitloom-lsp` on the CLI package risks blocking or silently complicating `cargo publish -p bitloom`.

**Approach:** Adopt FR152 **(b)**: keep lsp `publish=false`; remove lsp from `bitloom` deps; move lib-using ATDD into `bitloom-lsp/tests/`; document that (a) needs a new contract.

## Boundaries & Constraints

**Always:** (b) non-blocking; document; ATDD dry-run without lsp on registry.
**Never:** FR152(a) publish lsp; expand FR142; swallow NFR59.

</frozen-after-approval>

## Story

As a 维护者,
I want 钉死 lsp 不挡 `bitloom` 打包的策略,
So that CLI 可上架而不强制 lsp crates.io 发布。

## Acceptance Criteria

1. 成文策略 (b)；调整依赖使 publish 不因 lsp path 失败
2. 文档：lsp 默认不上架；(a) 须新合同
3. 不得 silent 保留挡 publish 的 path-only
4. ATDD 断言打包不要求 bitloom-lsp 在 registry

## Tasks / Subtasks

- [x] Remove bitloom-lsp from bitloom Cargo.toml
- [x] Move lib ATDD to bitloom-lsp/tests
- [x] docs/fr152 + ATDD + review + just test + commit

## Dev Agent Record

### Completion Notes List

- FR152(b) documented; CLI dry-run succeeds without lsp on registry
- Lib ATDD relocated to `bitloom-lsp/tests/`
