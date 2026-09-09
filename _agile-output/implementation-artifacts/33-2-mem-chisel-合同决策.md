---
title: '33.2 Mem→Chisel 合同决策'
type: 'chore'
created: '2026-09-09'
status: 'done'
baseline_commit: 'e6fe52e'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic33-chisel-mem.md'
  - '{project-root}/_agile-output/implementation-artifacts/33-1-epic-33-nfr14-风险记录.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md'
  - '{project-root}/_agile-output/implementation-artifacts/26-2-闭包决策表.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR81 尚未选定唯一路径；若同时暗示「支持子集」与「永久非目标」，或继续用 FR28 done + E0901 回避深度（NFR37），则 33.3–33.4 无可测关闭条件。

**Approach:** 在架构目录写入 FR81 决策页，选定 **Path A（支持文档化 Mem 子集）**；列出 AD-21 单时钟 `Mem`/`SyncReadMem`（含可选 init）形态与 NFR12 钉死对；显式引用 NFR37；PRD addendum / epics / NFR14 回链；ATDD 锁住「文件存在 + 恰好 Path A」。本故事**不**实现 `emit_chisel` Mem 降级（→ 33.3）。

## Boundaries & Constraints

**Always:** 唯一路径 Path A；支持形态表 + NFR12（Chisel 7.14.0 ↔ firtool 1.155.0）；引用 NFR37；链接 Epic 33；品牌 Bitloom；AD-27 / AD-21 / AD-9。

**Ask First:** 无（用户管道已偏好 Path A，且与 AD-27 一致）。

**Never:** 选定 Path B 或双路径并存；实现 Mem→Chisel / 删除 E0901（→ 33.3）；破坏 FR71；漂移 NFR12；用 FR28 done 冒充 FR81 关闭。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 决策齐全 | 决策页 Path A + 形态表 + NFR12 + NFR37 | ATDD 绿 | N/A |
| 缺文件 | 路径不存在 | ATDD 红 | 落盘决策页 |
| 双路径 / 仅 Path B | 同时采纳 A+B 或只写非目标 | ATDD 红 | 改为唯一 Path A |
| 缺形态或版本 | 无 Mem/SyncReadMem 或无 7.14.0/1.155.0 | ATDD 红 | 补齐子集与钉死对 |

</frozen-after-approval>

## Code Map

- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/fr81-mem-chisel-contract-decision-2026-09-09.md` — FR81 决策页
- `_agile-output/implementation-artifacts/nfr14-risk-epic33-chisel-mem.md` — 利弊表回链 Path A
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` — 回链
- `_agile-output/planning-artifacts/epics.md` — FR81 / Story 33.2 链接
- `crates/bitloom/tests/fr81_mem_chisel_contract_decision.rs` — ATDD
- `crates/rhdl-firrtl/src/chisel.rs` — 现状 E0901（本故事不改）

## Story

As a PM / 架构师,
I want 明确选择「支持文档化 Mem 子集」或「永久非目标+替代验收」,
So that FR81 有可测关闭条件。

## Acceptance Criteria

1. Given Story 33.1，when 决策写入 PRD addendum 或架构/规划注记并链接本 epic，then 选定**唯一**路径（FR81）
2. And 若选非目标：须定义替代验收（本故事未选此路径）
3. And 若选支持子集：须列出支持的 Mem 形态与版本约束（NFR12）
4. And 显式引用 NFR37（历史 done ≠ 深度关闭）

## Tasks / Subtasks

- [x] T1: 决策页 Path A + Mem 形态表 + NFR12 + NFR37（AC: 1, 3, 4）
- [x] T2: addendum / epics FR81 / NFR14 回链（AC: 1）
- [x] T3: ATDD 锁住文件存在与恰好 Path A（AC: 1, 3, 4）
- [x] T4: sprint → done；code-review Approve

## Dev Notes

- 偏好 Path A：AD-27 要求可编译 Scala；最小子集 = AD-21 `Mem` / `SyncReadMem` + 可选 init。
- 子集外保留 E0901；不得为交差删诊断或破坏 FR71。
- NFR37：FR28 done + 全量 E0901 不能关闭 FR81。
- 设计 crate 只依赖 `bitloom-prelude`；本故事仅文档+测试。

### Project Structure Notes

- 决策页与 `closure-decision-table-2026-09-08.md` 同目录体例
- 测试落在 `crates/bitloom/tests/`

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- 选定 **Path A**；决策页 + addendum/epics/NFR14 回链
- ATDD `fr81_mem_chisel_contract_decision` 锁唯一路径
- 审查 Approve；sprint `33-2-…: done`；`epic-33: in-progress`

### File List

- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/fr81-mem-chisel-contract-decision-2026-09-09.md`
- `_agile-output/implementation-artifacts/33-2-mem-chisel-合同决策.md`
- `_agile-output/implementation-artifacts/33-2-code-review.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic33-chisel-mem.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md`
- `_agile-output/planning-artifacts/epics.md`
- `crates/bitloom/tests/fr81_mem_chisel_contract_decision.rs`

## Change Log

- 2026-09-09: Story 33.2 FR81 Path A 合同决策 + ATDD + 回链

## Suggested Review Order

**决策页** → **ATDD** → **addendum/epics/NFR14 回链** → **sprint 键**
