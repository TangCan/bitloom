---
title: '33.4 ATDD + FR71 回归'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: 'b3a1e3d'
review_loop_iteration: 0
followup_review_recommended: false
context:
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/fr81-mem-chisel-contract-decision-2026-09-09.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic33-chisel-mem.md'
  - '{project-root}/_agile-output/implementation-artifacts/33-3-实现所选-mem-chisel-路径.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/docs/fr28-chisel-compilable.md'
  - '{project-root}/crates/bitloom/tests/fr81_path_a_mem_chisel_emit.rs'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Stories 33.2–33.3 delivered FR81 Path A `emit_chisel` Mem subset, but Epic 33 lacks UJ close-out: Mem contract fixtures + FR71 JVM regression evidence, NFR14 Epic 33 close checkboxes unticked, and Mem↔Chisel user/maintainer boundary docs incomplete for depth honesty (NFR37).

**Approach:** ATDD + fixtures close-out (mirror 32.4 / 31.4). Keep FR71 no-Mem golden (`fr28_golden_counter.scala`) + `just chisel-fr28-jvm` / GHA `fr28-chisel-jvm` green and unchanged as the required gate. Add Path A Mem contract Scala fixture(s) under `testdata/` + maintainer recipe to compile under NFR12 when JDK17+sbt present. Extend Rust/docs ATDD locking Path A positives, OOS E0901, FR71 gate scripts, docs boundary, and NFR14 Epic 33 ticks. Do **not** weaken FR71, drift NFR12, or claim full Mem surface.

## Boundaries & Constraints

**Always:** Depend on 33.2–33.3 Path A; ATDD covers Path A emit + OOS E0901; FR71 counter golden + required scripts stay green/unchanged contract; document Mem Path A JVM fixture + NFR12 pin; tick NFR14 Epic 33 close; Bitloom brand; AD-27 / FR81 / NFR37.

**Ask First:** Adding Mem fixture as a second **required** GHA step (default: document + optional `just` only so FR71 surface stays the no-Mem counter).

**Never:** `cargo clean` / full `just test` as forced gate; git commit/push; delete E0901 wholesale; private Chisel/firtool bump; set `BITLOOM_CHISEL_JVM_SKIP` in CI; weaken `fr28-chisel-jvm` / `continue-on-error`.

## I/O & Edge-Case Matrix

| Scenario | Expected |
|----------|----------|
| Path A SyncReadMem / Mem+init | Existing `fr81_path_a_*` emit ATDD still green |
| OOS MemDecl | E0901 still asserted |
| FR71 no-Mem golden | `fr28_golden_counter.scala` unchanged; `just chisel-fr28-jvm` still points at it |
| FR71 script ATDD | `just chisel-fr28-atdd` / `test-just-chisel-fr28-jvm.sh` still PASS |
| Mem Path A fixture | Checked-in Scala under testdata; docs + optional just recipe for NFR12 compile |
| User/maintainer docs | Mem↔Chisel Path A / E0901 / FR71 boundary updated |
| NFR14 | Epic 33 close checklist all `[x]` + evidence |
| Still OOS | Dual-clock naked mem; multi-port/mask beyond HIR; Path B |

</frozen-after-approval>

## Code Map

- `crates/rhdl-firrtl/testdata/fr81_path_a_sync_read_mem.scala` — **NEW** Mem Path A JVM contract fixture
- `crates/bitloom/tests/fr81_mem_chisel_atdd_fr71.rs` — **NEW** Story 33.4 close-out ATDD
- `docs/fr28-chisel-compilable.md` — Mem fixture + FR71 regression notes
- `Justfile` — optional `chisel-fr81-mem-jvm` (Mem fixture; FR71 counter recipe unchanged)
- `_agile-output/specs/spec-rhdl/language-surface.md` — Story 33.4 close mark
- `_agile-output/implementation-artifacts/nfr14-risk-epic33-chisel-mem.md` — Epic 33 close ticks
- `crates/bitloom/tests/fr81_path_a_mem_chisel_emit.rs` — keep green (33.3)

## Story

As a 质量负责人,
I want Mem 合同夹具与 JVM CI 门禁回归,
So that 深度不破坏 Phase 8.

## Acceptance Criteria

1. Given Story 33.3, when 增加 ATDD 覆盖决策路径；并跑 `just chisel-fr28-jvm`（或等价）回归, then 测试稳定通过（FR81）
2. And FR71 / GHA `fr28-chisel-jvm` 合同路径仍绿（或不因本变更变红）
3. And 用户/维护者文档更新 Mem↔Chisel 边界
4. And NFR14 记录勾选 Epic 33 关闭条件

## Tasks / Subtasks

- [x] T1: Mem Path A contract Scala fixture + optional just recipe (AC: 1–2)
- [x] T2: ATDD close-out locking Path A / OOS / FR71 / docs / NFR14 (AC: 1–4)
- [x] T3: Update fr28 docs + language-surface Mem↔Chisel boundary (AC: 3)
- [x] T4: Tick NFR14 Epic 33 close; code-review Approve; sprint `33-4` + `epic-33` → done (AC: 4)

## Spec Change Log

- 2026-09-09: FR81 UJ close-out — Mem Path A JVM fixture; FR71 regression ATDD; NFR14 Epic 33 close.

## Design Notes

Docs + contract fixtures + matrix ATDD; FR71 required surface stays no-Mem counter. Optional Mem JVM via same compile script.

## Verification

**Commands:**
- `cargo test -p bitloom --test fr81_mem_chisel_atdd_fr71` — **PASS** (6)
- `cargo test -p bitloom --test fr81_path_a_mem_chisel_emit` — **PASS** (4)
- `bash scripts/test-just-chisel-fr28-jvm.sh` — **PASS**
- `bash scripts/test-gha-fr28-chisel-jvm.sh` — **PASS**
- `just chisel-fr28-jvm` — expected fail on Java 11 (required contract); CI Temurin 17 covers true compile

## Review Triage Log

### 2026-09-09 — Formal review（Approve）
- intent_gap: 0
- bad_spec: 0
- patch: 0
- defer: none (local JDK17 true Mem compile optional for maintainers)

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

- ATDD close-out + Path A emit + decision + NFR14 risk tests PASS
- FR71 just/GHA script ATDD PASS
- testarch-automate: close-out ATDD covers AC; no extra suite
- User constraint: no `cargo clean` / full `just test` / commit

### Completion Notes List

- Mem Path A fixture `fr81_path_a_sync_read_mem.scala` + optional `just chisel-fr81-mem-jvm`
- FR71 counter golden / GHA job unchanged
- Docs Mem↔Chisel boundary; NFR14 Epic 33 6/6 ticked
- Review Approve; `33-4-…: done`; `epic-33: done`

### File List

- `crates/rhdl-firrtl/testdata/fr81_path_a_sync_read_mem.scala`
- `crates/bitloom/tests/fr81_mem_chisel_atdd_fr71.rs`
- `docs/fr28-chisel-compilable.md`
- `Justfile`
- `README.md`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic33-chisel-mem.md`
- `_agile-output/implementation-artifacts/33-4-atdd-fr71-回归.md`
- `_agile-output/implementation-artifacts/33-4-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Story 33.4 ATDD + FR71 回归；Epic 33 close

## Suggested Review Order

**Mem fixture** → **FR71 counter unchanged** → **ATDD close-out** → **docs** → **NFR14 ticks** → **sprint**
