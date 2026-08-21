# Episode I / II ISA subset (Stories 15.2–15.3 + Epic 17)

Public brand: **Bitloom**. Unrelated to `samitbasu/rhdl`.

## Implemented

| Instr | Notes |
|-------|--------|
| `ADDI` | I-imm **sign-extended** from instr bit31 (Story 17.3) |
| `ADD` | R-type, funct3/funct7 = 0 |
| `BEQ` | B-imm `{31,7,30:25,11:8,0}` then sign-extend from bit12 (=instr[31]); positive and negative offsets |
| `LW` | Word load from DMEM[ea[3:0]] (async Mem); I-imm sign-extended |
| `SW` | Word store gated by `assign_mem_write_en(..., dmem_we)` where `dmem_we = is_sw && !is_mmio`; ea=`0x100` → LED MMIO only (no DMEM[0] bypass); S-imm sign-extended |

Decode also rebuilds **U-imm** (`{instr[31:12],12'b0}`) and **J-imm** (`{31,19:12,20,30:21,0}` + sext) into a unified `imm` bus for future LUI/AUIPC/JAL — those opcodes are **not** executed in this subset yet.

Architectural regs: `x0`=0, `x1`–`x4`.

## Fetch strategy (FR69 / Story 17.2) — LOCKED

**Chosen: (b) harness `instr` port** — the design exposes a 32-bit input `instr`; the test/tutorial harness presents the instruction word each cycle. This continues Episode I and is the **sole** Episode II first-cut fetch contract through Stories 17.3–17.5.

**Do not silently mix** with **(a)** on-chip `SyncReadMem` instruction memory in IF. A design must pick one semantic; dual paths without an explicit story/SUBSET change are a contract violation.

### CPI / teaching implications (b)

- IF does **not** model on-chip IMEM or SyncReadMem read latency=1; CPI teaching for fetch is “instr arrives from the harness,” not “PC indexes IMEM then data appears next cycle.”
- Golden `tick` tests remain responsible for driving `instr` in lockstep with the PC / pipeline stage they intend to exercise (same pattern as Episode I).
- Tutorials must state that instruction bytes come from the harness/`instr` port, not from an internal ROM, until a later story flips this section.

### Deferred on-chip I-fetch (a) — when allowed

On-chip SyncReadMem I-fetch (AD-21) is **deferred**, not rejected. Surface proof already exists in `examples/rv32_feasibility` (Story 15.1). Adopt (a) only via a dedicated later story that:

1. Replaces or clearly supersedes this SUBSET section (still **one** strategy — no silent dual semantics).
2. Documents IF ports / init (address from PC, SyncReadMem depth/width, reset/init contents, read-enable if any) for the pipeline package.
3. Updates harness/tests so they no longer pretend `instr` is the architectural fetch path.

Until then, 17.4–17.5 must obey **(b)**.

## Deferred (documented, not silent)

- **SyncReadMem instruction fetch** — see Fetch strategy above; optional/later story after hazard lands.
- **LB / LH / LBU / LHU / SB / SH** — byte/half load-store and load sign-/zero-extend are **out of subset**. DMEM is word-addressed (`ea[3:0]` → 16×32); teaching path stays `LW`/`SW` only so imm/sign-extend freeze (17.3) is not entangled with sub-word align/mux. A later story may add them with explicit SUBSET rows.
- **LUI / AUIPC / JAL / JALR** and remaining RV32I ops — U/J imm rebuilt in decode; execute paths not wired yet.
- No EBREAK/FENCE, no MMU/Linux. Optional Zicsr/ECALL/MRET teaching path：`examples/rv32_priv`（非本包）。
- Classic 5-stage + forward + load-use stall + branch flush lives in **`examples/rv32_pipe`** (Stories 17.4–17.5 / `PIPE.md`); this package stays Episode I single-cycle.
- **Optional Zicsr + M-mode trap (FR65 / NFR32):** **optional implemented** in [`examples/rv32_priv`](../rv32_priv/) — teaching minimum only; see [`docs/tutorials/rv32-episode-ii/06-csr-m-trap.md`](../../docs/tutorials/rv32-episode-ii/06-csr-m-trap.md). Does **not** block Epic 17 Done. Not Privileged/arch-test compliance.
