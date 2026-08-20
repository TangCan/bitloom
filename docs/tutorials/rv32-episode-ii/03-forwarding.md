# Chapter 3 — ALU RAW 转发

一步一变：只加 **EX/MEM→EX**（优先，且生产者非 LW）与 **MEM/WB→EX** 转发；仍不做 load-use stall。

## 引入

ALU 生产者在 EX+ 后即可旁路到 EX 的操作数 mux；`rd≠0` 才匹配。

故意用紧邻 `ADDI`→`ADDI`/`ADD` 序列测 RAW：没有转发会读到陈旧 RF。

阅读：`PIPE.md`「转发」行；实现见 `examples/rv32_pipe/src/lib.rs` 转发 mux。

## 验收

```bash
cargo test -p rv32_pipe tick_alu_alu_raw_forward_golden
```

类型：`tick`。本批**不**要求 CSR。
