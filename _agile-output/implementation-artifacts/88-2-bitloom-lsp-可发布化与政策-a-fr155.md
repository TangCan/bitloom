---
title: '88.2 bitloom-lsp 可发布化与政策 (a)（FR155）'
type: 'feat'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: 'a809dfe Story 88.1: Epic 88 NFR14 risk record for FR152(a)/FR155.'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr152-bitloom-lsp-publish-policy.md'
  - '{project-root}/crates/bitloom-lsp/Cargo.toml'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic88-bitloom-lsp-fr152a-fr155.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** `bitloom-lsp` still `publish=false` under FR152(b); live crates.io publish cannot proceed without packaging + policy rewrite to (a).

**Approach:** Set `publish = true` + crates.io metadata; keep CLI free of lsp path dep (FR151); rewrite `docs/fr152-*` to **(a)** with honesty that (b) remains historically closed; ATDD asserts dry-run for lsp and bitloom. Live publish → 88.3.

## Boundaries & Constraints

**Always:** dry-run green; policy (a); (a) ≠ LSP deepen; protect FR151; NFR72 no install claim yet.
**Never:** live `cargo publish`（→ 88.3）；Epic 88 close（→ 88.4）；expand FR142.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Packaging | `publish=true` + versioned workspace deps | `cargo publish -p bitloom-lsp --dry-run` 成功 | Fix deps/metadata |
| CLI path | bitloom has no lsp dep | `cargo publish -p bitloom --dry-run` 仍绿 | Restore FR151 boundary |
| Honesty | docs claim (a) selected | ATDD 绿；(b) marked superseded | Rewrite doc |

</frozen-after-approval>

## Code Map

- `crates/bitloom-lsp/Cargo.toml` — `publish = true` + metadata
- `docs/fr152-bitloom-lsp-publish-policy.md` — (b)→(a)
- `crates/bitloom/tests/fr152_bitloom_lsp_publish_policy.rs` — ATDD

## Story

As a 维护者,
I want `bitloom-lsp` 变为可 crates.io 发布并重写 FR152 政策为 (a),
So that live publish 具备前置条件。

## Acceptance Criteria

1. Given Story 88.1, when set publish=true + align deps/metadata + rewrite fr152 to (a), then dry-run succeeds
2. And docs: (b) superseded; (a)=live target; ≠ LSP deepen
3. And ATDD verifies; FR151 path unbroken

## Tasks / Subtasks

- [x] T1: `publish = true` + repository/homepage/documentation
- [x] T2: Rewrite `docs/fr152-bitloom-lsp-publish-policy.md` to (a)
- [x] T3: Rewrite ATDD `fr152_bitloom_lsp_publish_policy.rs`
- [x] T4: dry-run lsp + bitloom; sprint → 88-2 done / 88-3 ready
- [x] T5: code-review + automation-summary

## File List

- `crates/bitloom-lsp/Cargo.toml`
- `docs/fr152-bitloom-lsp-publish-policy.md`
- `crates/bitloom/tests/fr152_bitloom_lsp_publish_policy.rs`
- `_agile-output/implementation-artifacts/88-2-bitloom-lsp-可发布化与政策-a-fr155.md`
- `_agile-output/implementation-artifacts/88-2-code-review.md`
- `_agile-output/implementation-artifacts/88-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
