# Chapter 2 — 译码

引入：从 `instr` 抽出 `opcode` / `rd` / `rs1` / `rs2` / 立即数（`assign_shr` / `assign_and`）。

验收：`cargo test -p rv32_core elaborate_ok`；对照 `SUBSET.md`。
