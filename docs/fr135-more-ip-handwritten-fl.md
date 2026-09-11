# FR135 — More IP handwritten FL (beyond Gpio)

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 74 / FR135 closed** (Story **74.3**). F1–F3 delivered in Story **74.2** (`UartTxFunctional` ≡ tick).

Phase 16 **规划故事已齐（Epic 72–78）**；实现关闭态：**Epic 72**（闸门 FR133）、**Epic 73**（FR134）、**Epic 74**（本 FR）、**Epic 75**（FR136）、**Epic 78**（FR139）**已关闭**；**Epic 76–77** 仍须各自实现关闭。FR126 Gpio F1–F3 **仍有效**（NFR56）and is **not** this face alone. 未列入协议手写 FL 仍属 **NFR59**。终局宣称须对应 **FR133–139** 关闭后方可勾选（**FR140**）。

Beyond FR126 `GpioFunctional`, FR103 SyncFifo handwritten, and FR103 UART/SPI/I2C/AXI **GeneratedFunctional**. See also [`docs/fr126-more-ip-handwritten-fl.md`](fr126-more-ip-handwritten-fl.md).

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

FR103 / FR112 / FR119 / **FR126 Gpio** closes remain valid. FR126 alone ≠ FR135.

## Deferred (NFR59)

Unlisted protocols (`UartRx` / SPI / I2C / AXI handwritten FL, etc.) remain **NFR59** — require a new contract.

ATDD: `cargo test -p bitloom --test fr135_more_ip_handwritten_fl`  
Closeout: `cargo test -p bitloom --test fr135_epic74_closeout`
