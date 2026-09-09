# Automation Summary — Story 41.4 (FR95/FR96 closeout)

**Mode:** Expand after implementation (Epic 41 docs/ATDD closeout)  
**Date:** 2026-09-09

## Coverage decision

Existing ATDD from 41.2/41.3 (`fr95_*`, `fr96_*`) already lock schedule IR / no-Bambu / closure dissolve. Story 41.4 adds **closeout-only** docs gates; no further product-code automation needed.

## New / updated tests

| Test | Role |
| ---- | ---- |
| `fr95_fr96_epic41_closeout` (6) | NFR14 close, README honesty, external≠sole FR95, deferred Epic 41 closed, sprint epic-41 done / epic-42 untouched, brand |
| `hls_supported_docs` (updated) | Align with in-tree FR95 public surface |

## Risk residual

| Risk | Level | Mitigation |
| ---- | ---- | ---- |
| Public docs re-forbid in-tree | High | Closeout ATDD + hls_supported_docs |
| External path steal FR95 | High | README/fr35 + closeout string gates |
| Silent Epic 42 start | Medium | sprint ATDD |

**Verdict:** ATDD sufficient; no extra automate suite beyond closeout + regression.
