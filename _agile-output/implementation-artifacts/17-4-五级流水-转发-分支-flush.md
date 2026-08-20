---
title: '17.4 五级流水 + 转发 + 分支 flush'
type: 'feature'
created: '2026-08-20'
status: 'done'
baseline_commit: '3710205b14409d82342c6113070e6e8fe1d5f9b1'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-17-context.md'
  - '{project-root}/examples/rv32_core/SUBSET.md'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Episode I 单周期核无法演示数据/控制冒险；需经典 IF/ID/EX/MEM/WB、ALU 转发与 predict-not-taken 分支冲刷（FR64），且不得破坏 Episode I 绿测。

**Approach:** 新建 `examples/rv32_pipe`（Episode II 核），复用 `rv32_core` 的 signed imm / 子集 ISA / harness `instr`；级间 Reg + EX/MEM→EX 与 MEM/WB→EX 转发；taken BEQ flush 错误路径并改 PC。load-use / CSR 不做。

## Boundaries & Constraints

**Always:**
- 品牌 Bitloom；设计 `[dependencies]` 仅 `bitloom-prelude`
- 取指锁定 (b) harness `instr`（17.2）；禁止 SyncReadMem I-fetch
- 保留 17.3 signed I/S/B/U/J imm（符号位 bit31；B-imm 字段拼装）
- `bitloom-sim` 级间赋值：**下游 Reg 先于上游**（WB←MEM←EX←ID←IF / 链式 `s1` before `s0`）
- Comb 滞后 seq 一拍：测试文档化 `instr` 与 pipe 对齐时序
- `elaborate` + `tick` + `cargo bitloom build --package rv32_pipe`；`cargo test -p rv32_core` 与 `-p rv32_pipe` 绿

**Ask First:**
- 若必须改 `bitloom-builder`/`bitloom-sim` 才能表达五级/转发/flush → 停并上报

**Never:**
- load-use stall（17.5）；CSR/trap；改 `rv32_core` 成流水；静默混用片上 I-fetch；宣称 arch-test 等价

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Clean path | 无 RAW/分支的 ADDI/ADD 序列 | 五级后 x 寄存器正确 | N/A |
| ALU→ALU RAW | `ADDI x1,…` 后紧跟 `ADD`/`ADDI` 用 x1 | 靠转发得正确结果（无 load-use stall） | 错转发则黄金失败 |
| Taken BEQ flush | BEQ taken；错误路径有会写 RF 的指令 | 错误路径不提交；PC 跳到目标 | 未 flush 则 RF 脏 |
| Predict-not-taken miss | 同上 | flush IF/ID（及所需气泡）+ redirect | N/A |

</frozen-after-approval>

## Code Map

- `examples/rv32_pipeline_feasibility/src/lib.rs:32-45` — 转发 mux、stall hold 模式；**先 `s1` 后 `s0`**
- `examples/rv32_pipeline_feasibility/FEASIBILITY.md` — PASS；flush 用 NOP mux
- `examples/rv32_core/src/lib.rs` — 只读：imm `:232-303`、ALU/`mask32` `:323-328`、RF/PC/BEQ `:305-371`、DMEM/LED `:346-389`、`enc_*` `:401-449`
- `examples/rv32_core/SUBSET.md:19-39` — 取指 (b) 合同；PIPE 包须遵守并交叉引用
- `examples/rv32_core/Cargo.toml` — 新包依赖模板
- 根 `Cargo.toml` members — 追加 `examples/rv32_pipe`
- `crates/bitloom-sim/src/lib.rs:317-400` — seq→comb；RegD 原地更新

## Tasks & Acceptance

**Execution:**
- [x] `examples/rv32_pipe/` — 新建包：`EpisodeIIPipe` IF/ID/EX/MEM/WB Reg；decode/imm/ALU/RF/DMEM 自 core 演化；harness `instr`
- [x] `examples/rv32_pipe/src/lib.rs` — EX/MEM→EX 与 MEM/WB→EX 转发；predict-not-taken；taken flush + PC redirect；seq 下游先赋
- [x] `examples/rv32_pipe/src/lib.rs`（tests）— 黄金：clean；ALU→ALU RAW 转发；taken 分支错误路径未提交；文档化 instr 时序
- [x] `examples/rv32_pipe/PIPE.md` + SUBSET/COMPLIANCE 摘记 — 取指 (b)、无 load-use、无 CSR；17.4 住于此包；Episode I 仍 `rv32_core`
- [x] 根 `Cargo.toml` — members 加入 `examples/rv32_pipe`
- [x] 验证 — `cargo test -p rv32_pipe`、`-p rv32_core`；`cargo bitloom build --package rv32_pipe`
- [x] 收口 — 本故事 Status done；sprint-status；`17-4-code-review.md`

