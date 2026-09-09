---
title: '35.4 Formal 夹具 + LSP deferred 收口（FR85）'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: '6b16e5f'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic35-residual-partials.md'
  - '{project-root}/_agile-output/implementation-artifacts/35-1-epic-35-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/35-3-softf16-可综合或显式-defer-fr84.md'
  - '{project-root}/docs/fr39-formal-sva.md'
  - '{project-root}/docs/fr38-viz-lsp.md'
  - '{project-root}/crates/rhdl-formal/src/lib.rs'
warnings: []
deferred:
  - 'Full commercial model-checker / SymbiYosys product integration beyond documented Verilator (or sby) invocation → future deepen'
  - 'LSP hover/goto language-server binary → remains deferred (not Epic 35 completion)'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR39 historical done delivered `emit_sva` + `check_sva_text` toy string heuristics. NFR14 forbids closing FR85 with that toy path. LSP hover/goto remains deferred; Epic 35 must document that LSP is **not** a completion criterion.

**Approach:** Land FR85 **Option A** (minimal honest): real design (Counter HIR) → `emit_sva` export to a concrete `*_sva.sv` + documented `just formal-sva-check` / `scripts/formal-sva-check.sh` that invokes an **external** checker (`verilator --lint-only --assert`, or `sby` when present) and **fails clearly** when no checker is available (never silent success). Update user docs for LSP deferred; tick all Epic 35 NFR14 close checkboxes. Do **not** ship a fake language-server.

## Boundaries & Constraints

**Always:** Story 35.1–35.3 gate (FR83 A, FR84 B done); FR85 exceeds `check_sva_text`; documented external checker path; missing tool → non-zero exit + readable error; LSP hover/goto explicit deferred / not Epic 35 done; NFR14 Epic 35 close checklist fully ticked; Bitloom brand; NFR37 honesty.

**Ask First:** 无（Option A is the preferred honest close; Option B only if external checker path cannot be documented without fake success）.

**Never:** Close FR85 with `check_sva_text` alone; silent success when checker missing; claim FR39 depth done via toy only; deliver fake LSP / language-server binary; claim LSP hover/goto is an Epic 35 completion criterion.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Export | Counter FrozenHir + AssertProp | `*_sva.sv` with `assert property` | emit failure → fail |
| Checker present | `verilator` or `sby` on PATH | Script invokes checker on exported SVA; exit 0 on pass | Checker fail → non-zero |
| Checker missing | No verilator/sby (and no override) | Exit non-zero; clear stderr | **Not** silent OK |
| Toy path | `check_sva_text` | Remains FR39 unit-test helper only; **not** FR85 close | ATDD forbids toy-only close |
| LSP docs | `docs/fr38-viz-lsp.md` (+ FR39/Epic 35 note) | hover/goto deferred; **not** Epic 35 done criterion | ATDD scan |
| NFR14 | options + close checklist | FR85 **已选 A**; all Epic 35 close boxes ticked | ATDD / review |

</frozen-after-approval>

## Code Map

- `crates/rhdl-formal/` — Counter export example/fixture + keep `check_sva_text` as toy-only
- `scripts/formal-sva-check.sh` — export + external checker; fail if missing
- `Justfile` — `formal-sva-check` recipe (not required by default `just test`)
- `docs/fr39-formal-sva.md` — FR85 Option A contract; toy vs external path
- `docs/fr38-viz-lsp.md` — Epic 35 does not complete LSP
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` — FR85 Option A
- `_agile-output/implementation-artifacts/nfr14-risk-epic35-residual-partials.md` — FR85 已选 A + Epic 35 close ticks
- `crates/bitloom/tests/fr85_formal_fixture_beyond_toy.rs` — ATDD

## Story

As a 质量负责人,
I want 超出 toy check 的 SVA/formal 夹具，并声明 LSP 仍 deferred,
So that FR85 与可视化边界清晰。

## Acceptance Criteria

1. Given Story 35.1–35.3, when 至少一真实设计导出 SVA（或文档钉死的 formal 工具链调用）并执行非玩具检查（FR85）, then 验收超出 `check_sva_text` 级玩具断言
2. And 用户文档写明 LSP hover/goto **非**本 epic 完成条件（继续 deferred）
3. And NFR14 记录勾选 Epic 35 关闭条件

## Tasks / Subtasks

- [x] T1: Counter (or equiv.) → `emit_sva` export path + committed/generated fixture（AC: 1）
- [x] T2: `scripts/formal-sva-check.sh` + `just formal-sva-check`; missing tool fails clearly（AC: 1）
- [x] T3: Docs FR39 FR85 Option A; FR38 LSP not Epic 35 done; PRD addendum（AC: 1, 2）
- [x] T4: ATDD `fr85_formal_fixture_beyond_toy`; NFR14 FR85 已选 A + full Epic 35 close ticks; review / sprint done（AC: 1–3）

## Dev Notes

- **Decision:** Path A — non-toy formal fixture (export + external checker script). Keep `check_sva_text` for historical FR39 unit tests only.
- Prefer Verilator `--lint-only --assert` as default documented checker when present; optional `sby` if found. Do not invent a fake in-repo model checker that only greps strings.
- Prefer targeted tests: `cargo test -p bitloom --test fr85_formal_fixture_beyond_toy` (+ optional `cargo test -p rhdl-formal`, `just formal-sva-check`).
- Do **not** run full `cargo clean && just test`; do **not** commit (parent owns commit).
- Leave `_bmad/scripts/render_skill.py.bak` untracked.

### Project Structure Notes

- Formal crate + scripts/docs/ATDD; no language-server crate.

### References

- [Source: `_agile-output/planning-artifacts/epics.md` Story 35.4]
- [Source: `nfr14-risk-epic35-residual-partials.md` FR85 Option A]
- [Source: `docs/fr39-formal-sva.md`; `crates/rhdl-formal`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

- `cargo test -p bitloom --test fr85_formal_fixture_beyond_toy`
- `just formal-sva-check`
- `cargo test -p rhdl-formal`
- `cargo test -p bitloom --test nfr14_risk_epic35_residual_partials`
- `cargo test -p bitloom --test fr84_softf16_explicit_defer`

### Completion Notes List

- FR85 Option A: Counter → emit_sva → fixture + Verilator via `just formal-sva-check`
- Missing checker fails clearly (`BITLOOM_FORMAL_FORCE_MISSING=1`)
- LSP hover/goto documented as not Epic 35 completion
- NFR14 FR85 已选 A + full Epic 35 close checklist
- Code review Approve; no commit (parent); Epic 35 → done

### File List

- `crates/rhdl-formal/examples/export_fr85_counter.rs`
- `crates/rhdl-formal/fixtures/fr85_counter_sva.sv`
- `scripts/formal-sva-check.sh`
- `Justfile`
- `docs/fr39-formal-sva.md`
- `docs/fr38-viz-lsp.md`
- `crates/bitloom/tests/fr85_formal_fixture_beyond_toy.rs`
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic35-residual-partials.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `_agile-output/implementation-artifacts/35-4-formal-夹具-lsp-deferred-收口-fr85.md`
- `_agile-output/implementation-artifacts/35-4-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Story context created (ready-for-dev)
- 2026-09-09: Option A implemented; review Approve; done; Epic 35 closed

## Suggested Review Order

**export fixture** → **formal-sva-check script** → **docs FR39/FR38** → **NFR14 ticks** → **ATDD**
