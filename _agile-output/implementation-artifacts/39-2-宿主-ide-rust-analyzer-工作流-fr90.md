---
title: '39.2 宿主 IDE / rust-analyzer 工作流（FR90）'
type: 'docs'
created: '2026-09-09'
status: 'done'
baseline_commit: '2a586cd'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic39-ide-multiview.md'
  - '{project-root}/_agile-output/implementation-artifacts/39-1-epic-39-nfr14-风险记录.md'
  - '{project-root}/docs/fr38-viz-lsp.md'
  - '{project-root}/docs/nfr13-msrv-1.97.1.md'
  - '{project-root}/rust-toolchain.toml'
  - '{project-root}/examples/counter_ports'
  - '{project-root}/README.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
  - '{project-root}/_agile-output/implementation-artifacts/process-one-story-one-commit.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Wave D / FR90 要求设计者有**可复现**的宿主 IDE（rust-analyzer）工作流，但仓库尚无专用用户文档页把「打开 Bitloom 设计 crate → 补全 / 跳转 / rustc 诊断」钉成步骤 + 夹具说明；易与 FR91 自研 Bitloom LSP、FR38 层次 HTML、或硬件语义 netlist LSP 混淆。

**Approach:** 新增（或明确扩展）`docs/` 下 FR90 宿主 IDE 工作流页：钉死工具链（`rust-toolchain.toml` / MSRV 1.97.1）、打开 workspace/设计 crate、期望的 rust-analyzer 能力（补全、goto、诊断）、与硬件语义 / 层次导航 / Bitloom LSP 的边界；并至少一夹具工程说明（推荐既有 `examples/counter_ports`：只依赖 `bitloom-prelude`）。用 docs ATDD 锁住 FR90 关键句与边界。本故事**不**落地 FR91 defer 收口（→ 39.3），**不**做同刺激夹具（→ 39.4），**不**交付 language-server 二进制。

## Boundaries & Constraints

**Always:** Gate 39.1 / `nfr14-risk-epic39-ide-multiview.md` 已存在且 FR91 Path B；可复现步骤；至少一夹具说明；宿主语言智能 ≠ 硬件语义 / netlist LSP；品牌 Bitloom；设计 crate 只依赖 `bitloom-prelude`；交叉链 `docs/fr38-viz-lsp.md`（HTML ≠ LSP）；one-story-one-commit。

**Ask First:** 无（除非要把夹具改成全新示例工程而非复用 `counter_ports` / `cargo bitloom new` 脚手架叙事）。

**Never:** 开工 39.3（FR91 Path B 文档收口）或 39.4（FR92）；交付 Bitloom language-server / 半成品 LSP 二进制；把层次 HTML 或宿主 RA 写成「Bitloom LSP 已交付」；硬件语义 netlist LSP；勾选 Epic 39 关闭条件（→ 39.4）；改选 FR91 分支 A。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 工作流文档齐全 | 新/更新 `docs/fr90-*.md`（或等价） | 含可复现步骤：打开设计 crate → RA 补全/跳转/诊断；品牌 Bitloom；钉工具链 | ATDD 红直至落盘 |
| 夹具说明 | 至少一工程（建议 `examples/counter_ports`） | 文档写明路径、依赖仅 `bitloom-prelude`、如何在 IDE 打开验证 | ATDD 红 |
| 边界诚实 | 全文 | 明确宿主语言智能 vs 硬件语义/层次导航；非 Bitloom LSP；链 fr38 | ATDD 红 |
| 范围越界 | 试图交付 LSP 二进制或开工 39.3/39.4 | 审查拒收；本故事不做 | 审查 / sprint |

</frozen-after-approval>

## Code Map

- `docs/fr90-host-ide-rust-analyzer.md` — **NEW**（建议路径；若改名须同步 ATDD/README）宿主 IDE / rust-analyzer 可复现工作流（FR90）
- `docs/fr38-viz-lsp.md` — **UPDATE** 轻量交叉链至 FR90 宿主路径（勿把本故事写成 FR91 完成；完整 LSP 仍 deferred）
- `README.md` — **UPDATE** 文档索引 / 状态区增加 FR90 宿主 IDE 链（一句即可）
- `examples/counter_ports/` — **只读夹具**（说明用）；可选修正模块注释里过时 `rhdl-prelude` / `cargo rhdl` 措辞为 Bitloom（若动文件须保持行为不变）
- `rust-toolchain.toml` / `docs/nfr13-msrv-1.97.1.md` — **只读** 工具链钉死依据
- `crates/bitloom/tests/fr90_host_ide_rust_analyzer.rs` — **NEW** docs ATDD
- `_agile-output/implementation-artifacts/nfr14-risk-epic39-ide-multiview.md` — 只读门禁；**勿**勾选 Epic 39 关闭条件（留给 39.4）
- `_agile-output/implementation-artifacts/sprint-status.yaml` — `39-2-…` → done（实现后）；**勿**改 39-3/39-4

