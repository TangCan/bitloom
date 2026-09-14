---
title: '117.2 README / deferred / 状态页诚实更新（FR184）'
type: 'chore'
created: '2026-09-14'
status: 'done'
route: 'oneshot'
baseline_commit: '636a6b8 Story 117.1: Epic 117 NFR14 risk record for Phase 22 claim honesty (FR184).'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr184-phase22-claim-honesty.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Phase 22 deepen FRs closed without a single claim-honesty doc/map.

**Approach:** Add `docs/fr184-*` claim map; wire README/deferred pointers; keep Epic close for 117.3.

## Boundaries & Constraints

**Always:** Cite FR178–184; forbid Phase 21 alone / NFR81-empty; Bitloom.

**Never:** Claim Epic 117 / FR184 closed; claim NFR81 ledger empty.

</frozen-after-approval>

## Story

As a 维护者,
I want 公开状态页按已关 FR 诚实宣称 Phase 22,
So that 对外口径可审计。

## Acceptance Criteria

1. `docs/fr184-phase22-claim-honesty.md` claim map
2. README / deferred point at FR184 + deepen docs
3. FR184 still not marked closed (→ 117.3)

## Tasks / Subtasks

- [x] T1: docs/fr184-*
- [x] T2: README / deferred pointers
- [x] T3: ATDD
- [x] T4: code-review + automation-summary

## Testing

- `cargo test -p bitloom --test fr184_phase22_claim_honesty`
- 回归：`cargo clean && cargo fmt --all && just test`
