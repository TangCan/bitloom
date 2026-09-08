# Import extraction — PRD / Epics / Architecture Spine (D2 Contract surface)

**Run:** `technical-requirements-implementation-gap-generic-2026-09-08`  
**Dimension:** D2 — what PRD / Epic / Architecture already commit vs what requirements newly demand  
**Access date:** 2026-09-08  
**Epistemic rule:** Evidence only from named imports below. No claims from `docs/requirements/*`, crates, or training.

## Named imports (this file)

| Path | Role |
|------|------|
| `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md` | Stage-2+ product FR/NFR contract (incl. overview-literal-C amendment + FR71) |
| `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` | CDC/Mem/HLS/FR46 options; FR71/NFR34 note |
| `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/review-rubric-2026-08-21.md` | Review thesis wording (closure = “闭环”) |
| `_agile-output/planning-artifacts/epics.md` | FR inventories Phases 1–8; epic lists; `phaseNStatus` |
| `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md` | AD-1…AD-28 invariants |

**Out of firewall (not used as evidence):** `docs/requirements/` roadmap phases 1–7. Theme mapping in §5 uses **themes named inside these imports only**, labeled as contract themes (not roadmap-file citations).

---

## 1. Epic phases (1–8+) — status and scope

**Source:** `epics.md` YAML frontmatter `phase1Status`…`phase8Status` + Overview § + phase inventories.  
**Document status:** `status: complete` (planning artifact complete; all eight phase status fields = `complete`).

| Epic phase | Status (epics.md) | Scope (frontmatter / Overview) | Epics | Primary FR IDs (phase inventory) |
|------------|-------------------|--------------------------------|-------|----------------------------------|
| **1** | complete | SPEC CAP-1…CAP-9 | 1–4 | FR1–FR21 (stage-1 inventory); NFR1–NFR9 |
| **2** | complete | PRD `prd-rhdl-2026-08-19` baseline | 5–10 | FR21–FR40; NFR3, NFR10–NFR13 |
| **3** | complete | Bitloom rename + maturity + crates.io publish | 11–12 | FR41–FR46(**-tp**); NFR14(**-crates**), NFR15–NFR17 |
| **4** | complete | True standalone after `cargo install` | 13–14 | FR47–FR55 (**phase-4 IDs**); NFR18–NFR22 |
| **5** | complete | Teaching RV32 Episode I + tutorial | 15–16 | FR56–FR62; NFR23–NFR27 |
| **6** | complete | Episode II (immediates → 5-stage + optional CSR) | 17–18 | FR63–FR70; NFR28–NFR33 |
| **7** | complete | Overview-literal **closure** (PRD amendment — FR46–52 / strengthened FR28–40 / NFR14 risk gate) | 19–24 | **PRD** FR28/29/30/35/37/38/40 + FR46–FR52; NFR14 (risk), NFR10, NFR3/12/13 |
| **8** | complete | FR28 JVM true-compile in default CI — FR71/NFR34 | 25 | FR71; strengthens FR28; NFR34; NFR12 |

**No Phase 9+** appears in `epics.md` frontmatter or Overview (access 2026-09-08).

### Phase → Epic title quick index

| Epic | Title (epics.md) | Phase |
|------|------------------|-------|
| 1 | 写出可综合模块并得到 Verilog | 1 |
| 2 | 参数化复用与层次连接 | 1 |
| 3 | 测试里仿真并对照功能视图 | 1 |
| 4 | 与 FIRRTL 6 文本互转 | 1 |
| 5 | 身份可信与可复现 firtool | 2 |
| 6 | 可写的单时钟 RTL 表面 | 2 |
| 7 | 存储器与多时钟时序 | 2 |
| 8 | 更丰富的仿真体验 | 2 |
| 9 | 外挂桥接（HLS / Scala / formal） | 2 |
| 10 | 生态与主机扩展 | 2 |
| 11 | 世界认出 Bitloom | 3 |
| 12 | 干净结项并上架 bitloom | 3 |
| 13 | 装完就能写电路并出 Verilog | 4 |
| 14 | 不 clone 也能 tick（可选第二波） | 4 |
| 15 | 能跑的教学 RV32（Episode I） | 5 |
| 16 | Step-by-step 教程与公开入口 | 5 |
| 17 | Episode II 教学核（ISA 冻结 → 5 级 + hazard） | 6 |
| 18 | Episode II 教程与可选特权 | 6 |
| 19 | 语言表面与合同解锁 | 7 |
| 20 | 与 Chisel 双向互操作 | 7 |
| 21 | 双视图模拟器生成 | 7 |
| 22 | 一级 IP 库 | 7 |
| 23 | 内置层次与时序可视化 | 7 |
| 24 | HLS 产品路径 | 7 |
| 25 | 默认 CI 兑现「Chisel 必须编译」 | 8 |

