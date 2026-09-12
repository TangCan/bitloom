---
title: '88.3 live 发布 bitloom-lsp（FR155）'
type: 'feat'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: 'c349df1 Story 88.2: Make bitloom-lsp publishable and adopt FR152(a).'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr155-bitloom-lsp-publish.md'
  - '{project-root}/docs/fr152-bitloom-lsp-publish-policy.md'
  - '{project-root}/crates/bitloom-lsp/Cargo.toml'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR152(a) packaging is ready but `bitloom-lsp` is not yet on crates.io.

**Approach:** Live `cargo publish -p bitloom-lsp` at workspace **1.0.0**; document upload + `cargo install bitloom-lsp`; ATDD locks evidence. Formal Epic 88/README close → 88.4.

## Boundaries & Constraints

**Always:** live upload or documented failure; Bitloom / `bitloom-lsp` name; protect FR151; (a)≠ deepen.
**Never:** claim FR155 epic closed without 88.4; expand FR142; publish `rhdl-*`.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Live publish | credentials present | Published bitloom-lsp v1.0.0 | Doc failure + no FR155 close |
| ATDD | fr155 doc + publish=true | Green without re-uploading | N/A |

</frozen-after-approval>

## Code Map

- `docs/fr155-bitloom-lsp-publish.md` — live evidence + install
- `docs/fr152-bitloom-lsp-publish-policy.md` — pointer to live success
- `crates/bitloom/tests/fr155_bitloom_lsp_publish.rs` — ATDD

## Story

As a 用户/维护者,
I want 从 crates.io 获得已发布的 `bitloom-lsp`,
So that FR152(a) 可公开勾选。

## Acceptance Criteria

1. Given 88.2 dry-run, when live publish 1.0.0, then crates.io has the version
2. And install path documented; failure would block FR155 close
3. Brand Bitloom; no `rhdl-*` publish name

## Tasks / Subtasks

- [x] T1: docs/fr155 + policy pointer
- [x] T2: live `cargo publish -p bitloom-lsp` → Published v1.0.0
- [x] T3: ATDD fr155_bitloom_lsp_publish.rs
- [x] T4: sprint 88-3 done / 88-4 ready; review + automation

## File List

- `docs/fr155-bitloom-lsp-publish.md`
- `docs/fr152-bitloom-lsp-publish-policy.md`
- `crates/bitloom/tests/fr155_bitloom_lsp_publish.rs`
- `_agile-output/implementation-artifacts/88-3-live-发布-bitloom-lsp-fr155.md`
- `_agile-output/implementation-artifacts/88-3-code-review.md`
- `_agile-output/implementation-artifacts/88-3-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
