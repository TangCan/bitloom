---
title: '27.1 LUT/常量初值生成器闭包 API'
type: 'feature'
created: '2026-09-08'
status: 'done'
baseline_commit: 'e0939c01b819bbf5a364396bec56b408833480b8'
review_loop_iteration: 1
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/closure-decision-table-2026-09-08.md'
  - '{project-root}/_agile-output/specs/spec-rhdl/language-surface.md'
warnings: []
deferred:
  - 'comb/seq inlined synthesizable closures → Epic 28'
  - 'capture Wire/Reg diagnostics → Story 27.3'
  - 'CRC golden vs handwritten table tick → Story 27.4'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Designers must hand-write large Mem/ROM/LUT tables; Epic 26 unlocked elaborate-time non-capturing `Fn` (AD-18 revised / FR73) but no positive generator API yet.

**Approach:** Minimal elaborate-time API on `ElaborateSession` (design crates via `bitloom-prelude`): run `Fn(usize) -> u64` inside the session, store plain init words on `Stmt::MemDecl`, emit as Verilog `initial` (and FIRRTL comment). Closure dissolves before freeze — no HIR closure nodes.

## Boundaries & Constraints

**Always:** FR73 MVP init path; NFR36 (no closure residue); AD-1/7/13 via ElaborateSession; Bitloom / `bitloom-prelude` only for design surface.

**Never:** comb/seq inlined closures (Epic 28); require capturing hardware Signal/Reg (27.3); rustc-time netlist extract; FIRRTL/Chisel closure IR.

## I/O & Edge-Case Matrix

| Scenario | Expected |
|----------|----------|
| `declare_*_mem_with_init_fn` + emit `.v` | mem + `initial` with table values |
| Freeze after generator | MemDecl.init is `Vec<u64>`; no Closure stmt/expr |
| depth/width 0 or init len mismatch | diagnostic fail |
| Design crate deps | only `bitloom-prelude` |

</frozen-after-approval>

## Story

As a 设计者,
I want 在 elaborate 时用 `Fn`（或等价）生成 Mem/ROM/常量初值,
So that 不必手写大表也能得到可综合初始化数据。

## Acceptance Criteria

1. Documented API (`declare_*_mem_with_init_fn` / `generate_mem_init`) runs user `Fn` inside `ElaborateSession` and writes init
2. Fixture: closure-built table visible as mem init via elaborate → emit `.v` (and/or FIRRTL) (FR73)
3. After freeze, FrozenHir has **no** closure nodes/residue (NFR36)
4. Obey AD-1/7/13; no rustc-time netlist extract
5. Does **not** require comb/seq inlined closures (→ Epic 28)

## Tasks / Subtasks

- [x] T1: HIR `MemDecl.init: Option<Vec<u64>>`
- [x] T2: Builder `declare_mem_with_init_fn` / `declare_sync_read_mem_with_init_fn` (+ raw `with_init` vec path)
- [x] T3: Prelude `generate_mem_init` + language-surface Mem/FR73 snippet
- [x] T4: vlog `initial` + firrtl init comment; sim loads init
- [x] T5: ATDD red→green (`fr73_mem_init_generator.rs`)
- [x] T6: code-review Approve + sprint-status

## Dev Agent Record

### Agent Model Used

Composer (Auto)

### Debug Log References

- bmad-build `render_skill.py` HALT: ambiguous `implementation_artifacts` (bmm/gds); proceeded per established Epic 26 artifact pipeline
- Review finding: width≤64 gate scoped to init path only

### Completion Notes List

- FR73 MVP: elaborate-time `Fn(usize)->u64` → `MemDecl.init`; Verilog `initial`; FIRRTL `; mem-init`
- Prelude re-exports `generate_mem_init`; language-surface + fr22 construct bar updated
- ATDD green; code-review Approve; epic-27 in-progress, 27-1 done

### File List

- `crates/bitloom-hir/src/lib.rs`
- `crates/bitloom-builder/src/lib.rs`
- `crates/bitloom-prelude/src/lib.rs`
- `crates/bitloom-vlog/src/lib.rs`
- `crates/rhdl-firrtl/src/lib.rs`
- `crates/bitloom-sim/src/lib.rs`
- `crates/bitloom/Cargo.toml`
- `crates/bitloom/tests/fr73_mem_init_generator.rs`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `docs/fr22-construct-bar.md`
- `_agile-output/implementation-artifacts/27-1-lut-常量初值生成器闭包-api.md`
- `_agile-output/implementation-artifacts/27-1-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
