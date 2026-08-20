# Chapter 5 — 分支

引入：`BEQ`；B-type immediate 位域解码（正偏移）；`branch_tgt = pc + imm_b`。

验收：

```bash
cargo test -p rv32_core tick_beq_taken_jumps_plus8
```

对照表：[femtorv-compare.md](./femtorv-compare.md)。