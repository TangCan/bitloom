# ATDD Checklist — Story 46.1

**Story:** 46-1-epic-46-nfr14-风险记录  
**Phase:** red → green (NFR14 SystemC TLM-2.0 product-path gate ATDD)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| NFR14 fields (a)–(d) | `nfr14_risk_epic46_systemc_tlm_has_required_fields` | missing sections |
| TLM-2.0 deliverables (lib/generator/examples/deps) | same | FR101 deliverables undefined |
| LT/AT scope | same | abstraction scope missing |
| Relationship to cycle-accurate path | same | CA confusion |
| Forbid docs-slogan-only FR101 close | same | slogan fake green |
| Forbid host Rust FL as SystemC TLM | same | FR47 fake green |
| Contrast FR47 vs FR101 | same | confusion with Rust FL gen |
| Cite revised AD-5 / NFR41 | same | NFR41 violation |
| Gate 46.2–46.3 | same | premature ready |
| Owner NFR14 / NFR41 | same | no accountable owner |
