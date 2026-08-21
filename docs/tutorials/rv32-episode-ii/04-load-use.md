# Chapter 4 — Load-use 停顿

一步一变：在已有转发之上，只加 **load-use hazard 检测 + stall**。

## 引入

`LW` 数据在 MEM 末才就绪，紧邻消费者在 EX 初就要用 → 不能单靠 EX/MEM 旁路（且 `em_not_lw` 禁止把 EA 当 load 数据转发）。

Harris 式：

1. ID/EX 为 LW 且 IF/ID 源寄存器匹配 → **冻结 PC / IF-ID**
2. 向 ID/EX 插入 **bubble**
3. 随后靠既有 **MEM/WB→EX** 转发

Stall 用 **mux hold**（非模块级 `en`）。`rs2` 匹配仅对 ADD/BEQ/SW 门控。

## 验收

独立 ATDD（无停顿会失败）：

```bash
cargo test -p rv32_pipe tick_load_use_stall_atdd_golden
cargo test -p rv32_pipe tick_load_use_rs2_consumer_atdd_golden
```

- `tick_load_use_stall_atdd_golden` — rs1 消费者（`ADDI`）
- `tick_load_use_rs2_consumer_atdd_golden` — rs2 消费者（`ADD`）

类型：`tick`。合规阶梯见 `COMPLIANCE.md`（定向黄金；可选 `rv32ui` 延期；非完整 DV）。Harris/FemtoRV 对照见 [femtorv-compare.md](./femtorv-compare.md)。
