---
title: '44.3 按键全设计 elaborate 诊断 / 符号（FR99）'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: '10862c7'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic44-full-elaborate-lsp.md'
  - '{project-root}/_agile-output/implementation-artifacts/44-2-bitloom-lsp-服务器-mvp-fr99.md'
  - '{project-root}/docs/fr99-bitloom-lsp.md'
  - '{project-root}/crates/bitloom-lsp/src/main.rs'
  - '{project-root}/crates/bitloom/tests/fr99_bitloom_lsp_server_mvp.rs'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
warnings: []
deferred:
  - 'FR99 收口；撤销 Path B 完成口径；勾选 Epic 44 关闭 → Story 44.4'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Story 44.2 已交付可启动的 `bitloom-lsp` MVP（initialize / capabilities / 接线），但**尚未**在编辑路径上跑全设计 elaborate，也未暴露硬件语义诊断与符号/跳转。NFR14 明确：**不得仅浅层诊断关闭 FR99**；P1–P6 钉死触发、elaborate 合同、交互预算、规模上限、增量非目标、能力最小集。

**Approach:** 在 `bitloom-lsp` 上实现**文档化编辑触发**（推荐 `textDocument/didSave`，或 debounce 后的 `didChange`——须在 `docs/fr99-bitloom-lsp.md` 写明）路径：对**文档钉死的设计根 / 夹具模块集**执行 `ElaborateSession::finish()`（或等价 `Elaboratable::elaborate`）全设计 elaborate；将 `Diagnostics` 映射为 `publishDiagnostics`；从 `FrozenHir` 暴露 `documentSymbol` 和/或 `gotoDefinition`（或文档记录的等价硬件语义能力集）。提供可测的**浅层/非 elaborate**对照路径，使 ATDD 证明全路径真正调用 elaborate 合同。超时/过大设计按 P3/P4 发可读诊断或文档化降级。**不**做 44.4 收口。

## Boundaries & Constraints

**Always:** 编辑触发路径跑全设计 elaborate（非仅词法扫）；至少诊断 + 符号/跳转（或文档等价能力集）；ATDD 可区分浅层 vs 全 elaborate；失败诊断可读（含 `code` / en 或 zh）；P1–P6 文档化；品牌 Bitloom；设计 crate 只依赖 `bitloom-prelude`；`bitloom-lsp` 可依赖 `bitloom-builder`/`bitloom-hir`（工具链面）；`epic-44` 保持 in-progress；`44-4` 保持 backlog。

**Ask First:** 若改选「仅 workspace/symbol、无 documentSymbol/goto」作为符号面——须在文档写明等价能力集并更新 ATDD；若触发面从 didSave 改为纯 didChange 无 debounce——须更新 P3 预算说明。

**Never:** 勾选 FR99 / Epic 44 全关或撤销 Path B「完成口径」全文收口（→ **44.4**）；用宿主 rust-analyzer / HTML / 仅浅层扫冒充本故事；假装已全 elaborate 却跳过 `finish()`/`elaborate()`；无限阻塞 UI（违反 P3）；要求设计 crate 依赖 `bitloom` / `bitloom-lsp`；把本故事标成 Epic 44 done。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Edit trigger (P1) | didSave 或文档钉死的 didChange+debounce | 启动全设计 elaborate 路径 | 未接线触发 → ATDD 红 |
| Full elaborate (P2) | 夹具设计根 | 调用 `ElaborateSession::finish()`（或等价）；成功 → FrozenHir；失败 → Diagnostics | 可读 publishDiagnostics |
| Shallow contrast | 同夹具走浅层模式 | **不**调用 finish/elaborate；无硬件语义 elaborate 诊断 | ATDD 证明可区分 |
| Diagnostics (P6) | elaborate 错误（如 E0141/E0142） | LSP Diagnostic（message 含 code 或可读文案） | 空 message → 红 |
| Symbols (P6) | elaborate 成功 | documentSymbol 和/或 definition 含模块/端口（或文档等价集） | 无符号能力且无文档等价 → 红 |
| Timeout (P3) | 超过 ≤2s 夹具预算（或注入超时） | 进行中/超时诊断或可取消；不得挂死 | 文档说明冷启动另计 |
| Oversized (P4) | 超过文档钉死模块/端口上限 | 明确失败或降级提示；不得假装已全 elaborate | 文档 + ATDD |
| Scope | 44.4 / epic close | **不**收口 FR99；`44-4` backlog；`epic-44` in-progress | Never |

