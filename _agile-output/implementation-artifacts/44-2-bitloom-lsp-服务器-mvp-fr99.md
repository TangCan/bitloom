---
title: '44.2 Bitloom LSP 服务器 MVP + 编辑器接线（FR99）'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: '276048a'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic44-full-elaborate-lsp.md'
  - '{project-root}/_agile-output/implementation-artifacts/44-1-epic-44-nfr14-风险记录.md'
  - '{project-root}/docs/fr38-viz-lsp.md'
  - '{project-root}/docs/fr90-host-ide-rust-analyzer.md'
  - '{project-root}/crates/bitloom/tests/fr91_bitloom_lsp_explicit_defer.rs'
  - '{project-root}/crates/bitloom/tests/fr90_host_ide_rust_analyzer.rs'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
warnings: []
deferred:
  - '按键全 elaborate 诊断 / 符号（P1–P6）→ Story 44.3'
  - 'FR99 收口；撤销 Path B 完成口径；勾选 Epic 44 关闭 → Story 44.4'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Epic 44 / FR99 需要真实的自研 Bitloom language-server 产品面。现状仍是 Epic 39 **FR91 Path B**（显式 defer）+ FR90 宿主 rust-analyzer + FR38 HTML——**不得**用其中任一冒充本故事完成。仓库尚无 `bitloom-lsp` 二进制/crate。

**Approach:** 新增可安装/可启动的 **`bitloom-lsp`** crate + `[[bin]]`（stdio LSP），实现**最小** `initialize` / `initialized` / `shutdown` 与非空 `ServerCapabilities`（MVP 可声明少量能力；**不**实现按键全 elaborate 诊断）。提供 VS Code（或等价）接线文档；夹具工程可复现启动 LSP 会话（stdio initialize 往返）。同步把 Epic 39「禁止半成品 `bitloom-lsp` 存在」的 ATDD **改写为历史合同 + Epic 44 超集交付**（保留 FR91 Path B 已关闭叙事），并诚实标注「本故事 ≠ FR99 全关 / ≠ 44.3 elaborate」。

## Boundaries & Constraints

**Always:** 可 `cargo install` / `cargo run -p bitloom-lsp` 启动；stdio LSP `initialize` → capabilities；接线文档；夹具可复现会话；公开品牌 **Bitloom**；设计 crate 仍只依赖 `bitloom-prelude`（LSP 属工具链面，设计 crate **不得**依赖 CLI/`bitloom-lsp`）；不得声称「仅 rust-analyzer」完成本故事；NFR14 Epic 44 记录保持有效；`epic-44` 保持 in-progress。

**Ask First:** 若改选 TCP-only（无 stdio）或把 LSP 塞进 `cargo-bitloom` 子命令而非独立 `bitloom-lsp` 二进制——须在故事/文档显式等价声明并更新 ATDD 包名断言。

**Never:** 实现按键全设计 elaborate 诊断/符号导航（→ **44.3**）；勾选 FR99 / Epic 44 关闭条件或撤销 Path B「完成口径」全文收口（→ **44.4**）；把 HTML 可视化计入 LSP；用宿主 rust-analyzer 冒充本故事；交付无 initialize/无接线文档/无法复现会话的半成品却宣称 44.2 完成；要求设计 crate 依赖 `bitloom` / `bitloom-lsp`。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Install/start | `cargo run -p bitloom-lsp` / install path | 进程监听 stdio LSP | 缺 binary → ATDD 红 |
| Initialize | LSP `initialize` request | `InitializeResult` + 非空/文档化 capabilities；`initialized`/`shutdown` | 协议错误可读 |
| Editor wiring | VS Code `settings.json` / launch 文档 | 可照做启动 Bitloom LSP（非仅 RA） | 缺文档 → ATDD 红 |
| Fixture session | 夹具 crate/脚本发 initialize | 可复现成功会话证据 | 超时/失败 → 测红 |
| Brand / RA | 文档 | Bitloom；明确 ≠ rust-analyzer 单独完成 | 审查 |
| FR91/FR90 回归 | 历史 Path B 测试 | 改写允许**已文档化**的 `bitloom-lsp`；仍禁未文档半成品；Epic 39 FR91 checkbox 仍勾 | 勿删 FR91 历史关闭证据 |
| Scope | 44.3/44.4 | **不**做 elaborate；**不**勾选 Epic 44 关闭 | Never |

</frozen-after-approval>

## Code Map

