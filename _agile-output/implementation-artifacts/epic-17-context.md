# Epic 17 Context: Episode II 教学核（ISA 冻结 → 5 级 + hazard）

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

交付可 `elaborate`/`tick`/`build` 的 Episode II 核：先在单周期路径冻结用户态立即数与符号扩展，再实现经典 IF/ID/EX/MEM/WB、转发、load-use 停顿与分支 flush；语言表面闸门与取指策略必须在改核前书面钉死；验证以定向测试为主、可选 `rv32ui`。CSR/trap 不在本 epic。

## Stories

- Story 17.1: 流水语言表面可行性闸门
- Story 17.2: 取指策略钉死
- Story 17.3: 冻结用户态立即数与符号扩展
- Story 17.4: 五级流水 + 转发 + 分支 flush
- Story 17.5: Load-use 停顿 ATDD + 验证阶梯

## Requirements & Constraints

- 立即数：I/S/B/U/J 字段重建；符号位取自指令字 bit31；B-imm 为 `{bit31, bit7, bits30:25, bits11:8, 0}`，禁止对已拼字段再盲目左移。
- 流水：级间寄存器 + ALU 转发（至少 EX/MEM→EX、MEM/WB→EX）+ load-use 停顿 + predict-not-taken 分支 flush；load-use 须独立 ATDD。
- 取指：书面选定片上 SyncReadMem IF **或** harness/`instr` 口之一，不得静默混用。
- 语言闸门：先证明级间 Reg + 转发 mux 可表达；缺口须先立语言故事，不得静默缩小「经典 5 级」。
- 验证阶梯：定向 →（可选）`rv32ui`；不得把 arch-test 绿当作流水正确。
- MSRV rustc 1.97.1 / edition 2024；设计依赖仅 `bitloom-prelude`；仿真仅 `bitloom-sim` 作 dev-dep；主路径 cargo/`tick`/`build`。
- 公开品牌 Bitloom；与 `samitbasu/rhdl` 无关。CSR 不得阻塞本 epic DoD。

## Technical Decisions

- 先 elaborate 得 FrozenHir 再 tick；禁止 rustc 编译期旁路网表。
- 默认单时钟 + 同步高有效复位；周期精确只走原生 tick。
- 片上同步读存储器用 CHIRRTL 友好名 `Mem`/`SyncReadMem`；互转合同仍是规范 `mem`；禁止未立项 Bundle/Vec 可综合路径。
- 流水 stall 用按级 mux hold，勿把全模块 clock-enable 当成 load-use 方案。
- 顺序：闸门 → 取指策略 → ISA 冻结（仍单周期）→ 五级+转发/flush → load-use ATDD。

## Cross-Story Dependencies

- 依赖 Epic 15 的 `examples/rv32_core` 基线；不依赖 Epic 18。
- 17.1 PASS 是 17.4 前置；17.2 决议约束 17.4 取指行为；17.3 须在插流水前完成；17.5 依赖 17.4。
- Epic 18（教程/可选 CSR）消费本 epic 产物；CSR 失败不得回溯判定 Epic 17 未完成。