---

## 2. FR / NFR inventory (all IDs found in named imports)

### 2.1 Critical ID-collision rule

`epics.md` `idCollisionNote` + Phase 7 inventory table:

| ID | Historical (Phase 3–4 epics) | Current PRD / Phase 7+ contract |
|----|------------------------------|--------------------------------|
| **FR46** | Trusted Publishing / release-plz → alias **FR46-tp** | Bitloom ↔ Chisel bidirectional |
| **NFR14** | crates.io FCFS → alias **NFR14-crates** | P3 risk gate before epic `ready` |
| **FR47–FR55** | Phase 4 standalone publish graph | **Do not confuse** with PRD FR47–FR52 (overview-literal). Phase-4 IDs live only in epics Phase 4 inventory. |
| **FR51–FR52** | Phase 4: `--package` metadata / `cargo bitloom new` | PRD/Phase 7: Bundle/Vec; ClockDomain narrative |

PRD (`prd.md` §0): FR numbering **jumps FR40 → FR46** intentionally (FR41 is Stage-3 identity in epics, not re-listed as PRD FR).

### 2.2 Stage 1 (epics Requirements Inventory) — FR1–FR21, NFR1–NFR9

| ID | One-line (epics.md) | Epic coverage |
|----|---------------------|---------------|
| FR1 | Legal Rust RTL types; Input/Output; forced comb/seq | Epic 1 |
| FR2 | Module elaborates to FrozenHir | Epic 1 |
| FR3 | Incomplete comb assignment rejected; comb/seq write rules | Epic 1 |
| FR4 | Exactly one Clock + sync active-high Reset; tick = posedge | Epic 1 |
| FR5 | Same-width surface; explicit pad/trunc | Epic 1 |
| FR6 | Const generics; two widths | Epic 2 |
| FR7 | Hierarchical instantiate + connect | Epic 2 |
| FR8 | Width/dir mismatch fails before emit | Epic 2 |
| FR9 | Undriven inputs fail unless dangling mark | Epic 2 |
| FR10 | Yosys-friendly `<abi>.v` | Epic 1 |
| FR11 | cargo test: elaborate then tick; counter golden | Epic 3 |
| FR12 | tick path dump VCD | Epic 3 |
| FR13 | Handwritten `#[functional_model]`; PortValues; no functional_state in HIR | Epic 3 |
| FR14 | Forbid HIR→TLM / untimed functional sim gen *(later overturned for Rust gen by Phase 7 / AD-5)* | Epic 3 |
| FR15 | freeze rejects multi-drive | Epic 1 |
| FR16 | Cycle-accurate path rejects heap, unbounded recursion, `dyn Trait`, **捕获闭包**, file/net/thread, default f32/f64 | Epic 1 |
| FR17 | Structured diagnostics `rhdl::E0xxx` | Epic 1 |
| FR18 | FrozenHir ↔ FIRRTL 6.0.0 text round-trip | Epic 4 |
| FR19 | Import last-connect normalize then freeze | Epic 4 |
| FR20 | `#[rhdl::top]` sole discovery; `cargo rhdl build` | Epic 1 |
| FR21 | Disclaimer + publish name `rhdl-rs` *(superseded by Bitloom / FR41)* | Epic 1 (also Phase 2 Epic 5) |
| NFR1 | Embedded DSL; legal Rust | — |
| NFR2 | No cloud control plane; local cargo CLI | — |
| NFR3 | Pinned firtool download (also Phase 2) | Epic 5 |
| NFR4 | Host linux-x64; Yosys/Verilator optional | — |
| NFR5 | MSRV 1.97.1; edition 2024 | — |
| NFR6 | Design deps = prelude only | — |
| NFR7 | Apache-2.0 OR MIT toolchain | — |
| NFR8 | Ownership not multi-drive soundness proof | — |
| NFR9 | No maintainable Chisel Scala commitment *(overturned Phase 7 / AD-27)* | — |

### 2.3 Stage 2 / PRD baseline — FR21–FR40, NFR3, NFR10–NFR13

**Authoritative success text:** `prd.md` §5 (2026-08-21 revisions). Epics Phase 2 inventory retains older weaker wording for FR28 (“尽力”) — Phase 7 inventory restates strengthened PRD success.

