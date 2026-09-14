---
title: '113.2 Handshake dialect 加深实现与验收（FR180）'
type: 'feature'
created: '2026-09-14'
status: 'done'
route: 'oneshot'
baseline_commit: 'b531bac Story 113.1: Epic 113 NFR14 risk record for Handshake deepen (FR180).'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr180-handshake-dialect-deepen.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic113-handshake-dialect-deepen-fr180.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Product only had FR129 func/buffer + multi-clock; FR180 needs Handshake dialect deepen.

**Approach:** Add `handshake.fork`+`handshake.join` deepen path; revise AD-25; FORCE_MISSING non-zero; FR129 remains regressable.

## Boundaries & Constraints

**Always:** ≠ FR129 alone; AD-25 revise; Bitloom.

**Never:** Claim full Handshake lower suite; claim FR181–184 closed; silent-Ok under FORCE_MISSING.

</frozen-after-approval>

## Story

As a 工具链维护者,
I want 产品路径交付超出 FR129 / FR175 的 Handshake dialect 加深,
So that NFR81 Handshake leftover 可勾选关闭。

## Acceptance Criteria

1. fork+join deepen path + ATDD/CLI
2. AD-25 revised (NFR85); FR129/FR175 closes still valid (NFR83)
3. Wrong shape / FORCE_MISSING → non-zero readable failure

## Tasks / Subtasks

- [x] T1: API + CLI + docs/fr180-*
- [x] T2: AD-25 revise
- [x] T3: ATDD `fr180_handshake_dialect_deepen`
- [x] T4: sprint 113.2 done；113.3 ready-for-dev
- [x] T5: code-review + automation-summary

## Testing

- `cargo test -p bitloom --test fr180_handshake_dialect_deepen`
- 回归：`cargo clean && cargo fmt --all && just test`
