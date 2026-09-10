# FR131 — `ip.rs` protocol-module split

**Product:** Bitloom (`bitloom-prelude::ip`). Unrelated to `samitbasu/rhdl`.

**Status:** Product path delivered in Story **71.2** (Epic 71 close → Story **71.3**).

Beyond a single ~3213 LOC `ip.rs` monolith: protocol modules under
`crates/bitloom-prelude/src/ip/` with stable public re-exports.

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

```text
cargo test -p bitloom --test fr131_ip_protocol_split
```
