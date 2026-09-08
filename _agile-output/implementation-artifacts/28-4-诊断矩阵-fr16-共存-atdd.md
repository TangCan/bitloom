---
title: '28.4 诊断矩阵 + FR16 共存 ATDD'
type: 'feature'
created: '2026-09-08'
status: 'done'
baseline_commit: 'a4ffc96547b029ed4749c3b9620ec062fcb9a1ac'
review_loop_iteration: 1
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/closure-decision-table-2026-09-08.md'
  - '{project-root}/_agile-output/specs/spec-rhdl/language-surface.md'
  - '{project-root}/_agile-output/implementation-artifacts/28-1-synthesizableclosure-约束与检查钩子.md'
  - '{project-root}/_agile-output/implementation-artifacts/28-2-组合逻辑内联可综合闭包.md'
  - '{project-root}/_agile-output/implementation-artifacts/28-3-时序逻辑内联-所有权.md'
warnings: []
deferred:
  - 'automatic rustc closure-body / borrow analysis → later when macros gain typed inline'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** QA needs a single ATDD matrix proving legal synthesizable closures (FR74/FR75) coexist with capturing-closure / heap / impure bans (FR16 / NFR35), plus NFR36 emit opacity and user-facing comb/seq examples.

**Approach:** One matrix integration test consolidates positive comb/seq inline + Cap-R-60 legal path, negatives E0141–E0146 / Wire capture, and NFR36 emit spot-check; deep rows stay in existing `fr74_*` / `fr75_*` / `fr73_*` fixtures (referenced). Docs: README + language-surface minimal comb/seq examples and “do not capture Wire” warning.

## Boundaries & Constraints

**Always:** FR74/FR75/FR16 matrix; NFR35 coexistence; NFR36 emit spot-check; Bitloom / `bitloom-prelude` design surface; reuse 28.1–28.3 APIs (no new closure IR).

**Never:** cargo clean / full `just test` as sole gate in this story run (targeted `cargo test` OK); reclassify E0141/E0142; store Fn in FrozenHir; invent automatic rustc body analysis.

## I/O & Edge-Case Matrix

| Scenario | Expected |
|----------|----------|
| Legal `inline_comb_fn` / `inline_seq_fn` | elaborate OK; ordinary HIR |
| Cap-R-60 empty / marker legal | pass |
| Heap / runtime capture / impure | `E0143` / `E0144` / `E0145` |
| Capture Wire (HwCaptureRef) | `E0142` |
| FR16 capturing closure | `E0141` (unchanged) |
| Cap-R-70 second Reg.d | `E0146` |
| Emit `.v` / FIRRTL after legal inline | no closure/callback IR (NFR36) |
| Docs | comb/seq minimal + do-not-capture-Wire |

</frozen-after-approval>

## Story

As a 质量负责人,
I want 正/负例矩阵证明可综合闭包与捕获闭包禁令共存,
So that FR74/FR75/FR16/NFR35 可回归.

## Acceptance Criteria

1. ATDD matrix: legal synthesizable closures pass; heap/capture/illegal IO fail; FR16 original negatives still fail (FR74, FR75, NFR35)
2. Stable under `just test` (or documented recipe) (FR74, FR75, NFR35)
3. Spot-check emit has no closure IR (NFR36)
4. User docs include comb/seq closure minimal examples + “do not capture Wire” warning

## Tasks / Subtasks

- [x] T1: ATDD `fr74_fr75_fr16_coexist_matrix.rs` (+ refs to fr74/fr75/fr73 fixtures)
- [x] T2: README + language-surface comb/seq examples + Wire capture warning
- [x] T3: fr22 construct bar — mark matrix delivered
- [x] T4: story artifact + code-review Approve + sprint-status (`28-4` + `epic-28` done)

## Dev Agent Record

### Agent Model Used

Composer (Auto)

### Debug Log References

- bmad-build `render_skill.py` HALT: ambiguous `implementation_artifacts` (bmm/gds); proceeded per Epic 28 artifact pipeline
- User constraint: no `cargo clean` / full `just test` / commit — targeted crate tests only

### Completion Notes List

- Matrix consolidates positive comb/seq inline + NFR36 emit; negatives E0141–E0146 / Wire capture; sibling fixture presence asserted
- Docs: README comb add + seq Inc; language-surface coexistence table + do-not-capture-Wire
- `epic-28: done` (all 28.x done)
- Deferred: automatic rustc body analysis

### File List

- `crates/bitloom/tests/fr74_fr75_fr16_coexist_matrix.rs`
- `README.md`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `docs/fr22-construct-bar.md`
- `_agile-output/implementation-artifacts/28-4-诊断矩阵-fr16-共存-atdd.md`
- `_agile-output/implementation-artifacts/28-4-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
