# Digest: landscape-compiler-ir r1-1

## Findings

**CIRCT’s latest tagged compiler is `firtool-1.155.0` (2026-08-11); Chisel 7.14.0 (2026-08-13) is released against that same firtool.** That pairing is the current production Chisel→Verilog stack, not an experimental sidecar.
**source** https://github.com/llvm/circt/releases/tag/firtool-1.155.0 ; https://github.com/chipsalliance/chisel/releases/tag/v7.14.0
**publisher** LLVM/CIRCT; CHIPS Alliance / chipsalliance/chisel
**pub_date** 2026-08-11; 2026-08-13
**accessed** 2026-08-18
**confidence** high
**class** version/compat

**The FIRRTL dialect in CIRCT is documented as a generally complete, actively maintained implementation of the FIRRTL spec, including the CHIRRTL flavor Chisel emits, plus SFC annotation ingest.** Goal is drop-in replacement of the Scala FIRRTL Compiler (SFC) for the Chisel-produced subset, matching SFC on spec-undefined behavior.
**source** https://circt.llvm.org/docs/Dialects/FIRRTL/RationaleFIRRTL/
**publisher** CIRCT / LLVM
**pub_date** living doc (no dated release on page)
**accessed** 2026-08-18
**confidence** high
**class** landscape

**Chisel has used firtool (CIRCT) for Verilog since v3.6; each Chisel release is tested against one firtool and auto-resolves it (Chisel ≥6.0 manages the binary).** Users may override with `CHISEL_FIRTOOL_PATH` or `org.chipsalliance::llvm-firtool`; mismatch is unsupported. Docs still encourage emitting `.fir` and invoking firtool from the build as an alternative to in-process `ChiselStage`.
**source** https://www.chisel-lang.org/docs/appendix/versioning ; https://www.chisel-lang.org/docs/installation
**publisher** chisel-lang.org (CHIPS Alliance)
**pub_date** living docs; installation example cites Chisel 7.2.0
**accessed** 2026-08-18
**confidence** high
**class** version/compat

**The Scala FIRRTL compiler repo `chipsalliance/firrtl` is archived (read-only since 2024-08-20) and points users to CIRCT/`firtool`, `firrtl-spec`, and Chisel.** That is the replacement, not a dual-maintained backend.
**source** https://github.com/chipsalliance/firrtl
**publisher** CHIPS Alliance
**pub_date** archived 2024-08-20 (repo banner)
**accessed** 2026-08-18
**confidence** high
**class** ecosystem

**Chisel’s CIRCT phase invokes `firtool` with IR/language targets including `-ir-fir` (FIRRTL), `-ir-hw` (HW dialect), Verilog/SystemVerilog (`--split-verilog`), and `--btor2`.** Default path is FIRRTL file/stdin → firtool → SystemVerilog, not Scala lowering.
**source** https://github.com/chipsalliance/chisel/blob/main/src/main/scala/circt/stage/phases/CIRCT.scala
**publisher** chipsalliance/chisel
**pub_date** living `main` (also present at v7.1.1 / v7.11.0)
**accessed** 2026-08-18
**confidence** high
**class** pattern

**CHIRRTL is a Chisel-facing FIRRTL flavor (memories/undocumented constructs) that CIRCT parses; it is not a separate interchange standard.** FIRRTL dialect + CHIRRTL dialect live in the same library because flow-checking created a circular dependency when split. Lowering to HW still uses FIRRTL-origin ops such as `seq.firreg` / `seq.firmem` as intermediaries before SystemVerilog.
**source** https://circt.llvm.org/docs/Dialects/FIRRTL/RationaleFIRRTL/ ; https://circt.llvm.org/docs/Dialects/Seq/
**publisher** CIRCT / LLVM
**pub_date** living docs
**accessed** 2026-08-18
**confidence** medium-high
**class** pattern

**HW + Comb + Seq + SV is CIRCT’s core lowering stack for emission, not a published multi-language interchange.** `sv` is an AST-like SystemVerilog dialect mixed into `hw`/`comb`, designed for predictable emission (including `sv.verbatim` escape hatches), not for analysis. `seq` is the hardware-independent sequential layer (`seq.compreg`, `seq.firreg`, memories) intended to lower to `sv.always_ff` / primitives.
**source** https://circt.llvm.org/docs/Dialects/SV/RationaleSV/ ; https://circt.llvm.org/docs/Dialects/Seq/RationaleSeq/ ; https://circt.llvm.org/docs/Dialects/SV/ ; https://circt.llvm.org/docs/Dialects/Seq/
**publisher** CIRCT / LLVM
**pub_date** living docs (Seq rationale still says “(future) seq dialect” in the intro while the dialect pages list a full opset — doc lag)
**accessed** 2026-08-18
**confidence** high on roles; medium on “production vs experimental” labels (pages do not stamp that matrix)
**class** landscape