## Story

As a 硬件设计者,
I want 可复现的 rust-analyzer（宿主 LSP）工作流文档与夹具说明,
So that Bitloom 设计 crate 获得补全 / 跳转 / 诊断，而无需自研 netlist LSP。

## Acceptance Criteria

1. Given Story 39.1（NFR14；FR91 Path B）, when 新增或更新用户文档（建议 `docs/fr90-host-ide-rust-analyzer.md`）+ 至少一夹具/示例工程说明, then 步骤可复现：打开设计 crate → rust-analyzer（或文档等价宿主 LSP）提供补全与跳转与 rustc 诊断可见（FR90）
2. And 明确区分宿主语言智能 vs 硬件语义 / 层次导航（非本故事范围）；不得声称 Bitloom 自研硬件语义 LSP 已交付
3. And 公开品牌表述为 Bitloom；设计依赖叙事仍为 `bitloom-prelude` only
4. And 不开工 39.3 / 39.4；不交付 language-server 二进制；不勾选 Epic 39 关闭清单

## Tasks / Subtasks

- [x] T1: 撰写 `docs/fr90-host-ide-rust-analyzer.md`（可复现步骤 + 边界 + 品牌）（AC: 1–3）
  - [x] 钉死：打开本仓库 workspace（`rust-toolchain.toml` → rustc 1.97.1）；对设计 crate 启用 rust-analyzer
  - [x] 期望能力：符号补全、goto definition、rustc/RA 诊断可见
  - [x] 边界：宿主智能 ≠ 硬件语义 netlist LSP；层次 HTML ≠ LSP（链 `docs/fr38-viz-lsp.md`）；FR91 仍 Path B / 留给 39.3
- [x] T2: 至少一夹具工程说明（推荐 `examples/counter_ports`）（AC: 1, 3）
  - [x] 路径、如何打开、依赖仅 `bitloom-prelude`、可选 `cargo test -p counter_ports` / `cargo bitloom build --package counter_ports` 交叉
  - [x] 可选：修正 `counter_ports` 注释中的历史 `rhdl` 品牌措辞（不改行为）
- [x] T3: README + `docs/fr38-viz-lsp.md` 轻量交叉链（AC: 2–3）
- [x] T4: ATDD `fr90_host_ide_rust_analyzer.rs`（AC: 1–4）
- [x] T5: sprint `39-2` → done；**保持** `39-3`/`39-4` backlog；code-review / automate 收口

## Dev Notes

### 必须落进用户文档的要点

| 主题 | 要求 |
|------|------|
| 产品名 | **Bitloom**（`cargo bitloom`；与 `samitbasu/rhdl` 无关） |
| 工具链 | 打开仓库根；`rust-toolchain.toml` channel **1.97.1**；见 `docs/nfr13-msrv-1.97.1.md` |
| 设计依赖 | 设计 crate `[dependencies]` **仅** `bitloom-prelude`（AD-6） |
| 宿主 LSP | **rust-analyzer**（VS Code Rust 扩展 / rust-analyzer 等价宿主）；补全、跳转、诊断 |
| 夹具 | 至少一：建议 `examples/counter_ports`（`#[module]` + ports；已有 elaborate 测） |
| 非目标 | 硬件语义 / netlist LSP；Bitloom 自研 LSP（FR91 Path B → 39.3）；层次/时序 HTML 冒充 LSP（FR38） |
| 脚手架可选 | 可提及 `cargo bitloom new` 生成最小设计 crate，但不要求本故事新增示例包 |

### 建议文档结构（可微调）

