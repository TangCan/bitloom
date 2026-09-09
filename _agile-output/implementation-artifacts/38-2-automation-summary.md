# Automation summary — Story 38.2

**Mode:** Expand after implementation (FR89 UartTx deepen)  
**Date:** 2026-09-09

## Coverage decision

Story 38.2 implementation is guarded by ATDD
`crates/bitloom/tests/fr89_uarttx_programmable_baud.rs`:

- elaborate → emit `.v` → tick with `baud_div > 0` (bit hold)
- `baud_div=0` FR82 one-clk-per-bit compatibility
- docs/ip README: FR89 / programmable baud + non-goals (RX / VIP / 全双工)
- no generator closures; no `UartRx` (branch B)

Prelude unit tests also cover frame + programmable hold
(`uart_tx_elaborate_emit_tick`, `uart_tx_programmable_baud_holds_bits`).

**No additional automate tests** — would duplicate ATDD without new risk surfaces.
Epic 38.3 owns closure ATDD / deferred-work cross-ref / optional out-of-tree example.

## Harm / priority

| Risk | Level | Automation |
| ---- | ----- | ---------- |
| Deepen claimed without baud timing | High | Covered by FR89 ATDD |
| FR82 regression via baud_div | High | baud_div=0 ATDD + fr82 fixtures |
| RX / VIP silently claimed | High | docs + no-UartRx ATDD |

## Outcome

Automate step: **accept ATDD + prelude tests as sufficient guardrail suite**; no new files.
