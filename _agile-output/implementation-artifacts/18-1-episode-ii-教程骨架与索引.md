---
title: '18.1 Episode II 教程骨架与索引'
type: 'feature'
created: '2026-08-20'
status: 'done'
baseline_commit: '1804929c18414816fbff120f6367828b4beb85a1'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/docs/tutorials/rv32-episode-i/README.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Episode I 完成后学习者缺少独立的 Episode II 教程入口与章节 DoD，只能猜路径。

**Approach:** 新建 `docs/tutorials/rv32-episode-ii/`：README 索引 + 工具链（MSRV / cargo bitloom / bitloom-sim）、真独立 vs monorepo、链回 Episode I、指向 `rv32_pipe` / `rv32_core`；CSR 标可选。

## Boundaries & Constraints

**Always:**
- 品牌 Bitloom；与 samitbasu/rhdl 无关
- MSRV 1.97.1；不得把 clone 写成唯一入口
- 章节：ISA/imm → 五级 → 转发 → load-use → 分支 flush →（可选）CSR；每章标明验收类型

**Never:**
- 宣称 CSR 已实现（留给 18.3）
- 以 Make/SBT 为必装

</frozen-after-approval>

## Tasks & Acceptance

**Execution:**
- [x] `docs/tutorials/rv32-episode-ii/README.md` — 索引、范围/非目标、路径 A/B、合规措辞
- [x] 链回 Episode I；CSR 可选 / 延期声明（NFR32）
- [x] 指向 `examples/rv32_pipe`、`rv32_core`

**Acceptance Criteria:**
- Given Epic 17 包可引用，when 创建 episode-ii 索引，then 列出拟议章节与 DoD 类型
- And 真独立 vs monorepo；Bitloom 免责声明

## Dev Agent Record

### Completion Notes

- README 含 MSRV、cargo bitloom、bitloom-sim、章节表与合规段
- **未 git commit**

### File List

- `docs/tutorials/rv32-episode-ii/README.md`
- `_agile-output/implementation-artifacts/18-1-episode-ii-教程骨架与索引.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
