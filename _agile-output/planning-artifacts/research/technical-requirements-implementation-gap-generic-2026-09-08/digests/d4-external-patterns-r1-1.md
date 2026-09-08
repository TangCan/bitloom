# D4 Digest — External generator / closure patterns (r1-1)

**Dimension:** D4 External generator/closure patterns in HDLs and Rust eDSLs  
**Decision lens:** Feasibility of *受控泛型闭包* (controlled synthesizable/generator closures) for a Rust HDL eDSL  
**Accessed:** 2026-09-08  
**Firewall:** External landscape only; no project-internal evidence.  
**Budget:** ~8 sources; claims below are evidence-backed this run.

---

## Verdict (decision-facing)

Cross-HDL practice splits into two workable models: **(A) host-language elaboration generators** (Chisel/Spinal/Amaranth/HardCaml/kaze — closures run at design-construction time and *emit* netlist) vs **(B) synthesizable functional cores** (Clash — higher-order code must normalize to monomorphic first-order hardware; user recursion/closures are heavily constrained). Rust eDSLs today mostly follow (A) with graph builders or restricted kernels (`#[hdl_gen]`), not free Rust closures on the synth path. Controlled generics are feasible if the language **hard-separates elaboration-time Fn from synthesizable purity**, and **forbids capturing hardware signals as construction parameters**.

---

## Claim digest

### Q1 — How tools express compile-time generators / higher-order construction

```yaml
- claim: "Chisel is an eDSL: Scala programs construct/connect hardware objects and emit FIRRTL/Verilog; generators use host for/if/def and FP, not HLS of arbitrary Scala."
  source_url: "https://github.com/chipsalliance/chisel"
  publisher: "CHIPS Alliance / chipsalliance"
  pub_date: null  # README current; page retrieved 2026-09-08
  accessed: "2026-09-08"
  confidence: high
  class: primary_docs

- claim: "Chisel Modules expose late generator hooks (e.g. atModuleBodyEnd(gen: => Unit)) that register by-name generator thunks after the module body closes—explicit host-side generator closures."
  source_url: "https://www.chisel-lang.org/api/latest/chisel3/Module.html"
  publisher: "chisel-lang.org"
  pub_date: null  # API for chisel 7.13.0
  accessed: "2026-09-08"
  confidence: high
  class: primary_docs

- claim: "SpinalHDL elaborates by executing Scala that builds an in-memory netlist graph; Scala if/for/map/reduce are elaboration (unrolled), while when/Reg/etc. are hardware."
  source_url: "https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Getting%20Started/Scala%20Guide/interaction.html"
  publisher: "SpinalHDL docs (RTD)"
  pub_date: null
  accessed: "2026-09-08"
  confidence: high
  class: primary_docs

- claim: "SpinalHDL Scala functions may instantiate regs/logic/components, take buses by reference, and return Components/buses—higher-order hardware construction via ordinary host functions."
  source_url: "https://spinalhdl.github.io/SpinalDoc-RTD/dev/SpinalHDL/Structuring/function.html"
  publisher: "SpinalHDL docs (RTD)"
  pub_date: null
  accessed: "2026-09-08"
  confidence: high
  class: primary_docs

- claim: "Hardcaml hierarchical designs use create functions of type Scope.t -> inputs -> outputs; Hierarchy.In_scope.hierarchical records circuits and optionally flattens vs modular RTL."
  source_url: "https://docs.hardcaml.org/hardcaml-docs/using-interfaces/module_hierarchies/"
  publisher: "Hardcaml docs (Jane Street / hardcaml.org)"
  pub_date: null  # docs site; Hardcaml v0.17.1 API also confirmed
  accessed: "2026-09-08"
  confidence: high
  class: primary_docs

- claim: "Amaranth elaborates Elaboratable.elaborate(platform) into Module/Fragment graphs; host Python (loops, callables, missing_domain lambdas) drives generation—not synthesizable Python lambdas."
  source_url: "https://github.com/RobertBaruch/amaranth-tutorial/blob/main/3_modules.md"
  publisher: "Amaranth community tutorial (RobertBaruch)"
  pub_date: null
  accessed: "2026-09-08"
  confidence: medium
  class: secondary_docs

- claim: "Clash promotes higher-order Vec combinators as the parametric generator style; Synthesize-annotated entities must be monomorphic and first-order (HO args specialized away before emit)."
  source_url: "https://docs.clash-lang.org/compiler-user-guide/developing-hardware/annotations.html"
  publisher: "Clash Compiler User Guide"
  pub_date: null
  accessed: "2026-09-08"
  confidence: high
  class: primary_docs

- claim: "kaze (Rust eDSL) builds Module/Signal graphs via Context API then generates Verilog/sim; generators are host Rust that call the API—not free closures on a synth IR."
  source_url: "https://docs.rs/kaze/latest/kaze/"
  publisher: "docs.rs / yupferris/kaze"
  pub_date: null  # crate docs current
  accessed: "2026-09-08"
  confidence: high
  class: primary_docs

- claim: "rust-hdl models circuits as LogicBlock structs; #[hdl_gen] update kernels are a restricted Rust subset lowered to Verilog—composition is structural wiring, not arbitrary closures."
  source_url: "https://docs.rs/rust-hdl/latest/rust_hdl/"
  publisher: "docs.rs / samitbasu/rust-hdl"
  pub_date: null  # rust-hdl 0.46.x docs
  accessed: "2026-09-08"
  confidence: high
  class: primary_docs
```

