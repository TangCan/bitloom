# Digest D1 — Requirements baseline (r1-1)

**Dimension:** D1 Requirements baseline  
**Import:** [`../imports/requirements-capability-inventory.md`](../imports/requirements-capability-inventory.md)  
**Accessed:** 2026-09-08  
**Firewall:** `docs/requirements/**` only

## Verdict

`docs/requirements` defines **72 Cap-IDs**: 46 pre-closure baseline (A) + **26 NEW/changed by 受控泛型闭包 (B)**. Closure rollout is phased **P2 generator → P5 HLS/IP → P7 bridge**, with synthesizable comb/seq closures (Cap-R-55/56) **unphased** relative to doc 19. Internal contradictions exist (HLS free vs constrained; principle count; multi-view narrative vs P7 delivery).

## Load-bearing claims

```yaml
- claim: "72 Cap-IDs extracted; Cap-R-01..46 bucket A; Cap-R-47..72 bucket B (closure integration)."
  source: "imports/requirements-capability-inventory.md Cap-ID summary"
  publisher: "project-docs"
  pub_date: null
  accessed: "2026-09-08"
  confidence: high
  class: capability

- claim: "Closure yes=26, partial=10, no=36."
  source: "imports/requirements-capability-inventory.md Cap-ID summary"
  publisher: "project-docs"
  accessed: "2026-09-08"
  confidence: high
  class: capability

- claim: "Roadmap places generator closures in P2, HLS/IP closures in P5, bridge templates in P7."
  source: "docs/requirements/19. 实施路线图.md via inventory Roadmap phase map"
  publisher: "project-docs"
  accessed: "2026-09-08"
  confidence: high
  class: roadmap

- claim: "Conflict: outline/§3 routes HLS DF closures as '完全自由'; §15.5/§17.2.5 require synthesizable constraints."
  source: "inventory Contradictions #2"
  publisher: "project-docs"
  accessed: "2026-09-08"
  confidence: high
  class: constraint

- claim: "No explicit roadmap phase for Cap-R-55/56 synthesizable comb/seq closures despite language chapters specifying them."
  source: "inventory Contradictions #3"
  publisher: "project-docs"
  accessed: "2026-09-08"
  confidence: high
  class: roadmap
```

## Top closure caps (decision-facing)

| Cap | Name | Phase |
|-----|------|-------|
| Cap-R-47 | Controlled closure abstraction principle | cross-cutting |
| Cap-R-50 | SynthesizableClosure constraints | with synth closures |
| Cap-R-52 | Generator closures (module factory / LUT) | **P2** |
| Cap-R-55/56 | Comb/seq synthesizable closures | unphased (R7: after generators) |
| Cap-R-62/63 | HLS DF + IP generator closures | **P5** |
| Cap-R-65 | Bridge adapter closure templates | **P7** |
| Cap-R-72 | Phased rollout R7 | P2→P5→P7 |

## Gaps / leads

- Need PRD FR IDs for Cap-R-47..72 (D2).
- Need crates evidence for Cap-R-52+ (D3).
- Resolve HLS constraint conflict before contracting Cap-R-62.
