# Digest D2 — Contract surface (r1-1)

**Dimension:** D2 PRD / Epic / Spine contract  
**Import:** [`../imports/prd-epic-spine-contract.md`](../imports/prd-epic-spine-contract.md)  
**Accessed:** 2026-09-08  
**Firewall:** PRD dir + `epics.md` + `ARCHITECTURE-SPINE.md` only

## Verdict

Epic phases **1–8 are all `complete`** in `epics.md`. **Positive 泛型闭包 / generator-closure is ABSENT** from PRD, epics FR lists, and spine ADs. Contract **explicitly bans 捕获闭包** on the cycle-accurate path (FR16 / AD-18). English “closure” in Phase 7 means **闭环**, not Rust closures. Implementing requirements Cap-R-47..72 needs **new PRD FRs + AD-18 (and likely AD-1/13) amendment**, not silent extension of FR6/FR51.

## Load-bearing claims

```yaml
- claim: "epics.md phase1Status..phase8Status all complete; Epics 1–25 listed; no Phase 9+."
  source: "imports/prd-epic-spine-contract.md §1"
  publisher: "project-planning"
  accessed: "2026-09-08"
  confidence: high
  class: contract

- claim: "Product feature 泛型闭包 ABSENT from prd.md, addendum.md, epics FR inventories, ARCHITECTURE-SPINE.md."
  source: "imports/prd-epic-spine-contract.md §3.2"
  publisher: "project-planning"
  accessed: "2026-09-08"
  confidence: high
  class: gap

- claim: "FR16 / AD-18 ban capturing closures on cycle-accurate path."
  source: "epics.md FR16; spine AD-18 via import §3.3"
  publisher: "project-planning"
  accessed: "2026-09-08"
  confidence: high
  class: constraint

- claim: "AD-1 forbids rustc-compile-time netlist; closures must elaborate to FrozenHir before freeze (AD-7)."
  source: "imports/prd-epic-spine-contract.md §4"
  publisher: "project-planning"
  accessed: "2026-09-08"
  confidence: high
  class: architecture

- claim: "Phase 7 English 'closure' = overview-literal 闭环 (FR46–52), not Fn closures."
  source: "epics.md phase7Scope; review-rubric via import §3.3"
  publisher: "project-planning"
  accessed: "2026-09-08"
  confidence: high
  class: terminology
```

## Implication for plan

1. Correct Course / PRD amendment to contract Cap-R-47..72 (or a scoped subset).  
2. Architecture AD: distinguish **elaboration-time Fn** (allowed if dissolved before freeze) vs **runtime capturing closures** (remain banned).  
3. Do not treat completed Phase 1–8 as satisfying docs/requirements closure chapters.
