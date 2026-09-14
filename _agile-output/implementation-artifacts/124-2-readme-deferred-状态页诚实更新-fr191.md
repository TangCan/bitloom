---
title: '124.2 README / deferred / 状态页诚实更新（FR191）'
type: 'chore'
created: '2026-09-14'
status: 'done'
route: 'oneshot'
baseline_commit: '0db6b2f Story 124.1: Epic 124 NFR14 risk record for Phase 23 claim honesty (FR191).'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr191-phase23-claim-honesty.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Phase 23 deepen FRs need a single claim-honesty doc/map; FR189 must stay honest blocked.

**Approach:** Add `docs/fr191-*` claim map; wire README/deferred pointers; keep Epic close for 124.3.

## Boundaries & Constraints

**Always:** Cite FR185–191; forbid Phase 22/closeout alone / NFR86-empty; list FR189 blocked; Bitloom.

**Never:** Claim Epic 124 / FR191 closed; claim FR189 delivered; claim NFR86 ledger empty.

</frozen-after-approval>

## Story

As a 维护者,
I want 公开状态页反映 Phase 23 宣称纪律,
So that 读者须引对应 FR。

## Acceptance Criteria

1. `docs/fr191-phase23-claim-honesty.md` claim map (incl. FR189 blocked)
2. README / deferred point at FR191 + deepen docs
3. FR191 still not marked closed (→ 124.3)

## Tasks / Subtasks

- [x] T1: docs/fr191-*
- [x] T2: README / deferred pointers
- [x] T3: ATDD
- [x] T4: code-review + automation-summary

## Testing

- `cargo test -p bitloom --test fr191_phase23_claim_honesty`
- 回归：`cargo clean && cargo fmt --all && just test`
