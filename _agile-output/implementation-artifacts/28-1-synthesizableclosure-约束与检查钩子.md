---
title: '28.1 SynthesizableClosure 约束与检查钩子'
type: 'feature'
created: '2026-09-08'
status: 'done'
baseline_commit: 'd44337d009554312996b5ded39437416c44e4610'
review_loop_iteration: 1
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/closure-decision-table-2026-09-08.md'
  - '{project-root}/_agile-output/specs/spec-rhdl/language-surface.md'
  - '{project-root}/_agile-output/implementation-artifacts/27-4-atdd-golden-后端无感知抽检.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-phase9-closures.md'
warnings: []
deferred:
  - 'comb/seq inlined synthesizable closures → Story 28.2 / 28.3'
  - 'full FR74/FR75/FR16 diagnostic matrix ATDD → Story 28.4'
  - 'automatic rustc closure-body analysis → later when macros gain typed inline'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Designers lack a documented SynthesizableClosure contract and reachable check entry before comb/seq inline (FR75); FR74 Cap-R-48…50 / Cap-R-60 must land first.

**Approach:** Document marker trait `SynthesizableClosure` (pure / no heap / no runtime capture state) plus Cap-R-60 hook `ElaborateSession::check_synthesizable_closure` / `reject_unsynthesizable_closure` (+ free `diagnose_synthesizable_closure_violations`). Stable codes **E0143** (heap), **E0144** (runtime capture state), **E0145** (impure). Reuse E0141/E0142 patterns; no new closure IR (NFR36).

## Boundaries & Constraints

**Always:** FR74 documentation + check hook; stable diagnostic codes; legal empty/simple pass; Bitloom / `bitloom-prelude`; NFR36 (no FIRRTL/Chisel closure IR); Epic 27 green; Phase 9 NFR14 already covered by `nfr14-risk-phase9-closures.md` (Epic 26–30).

**Never:** comb/seq inline expansion (28.2/28.3); silent success on documented violations; new closure nodes in FrozenHir/backends; reclassify E0141/E0142.

## I/O & Edge-Case Matrix

| Scenario | Expected |
|----------|----------|
| `reject_unsynthesizable_closure(heap)` | `rhdl::E0143` |
| `check_synthesizable_closure([runtime_capture_state])` | `rhdl::E0144` |
| `reject_unsynthesizable_closure(impure)` | `rhdl::E0145` |
| empty check / `LegalEmptyClosure` / `LegalSimpleClosure` | success (paves 28.2) |
| FrozenHir after legal check | no closure IR (NFR36) |
| FR16 `reject_unsynthesizable("capturing closure")` | still `rhdl::E0141` |

</frozen-after-approval>

## Story

As a 设计者,
I want 明确的可综合闭包约束与检查入口,
So that 我知道哪些闭包能进硬件路径。

## Acceptance Criteria

1. Document `SynthesizableClosure` (or equiv): pure, no heap, no runtime capture state (FR74 / Cap-R-48…50)
2. Provide prelude/builder or `cargo bitloom check` reachable check hook (Cap-R-60)
3. Violating examples → stable diagnostic codes (FR74)
4. Legal empty/simple closures pass check (paves 28.2)
5. No new closure IR in FIRRTL/Chisel (NFR36)

## Tasks / Subtasks

- [x] T1: `SynthesizableClosure` trait + `SynthesizableClosureViolation` (E0143–E0145) + diagnose free fn
- [x] T2: `ElaborateSession::check_synthesizable_closure` / `reject_unsynthesizable_closure` / marker helper
- [x] T3: Prelude re-export + language-surface + fr22 construct bar
- [x] T4: ATDD `fr74_synthesizable_closure_check.rs`
- [x] T5: Builder unit tests
- [x] T6: code-review Approve + sprint-status (`epic-28: in-progress`, `28-1: done`)
- [x] T7: NFR14 note — Phase 9 record (`nfr14-risk-phase9-closures.md`) already covers Epic 28; no separate epic-28 NFR14 story

## Dev Agent Record

### Agent Model Used

Composer (Auto)

### Debug Log References

- bmad-build `render_skill.py` HALT: ambiguous `implementation_artifacts` (bmm/gds); proceeded per Epic 27 artifact pipeline

### Completion Notes List

- Cap-R-60 surface: session check + free `diagnose_synthesizable_closure_violations` (CLI check equivalent)
- E0143/E0144/E0145 distinct from E0141 (FR16) and E0142 (hw capture)
- NFR14: Epic 26.1 Phase 9 record covers 27–30; no epic-28-only NFR14 file
- Deferred: comb/seq inline (28.2/28.3), full matrix (28.4)

### File List

- `crates/bitloom-builder/src/lib.rs`
- `crates/bitloom-prelude/src/lib.rs`
- `crates/bitloom/tests/fr74_synthesizable_closure_check.rs`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `docs/fr22-construct-bar.md`
- `_agile-output/implementation-artifacts/28-1-synthesizableclosure-约束与检查钩子.md`
- `_agile-output/implementation-artifacts/28-1-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
