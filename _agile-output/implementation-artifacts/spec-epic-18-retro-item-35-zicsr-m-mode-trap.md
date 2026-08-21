---
title: 'epic-18-retro-item-35 可选 Zicsr + M-mode trap 教学最小集（FR65）'
type: 'feature'
created: '2026-08-21'
status: 'done'
baseline_commit: '52e9f4ce8b5391ab98ff3fe1552c9025c1940de4'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-18-context.md'
  - '{project-root}/examples/rv32_pipe/PIPE.md'
  - '{project-root}/examples/rv32_core/SUBSET.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Epic 18 回顾开放项要求兑现可选 FR65：教学向 Zicsr + M-mode trap 最小集，并替换 Ch.06 延期 stub。

**Approach:** 新建 `examples/rv32_priv`（边沿提交 + SYSTEM/CSR/`mret`/`ecall`），不污染 `rv32_pipe` 流水 DoD；教程 Ch.06 改为已实现并指向黄金测试；同步 SUBSET/PIPE/COMPLIANCE/deferred-work。

## Boundaries & Constraints

**Always:**
- 设计依赖仅 `bitloom-prelude`；公开品牌 Bitloom；与 `samitbasu/rhdl` 无关
- CSR 最小集：`mstatus`/`mtvec`/`mepc`/`mcause`/`mscratch`（± `mie`）；指令：`CSRRW`/`CSRRS`（或等价最小子集）+ `MRET`；trap 写 mepc/mcause/mstatus 并跳 `mtvec`
- 写影响中断使能的 CSR 后须文档化串行化/flush；至少一则可观测 tick 黄金
- harness `instr` 取指仍 OK
- 目标「能教 / 能跑 mret」——不得宣称 Privileged / arch-test 合规；禁止 PicoRV32 自定义 IRQ 模板

**Ask First:**
- 若语言表面无法表达 SYSTEM/CSR 路径而只能交付半成品

**Never:**
- 改坏 `rv32_pipe` / `rv32_core` 既有绿测
- 把 CSR 写成 Epic 17 流水完成前置
- 半成品 CSR 冒充绿测或合规

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| CSR 写 mtvec + ECALL | 程序写 mtvec 后 ECALL | mepc/mcause/mstatus 更新；PC→mtvec；handler 可跑 | 未宣称完整异常码表 |
| MRET | handler 末 MRET | PC←mepc；mstatus IE 栈恢复 | 教学子集 |
| CSR RMW | CSRRW/CSRRS 可见 | rd 得旧值、CSR 更新可从端口观测 | 仅实现的 CSR 地址 |
| IE CSR 串行化 | 写 mstatus/mie | 文档 + 下一拍可见新值（边沿提交天然串行） | 流水并入时须 flush |

</frozen-after-approval>

## Code Map

- `examples/rv32_core/src/lib.rs` — 边沿提交模板
- `examples/rv32_pipe/PIPE.md` — 流水无 CSR；仅文档交叉引用
- `docs/tutorials/rv32-episode-ii/06-csr-m-trap.md` — 已实现章
- `examples/rv32_core/SUBSET.md` / `COMPLIANCE.md` — FR65 状态
- `_agile-output/implementation-artifacts/deferred-work.md` — 关闭延期条目
- `Cargo.toml` — members 含 `rv32_priv`

## Tasks & Acceptance

**Execution:**
- [x] `examples/rv32_priv/` — 新建包
- [x] tick 黄金（trap + IE 串行化）
- [x] Ch.06 + README/99 替换 stub
- [x] PIPE/SUBSET/COMPLIANCE/deferred-work 对齐
- [x] workspace members
- [x] sprint action done

**Acceptance Criteria:**
- Given 新包 elaborate 成功，when 跑 tick 黄金，then 可观测 trap 或 CSR+mret 路径
- And `cargo test -p rv32_priv` 与 `cargo test -p rv32_core` 绿
- And 文档不宣称 Privileged/arch-test 合规；Ch.06 不再是延期 stub
- And 回顾项标记 done

## Spec Change Log

## Design Notes

独立 `rv32_priv` 边沿提交核；串行化靠边沿提交天然生效；并入流水须 flush。

## Verification

**Commands:**
- `cargo test -p rv32_priv` — 通过
- `cargo test -p rv32_core` — 通过
- `cargo test -p rv32_pipe` — 通过

## Dev Agent Record

### Completion Notes

- `examples/rv32_priv`：CSRRW/CSRRS、ECALL、MRET、六 CSR；教程 `06-csr-m-trap.md`；action **done**；未 git commit

### File List

- `examples/rv32_priv/`
- `docs/tutorials/rv32-episode-ii/06-csr-m-trap.md`
- 文档交叉引用与 `Cargo.toml` / sprint-status / deferred-work