1. 标题 + FR90 + Bitloom 品牌一句  
2. 前置（工具链 / rust-analyzer 安装提示——高阶，不写 OS 安装百科）  
3. 可复现步骤（编号）  
4. 夹具：`examples/counter_ports`  
5. 边界表：宿主智能 | 层次 HTML | Bitloom LSP deferred  
6. 相关链接：fr38-viz-lsp、NFR13、README  

### ATDD 形状（建议断言）

落在 `crates/bitloom/tests/fr90_host_ide_rust_analyzer.rs`：

- 文档文件存在且含 `FR90`、`rust-analyzer`、`Bitloom`、`bitloom-prelude`
- 含补全 / 跳转（或 goto）/ 诊断 三类能力措辞
- 含夹具路径 `examples/counter_ports`（或文档钉死的等价夹具名）
- 明确非硬件语义 / netlist LSP 或「非 Bitloom LSP」边界；链或提及 `fr38`
- **不得**出现「Bitloom LSP 已交付」类完成宣称；**不得**勾选 NFR14 Epic 39 关闭清单（本故事不改关闭勾选）

### Previous story intelligence（39.1）

- FR91 **Path B** 已钉死；宿主 rust-analyzer 仍是 Wave D **必做**（本故事）
- 禁止：SystemC TLM、自动 FL≡RTL、HTML≡LSP、半成品 LSP 二进制
- ATDD 体例：`crates/bitloom/tests/nfr14_risk_epic39_ide_multiview.rs`；docs ATDD 体例可参考 `fr84_softf16_explicit_defer.rs` / `fr87_doc19_contract_green.rs`
- sprint：`epic-39: in-progress`；实现后仅推进 `39-2`

### Git intelligence

- 上一提交 `2a586cd`：Complete story 39.1（NFR14 + ATDD；Path B）
- 文档故事模式：`docs/fr*.md` + `crates/bitloom/tests/fr*_*.rs` + README 索引一行
- **one story → one commit**；勿夹带 39.3/39.4 或无关 reformat

### Project Structure Notes

- 用户文档放 `docs/`（与 `docs/fr38-viz-lsp.md` 并列）
- 测试放 `crates/bitloom/tests/`
- 不新增 language-server crate；不改 prelude API

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 39 / Story 39.2 / FR90]
- [Source: `_agile-output/implementation-artifacts/nfr14-risk-epic39-ide-multiview.md` — FR90 边界；Gate]
- [Source: `docs/fr38-viz-lsp.md` — LSP deferred；HTML ≠ LSP]
- [Source: `docs/nfr13-msrv-1.97.1.md` / `rust-toolchain.toml`]
- [Source: `examples/counter_ports`]
- [Source: `_agile-output/implementation-artifacts/process-one-story-one-commit.md`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

- ATDD red→green: `cargo test -p bitloom --test fr90_host_ide_rust_analyzer` — 6 passed
- code-review: Approve (`39-2-code-review.md`)
- automate: ATDD sufficient (`39-2-automation-summary.md`)

### Completion Notes List

- 新增 `docs/fr90-host-ide-rust-analyzer.md`：可复现 rust-analyzer 步骤（补全/跳转/诊断）+ `examples/counter_ports` 夹具说明 + 边界表
- README / fr38 / deferred-work 交叉链；counter_ports 注释品牌修正
- ATDD 含范围守卫（无 LSP 二进制；39.3/39.4 backlog）
- sprint：`39-2: done`；`39-3`/`39-4` 仍 backlog；未勾选 Epic 39 关闭清单

### File List

- `docs/fr90-host-ide-rust-analyzer.md`
- `docs/fr38-viz-lsp.md`
- `README.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `examples/counter_ports/src/lib.rs`
- `crates/bitloom/tests/fr90_host_ide_rust_analyzer.rs`
- `_agile-output/implementation-artifacts/39-2-宿主-ide-rust-analyzer-工作流-fr90.md`
- `_agile-output/implementation-artifacts/atdd-checklist-39-2-宿主-ide-rust-analyzer-工作流-fr90.md`
- `_agile-output/implementation-artifacts/39-2-code-review.md`
- `_agile-output/implementation-artifacts/39-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: FR90 host IDE / rust-analyzer workflow docs + fixture + ATDD（Story 39.2）

## Suggested Review Order

**fr90 工作流文档（步骤）** → **夹具说明（counter_ports）** → **边界（≠ Bitloom LSP / ≠ HTML）** → **README/fr38/deferred 链** → **ATDD** → **sprint 仅 39-2 done**
