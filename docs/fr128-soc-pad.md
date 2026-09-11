# FR128 — Full SoC pad / commercial scoreboard depth

**Product:** Bitloom (`bitloom_prelude::ip::GpioSocPad`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 68 / FR128 closed** (Story **68.3**). Product path delivered in Story **68.2**.

FR120 `GpioVip` C1–C4 close **remains valid** (NFR52). Multi-peripheral / full-chip pad ring beyond `GpioSocPad` D1–D4: **FR136 / Epic 75 closed** (Story **75.3**; `ChipPadRing`); see [`docs/fr136-multi-peripheral-full-chip-pad-ring.md`](fr136-multi-peripheral-full-chip-pad-ring.md). Broader pad/peripherals remain **NFR59**.

Beyond FR120 `GpioVip` C1–C4: dual-bank pad ring, falling-edge IRQ, AXI-style CSR
window, and scoreboard ATDD.

## Contract (NFR14 D1–D4)

| # | Gate | Evidence |
|---|------|----------|
| **D1** | Dual-bank pad | 16-bit `pad_in`/`pad_out`/`dir`/`rd_data` (bank0=`[7:0]`, bank1=`[15:8]`) |
| **D2** | Falling-edge IRQ | `irq_fall_en` / `irq_fall_status` / `irq_fall_clear`; `irq_out` = rise∨fall |
| **D3** | AXI-style CSR | `csr_wen`/`csr_addr`/`csr_wdata`/`csr_rdata` |
| **D4** | Scoreboard ATDD | `cargo test -p bitloom --test fr128_soc_pad` |

## Forbidden closes

FR108 alone; `GpioVip` C1–C4 alone; docs-only.

## Non-regression (NFR52)

FR120 / FR108 / FR98 closes remain valid. Design crates still depend only on `bitloom-prelude`.

```text
cargo test -p bitloom --test fr128_soc_pad
cargo test -p bitloom --test fr128_epic68_closeout
```