</frozen-after-approval>

## Code Map

- **UPDATE** `crates/bitloom-lsp/` — 编辑触发；全 elaborate 引擎；publishDiagnostics；symbols/goto；浅层对照 API（供测）
- **UPDATE** `crates/bitloom-lsp/Cargo.toml` — 依赖 `bitloom-builder` / `bitloom-hir`（及所需 tokio 特性）
- **UPDATE** `docs/fr99-bitloom-lsp.md` — P1 触发面；P2 设计根；P3/P4 超时与规模；P6 能力；浅层 vs 全路径说明
- **UPDATE** `docs/fr38-viz-lsp.md` — 诚实指向 44.3 已交付 elaborate 面（仍 ≠ FR99 全关）
- **NEW** `crates/bitloom/tests/fr99_bitloom_lsp_full_elaborate.rs` — ATDD：全 vs 浅层；诊断；符号；超时/规模文档门；sprint 守卫
- **UPDATE** `crates/bitloom/tests/fr99_bitloom_lsp_server_mvp.rs` — 放开「44-3 必须 backlog」断言（改为允许 done）
- **UPDATE** `_agile-output/implementation-artifacts/sprint-status.yaml` — `44-3: done`；`44-4` backlog；`epic-44` in-progress
- **OPTIONAL** NFR14 关闭清单仅勾 `**44.3：**` 一行（其余留给 44.4）

## Story

As a 硬件设计者,
I want 在编辑路径触发全设计 elaborate 并得到硬件语义诊断/符号导航,
So that 字面绿 LSP 条可验收。

## Acceptance Criteria

1. Given Story 44.2, when 在按键（或文档化编辑触发）路径上实现全设计 elaborate，并暴露至少诊断 + 符号/跳转（或记录等价硬件语义能力集）, then 产品路径可用
2. Given 同一夹具设计, when 分别走全 elaborate 路径与浅层/非 elaborate 路径, then ATDD/集成测证明全路径执行 elaborate 合同（相对浅层可区分）
3. Given elaborate 失败或超时/过大设计, when 返回给客户端, then 诊断可读；超时/过大行为按 44.1 P3/P4 文档化
4. Given 范围纪律, when 交付本故事, then **未**做 FR99/Epic 44 收口（44.4）；`44-4` backlog；`epic-44` in-progress；设计 crate 仍只依赖 `bitloom-prelude`

## Tasks / Subtasks

- [x] T1: `bitloom-lsp` 编辑触发 + 全设计 elaborate 引擎（`finish`/`elaborate`）（AC: 1–2）
- [x] T2: `publishDiagnostics` 映射 `Diagnostics`；可读失败（AC: 1, 3）
- [x] T3: documentSymbol 和/或 gotoDefinition（或文档等价能力集）（AC: 1）
- [x] T4: 浅层对照路径 + ATDD 证明可区分；超时/规模文档 + 门禁测（AC: 2–3）
- [x] T5: 更新 `docs/fr99-bitloom-lsp.md`（及轻触 fr38）；修正 44.2 sprint 断言；sprint `44-3: done`（AC: 4）

## Dev Notes

### 技术方向（防踩坑）