### Q2 — Constraints synthesizable paths place on closures/lambdas

```yaml
- claim: "Clash Synthesize tops: no mutual recursion between Synthesize binders; must be monomorphic, first-order, representable args/results; prefer OPAQUE/NOINLINE so GHC does not inline away the boundary."
  source_url: "https://docs.clash-lang.org/compiler-user-guide/developing-hardware/annotations.html"
  publisher: "Clash Compiler User Guide"
  pub_date: null
  accessed: "2026-09-08"
  confidence: high
  class: primary_docs

- claim: "Clash does not synthesize dynamic data-dependent recursion or side-effecting IO/ST; only Signal-domain feedback and built-in Vec HO helpers are reliable; user-defined recursive map-style HO often fails compile-time unroll."
  source_url: "https://docs.clash-lang.org/compiler-user-guide/developing-hardware/limitations.html"
  publisher: "Clash Compiler User Guide"
  pub_date: null
  accessed: "2026-09-08"
  confidence: high
  class: primary_docs

- claim: "SpinalHDL forbids hardware Bool (etc.) as Component construction parameters (hierarchy violation); elaboration params must be Scala values (e.g. Boolean), not netlist signals."
  source_url: "https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Getting%20Started/Scala%20Guide/interaction.html"
  publisher: "SpinalHDL docs (RTD)"
  pub_date: null
  accessed: "2026-09-08"
  confidence: high
  class: primary_docs

- claim: "SpinalHDL passes hardware refs into functions by reference—assignments inside a host function mutate the same netlist object (capture is of graph nodes, not pure values)."
  source_url: "https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Getting%20Started/Scala%20Guide/interaction.html"
  publisher: "SpinalHDL docs (RTD)"
  pub_date: null
  accessed: "2026-09-08"
  confidence: high
  class: primary_docs

- claim: "Elaboration-time closures in Chisel/Spinal/Amaranth/Hardcaml/kaze are unconstrained as host code except that they must terminate with a finite, acyclic (or explicitly feedback) netlist; purity is 'no non-elaboration side effects that escape the graph', not const-fn purity."
  source_url: "https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Getting%20Started/Scala%20Guide/interaction.html"
  publisher: "SpinalHDL docs (RTD)"
  pub_date: null
  accessed: "2026-09-08"
  confidence: medium
  class: synthesis  # inferred across primary sources this run
```

### Q3 — Known failure modes of “closures in hardware”

