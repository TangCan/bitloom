---
title: '104.2 README / deferred / 状态页诚实更新（FR171）'
type: 'docs'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: '4cc8692 Story 104.1: Epic 104 NFR14 risk record for Phase 20 claim honesty (FR171).'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr171-phase20-claim-honesty.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic104-phase20-claim-honesty-fr171.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

Land FR171 honesty surface: claim map doc + README/deferred pointers for closed FR166–170; forbid Phase 19 alone / NFR71-empty; FR171 not closed yet (→ 104.3).

## Boundaries & Constraints

**Always:** Bitloom; NFR77; cite closed FRs.
**Never:** mark Epic 104 / FR171 closed here; clear NFR71/NFR76; Phase 19 alone.

</frozen-after-approval>

## Story

As a 维护者,
I want 公开状态页按已关 FR 诚实宣称 Phase 20,
So that 对外口径可审计。

## Tasks / Subtasks

- [x] T1: `docs/fr171-phase20-claim-honesty.md`
- [x] T2: README + deferred pointers
- [x] T3: ATDD
- [x] T4: sprint 104-2 done；104-3 ready-for-dev
- [x] T5: code-review + automation-summary

## Testing

- `cargo test -p bitloom --test fr171_phase20_claim_honesty`
- `cargo fmt --all && just test`