**What looks production vs still-churning inside CIRCT (inferred from use + 1.155.0 notes, not an official maturity table):** production for Chisel is FIRRTL dialect + firtool Verilog/SV emission through HW/SV/Seq. 1.155.0 still lands substantial work on Moore/ImportVerilog, ESI runtime, LLHD, Arc, OM evaluator removal, Synth, LTL, PyRTG, and a newly added AXI4 dialect — i.e. import, verification, and adjacent dialects remain active construction, not a frozen core.
**source** https://github.com/llvm/circt/releases/tag/firtool-1.155.0 ; https://circt.llvm.org/docs/Dialects/FIRRTL/RationaleFIRRTL/
**publisher** LLVM/CIRCT
**pub_date** 2026-08-11; living rationale
**accessed** 2026-08-18
**confidence** medium (maturity inferred; CIRCT does not publish a production/experimental dialect matrix in the pages retrieved)
**class** landscape

**FIRRTL language spec latest release is v6.0.0 (2026-05-12).** Release notes: fills in property operations long present in Chisel/CIRCT; **property ABI is still unmentioned and implementation-defined**, deferred to later versions. Versioning is SemVer: MAJOR = breaking syntax/semantics.
**source** https://github.com/chipsalliance/firrtl-spec/releases ; https://github.com/chipsalliance/firrtl-spec/releases/tag/v6.0.0
**publisher** CHIPS Alliance / chipsalliance/firrtl-spec
**pub_date** 2026-05-12
**accessed** 2026-08-18
**confidence** high
**class** version/compat

**FIRRTL version churn is real and recent:** spec history includes breaking 3.0.0 syntax (`connect` vs `<=`, radix literals), 4.x layers/substitutions, **5.0.0 backwards-incompatible `{{}}` substitutions**, 6.0.0 properties. CIRCT gates parser features on the file’s `FIRRTL version` line (`nextFIRVersion` / `missingSpecFIRVersion`) and added exporter `--firrtl-version` (commit 2026-04-08) with a minimum supported version.
**source** https://github.com/chipsalliance/firrtl-spec/releases ; https://circt.llvm.org/docs/Dialects/FIRRTL/RationaleFIRRTL/ ; https://github.com/llvm/circt/commit/574ccccadc9c9f83e35cae971d482a861cf70299
**publisher** CHIPS Alliance; CIRCT
**pub_date** spec releases through 2026-05-12; exporter commit 2026-04-08
**accessed** 2026-08-18
**confidence** high
**class** version/compat

**Old CIRCT↔SFC pain that the rationale says was fixed:** early FIRRTL-dialect type canonicalization diverged from SFC and broke verification; current policy is follow spec + SFC almost exactly. Remaining documented hazard: **invalid-value UB is context-sensitive**; “mundane” opts can still miscompile vs SFC-as-ground-truth (register-reset stripping, `when` vs `mux`+invalid). Debug vs release name-preservation (`-O=debug|release`) was a compromise after dead-wire-tap and bind-debug-module approaches failed (ports, coverage holes, lost dataflow in waveforms).
**source** https://circt.llvm.org/docs/Dialects/FIRRTL/RationaleFIRRTL/
**publisher** CIRCT / LLVM
**pub_date** living doc
**accessed** 2026-08-18
**confidence** high as CIRCT’s own retrospective; single-source for the historical failures
**class** failure

**Chisel 7.14.0 deprecates implicit truncation; paired firtool 1.155.0 warns and those warnings cannot be suppressed in firtool.** Width-ABI is still moving at the Chisel/CIRCT boundary.
**source** https://github.com/chipsalliance/chisel/releases/tag/v7.14.0
**publisher** chipsalliance/chisel
**pub_date** 2026-08-13
**accessed** 2026-08-18
**confidence** high
**class** failure

**SpinalHDL does not use FIRRTL/CIRCT.** It elaborates a Scala component graph and emits VHDL or Verilog (`SpinalVhdl` / `SpinalVerilog` / `SpinalConfig(mode=Verilog)`), optionally one-file-per-component; Verilog backend dated 2016-06-05 and claimed to pass the same regressions as VHDL (RISC-V, FIFOs, etc.). Interop story is generated VHDL/Verilog + `BlackBox`, plus Verilator-class sim — not a shared compiler IR.
**source** https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Other%20language%20features/vhdl_generation.html ; https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Introduction/SpinalHDL.html
**publisher** SpinalHDL project
**pub_date** living “master” docs
**accessed** 2026-08-18
**confidence** high on IR strategy; **low on 2026 version/maturity** (no current release tag retrieved)
**class** pattern

**Clash v1.10.0 (2026-04-23) compiles Haskell via CoreHW (System F + letrec + case) → normalisation → netlist → VHDL-1993 / Verilog-2001 / SystemVerilog-2012.** Own backends (`--verilog` / `--vhdl` / `--systemverilog`); not CIRCT. 1.10.0 notes Verilog vector-index consistency fix and dropping Clash-specific Verilator delay shims for Verilator >5.
**source** https://github.com/clash-lang/clash-compiler/releases/tag/v1.10.0 ; https://hackage.haskell.org/package/clash-lib ; https://docs.clash-lang.org/compiler-user-guide/developing-hardware/flags.html
**publisher** clash-lang
**pub_date** 2026-04-23 (release); living flags/docs; clash-lib package page
**accessed** 2026-08-18
**confidence** high
**class** ecosystem