| ID | Theme (prd.md) | PRD phase bucket | Epic (Phase 2 map) |
|----|----------------|------------------|--------------------|
| FR21 | README disclaimer / publish identity → Bitloom | P0 | 5 |
| FR22 | Single-clock surface thicken; Bundle/Vec **non-goal** here | P0 | 6 |
| FR23 | Multi-clock + language CDC | P1 | 7 |
| FR24 | Async reset | P1 | 7 |
| FR25 | Clock enable / gating | P1 | 7 |
| FR26 | Mem / SyncReadMem | P1 | 7 |
| FR27 | Top Analog/InOut/tristate | P2c | 10 |
| FR28 | FIRRTL→**compilable** Chisel Scala | P2b | 9 → strengthened Epic 20/25 |
| FR29 | bridge / abstraction / both | P2a | 8 → Epic 21 |
| FR30 | Dual-view formal equiv | P2a→P3 | 8 → Epic 21 |
| FR31 | Optional FST | P2a | 8 |
| FR32 | Interp vs compiled tick | P2a | 8 |
| FR33 | C ABI / cdylib | P2a | 8 |
| FR34 | Coverage | P2a | 8 |
| FR35 | HLS attach (product path w/ FR50) | P2b | 9 → Epic 24 |
| FR36 | Synthesizable float crate | P2c | 10 |
| FR37 | IP + blackbox (raised bar) | P2c | 10 → Epic 22 |
| FR38 | Viz / HTML (raised bar) | P2c | 10 → Epic 23 |
| FR39 | Formal / SVA export | P2b | 9 |
| FR40 | Extra CLI verbs (+ import/visualize/wave by P3) | P2c/P3 | 10 → 20/23 |
| NFR3 | linux-x64 firtool pin | P0 | 5 |
| NFR10 | HIR→source regen debug-only | P0 | 5 |
| NFR11 | Multi-arch firtool | P2c | 10 |
| NFR12 | firtool pin upgrade policy | P0 | 5 |
| NFR13 | MSRV 1.97.1 | P2c | 10 |

### 2.4 Overview-literal + CI — FR46–FR52, FR71, NFR14 (PRD), NFR34

| ID | Intent (prd.md) | Epic |
|----|-----------------|------|
| FR46 | Bitloom ↔ Chisel bidirectional maintainable-source interop | 20 |
| FR47 | Generate functional + cycle-accurate simulators (Rust crate) | 21 |
| FR48 | First-class IP: UART/SPI/I2C/FIFO/AXI4-Lite | 22 |
| FR49 | Built-in hierarchy + timing diagrams | 23 |
| FR50 | HLS product path | 24 |
| FR51 | Parameterized composites Bundle / Vec | 19 |
| FR52 | ClockDomain product narrative + cross-domain force | 19 |
| FR71 | Default CI required Chisel JVM true compile | 25 |
| NFR14 | P3 risk record before epic ready | template Epic 19; gates 20–24 |
| NFR34 | CI JVM toolchain contract (Java≥17, sbt cache, no continue-on-error) | 25 |

### 2.5 Phase 3–6 only (epics inventories; not PRD FR21–40 table)

**Phase 3:** FR41–FR46-tp; NFR14-crates, NFR15–NFR17 → Epics 11–12.  
**Phase 4:** FR47–FR55 (standalone); NFR18–NFR22 → Epics 13–14.  
**Phase 5:** FR56–FR62; NFR23–NFR27 → Epics 15–16.  
**Phase 6:** FR63–FR70; NFR28–NFR33 → Epics 17–18.

### 2.6 PRD scope statement (what is “in contract”)

`prd.md` §4: **In scope** = FR21–FR40 (strengthened), NFR3/NFR10–NFR13, **FR46–FR52**; plus §5.8 **FR71**/NFR34.  
Still forbidden: publish `rhdl`/`rhdl-bits`; cloud control; in-tree HLS **scheduler** (attach OK).

AD-26 (spine): Stage 2+ authority = this PRD (incl. FR21–40, FR46–52, NFR3, NFR10–14). Historical FR46-tp / NFR14-crates ≠ PRD IDs.

---

## 3. Closure / 闭包 / Fn / generator-closure — search finding

### 3.1 Method

Case-insensitive / Chinese search across: `prd.md`, `addendum.md`, `review-rubric*.md` in PRD dir; full `epics.md`; full `ARCHITECTURE-SPINE.md`. Patterns: `闭包`, `closure`, `FnMut`, `FnOnce`, `\bFn\b`, `generator-closure`, `泛型闭包`, `GeneratorClosure`, `捕获`.

