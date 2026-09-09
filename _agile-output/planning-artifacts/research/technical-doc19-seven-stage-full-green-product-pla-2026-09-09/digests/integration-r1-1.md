# Digest: integration — round 1

agent: 6c6782cd-a0d0-4e5e-893c-c338a6883ce0
accessed: 2026-09-09

## Findings (claims with source|publisher|pub_date|accessed|confidence|class)

1. **Official Chisel path is one-way: Scala Chisel → CHIRRTL/FIRRTL → CIRCT/`firtool` → Verilog/HW; FIRRTL text → live Scala `Circuit` / idiomatic Chisel is not a supported product surface post-CIRCT migration.** Maintainers state FIRRTL parsing into the Scala FIRRTL AST was dropped with the move to CIRCT (last workable line ~Chisel 3.6 / FIRRTL 1.6); recommended parse/lower path is CIRCT (`firtool -parse-only`, `circt-opt -import-firrtl`), or serialize Chisel/FIRRTL objects without re-parsing `.fir`, or move transforms into CIRCT plugins. | chipsalliance/chisel#4899 | GitHub | comments ~2024–2025 (issue open era) | accessed=2026-09-09 | confidence=0.92 | class=primary

2. **Public ChiselStage APIs expose elaborate/emit of CHIRRTL, FIRRTL dialect, HW dialect, etc.—not FIRRTL→Scala codegen.** API documents `emitCHIRRTL` / `emitFIRRTLDialect` / `emitHWDialect` from a Chisel generator; deprecated `convert` still returns CHIRRTL `firrtl.ir.Circuit` from elaboration, not from arbitrary `.fir` round-trip. | chisel-lang.org `circt.stage.ChiselStage` (v7.13.0) | Chips Alliance / Chisel | API current as of v7.13 | accessed=2026-09-09 | confidence=0.9 | class=primary

3. **CIRCT FIRRTL dialect is positioned as drop-in replacement for Scala FIRRTL Compiler for the Chisel-produced subset (parse `.fir` / CHIRRTL, annotations, lower toward Verilog)—not as a Chisel/Scala reconstruction backend.** Rationale: implement FIRRTL IR + SFC annotation support; CHIRRTL flavor from Chisel; target is FIRRTL→HW/SV emission, not regenerating maintainable Chisel. | circt.llvm.org FIRRTL Dialect Rationale | LLVM CIRCT | docs live (undated page; content tracks CIRCT/FIRRTL) | accessed=2026-09-09 | confidence=0.88 | class=primary

4. **Industry-realistic Chisel interoperability “done”: consume/produce FIRRTL/Verilog via CIRCT; do not require lossless FIRRTL↔idiomatic Scala.** Mechanical FIRRTL→Scala *emitter* (non-idiomatic) may exist in third-party tools but is outside official Chisel/CIRCT contract; official stance favors FIRRTL/MLIR as interchange, not round-trip source. | synthesis of #4899 + CIRCT rationale + ChiselStage | — | — | accessed=2026-09-09 | confidence=0.78 | class=secondary

5. **Bambu/PandA integrates with host RTL via CLI + files: `bambu` invokes GCC/Clang frontends, emits Verilog/VHDL, shells out to simulators (Icarus/Verilator/commercial) and backend synth tools; hand HDL IPs are bound by XML + `--file-input-data` / `--C-no-parse`, not an in-process host-HDL library API.** Cosim needs a C golden for non-HLS modules. | PandA Bambu tutorial/options; docs.bambuhls.eu PandA-2024.02 | PoliMi PandA | tutorial undated; Doxygen “Mon Feb 12 2024” | accessed=2026-09-09 | confidence=0.9 | class=primary

6. **Vitis HLS host integration is CLI/IDE workflow (`v++ -c --mode hls`, `vitis-run --mode hls --csim|--cosim|--package|--impl`) with config files and packaged IP/XO—not an in-process embed into foreign HDLs.** C/RTL cosim compares C testbench vs generated RTL; export is Vivado IP / Vitis XO for block design / acceleration flows. | AMD UG1399 / UG1702 (2026.1 English) | AMD | labeled 2026.1 | accessed=2026-09-09 | confidence=0.93 | class=primary

