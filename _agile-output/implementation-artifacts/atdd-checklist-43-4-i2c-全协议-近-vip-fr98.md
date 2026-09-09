# ATDD Checklist — Story 43.4

**Story:** 43-4-i2c-全协议-近-vip-fr98  
**Phase:** red → green (FR98 I2C near-VIP I1–I4)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| I1 ACK/NACK-driven Master write | `fr98_i2c_write_ack` + `fr98_i2c_addr_nack` | sticky sample only / ignore ACK |
| I1 documented Master read path | `fr98_i2c_read_byte` | write-only deepen |
| I2 START / 7-bit addr / data / STOP | write + read tick paths | FR82 data-only toy |
| I2 SCL real half-period edges | emit + tick `scl` high/low | `scl`恒高玩具 |
| I3 elaborate→emit→tick + ATDD | emit + tick tests | docs-only claim |
| I4 docs delivered vs non-goals | `fr98_docs_ip_i2c_near_vip_boundaries` | dishonest VIP / silent expand |
| Scope: not FR98 full green / not AXI·GPIO | docs + sprint discipline | single-class fake green |
| Brand Bitloom / prelude-only | docs test | wrong brand |