**Amaranth’s compiler IR is a global netlist (NIR), not Yosys RTLIL as the source of truth.** Frontend elaborates to NIR; `amaranth.back.rtlil` emits Yosys RTLIL; playground states RTLIL is used internally to produce Verilog. PR #1102: NIR enables CDC checks and driver-conflict detection the old per-module AST IR could not. No CIRCT/FIRRTL backend in retrieved pages.
**source** https://amaranth-lang.org/play/ ; https://github.com/amaranth-lang/amaranth/blob/main/amaranth/back/rtlil.py ; https://github.com/amaranth-lang/amaranth/pull/1102
**publisher** amaranth-lang
**pub_date** living playground/code; PR #1102 undated in retrieval
**accessed** 2026-08-18
**confidence** high on RTLIL/Verilog path; medium on whether NIR is fully landed on `main` (PR text retrieved, merge date not confirmed)
**class** pattern

**Hardcaml (Jane Street, OCaml) is a private `Circuit` eDSL; Verilog import is Yosys JSON netlist via `hardcaml_of_verilog` (latest retrieved package v0.17.0, published 2024-05-26), not FIRRTL/CIRCT.** Native Verilog *export* documentation was not in the retrieved README.
**source** https://github.com/janestreet/hardcaml_of_verilog ; https://opam.ocaml.org/packages/hardcaml_of_verilog/hardcaml_of_verilog.v0.17.0/ ; https://github.com/janestreet/hardcaml
**publisher** Jane Street
**pub_date** opam v0.17.0 2024-05-26; hardcaml repo retrieved 2026-08-18 (1112 stars; README body empty in fetch)
**accessed** 2026-08-18
**confidence** medium (import path solid; export/IR internals under-retrieved; v0.17 is older than the 12-month landscape window)
**class** ecosystem

**Consolidation vs churn:** **consolidating** = Chisel-class FIRRTL compilation onto CIRCT/`firtool` (SFC archived; Chisel 7 tracks firtool ~weekly). **not consolidating** = SpinalHDL, Clash, Amaranth, Hardcaml still use host-language IRs and emit Verilog/VHDL/RTLIL. **churning** = FIRRTL *language* versions (3→6 in a few years, properties ABI still TBD) and CIRCT non-core dialects (Moore, ESI, AXI4, LLHD). Practical 2026 interchange for a Rust eDSL is therefore **versioned `.fir` (for Chisel) plus Verilog (for everyone else)**; CIRCT HW dump (`-ir-hw`) is a CIRCT-internal waypoint, not an ecosystem ABI.
**source** synthesis of the version/compat and ecosystem sources above
**publisher** n/a (cross-source)
**pub_date** n/a
**accessed** 2026-08-18
**confidence** high on Chisel/CIRCT consolidation; high on non-adoption by named adjacent HDLs given retrieved backends
**class** landscape

## Leads worth chasing

- Pin RHDL Phase-1 FIRRTL emission to a **Chisel-tested firtool** (today 1.155.0 with Chisel 7.14.0), not “latest CIRCT main”; Chisel docs warn cross-version is untested.
- Emit **spec-versioned `.fir` (`FIRRTL version 6.0.0` or the minimum firtool accepts)** and avoid CHIRRTL-only memory forms unless Chisel interop requires them.
- Treat **Verilog as the Phase-1 multi-HDL interchange**; FIRRTL only for Chisel/`firtool`. Do not assume Spinal/Clash/Amaranth/Hardcaml will grow CIRCT frontends.
- Property/OM/`string.concat` path is where Chisel+CIRCT are extending the language; **do not depend on a stable property ABI** until `firrtl-spec` defines it.
- `firtool -ir-hw` / Seq/`sv.verbatim` are useful if RHDL later in-tree-lowers through CIRCT; they are not a 2026 interchange contract.
- Revisit CIRCT name-preservation (`-O=debug|release`) if multi-view sim must correlate HIR names with Verilog.

## Looked for and did not find

- An official CIRCT **production vs experimental dialect matrix** (HW/SV/Seq/FIRRTL vs Moore/ESI/AXI4/LLHD).
- 2025–2026 **quantitative firtool scale/perf** (compile time, max design size) from primary CIRCT/Chisel sources.
- **SpinalHDL current release version** or any Spinal→CIRCT/FIRRTL path.
- **Amaranth current release version** and confirmation NIR PR #1102 merge date.
- **Hardcaml native Circuit→Verilog emitter** docs (repo README body empty this run); no v0.18 package found.
- Adjacent-HDL **CIRCT backends** (Clash/Amaranth/Spinal/Hardcaml).
- A completed **FIRRTL property ABI** in spec 6.0.0 (explicitly still implementation-defined).
