# FR135 — More IP handwritten FL (beyond Gpio)

**Product:** Bitloom. **Status:** Story **74.2** delivers F1–F3 product path (Epic 74 closeout → **74.3**).

Beyond FR126 `GpioFunctional`, FR103 SyncFifo handwritten, and FR103 UART/SPI/I2C/AXI **GeneratedFunctional**.

## Selected IP (NFR14 F1–F3)

| Item | Contract |
|------|----------|
| IP | `bitloom_prelude::ip::UartTx` |
| FL | `bitloom_sim::UartTxFunctional` |
| API | `IpDualModelMatrix::verify_uart_tx_handwritten` |
| Stimulus | `uart_tx_dual_stimulus` |
| Ports | `tx`, `tx_byte`, `tx_busy` |

## Forbidden closes

FR126 Gpio alone；GeneratedFunctional alone；SyncFifo / FR103 alone；FR92 alone；FR100 F1-(i) alone；FR112 / FR119 alone；docs-only.

## Non-regression (NFR56)

FR103 / FR112 / FR119 / FR126 closes remain valid.

ATDD: `cargo test -p bitloom --test fr135_more_ip_handwritten_fl`
