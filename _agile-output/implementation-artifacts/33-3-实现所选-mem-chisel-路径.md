---
title: '33.3 实现所选 Mem/Chisel 路径'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: '0c43ea4'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/fr81-mem-chisel-contract-decision-2026-09-09.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic33-chisel-mem.md'
  - '{project-root}/_agile-output/implementation-artifacts/33-2-mem-chisel-合同决策.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/docs/fr28-chisel-compilable.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** `emit_chisel` still rejects all `MemDecl` with E0901; FR81 Path A (Story 33.2) requires a documented Mem subset to lower to compilable Chisel Scala.

**Approach:** In `crates/rhdl-firrtl/src/chisel.rs`, lower Path A subset:
- `Mem` (`sync_read=false`) → Chisel `Mem`
- `SyncReadMem` (`sync_read=true`) → Chisel `SyncReadMem`
- optional constant `init` → mechanical VecInit + reset-time fill (or equivalent that compiles on NFR12 stack)
Also emit real `MemWrite` / `MemRead` (not comment stubs). Keep E0901 for out-of-subset (e.g. init length ≠ depth, depth/width 0). Do not drift NFR12; do not break FR71 no-Mem goldens.

## Boundaries & Constraints

**Always:** Path A only per FR81 decision page; NFR12 Chisel 7.14.0 ↔ firtool 1.155.0; E0901 retained for out-of-subset; FR71 counter golden unchanged; Bitloom brand / `bitloom-prelude` for design crates.

**Ask First:** Private firtool/Chisel upgrade to make Mem compile.

**Never:** Delete E0901 wholesale; claim full Mem surface; break `fr28_golden_counter` / FR71; implement Path B; full `cargo clean && just test` in this story run.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| SyncReadMem in subset | `declare_sync_read_mem` | Scala contains `SyncReadMem(` | N/A |
| Mem in subset | `declare_mem` | Scala contains `Mem(` (not SyncReadMem) | N/A |
| Optional init | `declare_*_with_init` | Scala has Mem/SyncReadMem + init fill | N/A |
| Out-of-subset init len | MemDecl init len ≠ depth (seal bypass) | `rhdl::E0901` | structured error |
| No Mem (FR71) | counter HIR | emit succeeds; pin strings unchanged | N/A |
| NFR12 pins | CHISEL_TARGET / FIRTOOL_TARGET | 7.14.0 / 1.155.0 | no private bump |

</frozen-after-approval>

## Code Map

- `crates/rhdl-firrtl/src/chisel.rs` — Path A MemDecl / MemWrite / MemRead emit; E0901 for OOS
- `crates/rhdl-firrtl/src/lib.rs` — update `chisel_fr28_*` tests
- `crates/bitloom/tests/fr81_path_a_mem_chisel_emit.rs` — ATDD Path A
- `crates/bitloom/tests/fr73_crc_lut_golden.rs` — Chisel path now succeeds (no E0901)
- `docs/fr28-chisel-compilable.md` — Mem subset note
- `_agile-output/specs/spec-rhdl/language-surface.md` — Mem↔Chisel boundary one-liner

## Story

As a 工具链维护者,
I want `emit_chisel` 按决策收敛 Mem 行为,
So that FR81 可实现验收。

## Acceptance Criteria

1. Given Story 33.2 Path A decision, when implementing Mem subset → compilable Scala via `emit_chisel`, then behavior matches FR81 decision
2. And Chisel/firtool versions remain NFR12 pinned pair (no private upgrade)
3. And Mem shapes outside the decision still fail clearly (E0901 retained; not deleted wholesale)
4. And FR71 existing no-Mem JVM goldens must not break

## Tasks / Subtasks

- [x] T1: Path A Mem/SyncReadMem (+ init) emit in `chisel.rs` (AC: 1, 2)
- [x] T2: MemWrite/MemRead real Scala; E0901 for out-of-subset (AC: 1, 3)
- [x] T3: ATDD + unit tests; update fr73 Chisel expectation (AC: 1, 3, 4)
- [x] T4: Light docs; sprint → done after review Approve (AC: all)

## Dev Notes

- Decision authority: `fr81-mem-chisel-contract-decision-2026-09-09.md`
- Prefer mechanical Scala that compiles on pinned stack; reset-time init fill is OK for constant init
- Existing `chisel_fr28_fails_on_mem` must become success for in-subset SyncReadMem
- Out-of-subset test: `BuilderOwnedHir` + mismatched init length → E0901
- Do not change FR71 golden counter fixture contents unless necessary (should not be)
- Story 33.4 owns full JVM ATDD / `just chisel-fr28-jvm` regression; this story focuses on emit + Rust predicates

### Project Structure Notes

- Emit lives only in `rhdl-firrtl` Chisel leg; FIRRTL `mem` text path unchanged (AD-3)

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- Path A: `Mem`/`SyncReadMem` + optional VecInit reset fill; MemWrite/MemRead real Scala
- E0901 retained for OOS (init len mismatch / zero depth|width)
- ATDD `fr81_path_a_mem_chisel_emit`; unit `chisel_fr81_*`; fr73 Chisel now expects Mem emit
- Review Approve; sprint `33-3-…: done`; `epic-33: in-progress`

### File List

- `crates/rhdl-firrtl/src/chisel.rs`
- `crates/rhdl-firrtl/src/lib.rs`
- `crates/bitloom/tests/fr81_path_a_mem_chisel_emit.rs`
- `crates/bitloom/tests/fr73_crc_lut_golden.rs`
- `docs/fr28-chisel-compilable.md`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `_agile-output/implementation-artifacts/33-3-实现所选-mem-chisel-路径.md`
- `_agile-output/implementation-artifacts/33-3-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Story 33.3 Path A Mem→Chisel emit implementation

## Suggested Review Order

**chisel.rs Path A emit** → **E0901 OOS** → **unit/ATDD** → **fr73 Chisel** → **docs/pins** → **sprint**
