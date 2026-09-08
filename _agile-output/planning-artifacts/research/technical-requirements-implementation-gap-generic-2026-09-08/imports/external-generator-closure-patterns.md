# Import extraction — External generator / closure patterns

**Fetched/used:** 2026-09-08  
**Purpose:** Short extract from long docs consulted for D4 (do not re-ingest full pages).

---

## Clash — Limitations of the compiler

**URL:** https://docs.clash-lang.org/compiler-user-guide/developing-hardware/limitations.html  
**Publisher:** Clash Compiler User Guide  

### Recursion (structural vs behavioral)

- **Dynamic data-dependent recursion** (e.g. `fibR n = fibR (n-1) + fibR (n-2)`): **not synthesizable**. Describes infinitely deep structure; Clash refuses behavioral/stack synthesis. Design paradigm: only `Signal` functions → sequential; other non-HO functions → combinational.
- **Value-recursion** (e.g. `fibS` with `register` feedback): **synthesizable** → feedback loops.
- **Static/structure-dependent recursion** (user `mapV` over `Vec n`): *in principle* unrollable at compile time, but Clash’s compile-time evaluation is **poor**; user-defined recursive HO often **not synthesizable**. Mitigation: built-in HO in `Clash.Sized.Vector`.

### Types / effects

- Recursive datatypes (Haskell lists): unsupported (unbounded bit-size). Exceptions: `Vec`, `RTree` (hard-coded).
- No `IO`/`ST`/FFI side effects on synth path.
- No `Float`/`Double` (would need pipelined arith; Haskell ops are combinational by type).

### Implications for “closures in hardware”

- Higher-order at source is OK if specialized to **monomorphic first-order** before emit.
- Do not promise user-written recursive closures will unroll; prefer **stdlib HO** or **host elaboration** (other HDLs).

---

## Clash — Synthesize / troubleshooting (companion)

**URLs:**  
- https://docs.clash-lang.org/compiler-user-guide/developing-hardware/annotations.html  
- https://docs.clash-lang.org/compiler-user-guide/developing-hardware/troubleshooting.html  

- `Synthesize` binders: monomorphic, first-order, representable I/O; no mutual recursion between Synthesize entities; add `OPAQUE`/`NOINLINE`.
- Failure: “could not transform into expected normal form” → HO/polymorphic `topEntity` or non-representable types; remove them.
- Failure: remains recursive after normalization → rewrite with Vec HO or monomorphize.

---

## SpinalHDL — Interaction (elaboration vs hardware)

**URL:** https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Getting%20Started/Scala%20Guide/interaction.html  

- Execution builds netlist graph → VHDL/Verilog.
- Hardware args to Scala functions are **references**; assignments inside helpers mutate the graph.
- Scala `if`/`for`/`map`/`reduce` = elaboration unroll; `when` = hardware mux/conditional.
- **Wrong:** `Component(activeHigh: Bool)` — hardware as construction param → hierarchy violation.  
- **Right:** `Component(activeHigh: Boolean)` — Scala param.
- Function locals lose RTL names unless returned via `Area`.

---

## SpinalHDL — Function

**URL:** https://spinalhdl.github.io/SpinalDoc-RTD/dev/SpinalHDL/Structuring/function.html  

- Host functions may create regs/logic/Components; pass buses; return Components/buses (e.g. `bus.queue(size)` instantiates FIFO).

---

## Hardcaml — Hierarchy / create_fn

**URL:** https://docs.hardcaml.org/hardcaml-docs/using-interfaces/module_hierarchies/  

- Pattern: `create scope inputs = ...`; instantiate via `Hierarchy.In_scope.hierarchical`.
- `flatten_design true` for sim; `false` for modular RTL + circuit database.

---

## Rust eDSLs (short)

| Project | URL | Generator model |
|---------|-----|-----------------|
| kaze | https://docs.rs/kaze/latest/kaze/ | `Context`/`Module`/`Signal` graph API → Verilog/sim |
| rust-hdl | https://docs.rs/rust-hdl/latest/rust_hdl/ | `LogicBlock` + `#[hdl_gen]` update kernel subset → Verilog |

---

## Chisel (short)

| Source | Extract |
|--------|---------|
| https://github.com/chipsalliance/chisel | Scala eDSL generators → FIRRTL/CIRCT → Verilog; not HLS |
| https://www.chisel-lang.org/api/latest/chisel3/Module.html | `atModuleBodyEnd(gen: => Unit)` — by-name generator thunk after module close |
