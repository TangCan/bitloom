---
title: '27.4 ATDD golden + 后端无感知抽检'
type: 'feature'
created: '2026-09-08'
status: 'done'
baseline_commit: 'bcc6e8099704177c802f3d99773cb39835a49aa3'
review_loop_iteration: 1
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/closure-decision-table-2026-09-08.md'
  - '{project-root}/_agile-output/specs/spec-rhdl/language-surface.md'
  - '{project-root}/_agile-output/implementation-artifacts/27-1-lut-常量初值生成器闭包-api.md'
  - '{project-root}/_agile-output/implementation-artifacts/27-3-捕获硬件引用诊断-fr16-回归.md'
warnings: []
deferred:
  - 'comb/seq inlined synthesizable closures → Epic 28'
  - 'Mem → Chisel emit (E0901) → Epic 33; NFR36 for Mem path covered via Verilog/FIRRTL'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR73/NFR36 need automatic acceptance that elaborate-time generator closures produce the same Mem/ROM tables as handwritten goldens and that backends never see closure IR.

**Approach:** ATDD comparing CRC-8 (or LUT) tables built via `declare_*_mem_with_init_fn` vs hardcoded `Vec`/`declare_*_with_init`: emit `.v` / FIRRTL init equivalence and SyncReadMem `tick` reads; regex spot-check emit text for no `closure`/`callback`/`Fn(`/`||` IR residue. Docs: minimal Bitloom / `bitloom-prelude` example.

## Boundaries & Constraints

**Always:** Reuse 27.1 APIs; Bitloom brand; design surface `bitloom-prelude`; FR73 golden + NFR36 backend-opaque.

**Never:** cargo clean / full `just test` as sole gate in this story run (targeted `cargo test` OK); store closures in FrozenHir; invent Chisel Mem path (Epic 33); comb/seq inlined closures (Epic 28).

## I/O & Edge-Case Matrix

| Scenario | Expected |
|----------|----------|
| Closure CRC table vs handwritten `Vec` → emit `.v` | identical `initial` assignments |
| Same → FIRRTL | identical `; mem-init` word lists |
| SyncReadMem tick reads | same `rdata` for sampled addresses |
| Emit text NFR36 regex | no closure/callback/`Fn(`/`||` as IR |
| Optional Chisel | Mem still `rhdl::E0901` (Epic 33); no closure IR claimed on that path |

</frozen-after-approval>

## Story

As a 质量负责人,
I want LUT/CRC 闭包生成与手写表 golden 一致，并抽检后端无闭包语义,
So that FR73/NFR36 可自动验收。

## Acceptance Criteria

1. ATDD: same algorithm (CRC table or LUT) via closure generator vs handwritten const table → tick or emit equivalence (FR73)
2. Tests stable under `just test` (or documented recipe) (FR73)
3. Sample emit Verilog/FIRRTL (optional Chisel) has **no** closure/callback IR (NFR36)
4. README / language-surface has minimal Bitloom / `bitloom-prelude` usage example

## Tasks / Subtasks

- [x] T1: ATDD `fr73_crc_lut_golden.rs` (closure vs handwritten emit + tick)
- [x] T2: NFR36 emit regex spot-check (Verilog + FIRRTL)
- [x] T3: language-surface + README minimal FR73 example
- [x] T4: story artifact + code-review Approve + sprint-status (`27-4` + `epic-27` done)

## Dev Agent Record

### Agent Model Used

Composer (Auto)

### Debug Log References

- bmad-build `render_skill.py` HALT: ambiguous `implementation_artifacts` (bmm/gds); proceeded per Epic 27 artifact pipeline
- Targeted verification: `cargo test -p bitloom --test fr73_crc_lut_golden` (+ prior FR73 tests); no `cargo clean` / full `just test`

### Completion Notes List

- Handwritten CRC-8/SMBUS-style poly `0x07` depth-16 table vs `declare_mem_with_init_fn` / SyncReadMem tick golden
- Emit `.v` + FIRRTL init equivalence; NFR36 regex on emit text
- Chisel Mem remains E0901 (Epic 33); NFR36 covered on Verilog/FIRRTL
- epic-27 marked done (all 27.x done)

### File List

- `crates/bitloom/tests/fr73_crc_lut_golden.rs`
- `README.md`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `_agile-output/implementation-artifacts/27-4-atdd-golden-后端无感知抽检.md`
- `_agile-output/implementation-artifacts/27-4-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
