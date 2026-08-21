---
title: Reconcile — closing-gaps research vs Bitloom stage-2 PRD
status: extract
created: 2026-08-21
updated: 2026-08-21
inputs:
  - _agile-output/planning-artifacts/research/technical-closing-bitloom-overview-requirement-gap-2026-08-21/research.md
  - _agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md
  - _agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md
decision_context: 'User ①C — overview literal hard FRs; PRD-only (no overview.md edit)'
brand: Bitloom / bitloom
---

# Reconcile: closing-gaps research ↔ PRD + addendum

**Purpose:** Map research recommendations to the 2026-08-21 stage-2 PRD update; separate **intentional ①C overrides** from **accidental gaps** (research risks not yet captured in NFR14 / addendum). Naming: public product **Bitloom**, crates.io / CLI **`bitloom`**.

**Verdict:** PRD + addendum correctly record the **strategic override** of research Rec 1 (“redefine done”). Several **operational / sequencing / viz / HLS-CI** risks from research Rec 2–5 remain only partially mirrored in NFR14/addendum and should be closed in a follow-up PRD/addendum patch (not silent non-goal reversion).

---

## 1. Input map

| Artifact | Role |
|----------|------|
| `technical-closing-bitloom-overview-requirement-gap-2026-08-21/research.md` | Industry-aligned “done” + phased order; Rec 1–5 |
| `prd.md` (amendment `overview-literal-C-2026-08-21`) | Contract: FR46–FR52 + strengthened FR28/29/30/35/37/38; NFR14 |
| `addendum.md` §2026-08-21 | ①C decision log; known engineering realities; reject “redefine done” |

---

## 2. Research recommendations → PRD disposition

| # | Research recommendation | PRD / addendum disposition | Class |
|---|-------------------------|----------------------------|--------|
| **1** | Rewrite overview/PRD “done”: Chisel = FrozenHir↔FIRRTL + pinned firtool; **not** maintainable Scala round-trip. Multi-view = handwritten functional/bridge + bounded equiv; **not** HIR→TLM auto-gen / full auto EC. Bind FR18–19 / FR13–14 / FR29–30; soften overview wording. | **Rejected.** User ①C. PRD §0 overturn table; Vision / UJ-4–6; FR28+FR46 (bidirectional maintainable Chisel Scala); FR47+FR30 (generated dual sims + product equiv); SM-6/SM-7; addendum “拒绝调研…重定义 done”. Overview `.md` **unchanged** this round (② PRD-only). | **Intentional override** |
| **2A–F** | Phase order: language/IR → toolchain harden → **selected** stdlib IP → **single** HLS CLI → dump→external viewers → `.fir` matrix **without** Scala generator product. | **Partially overridden.** P0–P2 still roughly A→B; P3 forces literal Chisel Scala (F), full IP line (C expanded), built-in viz (E expanded), generated dual sims. Phasing table allows **P3 ∥ P2b/c** (ASSUMPTION). HLS still “Bambu **or** Vitis”, no in-tree scheduler (partial align with 2D). | **Intentional override** (scope/order); see §4 for accidental sequencing gap |
| **3** | IP governance: Spinal-style — fresh IP **out-of-tree** + doc index; core only “very stable”. | **Overridden for scope.** FR37+FR48 require UART/SPI/I2C/FIFO/AXI as first-class (in-tree or org-published). Addendum notes multi-year maintenance face — risk acknowledged, governance pattern **not** adopted as process. | **Intentional override** (+ residual process gap §4) |
| **4** | Do **not** parallelize five “overview green” epics. | **Not adopted as constraint.** P3 bucket + parallel ASSUMPTION. Anti-parallel / Chipyard-style pain **not** named in NFR14/addendum. | **Accidental gap** (risk not captured) |
| **5** | If insist on literal overview: multi-year budget; no peer success template for Scala↔FIRRTL product + auto TLM EC + deep HLS. | **Accepted as risk posture, not as non-goal.** PRD 反指标 forbids using “peers didn’t finish” as permanent exemption; NFR14 + addendum list CIRCT/Scala, TLM cost, IP years. “无同业成功模板” as explicit planning assumption is only **implicit**. | **Mostly intentional**; template-absence could be sharper in addendum |

---

## 3. Intentional overrides (user ①C) — detail

Documented explicitly so architecture/epics must not silently restore old non-goals.

| Domain | Research “done” | ①C contract (Bitloom PRD) | Evidence in PRD/addendum |
|--------|-----------------|---------------------------|---------------------------|
| **Chisel** | `.fir` + firtool only; Scala Circuit round-trip = non-goal | Bidirectional **maintainable, compilable Chisel Scala** + import path | FR28, FR46, UJ-4; overturn table; addendum #4899 risk |
| **Multi-view / sims** | Handwritten functional/bridge; no HIR→TLM generator; no full auto EC | **Generate** functional + cycle-accurate simulators; FR30 product equiv on generated path | FR29 deletion of “no HIR→TLM”; FR47, FR30, UJ-6, SM-7 |
| **IP** | Core stdlib subset; SPI/I2C/AXI out-of-tree | Full line UART/SPI/I2C/FIFO/AXI first-class | FR37 strengthen + FR48 |
| **Visualization** | Dump quality + Surfer/GTKWave; typed view optional research | **Built-in** hierarchy + timing; “user opens GTKWave alone” ≠ done | FR38, FR49 |
| **HLS** | Optional single CLI after IR stable | Product path + CI fixture; not permanent unsupported | FR35, FR50; addendum HLS |
| **Strategy** | Redefine completion vs overview literal | Overview literal = hard FRs; research redefine = rejected | prd §0; addendum ①C; SM-6 反指标 |

**Still aligned with research (not overridden):**

