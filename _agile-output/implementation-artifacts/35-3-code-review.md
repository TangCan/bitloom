# Code Review — Story 35.3 (FR84 SoftF16 explicit defer)

**Verdict: Approve**

**Date:** 2026-09-09  
**Scope:** Path B explicit defer — PRD addendum + `docs/fr36` + crate docs + NFR14 tick + ATDD

## Summary

FR84 **Option B** is landed: SoftF16 synthesizable HIR→emit remains deferred; user docs and PRD addendum forbid claiming synthesizable SoftF16 / float delivered; NFR14 marks 已选 B and ticks FR84 close checkbox. No Option A fake `Bits<16>`-as-float fixture. No blocking defects.

## AC checklist

| AC | Result |
| --- | --- |
| Given 35.1; exactly one of (A) SoftF16→HIR→emit fixture **or** (B) PRD/user docs explicit defer + forbid synthesizable claim | **pass** — Path B only |
| Close condition auto- or doc-checkable | **pass** — `cargo test -p bitloom --test fr84_softf16_explicit_defer` |
| If (B): addendum or equivalent contract paragraph exists and matches implementation | **pass** — PRD addendum + `docs/fr36-rhdl-float.md` + host-only `rhdl-float` |

## Findings

1. **None blocking.** Contract surfaces (fr36, addendum, NFR14, crate `//!`) agree on Option B / host-only.
2. **Non-blocking:** Historical `docs/requirements/15. 可综合子集与限制.md` still describes aspirational synthesizable float library wording — not the live FR84 user contract (live path is `docs/fr36-rhdl-float.md`). Out of story scope to rewrite Phase-1 requirements archive.
3. **Non-blocking:** FR85 / LSP close checklist items remain open for Story 35.4; `epic-35` stays in-progress.

## Verification run (targeted; no full `just test`)

- `cargo test -p bitloom --test fr84_softf16_explicit_defer` — 4 OK
- `cargo test -p rhdl-float` — OK
- `cargo test -p bitloom --test nfr14_risk_epic35_residual_partials` — OK

## testarch-automate

ATDD doc/source scans are sufficient for Path B; no additional automate pass required.
