# ATDD Checklist — Story 43.2

**Story:** 43-2-uart-全协议-近-vip-fr98  
**Phase:** red → green (FR98 UART near-VIP U1–U5)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| U1 TX+RX reachable (full-duplex = dual instantiate) | `fr98_uart_tx_rx_elaborate_emit` | RX missing / TX-only deepen |
| U2 programmable baud on TX and RX | `fr98_uart_loopback_baud_div` | baud only on TX |
| U3 8N1 frame + documented surface | `fr98_uart_loopback_8n1` + docs test | wrong framing |
| U4 elaborate→emit→tick both paths | loopback + emit tests | docs-only claim |
| U5 docs delivered vs non-goals | `fr98_docs_ip_uart_near_vip_boundaries` | dishonest VIP claim |
| Scope: not FR98 full green / not SPI·I2C·AXI | docs + sprint discipline | single-class fake green |
| Brand Bitloom / prelude-only | docs test | wrong brand |
