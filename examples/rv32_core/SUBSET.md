# Episode I ISA subset (Stories 15.2–15.3 + action follow-ups)

Public brand: **Bitloom**. Unrelated to `samitbasu/rhdl`.

## Implemented

| Instr | Notes |
|-------|--------|
| `ADDI` | I-imm zero-extended (tests use non-negative) |
| `ADD` | R-type, funct3/funct7 = 0 |
| `BEQ` | B-imm bit-field decode (positive offsets; sign-extend deferred) |
| `LW` | Word load from DMEM[ea[3:0]] (async Mem) |
| `SW` | Word store gated by `assign_mem_write_en(..., is_sw)`; ea=`0x100` → LED MMIO |

Architectural regs: `x0`=0, `x1`–`x4`. Harness presents `instr` each cycle.

## Deferred (documented, not silent)

- **SyncReadMem instruction fetch** — Episode II / later; Episode I keeps harness `instr` (tutorial Ch.1).
- **B-imm sign-extend** — negative offsets not required by current goldens.
- No CSR/ECALL/EBREAK/FENCE, no MMU/Linux/pipeline.
