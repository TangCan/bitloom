# ATDD Checklist — Story 45.1

**Story:** 45-1-epic-45-nfr14-风险记录  
**Phase:** red → green (NFR14 formal-equiv + dual-model gate ATDD)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| NFR14 fields (a)–(d) | `nfr14_risk_epic45_formal_equiv_dual_model_has_required_fields` | missing sections |
| Formal-equivalence bounds (tool/proof/fixture) | same | FR100 bounds undefined |
| Property-macro matrix inventory | same | FR102 matrix missing |
| Dual-model first-class IP set | same | FR103 IP set undefined |
| Forbid random scoreboard alone as formal product | same | FR92 fake green for FR100 |
| Forbid template-adapter-only close of FR102/103 | same | adapter fake green |
| Gate 45.2–45.4 | same | premature ready |
| Owner NFR14 / NFR40 | same | no accountable owner |