```yaml
- claim: "Clash: leaving topEntity polymorphic/higher-order causes normalize failures ('could not transform to expected normal form'); fix is monomorphic first-order signature and remove non-representable types."
  source_url: "https://docs.clash-lang.org/compiler-user-guide/developing-hardware/troubleshooting.html"
  publisher: "Clash Compiler User Guide"
  pub_date: null
  accessed: "2026-09-08"
  confidence: high
  class: primary_docs

- claim: "Clash: recursive functions describing infinite structure (e.g. fibR) are rejected; behavioral stack synthesis is explicitly out of scope."
  source_url: "https://docs.clash-lang.org/compiler-user-guide/developing-hardware/limitations.html"
  publisher: "Clash Compiler User Guide"
  pub_date: null
  accessed: "2026-09-08"
  confidence: high
  class: primary_docs

- claim: "SpinalHDL: locals created inside Scala helper functions lose RTL names unless wrapped in Area—generator helpers obscure debug/trace."
  source_url: "https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Getting%20Started/Scala%20Guide/interaction.html"
  publisher: "SpinalHDL docs (RTD)"
  pub_date: null
  accessed: "2026-09-08"
  confidence: high
  class: primary_docs

- claim: "Mixing elaboration control with hardware values as parameters (Spinal: Bool construction arg) causes hierarchy/elaboration errors—classic 'closure captured a wire' anti-pattern."
  source_url: "https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Getting%20Started/Scala%20Guide/interaction.html"
  publisher: "SpinalHDL docs (RTD)"
  pub_date: null
  accessed: "2026-09-08"
  confidence: high
  class: primary_docs

- claim: "Amaranth/nMigen experience reports: passing ClockDomain objects instead of domain names through module APIs yields unexpected RTLIL / domain failures—opaque object capture across elaboratable boundaries is brittle."
  source_url: "https://github.com/nmigen/nmigen/issues/567"
  publisher: "nmigen/amaranth GitHub issues"
  pub_date: null  # issue thread; historical but still cited as failure mode
  accessed: "2026-09-08"
  confidence: medium
  class: issue_tracker
```

---

## Patterns (external)

| # | Pattern | Who | Role of “closure” |
|---|---------|-----|-------------------|
| P1 | Host elaboration loops/`if`/`def` emit unrolled hardware | Chisel, Spinal | Generator thunks, not synth IR |
| P2 | Higher-order host functions take/return buses & Components | Spinal | Ref-capture of graph nodes |
| P3 | `create : Scope -> I -> O` + hierarchical database | Hardcaml | Explicit create_fn, not free lambda |
| P4 | Built-in sized-vector HO combinators + monomorphic top | Clash | HO at source; first-order after normalize |
| P5 | Graph API / `#[hdl_gen]` restricted kernel | kaze, rust-hdl | No arbitrary Rust Fn on synth path |
| P6 | `Elaboratable.elaborate` + Python host generators | Amaranth | Callables at elaborate-time only |

## Anti-patterns (external)

| # | Anti-pattern | Evidence class |
|---|--------------|----------------|
| A1 | Synthesizeable top with HO/polymorphic signature | Clash troubleshooting |
| A2 | Data-dependent recursion / behavioral stack synthesis | Clash limitations |
| A3 | Capturing hardware signals as module *construction* params | Spinal interaction |
| A4 | Anonymous helper locals without naming/Area → opaque RTL | Spinal naming |
| A5 | Passing rich runtime objects (clock domains) across module APIs instead of names/ids | nMigen #567 |
| A6 | Equating “closures in HDL” with HLS of host language | Chisel/Spinal/Clash all reject HLS framing |

---

## Freshness note

Prefer last-24-months primary docs where available. This run relied on **currently published** Clash Book pages, Spinal RTD master/dev, Hardcaml docs, Chisel 7.x API, and live crates.io/docs.rs for kaze/rust-hdl (versions as of access date). Dated blog posts were not required; Clash limitation/troubleshooting and Spinal interaction docs remain the load-bearing constraint sources.

## Source list (8+)

1. https://github.com/chipsalliance/chisel  
2. https://www.chisel-lang.org/api/latest/chisel3/Module.html  
3. https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Getting%20Started/Scala%20Guide/interaction.html  
4. https://spinalhdl.github.io/SpinalDoc-RTD/dev/SpinalHDL/Structuring/function.html  
5. https://docs.clash-lang.org/compiler-user-guide/developing-hardware/limitations.html  
6. https://docs.clash-lang.org/compiler-user-guide/developing-hardware/annotations.html (+ troubleshooting)  
7. https://docs.hardcaml.org/hardcaml-docs/using-interfaces/module_hierarchies/  
8. https://docs.rs/kaze/latest/kaze/  
9. https://docs.rs/rust-hdl/latest/rust_hdl/  
10. https://github.com/nmigen/nmigen/issues/567 (failure-mode corroboration)

## Gaps

- Chisel tutorial wiki “Scripting Hardware Generation” fetch timed out this run; Chisel README + Module API still cover generator model.  
- Public `samitbasu/rhdl` crates were not separately fetched; rust-hdl (same author lineage) covers the Rust “restricted kernel” pattern.  
- No single 2025–2026 academic survey located in budget; constraints drawn from live tool docs.
