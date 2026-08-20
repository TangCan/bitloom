# Episode I ISA subset (Stories 15.2–15.3)

Public brand: **Bitloom**. Unrelated to `samitbasu/rhdl`.

## Implemented

| Instr | Notes |
|-------|--------|
| `ADDI` | I-imm zero-extended (tests use non-negative) |
| `ADD` | R-type, funct3/funct7 = 0 |
| `BEQ` | Taken → **PC+8** (teaching offset) |
| `LW` | Word load from DMEM[ea[3:0]] (async Mem) |
| `SW` | Word store; ea=`0x100` updates **LED MMIO** (`led_out`) |

Architectural regs: `x0`=0, `x1`–`x4`. Harness presents `instr` each cycle.

## Non-goals

No CSR/ECALL/EBREAK/FENCE, no MMU/Linux/pipeline.
