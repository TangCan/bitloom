---
title: '85.3 bitloom-viz 可发布（FR150）'
type: 'feature'
created: '2026-09-11'
status: 'done'
route: 'oneshot'
baseline_commit: '52bf4c4 Story 85.2: Publish bitloom-firrtl 1.0.0 as crates.io FIRRTL package.'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/crates/rhdl-viz/Cargo.toml'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** `rhdl-viz` is `publish=false` without registry version, blocking CLI publish.

**Approach:** Rename package to `bitloom-viz` (dir may stay `rhdl-viz`), `publish=true`, version `1.0.0`, update refs, dry-run + live publish.

## Boundaries & Constraints

**Always:** package `bitloom-viz`; AD-2; never publish as `rhdl-viz`.
**Never:** lsp policy / CLI publish / Epic 86.

</frozen-after-approval>

## Story

As a 维护者,
I want 将现 `rhdl-viz` 变为可 crates.io 发布的 `bitloom-viz`,
So that CLI 可对 viz 腿使用 version 依赖。

## Acceptance Criteria

1. `publish=true`; dry-run OK; live publish勾选
2. 仓库引用 → `bitloom-viz`；目录可暂留
3. 禁止 `rhdl-viz` 发布名
4. ATDD + `docs/fr150-bitloom-viz-publish.md`

## Tasks / Subtasks

- [x] T1–T5: rename, ATDD, docs, dry-run, live publish, sprint, review, just test, commit

## Dev Agent Record

### Completion Notes List

- Published **bitloom-viz 1.0.0** to crates.io (when upload succeeds)

### File List

- `crates/rhdl-viz/Cargo.toml`, `Cargo.toml`, CLI/tests/docs, ATDD, sprint
