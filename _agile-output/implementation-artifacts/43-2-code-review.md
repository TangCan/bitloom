# Code Review — Story 43.2

**Story:** `43-2-uart-全协议-近-vip-fr98`  
**Date:** 2026-09-09  
**Verdict:** Approve

## Summary

1. **U1–U5 met.** `UartTx` retained; new `UartRx` (8N1, `baud_div`, falling-edge start, end-of-baud sample, `rd_valid` pulse). Full-duplex = dual instantiate (documented). ATDD loopback covers TX+RX elaborate→emit→tick at `baud_div=0` and `>0`.
2. **Scope discipline.** No SPI/I2C/AXI/GPIO implementation; NFR14 Epic 43 close checkboxes untouched; sprint leaves `43-3`–`43-5` backlog and `epic-43` in-progress.
3. **Regression honesty.** FR89 ATDD updated to keep Epic 38 historical “branch B not selected” without forbidding FR98 `UartRx`; FR82 matrix splits cleanly on `UartRx`.

## AC trace

| AC | Result |
| --- | --- |
| U1 TX+RX / full-duplex contract | pass |
| U2 baud on TX and RX | pass |
| U3 8N1 + docs frame surface | pass |
| U4 elaborate→emit→tick both paths | pass |
| U5 docs delivered vs non-goals; Bitloom | pass |
| No SPI/I2C/AXI; no FR98 full close | pass |

## Residual / non-blocking

- Mid-bit RX sampling not implemented (end-of-period documented); adequate for near-VIP MVP.
- Framing-error ports remain non-goals (documented).

**Approve** — mark `43-2` done; keep `epic-43` in-progress.
