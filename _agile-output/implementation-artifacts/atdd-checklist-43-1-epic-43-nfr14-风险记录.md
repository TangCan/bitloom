# ATDD Checklist — Story 43.1

**Story:** 43-1-epic-43-nfr14-风险记录  
**Phase:** red → green (NFR14 VIP / full-protocol IP gate ATDD)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| NFR14 fields (a)–(d) | `nfr14_risk_epic43_vip_full_protocol_ip_has_required_fields` | missing sections |
| UART/SPI/I2C/AXI（+GPIO）近 VIP 必选条 | same | FR98 criteria undefined |
| Forbid 只加深一类宣称 FR98 全绿（除非显式裁剪） | same | single-class fake green |
| Forbid 无 ATDD 宣称 VIP | same | docs-only VIP claim |
| Gate 43.2–43.5 | same | premature ready |
| Owner NFR14 / NFR40 | same | no accountable owner |
