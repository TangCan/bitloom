---
title: '58.2 Typed IDE 波形路径实现与验收（FR117）'
type: 'feature'
created: '2026-09-10'
status: 'done'
baseline_commit: '467ebb5'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic58-tywaves-typed-ide-waveform.md'
  - '{project-root}/_agile-output/implementation-artifacts/58-1-epic-58-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/epic-58-context.md'
  - '{project-root}/docs/fr104-interactive-wave.md'
  - '{project-root}/docs/fr114-lcov-coverage-gui.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/process-one-story-one-commit.md'
warnings: []
deferred:
  - 'FR117 / Epic 58 closeout docs + NFR14 checkboxes (Story 58.3)'
  - 'Subset (A) Tywaves first-class integration (NFR51 — stays deferred)'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Story 58.1 已钉死 NFR14 子集 **(B) 自研等价 typed IDE 波形**，但产品仍只有 FR104 `interactive.html` I1–I3（扁平信号名 timeline）与 FR114 LCOV/`coverage.html`；若仅加强 I1–I3、静态 VCD/GTKWave、或覆盖率 GUI，会假绿关闭 FR117。

**Approach:** （1）交付树内 **typed IDE 波形**产品路径：从 FrozenHir 导出结构化信号元数据（名称 + 类型/宽度 + 角色/模块层级），写出 **`typed-wave.html`**（专用查看器，明显超出 I1–I3）+ **`wave.typed.json`**（机器可读 typed 旁路）。（2）由既有 `cargo bitloom wave` 一并写出（一等入口；不破坏 VCD / `timing.html` / `interactive.html`）。（3）`docs/fr117-typed-ide-wave.md` 钉死可复现步骤、手动验收清单、≠ I1–I3 / ≠ LCOV / Tywaves deferred。（4）ATDD：产物标记 + typed 元数据可观察 + 负向（缺 typed 不得 silent 绿）+ NFR48 回归守卫。（5）sprint `58-2: done`；**保持** `58-3` backlog；`epic-58` in-progress。**不**勾选 Epic 58 / FR117 关闭整表。

## Boundaries & Constraints

**Always:** 子集 B 可复现产品路径；夹具/步骤；ATDD 或文档化手动清单；品牌 Bitloom；VCD / `interactive.html` / FR114 LCOV 默认路径仍可用（NFR48）；未选 A Tywaves 保持 deferred（NFR51）；设计 crate 只依赖 `bitloom-prelude`；one-story-one-commit。

**Ask First:** 若改选 (A) Tywaves 一等集成；若要把 FR117 收口文档并入本故事（→58.3）。

**Never:** 仅 FR104 I1–I3 / 仅 VCD·GTKWave / 仅 FR114 LCOV / docs-only 关闭 FR117；silent 宣称子集 A；实现 58.3 收口或勾选 Epic 58 关闭；把 typed 查看器泄漏进设计 crate / `bitloom-prelude`；冒充 NFR14-crates。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| wave 烟测 | `.fir` + `--out-dir` | 写出 `wave.vcd` + `timing.html` + `interactive.html` + **`typed-wave.html`** + **`wave.typed.json`** | IO 错误可读 |
| typed 浏览 | 打开 typed-wave.html | 信号树带类型/宽度/角色；可选中查看元数据；timeline 有值 | ATDD 标记 `data-bitloom-typed-wave` + type 字符串 |
| 超出 I1–I3 | 对比 interactive.html | typed 页含类型语义层级，不仅扁平名搜索 | 文档 + ATDD 断言类型字段 |
| 缺 typed | API 无元数据 | 不得 silent Ok 宣称 FR117 | 可读错误或空标记显式失败 |
| NFR48 | 同一次 wave / coverage | VCD + interactive.html + coverage 路径仍可用 | 回归测试 |
| NFR51 | 文档 | Tywaves (A) deferred；不得宣称已交付 | ATDD 文档守卫 |
| 非本故事 | 58.3 / epic-58 done | sprint 保持 58-3 backlog；epic-58 in-progress | 不得误关 |

</frozen-after-approval>

## Code Map

- `docs/fr117-typed-ide-wave.md` — **NEW** FR117 合同页（子集 B；可复现步骤；手动清单；≠ I1–I3 / ≠ LCOV；A deferred）
- `crates/rhdl-viz/src/lib.rs` — **UPDATE** `TypedSignal` / `typed_signals_from_hir` / `typed_wave_html` / `typed_wave_json`
- `crates/rhdl-viz/src/typed_wave.js` — **NEW** 嵌入式 JS（typed 树 + 选中元数据 + timeline）
- `crates/bitloom/src/main.rs` — **UPDATE** `run_wave` 写出 `typed-wave.html` + `wave.typed.json`；提示打开 typed 页
- `crates/bitloom/tests/fr117_typed_ide_wave.rs` — **NEW** ATDD
- `docs/fr104-interactive-wave.md` — **UPDATE** 交叉链至 fr117（不宣称 Epic 58 关闭）
- `docs/fr114-lcov-coverage-gui.md` — **UPDATE** 交叉链：Tywaves/typed 升格 → fr117（A 仍 deferred 于 56 记录语境下指向本 epic）
- `_agile-output/implementation-artifacts/sprint-status.yaml` — `58-2` → done；`58-3` backlog；`epic-58` in-progress

## Story

