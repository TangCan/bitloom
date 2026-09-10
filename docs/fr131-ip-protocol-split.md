# FR131 — `ip.rs` protocol-module split

**Product:** Bitloom (`bitloom-prelude::ip`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 71 / FR131 closed** (Story **71.3**). Product path delivered in Story **71.2**.

Phase 15 **规划故事已齐（Epic 64–71）**；剩余实现关闭态见各 epic。协议加深（全 SoC pad）仍属 **FR128**（NFR55）；拆分本身已关闭，≠ FR128.

## Contract (NFR14 P1–P4)

| # | Gate | Evidence |
|---|------|----------|
| **P1** | Protocol modules | `ip/{sync_fifo,uart,spi,i2c,axi,gpio,blackbox,crc}.rs` + `mod.rs` |
| **P2** | Stable public API | `bitloom_prelude::ip::*` (e.g. `GpioVip`, `UartTx`) unchanged |
| **P3** | Regression | FR98 four-class + FR108 + FR120 elaborate/tests green |
| **P4** | Split itself | Monolith `ip.rs` removed; ≠ docs-only / assess-and-defer |

## Forbidden closes

Docs-only assessment; silent export renames; breaking dual-model / near-VIP.

## Non-regression (NFR52)

FR98 / FR108 / FR120 closes remain valid. Design crates still depend only on `bitloom-prelude`.
Unlisted SoC-pad deepen remains **FR128** / NFR55 (split alone ≠ full SoC pad).

```text
cargo test -p bitloom --test fr131_ip_protocol_split
cargo test -p bitloom --test fr131_epic71_closeout
```
