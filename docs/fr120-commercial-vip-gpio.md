# FR120 — Commercial VIP GPIO

**Product:** Bitloom (`bitloom-prelude::ip::GpioVip`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 61 / FR120 closed** (Story **61.3**). Product path delivered in Story **61.2**
(`GpioVip` elaborate→emit→tick + ATDD). Phase 12 FR98 four-class near-VIP and Phase 13
FR108 `Gpio` P1–P4 remain closed (**NFR48**). Unlisted protocols stay **deferred** (**NFR51**).

This page is the **FR120 completion surface**. It delivers commercial VIP GPIO **beyond**
FR108 near-VIP (P1–P4) and **beyond** FR98 UART/SPI/I2C/AXI alone.

## Selected surface (NFR14)

See `_agile-output/implementation-artifacts/nfr14-risk-epic61-commercial-vip-gpio.md`.

| # | Mode | Product face |
|---|------|----------------|
| **C1** | Rising-edge IRQ | `irq_en` / `irq_clear` / `irq_status` / `irq_out` (1-bit bank OR) |
| **C2** | Open-drain + OE | per-bit `od`; `pad_oe`; `pad_out = out & pad_oe` |
| **C3** | Atomic set/clear | `set_en`/`set_data`, `clr_en`/`clr_data` (priority: wr → set → clr) |
| **C4** | ATDD | `cargo test -p bitloom --test fr120_gpio_commercial_vip` |

Baseline FR108 `Gpio` (P1–P4) remains available and regression-tested.

## Product entry

```bash
cargo test -p bitloom --test fr120_gpio_commercial_vip
cargo test -p bitloom-prelude --lib gpio_vip
```

| Type | Role |
|------|------|
| `bitloom_prelude::ip::GpioVip` | **FR120** commercial VIP bank |
| `bitloom_prelude::ip::Gpio` | **FR108** near-VIP only (≠ FR120 alone) |

Index: [`ip/README.md`](ip/README.md).

## Honest non-goals / uncovered (NFR51)

**FR120 closed ≠** the following (must not silent-claim):

- Full SoC pad ring / multi-bank interconnect / I/O standard library
- Commercial co-sim scoreboard vs third-party GPIO VIP
- Debounce, drive strength / slew, analog / differential pairs
- Level-sensitive / falling-edge / dual-edge IRQ mode family (this FR nails **rising-edge** only)
- GPIO↔AXI register-window first-class interconnect fixture

## Cross-links

| Doc | Role |
|-----|------|
| [`ip/README.md`](ip/README.md) | IP index (FR98 / FR108 / FR120) |
| [`fr103-ip-dual-model.md`](fr103-ip-dual-model.md) | Dual-model; GPIO commercial pointer |
| NFR14 Epic 61 | `_agile-output/implementation-artifacts/nfr14-risk-epic61-commercial-vip-gpio.md` |
| FR108 | Epic 50 near-VIP (`Gpio`) — still closed |
| FR98 | Epic 43 four-class near-VIP — still closed |
