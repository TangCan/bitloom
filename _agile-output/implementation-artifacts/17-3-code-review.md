# Code review — Story 17.3

**Verdict: approve**

## Findings

1. **(pass)** I/S/B/U/J imm rebuild present; sign from `instr[31]`; B-imm = `{31,7,30:25,11:8,0}` without whole-field `<<1`.
2. **(pass)** Golden ticks: negative ADDI, negative BEQ (−8), negative SW→LED; plus software U/J reconstruct contract.
3. **(pass)** `SUBSET.md` drops deferred B-imm sext; freezes LB/LH/… as out-of-subset (word LW/SW only).
4. **(pass)** Still single-cycle edge-commit; no IF/ID/EX/MEM/WB regs; design dep `bitloom-prelude` only.
5. **(pass)** `cargo test -p rv32_core` and `cargo bitloom build --package rv32_core` green (workspace CLI path backends).
6. **(info)** ALU/`ea`/`branch_tgt` masked with `0xffff_ffff` after `assign_add` so signed wrap matches 32-bit wires under `bitloom-sim` u64 Add.

No blocking defects.
