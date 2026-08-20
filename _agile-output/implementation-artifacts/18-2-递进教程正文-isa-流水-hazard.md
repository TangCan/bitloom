---
title: '18.2 递进教程正文（ISA → 流水 + hazard）'
type: 'feature'
created: '2026-08-20'
status: 'done'
baseline_commit: '1804929c18414816fbff120f6367828b4beb85a1'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/18-1-episode-ii-教程骨架与索引.md'
  - '{project-root}/examples/rv32_pipe/PIPE.md'
  - '{project-root}/examples/rv32_core/COMPLIANCE.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** 仅有索引时学习者无法一步一变从立即数跟到 hazard。

**Approach:** 编号章节 00–05：立即数 → 五级 → 转发 → load-use → 分支 flush；每章一类变化，指向 `rv32_core` / `rv32_pipe` 真实测试名；CSR 不作为必做。

## Boundaries & Constraints

**Always:**
- 主路径 Bitloom `cargo` / `tick` / `build`
- 合规措辞与 `COMPLIANCE.md` 一致（定向 ≠ 完整 DV）

**Never:**
- 把 CSR/trap 写入 00–05 必做
- 宣称 arch-test 等价

</frozen-after-approval>

## Tasks & Acceptance

**Execution:**
- [x] `00-getting-started.md` … `05-branch-flush.md`
- [x] 各章验收命令 / 测试名对齐 Epic 17 黄金
- [x] README 索引与章节交叉一致

**Acceptance Criteria:**
- Given 18.1 骨架，when 编写递进正文，then 覆盖 ISA→五级→转发→load-use→flush 且每章一变
- And CSR 非必做；合规措辞对齐 COMPLIANCE

## Dev Agent Record

### Completion Notes

- 测试名：`tick_addi_negative_imm_*`、`tick_clean_path_*`、`tick_alu_alu_raw_*`、`tick_load_use_stall_atdd_golden`、`tick_beq_taken_flush_*`
- **未 git commit**

### File List

- `docs/tutorials/rv32-episode-ii/00-getting-started.md`
- `docs/tutorials/rv32-episode-ii/01-isa-and-imm.md`
- `docs/tutorials/rv32-episode-ii/02-five-stage.md`
- `docs/tutorials/rv32-episode-ii/03-forwarding.md`
- `docs/tutorials/rv32-episode-ii/04-load-use.md`
- `docs/tutorials/rv32-episode-ii/05-branch-flush.md`
- `_agile-output/implementation-artifacts/18-2-递进教程正文-isa-流水-hazard.md`
