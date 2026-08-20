# Episode I ISA subset (Story 15.2)

Public brand: **Bitloom**. Unrelated to `samitbasu/rhdl`.

## Implemented

| Instr | Notes |
|-------|--------|
| `ADDI` | I-imm zero-extended (tests use non-negative) |
| `ADD` | R-type, funct3/funct7 = 0 |
| `BEQ` | Taken → **PC+8** (teaching offset; full B-imm decode later) |

Architectural regs in this core: `x0`=0, `x1`–`x4` only. Harness presents `instr` each cycle (IMEM deferred to 15.3).

## Non-goals

No CSR/ECALL/EBREAK/FENCE, no `lw`/`sw` (15.3), no MMU/Linux/pipeline.
