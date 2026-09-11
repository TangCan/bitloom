# Code Review — Story 77.2 / FR138

**Verdict: Approve**

**Summary:** Story 77.2 revises ARCHITECTURE-SPINE **AD-27** (2026-09-11) to allow FR138 Parser restore as a product close condition, naming product-equivalent API **`BitloomFirrtlParser.parse`** ≡ historical `firrtl.Parser.parse` / `Parser.parse`, implemented via AD-9 **firtool-1.155.0 `-parse-only`** paired with **Chisel 7.14.0**. Delivered `scripts/parser-restore-check.sh`, `just parser-restore-check`, FIR + Scala façade fixtures, `docs/fr138-parser-restore.md`, fr130 cross-link, and ATDD P1–P4 (7 tests green). Correct Course trail reuses Phase 16 `correctCoursePhase16Approved: 2026-09-11`. FR130 Style Guide close (S3 Parser not restored for that FR) remains valid (NFR56). Epic 77 not closed (→ 77.3).

## Findings

1. AD-27 revise before claim — **pass** (NFR58).
2. Product path is CLI/interop/fixture (not design-crate Scala runtime) — **pass**.
3. P3 FORCE_MISSING / version mismatch fail-closed — **pass**.
4. Alone bans vs FR130/122/111/97 / docs-only — **pass**.
5. Upstream Parser deleted → documented product-equivalent named in docs + AD-27 — **accept**.

## Residual / deferred to 77.3

- Epic 77 NFR14 closeout checkboxes / deferred / Phase 16 story-list pointer.
- Broader Parser ecosystem (NFR59) still out of scope.
