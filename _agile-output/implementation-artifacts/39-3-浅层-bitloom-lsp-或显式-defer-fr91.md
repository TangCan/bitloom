---
title: '39.3 浅层 Bitloom LSP 或显式 defer（FR91）'
type: 'docs'
created: '2026-09-09'
status: 'done'
baseline_commit: 'c6ac3f0'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic39-ide-multiview.md'
  - '{project-root}/_agile-output/implementation-artifacts/39-1-epic-39-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/39-2-宿主-ide-rust-analyzer-工作流-fr90.md'
  - '{project-root}/docs/fr38-viz-lsp.md'
  - '{project-root}/docs/fr90-host-ide-rust-analyzer.md'
  - '{project-root}/README.md'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/process-one-story-one-commit.md'
  - '{project-root}/_agile-output/implementation-artifacts/35-3-softf16-可综合或显式-defer-fr84.md'
warnings: []
deferred:
  - 'Bitloom self-hosted / shallow diagnostic LSP MVP (FR91 Option A) → future epic after Path B contract; must renegotiate NFR14'
  - 'Full elaborate netlist LSP → FR93 permanent non-goal (not FR91 A)'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR91 允许「浅层 Bitloom LSP MVP」或「显式 defer」。Story 39.1 已钉死 **Path B**；Story 39.2 已交付 FR90 宿主 rust-analyzer。若 README / `docs/fr38-viz-lsp.md` / NFR14 不把 FR91 B **合同化收口并勾选**，易把宿主 RA、层次 HTML、或半成品 language-server 冒充「自研 Bitloom LSP 已交付」。

**Approach:** **仅 Path B**（与 39.1 一致）：强化 README + `docs/fr38-viz-lsp.md`（及必要交叉链）显式 **deferred / 不得声称 LSP 已交付**；更新 deferred-work 证据指向 Story 39.3 收口；在 NFR14 **勾选 FR91 关闭条件**（非整 epic）；用 docs ATDD 锁住 Path B、禁半成品二进制、HTML≠LSP、39.4 仍 backlog。**不**实现浅层 LSP（分支 A）；**不**开工 39.4。

## Boundaries & Constraints

**Always:** Gate 39.1 Path B；39.2 FR90 已 done；README + fr38（或等价）合同化 defer；声明不得声称 Bitloom LSP 已交付；NFR14 FR91 勾选；无 `language-server` / `bitloom-lsp` crate；层次/时序 HTML ≠ LSP；品牌 Bitloom；one-story-one-commit。

**Ask First:** 仅当产品改选分支 A（浅层 LSP MVP）— 须先改 NFR14 并扩大本故事（当前证据不支持）。

**Never:** 交付 Bitloom language-server / 半成品 LSP 二进制；把宿主 rust-analyzer 或层次 HTML 写成「Bitloom LSP 已交付」；勾选整份 Epic 39 关闭清单（→ 39.4）；开工 39.4（FR92）；改选分支 A 而不改 NFR14。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Path B 合同 | README + `docs/fr38-viz-lsp.md` | FR91 / Path B / explicit defer；不得声称 LSP 已交付 | ATDD 红直至落盘 |
| NFR14 | 关闭清单 FR91 行 | `- [x] **FR91：**`；FR92 等仍 `[ ]` | ATDD |
| 半成品 LSP | `crates/language-server` 等 | **不得存在** | ATDD 路径守卫 |
| HTML ≠ LSP | fr38 / README | 层次/时序 HTML 不计入 LSP 完成 | ATDD |
| 范围 | sprint / 39.4 | `39-4` 仍 backlog；不勾选整 epic | ATDD / 审查 |

</frozen-after-approval>

## Code Map

