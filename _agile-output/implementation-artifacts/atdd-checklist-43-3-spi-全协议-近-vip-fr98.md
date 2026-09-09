# ATDD Checklist — Story 43.3

**Story:** 43-3-spi-全协议-近-vip-fr98  
**Phase:** red → green (FR98 SPI near-VIP S1–S4)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| S1 configurable CPOL/CPHA (4 modes; Mode-0 vs FR82) | `fr98_spi_cpol_cpha_idle_and_emit` + Mode-0 tick | Mode-0-ish only / no config |
| S2 Master multi-byte + `cs_n` frame boundary | `fr98_spi_multibyte_cs_frame` | single-byte only / CS glitch |
| S3 elaborate→emit→tick + ATDD | emit + Mode-0/multi tick tests | docs-only claim |
| S4 docs delivered modes vs non-goals | `fr98_docs_ip_spi_near_vip_boundaries` | dishonest VIP / silent expand |
| Scope: not FR98 full green / not I2C·AXI | docs + sprint discipline | single-class fake green |
| Brand Bitloom / prelude-only | docs test | wrong brand |
| RX readable (beyond FR82 TX-only latch) | Mode-0 tick `rx_data`/`rx_valid` | TX-only deepen |
