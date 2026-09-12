---
title: '90.2 LCOV GUI 一等集成实现与验收（FR158）'
type: 'feat'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: 'da4b31f Story 90.1: Epic 90 NFR14 risk record for FR158 LCOV GUI.'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic90-third-party-lcov-gui-fr158.md'
  - '{project-root}/docs/fr114-lcov-coverage-gui.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR114 exports LCOV + in-tree HTML; FR158 needs a first-class third-party `genhtml` path with explicit missing-tool failure.

**Approach:** `bitloom::lcov_gui::run_genhtml`; `cargo bitloom coverage --genhtml`; docs + ATDD (fake genhtml + empty PATH). FR114 alone ≠ FR158.

## Boundaries & Constraints

**Always:** genhtml contract; coverage.lcov input; explicit fail if missing; Bitloom brand.
**Never:** claim FR114 substitutes FR158; expand unrelated FR142 commands; Tywaves.

</frozen-after-approval>

## Story

As a 验证工程师,
I want 覆盖率产物可经一等路径打开或导入约定第三方 LCOV GUI,
So that 不必依赖临时脚本冒充产品面。

## Tasks / Subtasks

- [x] T1: `lcov_gui` helper + `--genhtml` CLI
- [x] T2: `docs/fr158-*` + FR114/FR142 notes
- [x] T3: ATDD
- [x] T4: sprint 90-2 done / 90-3 ready
- [x] T5: code-review + automation-summary
