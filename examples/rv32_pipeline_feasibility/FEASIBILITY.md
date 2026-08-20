# FR68 流水语言表面可行性决议（Story 17.1）

**Verdict: PASS — 可继续 17.4**

公开品牌 **Bitloom**。与 `samitbasu/rhdl` 无关。设计依赖仅 `bitloom-prelude`（NFR29）。

## 证明物

| 项 | 路径 / 测试 |
|----|-------------|
| Spike crate | `examples/rv32_pipeline_feasibility` |
| 两级 pipeline Reg | `s0` → `s1`（`declare_reg` + `assign_reg_d_from`） |
| 转发 mux | `assign_mux("fwd", "fwd_sel", "bypass", "s1", …)` — 教学映射 EX/MEM→EX |
| Stall hold | `assign_mux("s0_next", "stall", "s0", "din", …)` — **非**模块级 `en` |
| Tick 黄金 | `tick_two_stage_pipe_and_forward_golden`、`tick_stall_hold_mux_golden` |
| Elaborate | `elaborate_ok` |

命令：`cargo test -p rv32_pipeline_feasibility`

## 已证明可用

1. **≥2 级级间寄存器**：可 `elaborate` + `tick`。
2. **组合转发 mux**：`fwd_sel=1` 时输出旁路源，而非陈旧 `s1`。
3. **按级 stall hold**：用 mux 冻结 `s0`，不依赖 AD-23 全模块 `en`。
4. **级间赋值顺序**：`bitloom-sim` 解释器对 `RegD` **就地更新**；级间链须先写下游再写上游（本 spike：`s1 <= s0` 然后 `s0 <= s0_next`）。17.4 须遵守或改用显式 NBA 缓冲（若另开语言故事）。

## 书面延后（不阻塞 17.4）

| 项 | 决议 |
|----|------|
| Flush bubble（注入 NOP） | 与 hold 同属 `assign_mux`+常量；17.4 实现即可，本闸门不强制单独测例 |
| 完整 IF/ID/EX/MEM/WB 命名与 hazard 单元 | **17.4 / 17.5** |
| 取指 SyncReadMem vs harness | **17.2**（可引用 15.1 SyncReadMem PASS） |
| ISA / B-imm 符号扩展 | **17.3**（仍单周期） |
| CSR / trap | Epic 18；NFR32 |

## 非目标

本故事**不**交付经典 5 级核；**不**修改 `examples/rv32_core` 为流水；**不**宣称 FR64 已满足。

## 相对 15.1

`examples/rv32_feasibility` 已证明 FSM + SyncReadMem + Mux/Eq/Lit。本闸门在其上证明**流水级间 Reg 链 + 转发/hold mux**，不推翻 15.1 PASS。
