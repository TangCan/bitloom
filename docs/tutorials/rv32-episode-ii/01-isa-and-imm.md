# Chapter 1 — ISA / 立即数与符号扩展

一步一变：先确认用户态立即数与符号扩展已冻结，再谈流水。

## 引入

Episode I（Story 17.3）已在 `examples/rv32_core` 冻结：

- I/S/B/U/J 立即数拼装
- 符号扩展以 instr **bit31** 为符号位（B-imm 符号位 = bit12 重建后的最高位）
- 负向 `ADDI` / 负向 `BEQ` 有独立黄金

`rv32_pipe` **复用**同一 imm 合同；本集不重新发明编码。

阅读：`examples/rv32_core/SUBSET.md`（Implemented + Fetch strategy）。

## 验收

```bash
cargo test -p rv32_core tick_addi_negative_imm_golden
cargo test -p rv32_core tick_beq_taken_jumps_minus8
cargo test -p rv32_pipe tick_addi_negative_imm_pipe_golden
```

本批章节**不**要求 CSR/trap（NFR32）。