- **NEW** `crates/bitloom-lsp/` — package + `[[bin]] name = "bitloom-lsp"`；`tower-lsp-server` + `tokio` stdio 最小 LanguageServer
- **UPDATE** 根 `Cargo.toml` — workspace `members`
- **NEW** `docs/fr99-bitloom-lsp.md` — 安装、stdio、VS Code 接线、夹具复现
- **UPDATE** `docs/fr38-viz-lsp.md` / `docs/fr90-host-ide-rust-analyzer.md` / `README.md` / deferred-work — 诚实化
- **UPDATE** FR91/FR90 ATDD — 允许文档齐全的 `bitloom-lsp`
- **NEW** `crates/bitloom/tests/fr99_bitloom_lsp_server_mvp.rs`

## Story

As a 硬件设计者,
I want 可安装/可启动的 Bitloom language-server 与至少一种编辑器接线文档,
So that 存在真实 LSP 产品面。

## Acceptance Criteria

1. Given Story 44.1, when 交付 `bitloom-lsp`（或文档等价包名）二进制/crate + 最小 initialize/capabilities，并提供 VS Code 或等价接线步骤, then 夹具工程可复现启动 LSP 会话
2. Given 公开叙事, when 阅读接线/产品文档, then 品牌为 Bitloom；**不得**声称「仅 rust-analyzer」即完成本故事
3. Given 范围纪律, when 交付本故事, then **未**实现按键全 elaborate 诊断/符号（44.3）；**未**勾选 FR99 / Epic 44 关闭（44.4）；设计 crate 仍只依赖 `bitloom-prelude`

## Tasks / Subtasks

- [x] T1: 新建 `crates/bitloom-lsp` + workspace 成员；stdio 最小 `initialize`/`capabilities`/`shutdown`（AC: 1）
- [x] T2: 夹具/集成测复现 LSP 会话（stdio initialize 往返）（AC: 1）
- [x] T3: `docs/fr99-bitloom-lsp.md` + VS Code 接线；轻触 `fr38-viz-lsp.md` 诚实化（AC: 1–2）
- [x] T4: 改写 FR91/FR90 断言；新增 FR99 MVP ATDD（AC: 2–3）
- [x] T5: sprint `44-2: done`；`44-3`/`44-4` backlog；`epic-44` in-progress（AC: 3）

## Dev Notes

### 技术选型

- `tower-lsp-server` 0.23 + `tokio` stdio
- MVP capabilities: `textDocumentSync` Full + `serverInfo.name = bitloom-lsp`

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 44 / Story 44.2]
- [Source: `_agile-output/implementation-artifacts/nfr14-risk-epic44-full-elaborate-lsp.md`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- `crates/bitloom-lsp` stdio MVP；`initialize` 会话 ATDD 绿
- `docs/fr99-bitloom-lsp.md` + fr38/fr90/README/deferred 诚实补丁
- FR91/FR90 允许文档齐全 `bitloom-lsp`；Epic 39 Path B 历史关闭保留
- 审查 Approve；automate 认定 ATDD 足够
- sprint：`44-2: done`；`44-3`/`44-4` backlog；`epic-44` in-progress；未勾选 FR99 全关

### File List

- `crates/bitloom-lsp/Cargo.toml`
- `crates/bitloom-lsp/src/main.rs`
- `Cargo.toml`
- `Cargo.lock`
- `docs/fr99-bitloom-lsp.md`
- `docs/fr38-viz-lsp.md`
- `docs/fr90-host-ide-rust-analyzer.md`
- `README.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `crates/rhdl-viz/src/lib.rs`
- `crates/bitloom/tests/fr99_bitloom_lsp_server_mvp.rs`
- `crates/bitloom/tests/fr91_bitloom_lsp_explicit_defer.rs`
- `crates/bitloom/tests/fr90_host_ide_rust_analyzer.rs`
- `_agile-output/implementation-artifacts/44-2-bitloom-lsp-服务器-mvp-fr99.md`
- `_agile-output/implementation-artifacts/atdd-checklist-44-2-bitloom-lsp-服务器-mvp-fr99.md`
- `_agile-output/implementation-artifacts/44-2-code-review.md`
- `_agile-output/implementation-artifacts/44-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Story context created (ready-for-dev)
- 2026-09-09: Implement Bitloom LSP server MVP + ATDD（Story 44.2）

## Suggested Review Order

**`bitloom-lsp` + initialize** → **会话 ATDD** → **接线文档 / fr38** → **FR91/FR90** → **sprint**
