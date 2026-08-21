# Epic 18 Context: Episode II 教程与可选特权

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

学习者可按编号章节从「立即数/ISA」跟到「流水+hazard」，并可选择跟练精简 M-CSR/`mret`；大纲与 README 反映实现状态与非目标。CSR 延期或失败不得判定 Epic 17 未完成。

## Stories

- Story 18.1: Episode II 教程骨架与索引
- Story 18.2: 递进教程正文（ISA → 流水 + hazard）
- Story 18.3: 可选 Zicsr + M-mode trap（教学最小集）
- Story 18.4: 更新大纲 + README 范围与品牌

## Requirements & Constraints

- 可选 Zicsr + M-mode trap：最小 CSR 集 mstatus/mtvec/mepc/mcause/mscratch（开中断再加 mie/mip；mtval 可后补）+ mret；trap 写 mepc/mcause/mstatus 并跳 mtvec；写影响中断使能的 CSR 后必须 flush/串行化。
- 禁止以 PicoRV32 自定义 IRQ 为标准模板；默认「能教 / 能跑 mret」，除非另文声明不得宣称 Privileged/arch-test 合规。
- 教程路径：ISA/立即数 → 五级 → 转发 → load-use → 分支 flush →（可选）CSR/trap；CSR 章可选。
- 主路径 cargo/`tick`/`build`；设计依赖仅 bitloom-prelude；MSRV 1.97.1 / edition 2024。
- 公开品牌 Bitloom；与 samitbasu/rhdl 无关。
- 合规措辞区分教学能跑与 Privileged/arch-test；arch-test 绿 ≠ 完整 DV。

## Technical Decisions

- CSR 与流水 DoD 解耦：先用户态 ISA 与 hazard，再可选特权。
- 取指可继续 harness `instr`；片上 I-fetch 非 CSR 章前置。
- CSR 写副作用须像 taken branch 一样 flush，禁止 interrupt skid。
- 实现可扩展流水包或独立教学包；不得静默缩小「能跑 mret」却宣称 Privileged 合规。

## Cross-Story Dependencies

- 依赖 Epic 17（至少 ISA 冻结 + 流水 hazard 可引用）。
- 18.3 失败/延期不得回溯 Epic 17 或 18.2。
- 18.4 在 18.1–18.2 就位后更新大纲；CSR 可为完成或标可选/延期。