**Acceptance Criteria:**
- Given 17.1–17.3 完成，when 交付五级 + 至少两级转发 + branch flush，then 三项黄金 tick 通过且遵守 harness `instr`
- Given 设计 crate，when 检查依赖，then 仅 `bitloom-prelude`；无 CSR；无 load-use
- Given 包可构建，when elaborate/tick/bitloom build，then 通过；`rv32_core` 仍绿

## Spec Change Log

## Design Notes

- **PC / IF：** 每拍 PC→IF/ID.pc；harness 按「当前 IF 所见 PC」驱动 `instr`（测试维护 ROM 表）。predict-not-taken：默认 PC+4；EX 判 taken 则下一拍 PC=branch_tgt，并向 IF/ID（及 ID/EX 若已进入）插 bubble/NOP。
- **`pc_f`：** seq 在更新 `pc` 前锁存取指 PC，供 comb 与 `instr` 对齐（seq-then-comb 下直接采 `pc` 会得到已推进的值）。
- **转发：** EX 读操作数时 mux：优先 EX/MEM.rd 匹配，其次 MEM/WB；rd≠0。
- **WB：** MEM/WB 边沿写 RF（与 Episode I 边沿提交对齐，但经五级延迟）。
- **子集：** 同 Episode I（ADDI/ADD/BEQ/LW/SW，x1–x4）；本故事黄金可仅 ADDI/ADD/BEQ。
- **不实现 load-use：** 故意依赖 ALU-ALU 测转发；load 后用暂不测。

## Verification

**Commands:**
- `cargo test -p rv32_pipe` — 全部 pass
- `cargo test -p rv32_core` — 全部 pass
- `cargo bitloom build --package rv32_pipe` — success

## Dev Agent Record

### Completion Notes

- 新建 `examples/rv32_pipe`：`EpisodeIIPipe` 五级 + EX/MEM→EX（非 LW）/ MEM/WB→EX 转发 + BEQ flush
- 黄金：`tick_clean_path_addi_add_golden`、`tick_alu_alu_raw_forward_golden`、`tick_addi_negative_imm_pipe_golden`、`tick_beq_taken_flush_wrong_path_not_committed`
- `pc_f` 对齐 IF/ID PC 与 harness `instr`；BEQ 紧邻生产者靠 EX 转发比较
- 评审补丁：禁 LW EA 转发、`pc_plus4` mask、PIPE arming 文档
- 未改 `bitloom-builder` / `bitloom-sim`；`rv32_core` 仍绿；**未 git commit**

### File List

- `examples/rv32_pipe/Cargo.toml`
- `examples/rv32_pipe/src/lib.rs`
- `examples/rv32_pipe/PIPE.md`
- `examples/rv32_core/SUBSET.md`
- `examples/rv32_core/COMPLIANCE.md`
- `Cargo.toml`
- `_agile-output/implementation-artifacts/17-4-五级流水-转发-分支-flush.md`
- `_agile-output/implementation-artifacts/17-4-code-review.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Suggested Review Order

**流水骨架与时序**

- 入口：五级 Reg + `pc_f` + 下游先赋的 seq 顺序
  [`lib.rs:26`](../../examples/rv32_pipe/src/lib.rs#L26)

- EX 转发（EX/MEM 优先且非 LW，其次 MEM/WB）
  [`lib.rs:461`](../../examples/rv32_pipe/src/lib.rs#L461)

- predict-not-taken：`take_br` / flush bubble / `next_pc`
  [`lib.rs:520`](../../examples/rv32_pipe/src/lib.rs#L520)

**文档与合同**

- 取指 (b)、无 load-use、仿真 arming
  [`PIPE.md:1`](../../examples/rv32_pipe/PIPE.md#L1)

- Episode I 仍单周期；流水在 rv32_pipe
  [`SUBSET.md:43`](../../examples/rv32_core/SUBSET.md#L43)

**黄金测试**

- clean / RAW 转发 / 负向 imm / BEQ flush
  [`lib.rs:781`](../../examples/rv32_pipe/src/lib.rs#L781)