- **Reuse 44.2：** 已有 `tower-lsp-server` 0.23 + tokio stdio；`textDocumentSync: Full` 已声明——本故事实现 `didOpen`/`didChange`/`didSave` 中文档选定的触发。
- **Elaborate 合同：** 必须真实调用 `bitloom_builder::ElaborateSession::finish()` 或 prelude `Elaboratable::elaborate`；禁止用纯正则/词法「诊断」冒充。夹具可用已知会在 elaborate 失败的构造（如 `reject_hw_capture` / E0142）证明全路径命中硬件语义码。
- **浅层对照：** 同一 crate 内提供 `AnalysisMode::Shallow`（或不调用 finish 的路径）供 ATDD 对比；浅层**不得**产生 elaborate 专用诊断码。
- **设计根（P2/P4）：** MVP 文档钉死夹具模块集（例：单模块计数器 / 故意失败夹具）；不要求从任意打开的 `.rs` 反推完整 Cargo 设计图。
- **P3：** 夹具规模目标 ≤2s 首批诊断；用 `tokio::time::timeout` 或等价；超时发 Diagnostic（severity=`bitloom-lsp.timeout` 或可读文案）。
- **P5：** 允许全量重算，不做细粒度增量。
- **依赖边界：** `bitloom-lsp` → `bitloom-builder`/`bitloom-hir` OK；**设计** crate 仍只 `bitloom-prelude`。
- **44.2 回归：** `fr99_bitloom_lsp_mvp_scope_guards` 当前要求 `44-3: backlog`——本故事完成后必须改为期望 `done`，否则红。
- **不要做 44.4：** 不撤销 Path B 完成口径全文；不勾选 Epic 44 全部关闭条件；不把 `epic-44` 标 done。

### 现有代码现状（UPDATE）

- `crates/bitloom-lsp/src/main.rs`：仅 `initialize` / `initialized` / `shutdown`；注释写明 elaborate → 44.3。
- `docs/fr99-bitloom-lsp.md`：明确 publishDiagnostics/symbols out of scope until 44.3。
- `ElaborateSession::finish` → `Result<FrozenHir, Diagnostics>`；`Diagnostic { span, code, en, zh }`。

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 44 / Story 44.3]
- [Source: `_agile-output/implementation-artifacts/nfr14-risk-epic44-full-elaborate-lsp.md` — P1–P6]
- [Source: `_agile-output/implementation-artifacts/44-2-bitloom-lsp-服务器-mvp-fr99.md`]
- [Source: `crates/bitloom-builder/src/lib.rs` — `ElaborateSession::finish`]
- [Source: `crates/bitloom-hir/src/lib.rs` — `Diagnostic` / `Diagnostics`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

### Completion Notes List

- `bitloom_lsp` lib：FullElaborate vs Shallow；didSave → finish + publishDiagnostics
- documentSymbol / goto；P3 timeout / P4 oversized；ATDD + docs P1–P6
- sprint `44-3: done`；`44-4` backlog；`epic-44` in-progress；审查 Approve

### File List

- `crates/bitloom-lsp/Cargo.toml`
- `crates/bitloom-lsp/src/lib.rs`
- `crates/bitloom-lsp/src/main.rs`
- `crates/bitloom/Cargo.toml`
- `crates/bitloom/tests/fr99_bitloom_lsp_full_elaborate.rs`
- `crates/bitloom/tests/fr99_bitloom_lsp_server_mvp.rs`
- `docs/fr99-bitloom-lsp.md`
- `docs/fr38-viz-lsp.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic44-full-elaborate-lsp.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `_agile-output/implementation-artifacts/44-3-按键全-elaborate-诊断-符号-fr99.md`
- `_agile-output/implementation-artifacts/atdd-checklist-44-3-按键全-elaborate-诊断-符号-fr99.md`
- `_agile-output/implementation-artifacts/44-3-code-review.md`
- `_agile-output/implementation-artifacts/44-3-automation-summary.md`
- `Cargo.lock`

## Change Log

- 2026-09-09: Ultimate context engine analysis completed — comprehensive developer guide created (ready-for-dev)
- 2026-09-09: Implement didSave full-design elaborate diagnostics/symbols（Story 44.3）

## Suggested Review Order

**elaborate 引擎 + 触发** → **diagnostics 映射** → **symbols** → **浅层对照 ATDD** → **P3/P4 文档** → **sprint / 44.2 断言修复**
