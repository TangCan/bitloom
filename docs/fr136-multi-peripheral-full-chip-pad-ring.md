# FR136 — Multi-peripheral / full-chip pad ring (Bitloom)

**Status:** product path landed (Story 75.2). Epic 75 closeout → Story **75.3**.

## Product path

Public elaboratable type: `bitloom_prelude::ip::ChipPadRing`.

| Predicate | Delivery |
| --- | --- |
| **R1** Multi-peripheral | GPIO SocPad face (`dir` / `wr_*` / `pad_in` / `pad_out`, 24-bit) + UART pad side (`uart_tx` / `uart_wr_en` / `uart_wr_data` / `uart_baud_div` / `uart_tx_busy`) |
| **R2** Full-chip shape | `CHIP_PAD_RING_BANK_COUNT = 3`, `PINS_PER_BANK = 8`, `PAD_WIDTH = 24`; `chip_pad_ring_bank_pin_index(bank, pin)` — beyond FR128 dual-bank×8＝16 |
| **R3** Ring scoreboard | ATDD expected vs observed `pad_out` sequence + UART `uart_tx` bitstream side-effect; deliberate wrong model → Fail |
| **R4** ATDD / boundary | `crates/bitloom/tests/fr136_multi_peripheral_full_chip_pad_ring.rs` |

## Boundary (alone ≠ FR136)

- **≠ FR128 D1–D4 alone** — `GpioSocPad` dual-bank 16 remains valid (NFR56); alone does not close FR136.
- **≠ FR120 C1–C4 alone** — `GpioVip` remains valid.
- **≠ FR108 alone** — `Gpio` remains valid.
- **≠ docs-only.**

## Layout

- Implementation: `crates/bitloom-prelude/src/ip/gpio/chip_ring.rs` (FR139 `base`/`vip`/`socpad` intact).
- Design crates depend only on **`bitloom-prelude`**.

## Non-goals (NFR59)

SPI/I2C/AXI pad mux; drive strength / debounce / analog; third-party VIP binary co-sim.
