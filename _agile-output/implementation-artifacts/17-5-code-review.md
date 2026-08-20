# Code review — Story 17.5

**Verdict: approve**

## Findings

1. **(pass)** Load-use：`id_ex_is_lw` ∧ rd≠0 ∧ (rs1 匹配 ∨ 门控 rs2) → 冻 PC/IF-ID、bubble ID/EX；EX/MEM/WB 继续。
2. **(pass)** 无 `load_q`→EX 旁路；保留 `em_not_lw`；stall 后靠 MEM/WB→EX。
3. **(pass)** ATDD `tick_load_use_stall_atdd_golden`：强制 `do_stall=0` 时 x4=1，正确实现 x4=43。
4. **(pass)** 既有 clean / ALU RAW / BEQ flush / 负向 imm 黄金仍绿。
5. **(pass)** `COMPLIANCE.md` 验证阶梯 directed → 可选 rv32ui（延期）；未启用 arch-test；无完整 DV 宣称。
6. **(pass)** 无 CSR；设计依赖仅 `bitloom-prelude`；`cargo test -p rv32_pipe` / `-p rv32_core` / `cargo bitloom build --package rv32_pipe` 绿。
7. **(info)** `rs2` stall 仅 ADD/BEQ/SW：避免 I-type `imm[4:0]` 伪冲突。
8. **(info)** stall 用 mux hold，非模块级 `en`（与 17.1 闸门一致）。
9. **(defer)** 无独立 rs2 消费者黄金；ATDD 不断言 stall 拍数（见 `deferred-work.md`）。

## Review-loop patches applied

- 门控 `stall_rs2` 于 `use_rs2`
- 修正 ATDD 注释（无 stall → x4 错）

No blocking defects. **未 git commit**（按用户要求）。