7. **Realistic open-HLS↔host-HDL boundary: process wrap + RTL/IP artifacts (Verilog/VHDL/IP-XACT/XO); shared memory / in-process HLS inside Chisel/Rust compilers is not the documented industry pattern.** | Bambu + Vitis docs above | — | — | accessed=2026-09-09 | confidence=0.85 | class=secondary

8. **PyMTL3 multi-abstraction “done” practice: FL→CL→RTL refinement under one Python TB; RTL↔Python co-sim via Verilator placeholders/import; translate-import reuses same TB—not automated formal FL≡RTL.** Docs: placeholders + Verilator import; IEEE Micro paper: multi-level modeling, Verilog/SystemC black-box co-sim, FIRRTL backend mentioned as work. | pymtl3.readthedocs.io External Verilog Import (© 2017–2022); Jiang et al. IEEE Micro 2020 PDF | Cornell BRG | docs footer 2017–2022; paper 2020 | accessed=2026-09-09 | confidence=0.9 | class=primary

9. **SystemC TLM-2.0 practice separates LT (fast SW/VP, `b_transport` + quantum) vs AT (phase-accurate, `nb_transport`) vs cycle-accurate; LT/AT are not equivalent to CA, and CA↔TLM formal equivalence is not the TLM success criterion.** Mapping study restates LT/AT/CT definitions; VP guidance: pick abstraction by goal (boot SW vs performance vs microarch). Mixed TLM+RTL appears via adapters / remote-port / quantum sync (e.g. QEMU↔SystemC), not bit-identical cycle lockstep as default. | ACM TODAES 10.1145/3735641; Learn SystemC VP tutorials; AMD SystemC/TLM–QEMU wiki | ACM / community / AMD | TODAES 2025; wiki undated | accessed=2026-09-09 | confidence=0.86 | class=primary/secondary

10. **Cross-cutting realistic multi-view interoperability standard:** (a) stable IR/file boundaries (FIRRTL/Verilog/IP); (b) same stimulus/scoreboard across abstractions; (c) cycle-accurate path only where clocks/ports exist; (d) TLM/functional correctness ≠ cycle equivalence. Claiming “done” as full FIRRTL↔idiomatic Chisel + in-process HLS + formal TLM≡CA is **not** industry-aligned. | synthesis | — | — | accessed=2026-09-09 | confidence=0.8 | class=secondary

## Leads
- Accellera/IEEE 1666-2011 TLM-2.0 LRM §§ on LT/AT coding styles (authoritative normative text beyond tutorials).
- Historical Scala FIRRTL 1.6 parser (Chisel 3.6 era) for contrast baselines only.
- CIRCT pass-plugin injection points (high/low FIRRTL, HW, pre-Verilog) as the sanctioned extension model vs Scala phases.
- Vitis HLS Tcl legacy vs Unified IDE `vitis-run` migration notes for automation wrappers.
- PyMTL3 SystemC import pass + FIRRTL backend maturity (paper vs current repo docs).

## Looked for but not found
- Official Chisel/CIRCT document promising **idiomatic Chisel/Scala codegen from FIRRTL** or supported FIRRTL↔Scala round-trip after Chisel 5.
- Documented **in-process** Bambu or Vitis HLS library API meant to run inside a host HDL compiler (vs CLI/files/IP).
- Mainstream open tooling claiming **automatic formal equivalence** between SystemC TLM LT models and cycle-accurate RTL as a standard deliverable.

## Sources read (count)
**8** primary/secondary web sources used in digest (Chisel#4899 search+snippets; ChiselStage API; CIRCT FIRRTL Rationale; Bambu tutorial/docs; AMD UG1399/UG1702; PyMTL3 import docs; Jiang IEEE Micro 2020; ACM TODAES TLM survey + SystemC/AMD cosim context).