### 3.2 Positive product feature “泛型闭包 / generator-closure”

| Artifact | Finding |
|----------|---------|
| **prd.md** | **ABSENT** — no 闭包/closure/Fn/泛型闭包 as a deliverable FR. |
| **addendum.md** | **ABSENT** |
| **epics.md** inventories / Epic titles FR lists | **ABSENT** as a feature FR. No FR for generic/generator closures. |
| **ARCHITECTURE-SPINE.md** | **ABSENT** as an allowed surface feature or AD delivering closures. |

**Verdict:** Controlled **泛型闭包** as a new requirements axis is **not contracted** in PRD, epics, or spine. Class: **gap / uncontracted demand** (relative to this D2 surface).

### 3.3 Mentions that *do* exist (negative or homonym)

| Location | Text sense | Class |
|----------|------------|-------|
| `epics.md` FR16 | 周期精确路径**拒绝**…**捕获闭包**… | **Ban** of capturing closures on cycle-accurate path |
| `epics.md` Story 1.x AC (~L225) | same ban list including 捕获闭包 | Ban |
| Spine **AD-18** | same ban: 捕获闭包 | Ban |
| `epics.md` `phase7Scope` | “Overview-literal **closure**” | English = **闭环 / closing**, not Rust closure |
| `review-rubric-2026-08-21.md` | “P3 overview-literal **closure**” | Same English sense |
| Spine AD-7 / AD-17 / AD-19 | `fn(&FrozenHir)`, `fn cycle`, `fn elaborate` | Ordinary Rust **function type syntax** in rules — **not** Fn-trait / closure language feature |

### 3.4 Related contracted language features (not closures)

- **FR6 / Epic 2:** const **generics** for bit-widths.  
- **FR51 / Epic 19 / AD-20 (revised):** parameterized **Bundle / Vec** on synthesizable path.  
- **AD-1 paradigm:** design is executable Rust **generator**; elaboration is runtime — does **not** specify passing user `Fn` closures as HDL constructs.

---

## 4. AD decisions that constrain how 泛型闭包 *could* be implemented

If requirements later demand 泛型闭包, these **already adopted** spine rules bound the design space (source: ARCHITECTURE-SPINE.md):

| AD | Constraint relevant to closures |
|----|----------------------------------|
| **AD-1** | Only two HIR birth paths; both end in private `freeze` → `FrozenHir`. **No rustc-compile-time netlist** extraction. Closure expansion would have to run as **generator / elaborate-time** builder work (or import), not mid-typecheck MIR lowering. |
| **AD-7** | Unfrozen HIR invisible outside hir crate; backends pure `fn(&FrozenHir) -> Artifact`; no backend rewrite. Closure must lower to HIR **before** freeze. |
| **AD-12 / AD-13** | Hierarchical AST; builder holds sole mutable session; macros expand only to builder API — closures cannot invent a parallel circuit graph. |
| **AD-4 / AD-16** | Unique drive + abi_name at freeze; any closure-generated wires still subject to multi-drive / undriven checks. |
| **AD-18** | **Explicit ban: 捕获闭包** on cycle-accurate path (alongside heap/`dyn Trait`/…). A “generic closure” feature must either (a) be **non-capturing / elaborator-only** and never reach tick as a Rust closure, or (b) require **AD-18 revision**. |
| **AD-5 / AD-17** | Cycle-accurate sim only from FrozenHir/`tick`; functional_state never enters HIR. Closure body used only in functional_model would still be outside synthesizable freeze path. |
| **AD-6** | Design crates depend only on **bitloom-prelude**; macros must not depend on hir/backends. Closure sugar must live in prelude/macro → builder. |
| **AD-19** | Sole discovery `#[rhdl::top]` / `Elaboratable::elaborate()` — no alternate “closure entry” discovery. |
| **AD-20** | Surface thicken + **FR51 Bundle/Vec**; width/dir fail before emit. Closures are **not** listed; composites are the contracted parameterization axis. |
| **AD-3 / AD-27** | Emit / Chisel path consumes FrozenHir / `.fir` — anything closures produce must be representable in that IR subset. |
| **AD-26 / AD-28** | New product FR needs PRD altitude + NFR14-style risk gate for overview-scale work; spine AD numbers not silently forked. |

**Implication (evidence-bound):** Contract surface today **rejects capturing closures** and **never commits** a positive generic-closure FR; implementation would need **new PRD FR + AD-18 (and likely AD-1/13) amendment**, not a silent extension of FR51/FR6.

---

## 5. Capability themes — contracted by completed epics vs not contracted