- `docs/fr38-viz-lsp.md` — **UPDATE** FR91 Path B 合同化 defer
- `README.md` — **UPDATE** deferred 区 FR91 Path B / 不得声称已交付
- `docs/fr90-host-ide-rust-analyzer.md` — **UPDATE** 交叉至 39.3 已合同化
- `_agile-output/implementation-artifacts/deferred-work.md` — **UPDATE** LSP 条证据 → Story 39.3
- `_agile-output/implementation-artifacts/nfr14-risk-epic39-ide-multiview.md` — **UPDATE** 勾选 FR91
- `crates/bitloom/tests/fr91_bitloom_lsp_explicit_defer.rs` — **NEW** docs ATDD
- `crates/bitloom/tests/fr90_host_ide_rust_analyzer.rs` — **UPDATE** 范围守卫（锁 39.4/FR92）
- `_agile-output/implementation-artifacts/sprint-status.yaml` — `39-3` → done；`39-4` backlog

## Story

As a PM / 用户,
I want Bitloom 自研 LSP 要么有浅层 MVP、要么合同化 defer,
So that FR91 深度诚实。

## Acceptance Criteria

1. Given Story 39.1 所选分支 **B**, when 更新 README / `docs/fr38-viz-lsp.md`（或等价）显式 deferred，并声明不得声称 LSP 已交付, then FR91 关闭条件可检查勾选（NFR14 `- [x] **FR91：**`）
2. And 分支 B 下无半成品 language-server 二进制冒充完成（NFR39）
3. And 层次/时序 HTML 入口不计入 LSP 完成
4. And 不开工 39.4；不勾选 Epic 39 全部关闭条件；不交付分支 A 浅层 LSP MVP

## Tasks / Subtasks

- [x] T1: 合同化 Path B — README + `docs/fr38-viz-lsp.md`（+ fr90/deferred 交叉）（AC: 1, 3）
- [x] T2: NFR14 勾选 **仅 FR91** 关闭行（AC: 1, 4）
- [x] T3: ATDD `fr91_bitloom_lsp_explicit_defer.rs` + 调整 fr90 范围守卫（AC: 1–4）
- [x] T4: sprint `39-3` → done；**保持** `39-4` backlog；code-review / automate 收口

## Dev Notes

### Decision

**Path B — explicit defer.** No Bitloom language-server binary. Host rust-analyzer (FR90) remains the IDE path. Hierarchy HTML ≠ LSP.

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 39 / Story 39.3 / FR91]
- [Source: `nfr14-risk-epic39-ide-multiview.md`]
- [Source: `process-one-story-one-commit.md`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

- ATDD red→green: `cargo test -p bitloom --test fr91_bitloom_lsp_explicit_defer` — 6 passed（红相 2 失败：fr38 缺 Path B；NFR14 FR91 未勾）
- `cargo test -p bitloom --test fr90_host_ide_rust_analyzer` — 6 passed
- code-review: Approve (`39-3-code-review.md`)
- automate: ATDD sufficient (`39-3-automation-summary.md`)

### Completion Notes List

- FR91 Path B 合同化：`docs/fr38-viz-lsp.md` + README（不得声称 LSP 已交付；HTML ≠ LSP）
- NFR14 勾选 **仅 FR91**；FR92 / 整 epic 仍开
- deferred-work LSP 条指向 Story 39.3；无半成品 LSP crate
- ATDD + fr90 守卫调整；sprint：`39-3: done`；`39-4` backlog

### File List

- `docs/fr38-viz-lsp.md`
- `docs/fr90-host-ide-rust-analyzer.md`
- `README.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic39-ide-multiview.md`
- `crates/bitloom/tests/fr91_bitloom_lsp_explicit_defer.rs`
- `crates/bitloom/tests/fr90_host_ide_rust_analyzer.rs`
- `_agile-output/implementation-artifacts/39-3-浅层-bitloom-lsp-或显式-defer-fr91.md`
- `_agile-output/implementation-artifacts/atdd-checklist-39-3-浅层-bitloom-lsp-或显式-defer-fr91.md`
- `_agile-output/implementation-artifacts/39-3-code-review.md`
- `_agile-output/implementation-artifacts/39-3-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Ultimate context engine analysis completed — ready-for-dev
- 2026-09-09: FR91 Path B defer closeout + ATDD；review Approve；done

## Suggested Review Order

**fr38 LSP 合同段** → **README deferred** → **NFR14 FR91 勾选** → **deferred-work 交叉** → **ATDD + fr90 守卫调整** → **sprint 仅 39-3 done**
