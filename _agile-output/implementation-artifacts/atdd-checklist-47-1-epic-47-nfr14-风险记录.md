# ATDD Checklist — Story 47.1

**Story:** 47-1-epic-47-nfr14-风险记录  
**Phase:** red → green (NFR14 interactive waveform + coverage gate ATDD)  
**Date:** 2026-09-09

| Requirement | Test ID | Failure mode if missing |
| ---- | ---- | ---- |
| NFR14 fields (a)–(d) | `nfr14_risk_epic47_waveform_coverage_has_required_fields` | missing sections |
| Interactive acceptance (browse/zoom/search or embedded viewer) | same | FR104 interactive bar undefined |
| Coverage metric types + report format | same | FR105 metrics/format undefined |
| Forbid static-HTML-only FR104 close | same | FR38/49 fake green |
| Forbid docs-only / no-recorder FR105 close | same | docs slogan fake green |
| Contrast FR38/49 VCD / timing HTML | same | confusion with static viz |
| Contrast FR34 baseline coverage | same | confusion with toggle-only |
| Gate 47.2–47.3 | same | premature ready |
| Owner NFR14 / NFR40 | same | no accountable owner |