**Note:** `docs/requirements` roadmap phase 1–7 files were **not** named imports. Themes below are **capability themes appearing in PRD/epics/spine**. Mapping “roadmap phases 1–7” to these themes is a **hypothesis for D1**, not evidenced from roadmap files here.

| Theme (from PRD F-* / spine Capability map / epic scopes) | Contracted? | Covered by completed epic phases (status=complete) | Notes |
|-----------------------------------------------------------|-------------|-----------------------------------------------------|-------|
| Core eDSL / elaborate / Verilog emit | Yes FR1–10,15–17,20 | Phase 1 Epics 1–2 | |
| Const-generic widths / hierarchy checks | Yes FR6–9 | Phase 1 Epic 2 | |
| Native tick / VCD / handwritten functional | Yes FR11–13 | Phase 1 Epic 3 | FR14 ban later loosened for **generated** Rust functional (FR47) |
| FIRRTL 6 text round-trip | Yes FR18–19 | Phase 1 Epic 4 | |
| Identity / firtool pin / MSRV | Yes FR21/41+, NFR3/12/13 | Phases 2–4 | Bitloom supersession Phases 3–4 |
| Single-clock surface thicken | Yes FR22 | Phase 2 Epic 6 | |
| Mem / multi-clock CDC / async reset / enable | Yes FR23–26 | Phase 2 Epic 7 | FR52 narrative Phase 7 Epic 19 |
| Sim thicken (bridge, equiv, FST, engines, C ABI, coverage) | Yes FR29–34 | Phase 2 Epic 8; raised FR29/30 in Phase 7 Epic 21 | |
| HLS attach / formal / Chisel gen | Yes FR28/35/39 + FR46/50 | Phase 2 Epic 9; Phase 7 Epics 20/24; Phase 8 Epic 25 | |
| IO / float / IP start / viz / CLI | Yes FR27/36–38/40 + FR48/49 | Phase 2 Epic 10; Phase 7 Epics 22–23 | |
| Bundle / Vec composites | Yes FR51 | Phase 7 Epic 19 | Explicitly **not** FR22 |
| Dual-sim **generation** | Yes FR47 | Phase 7 Epic 21 | Overturns old FR14/AD-5 ban |
| Teaching RV32 Episode I/II | Yes FR56–70 | Phases 5–6 | Teaching, not language-closure |
| **泛型闭包 / generator-closure as HDL surface** | **No** | **No epic** | Only **ban** of 捕获闭包 (FR16/AD-18) |
| SystemC TLM-2.0 | Explicitly **not** contracted | — | AD-5 / Deferred |
| In-tree HLS scheduler | Explicitly **not** contracted | — | AD-25 / PRD |

**Completed vs planned:** Per `epics.md`, phases 1–8 are all **`complete`** — there is **no** remaining planned epic phase in this document for 泛型闭包. Any such work would be a **new** planning increment beyond Phase 8.

---

## 6. Claim ledger (D2)

| # | Claim | Source | Confidence | Class |
|---|-------|--------|------------|-------|
| C1 | Epic phases 1–8 all marked `complete` in epics.md frontmatter | epics.md L35–43 | high | fact |
| C2 | Product FR contract for overview-literal is FR46–52 (+ strengthened FR28–40); CI evidence FR71/NFR34 | prd.md §4–5.8; epics Phase 7–8 | high | fact |
| C3 | FR46/NFR14 and Phase-3/4 FR47–55 IDs collide; disambiguation required | epics.md idCollisionNote; Phase 7 table | high | fact |
| C4 | **ABSENCE** of 泛型闭包 / generator-closure as positive FR in PRD/epics/spine | search §3 | high | absence finding |
| C5 | **PRESENCE** of ban on 捕获闭包 on cycle-accurate path (FR16, AD-18) | epics FR16; spine AD-18 | high | fact |
| C6 | “Overview-literal closure” = English 闭环, not Rust closure | epics phase7Scope; review-rubric | high | fact |
| C7 | AD-1 forbids rustc-compile-time netlist; elaboration is staged runtime | spine AD-1 | high | fact |
| C8 | Parameterized composites contracted via FR51/AD-20, not via closures | prd FR51; spine AD-20 | high | fact |
| C9 | No Phase 9+ in named imports | epics.md | high | absence finding |
| C10 | Mapping to docs/requirements roadmap phases 1–7 not evidenced in this import | firewall | high | scope limit |

---

## 7. Files touched by this extraction

- This import: `.../imports/prd-epic-spine-contract.md`
- Companion digest: `.../digests/d2-contract-surface-r1-1.md`
