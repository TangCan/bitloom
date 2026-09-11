# FR126 — More IP handwritten FL (branch C)

**Product:** Bitloom. **Status:** **Epic 66 / FR126 closed** (Story **66.3**). F1–F3 delivered in Story **66.2**.

Beyond FR103 SyncFifo handwritten + GeneratedFunctional UART/SPI/I2C/AXI, and beyond FR112-B / FR119 alone. Further handwritten FL beyond Gpio: **FR135 / Epic 74 closed** (Story **74.3**; `UartTxFunctional`); see [`docs/fr135-more-ip-handwritten-fl.md`](fr135-more-ip-handwritten-fl.md). Unlisted protocols remain **NFR59**.

## Selected IP (NFR14 F1–F3)

| Item | Contract |
|------|----------|
| IP | `bitloom_prelude::ip::Gpio` |
| FL | `bitloom_sim::GpioFunctional` |
| API | `IpDualModelMatrix::verify_gpio_handwritten` |
| Stimulus | `gpio_dual_stimulus` |
| Ports | `pad_out`, `rd_data` |

## Forbidden closes

FR92 alone; FR100 F1-(i) alone; FR112 MemRead≡tick alone; FR119 sby alone; docs-only.

## Non-regression (NFR52)

FR103 / FR112 / FR119 closes remain valid.

ATDD: `cargo test -p bitloom --test fr126_more_ip_handwritten_fl`