- No in-tree HLS **scheduler** (ASSUMPTION / FR35).
- FST: do not self-build writer (ASSUMPTION).
- firtool pin + checksum + no default PATH trust (NFR3/NFR12).
- Brand: **Bitloom** / **`bitloom`**; forbid publish `rhdl` / `rhdl-bits`; do not treat `rhdl-rs` as current sole publish name (FR21, identity supersession).
- Design crates → **`bitloom-prelude`** only (FR48 success wording; brand lock).

---

## 4. Accidental gaps — research risk not (fully) in NFR14 / addendum

NFR14 today: *P3 items need risk+schedule records (e.g. CIRCT FIRRTL≠Scala round-trip); no silent downgrade.*

Addendum “已知工程现实” covers: (1) no FIRRTL→Scala Circuit upstream, (2) TLM↔RTL auto-equiv rare / FR47 costly, (3) peer IP = stdlib+out-of-tree → FR48 multi-year face.

| Gap ID | Research signal | Missing / weak in NFR14+addendum | Suggested capture (PRD hygiene, not non-goal revert) |
|--------|-----------------|----------------------------------|------------------------------------------------------|
| **G1** | Rec 4 + [36]: parallel five overview tracks → Chipyard-class maintenance & semantic LCD | PRD allows P3∥P2; **no named anti-parallel risk** | Addendum bullet + NFR14 example: sequencing/parallelism risk & preferred epic order |
| **G2** | Rec 3: out-of-tree until stable as **governance**, not only “years of work” | Maintenance years noted; **intake discipline** (stable-only core) absent | Process note under FR48: org-published OK, but freeze/API/tests before “prelude-grade” |
| **G3** | Rec 2E / [8][35]: industry = dump→viewer; built-in wave IDE is extra product surface | FR49 intentional; **viz maintenance / Surfer-vs-builtin** risk not listed | Addendum risk: internal hierarchy+timing vs dump-quality path; Tywaves-class typed view = optional research only |
| **G4** | Open Q + [20][21][22]: Bambu vs Vitis **license / install / CI** cost; single pinned backend | FR35 “or”; no spike/NFR for license or reproducible CI install | Addendum: require backend selection spike before P2b epic lock; NFR14-style CI reproducibility for HLS asset |
| **G5** | [18] CIRCT bump frequency; FR46 Scala must track Chisel↔firtool pairing | NFR12 pin policy exists; **FR46 coupling to bump cadence** not in addendum realities | Addendum: every Chisel/firtool bump = FR46 regen regression cost |
| **G6** | [30] AXI optional signals / shims / vendor naming | PRD Open Q7 = AXI4-Lite min; **shim/signature friction** not in addendum | Addendum under FR48: AXI “done” = documented shim surface, not naked signature match |
| **G7** | Rec 5: **no peer success template** for literal bundle | Implied by “cost high”; not stated as planning assumption | One-line addendum: ①C accepts no peer template for Scala↔ + auto dual-sim EC |

**Not accidental (already intentional or covered):** Rec 1 override; CIRCT #4899; TLM cost; IP multi-year face; HLS no in-tree scheduler; Bitloom naming.

---

## 5. Naming (Bitloom)

| Concern | Status |
|---------|--------|
| Public product name | **Bitloom** throughout revised PRD Vision / FR21 / SM-4 |
| crates.io / CLI | **`bitloom`** / `cargo bitloom`; `cargo-bitloom` binary narrative in journeys |
| Forbidden publish names | `rhdl`, `rhdl-bits` |
| Superseded working publish name | `rhdl-rs` must not remain “sole publish name” (FR21) |
| Design dependency | `bitloom-prelude` (FR48); float → `bitloom-float` naming revise (FR36) |
| Overview.md still says RHDL | Explicit ASSUMPTION: contract = Bitloom; overview align = **separate task** (② this round) |
| Research title already Bitloom | Aligns; no rename debt on research input |

No accidental **brand** gap between research and PRD; residual gap is **overview.md text** still RHDL (deferred by user ②).

---

## 6. FR / NFR coverage checklist (overview five gaps)

| Overview theme | Research preferred done | ①C FR | Risk in addendum/NFR14? |
|----------------|-------------------------|-------|-------------------------|
| Chisel deep interop | `.fir` only | FR28, FR46 | Yes (CIRCT); G5 bump coupling weak |
| HLS | Single CLI later | FR35, FR50 | Partial; **G4** license/CI |
| Full IP | Selected + out-of-tree | FR37, FR48 | Years yes; **G2/G6** governance/AXI shim weak |
| Visualization | Dump→viewer | FR38, FR49 | **G3** weak |
| Multi-view / dual sim | Handwritten + bounded | FR29, FR30, FR47 | TLM cost yes; **G7** template-absence soft |
| Cross-cutting | Don’t parallelize | P3∥ allowed | **G1** missing |

---

## 7. Recommended follow-ups (hygiene only)

1. Extend **addendum** “已知工程现实” with G1–G7 one-liners (keep ①C; do not restore non-goals).
2. Broaden **NFR14** examples beyond CIRCT/Scala to sequencing, HLS CI license, viz surface, firtool↔FR46 bump cost.
3. Before P3 epics: close PRD Open Q5–Q7; run HLS backend spike (G4).
4. Separate task: rename overview.md RHDL→Bitloom (out of this reconcile).

---

## 8. Traceability

| This extract | Points to |
|--------------|-----------|
| Intentional overrides | prd.md §0 overturn table; Vision; FR28/46/47/48/49/50; addendum ①C |
| Captured risks | addendum 已知工程现实; NFR14 |
| Accidental gaps | §4 G1–G7 |
| Brand | FR21; addendum Phase-3 identity; AGENTS.md brand lock |
