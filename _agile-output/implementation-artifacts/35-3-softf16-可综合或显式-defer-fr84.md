---
title: '35.3 SoftF16 可综合或显式 defer（FR84）'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: 'fc00dc6'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic35-residual-partials.md'
  - '{project-root}/_agile-output/implementation-artifacts/35-1-epic-35-nfr14-风险记录.md'
  - '{project-root}/docs/fr36-rhdl-float.md'
  - '{project-root}/crates/rhdl-float/src/lib.rs'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md'
warnings: []
deferred:
  - 'SoftF16 → HIR → emit synthesizable float operators (FR84 Option A) → future deepen / post-Epic-35'
  - 'Full IEEE SoftF16 synthesizable operator library → out of Epic 35 scope'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR36 historical done delivered host-only `SoftF16` goldens; emit treats values as `Bits<16>`. No SoftF16 float-operator → HIR → emit fixture exists. Without an explicit FR84 choice, docs/crate copy can market host SoftF16 as synthesizable float (NFR37 / NFR14 (c)).

**Approach:** Prefer **Path B — explicit defer** (user/pipeline guidance): contract SoftF16 synthesizable lowering as **deferred**; PRD addendum + user docs must declare **must not claim synthesizable SoftF16 / float delivered**; tick NFR14 FR84 Option B. Do **not** ship a fake `Bits<16>`-as-float Option A fixture.

## Boundaries & Constraints

**Always:** Story 35.1 gate; choose exactly one of FR84 Option A or B; Path B = PRD/user-doc explicit defer + forbid synthesizable-float delivery claims; close condition auto- or doc-checkable; Bitloom brand; NFR37 honesty; AD-18 (no capturing closures into `tick`).

**Ask First:** 无（Path B is the preferred default unless a real Option A fixture appears）.

**Never:** Claim SoftF16 synthesizable float delivered while on Path B; market host-only SoftF16 or `Bits<16>` bitvector emit as synthesizable float ops; invent Option A fixture that only renames `Bits<16>`; skip NFR14 Option B tick; leave docs claiming synthesizable float ops → HIR.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Path choice | Epic 35.3 | Exactly one: **B** (defer) | Dual claim A+B → fail review |
| User docs | `docs/fr36-rhdl-float.md` | host-only; FR84 deferred; forbid synthesizable claim | ATDD doc scan |
| PRD | addendum FR84 paragraph | Option B contract + NFR37 | ATDD scan |
| NFR14 | options table + close checklist | FR84 **已选 B**; FR84 checkbox ticked | ATDD / review |
| Crate surface | `rhdl-float` | host golden; no HIR float-op emit claim | ATDD source scan |
| False Option A | Bits\<16\> rename only | Rejected — not Path A | Never ship |

</frozen-after-approval>

## Code Map

- `docs/fr36-rhdl-float.md` — FR84 Path B user contract (host-only; deferred synthesizable)
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` — FR84 Option B addendum
- `crates/rhdl-float/src/lib.rs` — crate docs honesty (host-only; not synthesizable float ops)
- `_agile-output/implementation-artifacts/nfr14-risk-epic35-residual-partials.md` — FR84 已选 B + close checkbox
- `crates/bitloom/tests/fr84_softf16_explicit_defer.rs` — ATDD / doc-checkable close

## Story

As a 设计者 / PM,
I want SoftF16 要么进入可综合降低，要么合同化 defer,
So that FR36 深度诚实。

## Acceptance Criteria

1. Given Story 35.1, when 二选一落地：（A）SoftF16 → HIR → emit 可综合夹具；或（B）PRD/用户文档显式 deferred，并声明不得声称可综合浮点已交付（FR84 / NFR37）, then 关闭条件可自动或文档检查勾选
2. And 若选（A），至少一黄金数值/位宽夹具；若选（B），addendum 或等价合同段落存在且与实现一致

## Tasks / Subtasks

- [x] T1: Confirm Path B (no real SoftF16→HIR→emit float-op fixture without fake Bits\<16\> claims)（AC: 1）
- [x] T2: PRD addendum + `docs/fr36-rhdl-float.md` Path B contract; crate docs align（AC: 1, 2）
- [x] T3: ATDD `fr84_softf16_explicit_defer` guards path + forbid synthesizable claims（AC: 1）
- [x] T4: NFR14 FR84 已选 B + FR84 close checkbox; review / sprint done

## Dev Notes

- **Decision:** Path B — explicit defer. SoftF16 remains host RTE golden (FR36 minimal contract). Emit `Bits<16>` surface is **not** synthesizable float operators.
- Do not implement float ALU HIR nodes in this story.
- Prefer targeted tests: `cargo test -p bitloom --test fr84_softf16_explicit_defer` (+ optional `cargo test -p rhdl-float`).
- Do **not** run full `cargo clean && just test`; do **not** commit (parent owns commit).
- Leave `_bmad/scripts/render_skill.py.bak` untracked.

### Project Structure Notes

- Docs + ATDD + NFR14 + light crate-doc fix only; no new synthesizable float crate API.

### References

- [Source: `_agile-output/planning-artifacts/epics.md` Story 35.3]
- [Source: `nfr14-risk-epic35-residual-partials.md` FR84 Option B]
- [Source: `docs/fr36-rhdl-float.md`; `crates/rhdl-float`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

- `cargo test -p bitloom --test fr84_softf16_explicit_defer`
- `cargo test -p rhdl-float`
- `cargo test -p bitloom --test nfr14_risk_epic35_residual_partials`

### Completion Notes List

- FR84 Option B: explicit defer SoftF16→HIR→emit synthesizable float
- PRD addendum + `docs/fr36-rhdl-float.md` forbid synthesizable delivery claims
- `rhdl-float` crate docs aligned host-only / FR84
- NFR14 FR84 已选 B + close checkbox
- Code review Approve; no commit (parent)

### File List

- `docs/fr36-rhdl-float.md`
- `crates/rhdl-float/src/lib.rs`
- `crates/bitloom/tests/fr84_softf16_explicit_defer.rs`
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic35-residual-partials.md`
- `_agile-output/implementation-artifacts/35-3-softf16-可综合或显式-defer-fr84.md`
- `_agile-output/implementation-artifacts/35-3-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Story context created (ready-for-dev)
- 2026-09-09: Path B implemented; review Approve; done

## Suggested Review Order

**fr36 docs** → **PRD addendum** → **NFR14 tick** → **crate docs** → **ATDD**
