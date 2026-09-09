# ATDD Checklist — Story 43.5

**Story:** 43-5-axi-可选-gpio-fr98-收口  
**Phase:** red → green (FR98 AXI near-VIP A1–A4 + Epic 43 closeout)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| A1 AW/W/B + AR/R handshake | `fr98_axi_write_read_handshake` | broken ready/valid |
| A1 addr decode | `fr98_axi_multi_reg_decode` | ignore addr toy |
| A1 wstrb byte merge | `fr98_axi_wstrb_partial_write` | ignore wstrb |
| A2 multi-register window | `fr98_axi_multi_reg_decode` | single-reg toy |
| A3 elaborate→emit→tick | emit + tick tests | docs-only claim |
| A4 docs Lite vs Full / non-goals | `fr98_docs_ip_axi_near_vip_boundaries` | Full AXI fake / dishonest |
| GPIO optional not required | docs + closeout | claim GPIO VIP or fail on missing G0 |
| FR98 / Epic 43 close | `fr98_epic43_closeout` | permanent non-goal lock / unchecked NFR14 |
| Brand Bitloom / prelude-only | docs test | wrong brand |
| Sprint epic-43 done; 44+ backlog | closeout sprint assert | start Epic 44+ |
