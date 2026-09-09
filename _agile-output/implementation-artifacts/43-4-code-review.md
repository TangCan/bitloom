# Code Review — Story 43.4

**Story:** 43-4-i2c-全协议-近-vip-fr98  
**Date:** 2026-09-09  
**Verdict:** Approve

## Findings

1. **I1–I4 delivered.** `I2cMaster` has ACK/NACK-driven write, documented read (`rx_data`/`rx_valid`), START/7-bit addr/data/STOP, half-period SCL; ATDD `fr98_i2c_near_vip` covers emit + write ACK + addr NACK + read + docs.
2. **Scope discipline.** No AXI/GPIO deepen; NFR14 Epic 43 closeout remains open (UART+SPI+I2C rows checked; AXI/docs/epic close unchecked); sprint leaves `43-5` backlog and `epic-43` in-progress.
3. **Honest boundaries.** `docs/ip/README.md` states delivered vs non-goals (stretch/multi-master/10-bit/slave/PEC); no four-class FR98 green claim.
4. **FR82 regression.** Baseline I2C smoke updated for addr/rw + SCL half0; still proves non-stub.

| AC | Result |
| ---- | ---- |
| I1 ACK/NACK write + read | pass |
| I2 START/addr/data/STOP + SCL edges | pass |
| I3 elaborate→emit→tick ATDD | pass |
| I4 docs non-goals | pass |
| No AXI/GPIO; no FR98 full close | pass |

## Residual risk

- Single-byte payload only (I1–I4 do not require multi-byte); multi-byte / clock stretch remain deferred.
- Read ATDD is cycle-count coupled to FSM; prefer keeping phase comments in sync if FSM changes.