As a 调试 / 验证工程师,
I want 风险记录选定的 typed IDE 波形路径可复现使用,
So that 可观测性超出 Phase 12/13 波形与覆盖率 MVP。

## Acceptance Criteria

1. Given Story 58.1 已钉死子集 B, when 交付自研等价 typed IDE 波形产品路径 + 夹具/步骤, then ATDD 或文档化手动验收清单可检查「typed ≠ 仅 I1–I3」
2. And 公开品牌 Bitloom；VCD / `interactive.html` / FR114 LCOV 默认路径仍可用（NFR48）
3. And 未选子集 A（Tywaves）保持 deferred（NFR51），不得 silent 宣称；不实现 58.3 收口

## Tasks / Subtasks

- [x] T1: `docs/fr117-typed-ide-wave.md` + fr104/fr114 交叉（AC: 1–3）
- [x] T2: `rhdl-viz` typed 元数据 + `typed_wave_html` / `typed_wave_json` + JS（AC: 1）
- [x] T3: `cargo bitloom wave` 写出 `typed-wave.html` + `wave.typed.json`（AC: 1–2）
- [x] T4: ATDD `fr117_typed_ide_wave` — 产物 / typed 标记 / NFR48 / NFR51 / 负向（AC: 1–3）
- [x] T5: sprint `58-2: done`；保持 `58-3` backlog；`epic-58` in-progress；code-review / automate / fmt+test / 单 commit

## Dev Notes

### Decision — in-house typed IDE wave (subset B)

**Select self-contained `typed-wave.html` + `wave.typed.json`** emitted beside FR104 `interactive.html` from `cargo bitloom wave`.

Typed surface must show:
- Hierarchical / structured signal list with **type** (e.g. `UInt<8>`, `Clock`) and **kind/role** (port dir / reg / wire)
- Module ownership where available
- Value timeline for selected or filtered signals

This is **beyond** FR104 I1–I3 (name-only timeline). It is **not** FR114 coverage GUI. It is **not** Tywaves (A).

### Architecture

- Viewer lives in **`rhdl-viz`** (toolchain viz). Extract types from **`FrozenHir`** ports / wire / reg decls.
- Reuse `WaveSample` / `samples_from_vcd`.
- Do **not** remove or weaken `interactive.html` / `timing.html` / `wave.vcd` / coverage paths.
- Design crates stay on **`bitloom-prelude` only** (AD-6).

### Previous story intelligence (58.1 / `467ebb5`)

- Gate: `nfr14-risk-epic58-tywaves-typed-ide-waveform.md` — subset B selected; A deferred; bans I1–I3 / VCD / LCOV / docs-only.
- ATDD `nfr14_risk_epic58_tywaves_typed_ide_waveform` must stay green.
- Sprint: `epic-58: in-progress`, `58-1: done`, `58-2` backlog → this story ends `done`; `58-3` stays backlog.

### Git intelligence

- Pattern: docs + thin viz API + `crates/bitloom/tests/fr*.rs` ATDD; commit subject `Story 58.2: …`.

### Testing

- Primary: `cargo test -p bitloom --test fr117_typed_ide_wave`
- Regression: `fr104_interactive_wave`, `fr114_lcov_coverage_gui`, `wave_cli`
- Full gate: `cargo fmt --all && just test`

### Project Structure Notes

- Docs: `docs/fr117-typed-ide-wave.md`
- Viz: `crates/rhdl-viz/src/lib.rs` + `typed_wave.js`
- ATDD: `crates/bitloom/tests/fr117_typed_ide_wave.rs`
- No prelude changes

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 58 / Story 58.2 / FR117]
- [Source: `nfr14-risk-epic58-tywaves-typed-ide-waveform.md` — subset B]
- [Source: `docs/fr104-interactive-wave.md` — I1–I3 isolation]
- [Source: `docs/fr114-lcov-coverage-gui.md` — LCOV isolation]
- [Source: `process-one-story-one-commit.md`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

### Completion Notes List

- 自研 `typed-wave.html` + `wave.typed.json`（子集 B）：`typed_signals_from_hir` / `typed_wave_html` / `typed_wave_json` + `typed_wave.js`
- `cargo bitloom wave` 一并写出；缺 typed 元数据可读失败 `bitloom.typed-wave-empty`
- `docs/fr117-typed-ide-wave.md` + fr104/fr114 交叉；Tywaves A deferred；未关 Epic 58
- ATDD `fr117_typed_ide_wave` 6 项绿；审查 Approve；automate 认定 ATDD 已足够
- sprint：`58-2: done`；`58-3` backlog；`epic-58` in-progress

### File List

- `docs/fr117-typed-ide-wave.md`
- `docs/fr104-interactive-wave.md`
- `docs/fr114-lcov-coverage-gui.md`
- `crates/rhdl-viz/src/lib.rs`
- `crates/rhdl-viz/src/typed_wave.js`
- `crates/bitloom/src/main.rs`
- `crates/bitloom/tests/fr117_typed_ide_wave.rs`
- `_agile-output/implementation-artifacts/58-2-typed-ide-波形路径实现与验收-fr117.md`
- `_agile-output/implementation-artifacts/atdd-checklist-58-2-typed-ide-波形路径实现与验收-fr117.md`
- `_agile-output/implementation-artifacts/58-2-code-review.md`
- `_agile-output/implementation-artifacts/58-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
