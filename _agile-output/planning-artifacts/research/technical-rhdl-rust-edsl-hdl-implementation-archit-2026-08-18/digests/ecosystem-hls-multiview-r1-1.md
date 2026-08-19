# Digest: ecosystem-hls-multiview r1-1

## Findings

**XLS is Apache-2 open HLS with a Rust-like DSL (DSLX), a dataflow SSA IR, LLVM JIT for native host execution, and Verilog/SystemVerilog codegen; the project itself claims the host run and the generated hardware are functionally identical and checkable with Z3 LEC.** Source: [XLS: Accelerated HW Synthesis](https://google.github.io/xls/). Publisher: Google XLS project. pub_date: living docs (releases through 2026-08-18). accessed: 2026-08-18. confidence: high. class: landscape | implementation

**XLS is still labeled experimental, not an officially supported Google product, with regular DSLX breakage and no backward-compatibility commitment — a five-year regret risk if a corpus is built on the current language.** Source: same [XLS project page](https://google.github.io/xls/) (“State of the Project”). Publisher: Google XLS project. pub_date: living docs. accessed: 2026-08-18. confidence: high. class: failure | ecosystem

**XLS IR is a pure dataflow SSA specialized for circuitry: bits/array/tuple/token types; function / proc / block abstractions; tokens exist only to order channel ops and have no hardware correlate (untimed channel semantics vs scheduled hardware).** Source: [XLS IR semantics](https://google.github.io/xls/ir_semantics/). Publisher: Google XLS project. pub_date: living docs. accessed: 2026-08-18. confidence: high. class: implementation

**An experimental C++-and-pragmas frontend (`xlscc`) exists beside DSLX; the project prefers DSLX over C++-pragma HLS.** Source: [XLS project page](https://google.github.io/xls/) (`contrib/xlscc`) and [DSLX reference](https://google.github.io/xls/dslx_reference/). Publisher: Google XLS project. pub_date: living docs. accessed: 2026-08-18. confidence: high. class: landscape

**XLS release vitality is high in Aug 2026: daily-ish GitHub Actions tags (e.g. v0.0.0-10507-g32206619c on 2026-08-18) with PiperOrigin-RevId, LLVM bump 2026-08-10, and ongoing codegen/IR fixes — internal Google flow is still pumping the public repo.** Source: [google/xls releases](https://github.com/google/xls/releases). Publisher: GitHub / Google. pub_date: 2026-08-07 … 2026-08-18. accessed: 2026-08-18. confidence: high. class: ecosystem | version/compat

**Vitis HLS 2025.1 is closed, AMD-device HLS: C/C++ → RTL for Versal/Zynq/FPGA, C-sim then C/RTL co-sim, pragma/PERFORMANCE-driven; IP as Vivado `.zip` or Vitis `.xo`.** Source: [UG1399 2025.1](https://docs.amd.com/r/2025.1-English/ug1399-vitis-hls/Introduction-to-Vitis-HLS-Components). Publisher: AMD. pub_date: 2025.1 docs. accessed: 2026-08-18. confidence: high. class: landscape | version/compat

**Vitis HLS Classic IDE is discontinued in 2025.1+ (`vitis_hls` gone; Unified IDE / `v++` / `vitis-run` only); 2025.2 still ships known-issue ARs (e.g. Code Analyzer documented after discontinuation).** Source: [AR 75342](https://adaptivesupport.amd.com/s/article/75342). Publisher: AMD Adaptive Support. pub_date: 2025.1 / 2025.2 release notes on that AR. accessed: 2026-08-18. confidence: high. class: version/compat | failure

**Vitis 2026.1 still iterates HLS (array-partition library, `m_axi` unaligned/non-power-of-2, non-AXI XO packaging) — commercial HLS is alive but locked to AMD silicon and a moving IDE.** Source: [AMD Vitis HLS product page](https://www.amd.com/en/products/software/adaptive-socs-and-fpgas/vitis/vitis-hls.html) (2026.1 highlights listed). Publisher: AMD. pub_date: 2026.1 listing on that page. accessed: 2026-08-18. confidence: medium (product-page fetch timed out; snippets from this run). class: landscape | version/compat

**Part of Vitis HLS is LLVM-front-end open (`hls-llvm-project` branch 2025.1: clang/llvm only); the synthesizer/scheduler remains closed — open-core, closed product.** Source: [Xilinx/HLS](https://github.com/Xilinx/HLS) index (“Vitis HLS 2025.1 Open Source Resources”). Publisher: AMD/Xilinx. pub_date: 2025.1 branch listing. accessed: 2026-08-18. confidence: high. class: landscape

**Bambu (PandA) is open academic HLS: C/C++ via GCC or Clang/LLVM IR → Verilog/VHDL; ANSI-C except recursion; cosim with Verilator / Xsim / Modelsim; FPGA back-ends include Vivado/Quartus/Lattice plus OpenROAD ASIC.** Source: Fast ML for Science 2025 tutorial slides ([Tutorial_Intro_Bambu.pdf](https://indico.cern.ch/event/1496673/contributions/6637958/attachments/3126150/5544795/Tutorial_Intro_Bambu.pdf)). Publisher: Politecnico di Milano / CERN FastML. pub_date: 2025-09-01. accessed: 2026-08-18. confidence: high. class: landscape

**Bambu GitHub (`ferrandi/PandA-bambu`) is public (≈350 stars this run) and still positioned as a research framework, not a product toolchain.** Source: [ferrandi/PandA-bambu](https://github.com/ferrandi/PandA-bambu). Publisher: Politecnico di Milano. pub_date: living repo. accessed: 2026-08-18. confidence: medium (README body was thin in fetch). class: ecosystem

**Dynamatic is open MLIR HLS: C/C++ → handshake dialect → synchronous dynamically scheduled RTL, Xilinx FPGA target; authors report ~30 PRs/month and CI; they claim better/similar QoR vs commercial HLS on irregular memory / control-dominated code.** Source: [EPFL-LAP/dynamatic README](https://github.com/EPFL-LAP/dynamatic) and [arXiv 2603.19856](https://arxiv.org/html/2603.19856v1). Publisher: EPFL-LAP / ETH Zurich. pub_date: paper HTML 2026 (arXiv 2603.*); repo living. accessed: 2026-08-18. confidence: high. class: landscape | ecosystem

**MLIR-on-HLS regret is documented by Dynamatic’s own authors: no attributes on values/edges, awkward handshake.deps, C frontend (Polygeist/CIR) not competitive with LLVM HLS frontends, and a handshake→XLS MLIR dialect translation that “has a risk of getting outdated very quickly” because XLS LLVM is updated daily.** Source: [Xu, Murphy, Josipović, arXiv 2603.19856](https://arxiv.org/html/2603.19856v1) §4.3. Publisher: ETH Zurich / arXiv. pub_date: 2026. accessed: 2026-08-18. confidence: high. class: failure | pattern

**Rust-to-gates that actually ships as crates is rust-hdl 0.46.0 (MIT): synthesizable subset → Verilog, plus `rust-hdl-hls` widgets and `rust-hdl-sim`; this is an RTL eDSL with an HLS-named crate, not XLS/Vitis-class scheduling HLS.** Source: [docs.rs/rust-hdl/0.46.0](https://docs.rs/rust-hdl/0.46.0/rust_hdl/). Publisher: docs.rs / samitbasu. pub_date: crate 0.46.0 (undated on page). accessed: 2026-08-18. confidence: high. class: landscape | version/compat

**A documented research path for “HLS from Rust” is rustc → LLVM IR → Bambu, not a production Rust HLS compiler.** Source: [arewefpgayet.rs](https://arewefpgayet.rs/). Publisher: independent blog. pub_date: undated on page. accessed: 2026-08-18. confidence: medium. class: landscape

**IEEE SystemC TLM is the industry dual-view contract: LT vs AT coding styles, generic payload, temporal decoupling, DMI; it is part of IEEE Std 1666-2023.** Source: [systemc.org TLM overview](https://systemc.org/overview/systemc-tlm/). Publisher: Accellera / systemc.org. pub_date: IEEE 1666-2023 cited on page. accessed: 2026-08-18. confidence: high. class: pattern | version/compat

**What ships for TLM+RTL is bridging, not one-IR generation: AMD `libsystemctlm-soc` connects QEMU (PS) ↔ SystemC/TLM-2.0 ↔ RTL converted by Verilator or commercial equivalents.** Source: [Xilinx/libsystemctlm-soc](https://github.com/xilinx/libsystemctlm-soc) and [AMD Adaptive Computing Wiki – Co-simulation](https://xilinx-wiki.atlassian.net/wiki/spaces/A/pages/862421112). Publisher: AMD/Xilinx. pub_date: living wiki/repo. accessed: 2026-08-18. confidence: high. class: pattern | implementation

**Verilator `--sc` emits a SystemC `SC_MODULE` with pin-level `bool`/`uint`/`sc_bv` ports, not TLM sockets; internals are not pure SystemC (pin interconnect everywhere would cost ~10×).** Source: [Connecting to Verilated Models, Verilator 5.050](https://verilator.org/guide/latest/connecting.html). Publisher: Veripool. pub_date: docs for 5.050 (2026-07-01). accessed: 2026-08-18. confidence: high. class: implementation | failure

**UVM dual-model practice that actually ships is scoreboard vs an ISA/golden C model (Spike via DPI, or a generated ISA sim), not a second RTL view generated from the same module IR.** Source: [Qiu & Liu, arXiv 2505.10145](https://arxiv.org/html/2505.10145v1); [isbogdanov/riscv-core-dv-uvm](https://github.com/isbogdanov/riscv-core-dv-uvm). Publisher: Beihang / arXiv; GitHub. pub_date: arXiv 2025-05; repo living. accessed: 2026-08-18. confidence: high. class: pattern

**One academic UVM-TLM “vmodel” reports CoreMark wall time ~10–15 min vs estimated 10–12 h RTL (~order-of-magnitude), with ~14.1% CoreMark/MHz error vs Xiangshan Nanhu — TLM is fast and not cycle-accurate.** Source: [arXiv 2505.10145](https://arxiv.org/html/2505.10145v1) §5. Publisher: Beihang / arXiv. pub_date: 2025-05. accessed: 2026-08-18. confidence: medium (single study; RTL time estimated). class: performance/scale | failure

**Commercial dual-model IP that ships uses two source models, not one IR: Codasip CodAL instruction-accurate (IA) vs cycle-accurate (CA); tools generate ISA sim (UVM golden predictor) from IA and RTL DUT from CA, with FIFO scoreboarding because the C++ model is untimed relative to RTL.** Source: [Zachariasova et al., DVCon, Codasip+Mentor](https://dvcon-proceedings.org/wp-content/uploads/uvm-based-verification-of-a-risc-v-processor-core-using-a-golden-predictor-model-and-a-configuration-layer.pdf). Publisher: Codasip / Mentor. pub_date: undated on PDF. accessed: 2026-08-18. confidence: medium. class: pattern

**Chisel 7 testing that ships is ChiselSim: peek/poke/expect/step on Verilator or VCS of generated SystemVerilog — cycle-accurate DUT, not a generated untimed golden; ChiselTest is unmaintained after the SFC→CIRCT move.** Source: [Chisel Testing](https://www.chisel-lang.org/docs/explanations/testing); [Migrating from ChiselTest](https://www.chisel-lang.org/docs/appendix/migrating-from-chiseltest). Publisher: Chisel / chipsalliance. pub_date: living Chisel 7 docs. accessed: 2026-08-18. confidence: high. class: pattern | failure

**CIRCT’s Sim dialect is a compiler IR for talking to simulators (Verilator, VCS, Arc, …) — DPI, plusargs, queues, SV file I/O — not a TLM untimed model generator.** Source: [CIRCT Simulation Dialect](https://circt.llvm.org/docs/Dialects/Sim/). Publisher: LLVM CIRCT. pub_date: living; DPI work merged 2026-04-03 ([PR #9977](https://github.com/llvm/circt/pull/9977)); `sim.get_file` 2026-04-10. accessed: 2026-08-18. confidence: high. class: implementation | ecosystem

**CIRCT Arc / arcilator is a cycle-accurate compiled sim: flatten HW/Seq/Comb → state-transfer “arcs” → LLVM object; current docs include process/coroutine lowering from LLHD.** Source: [CIRCT Arc Dialect](https://circt.llvm.org/docs/Dialects/Arc/). Publisher: LLVM CIRCT. pub_date: living docs. accessed: 2026-08-18. confidence: high. class: implementation

**Upstream CIRCT is not yet a drop-in event-driven UVM simulator: a 2026 Normal Computing CIRCT fork added event-driven sim, VPI/cocotb, UVM runtime (2,968 commits / 43 days) rather than landing that stack in llvm/circt.** Source: [Normal Computing blog](https://normalcomputing.com/blog/building-an-open-source-verilog-simulator-with-ai-580k-lines-in-43-days). Publisher: Normal Computing. pub_date: 2026 (Jan–Feb work described). accessed: 2026-08-18. confidence: medium. class: ecosystem | failure

**Verilator 5.050 (2026-07-01) is a live cycle-accurate C++/SystemC compiler: covergroups, more SVA, DPI-C, limited VPI; eval is cycle/event-step, VPI deposits do not propagate until `eval()`.** Source: [Verilator 5.050 changes](https://verilator.org/guide/latest/changes.html); [connecting.html](https://verilator.org/guide/latest/connecting.html). Publisher: Veripool. pub_date: 2026-07-01. accessed: 2026-08-18. confidence: high (seconded by Gentoo guru commit 2026-07-03 adding 5.050). class: version/compat | ecosystem | implementation

**Yosys CXXRTL is a Yosys backend that writes cycle-based C++ from RTLIL (driver toggles clocks; black boxes in C++); it is documented in current Yosys command reference (fetched as v0.48 docs) and was not listed among removals in Yosys 0.68.** Source: [write_cxxrtl (Yosys v0.48 docs)](https://yosyshq.readthedocs.io/projects/yosys/en/v0.48/cmd/write%5Fcxxrtl.html); [Yosys 0.68](https://github.com/YosysHQ/yosys/releases/tag/v0.68). Publisher: YosysHQ. pub_date: docs v0.48; release 2026-08-05. accessed: 2026-08-18. confidence: high that cxxrtl exists; medium that v0.48 text matches 0.68. class: implementation | version/compat

**essent generates cycle-accurate C++ from FIRRTL (static schedule + skip inactive partitions); no multi-clock; not a dual untimed/TLM lowering.** Source: [ucsc-vama/essent](https://github.com/ucsc-vama/essent/). Publisher: UCSC VAMA. pub_date: living README. accessed: 2026-08-18. confidence: high. class: implementation

**FireSim ships mixed-abstraction full-system sim: FPGA-accelerated cycle-accurate RTL (10s–100s of MHz) plus user software models for I/O not written as RTL — co-sim of separately written models, not two views from one module IR.** Source: [fires.im](https://fires.im/); [firesim/firesim](https://github.com/firesim/firesim). Publisher: FireSim project. pub_date: living. accessed: 2026-08-18. confidence: high. class: pattern | performance/scale

**Google MPACT-Sim is a retargetable instruction-set simulator toolkit (`.isa` / `.bin_fmt` → C++ decoder); accuracy is “customizable” at ISS level, not an RTL/HLS dual-view compiler.** Source: [developers.google.com/mpact-sim](https://developers.google.com/mpact-sim); [google/mpact-sim](https://github.com/google/mpact-sim/). Publisher: Google. pub_date: living. accessed: 2026-08-18. confidence: high. class: landscape

**Feasibility verdict from this run: generating a functional (untimed) host model and a cycle-accurate sim from one IR is already XLS’s product claim (interpreter/JIT vs scheduled Verilog). Generating TLM-2.0 LT sockets plus cycle-accurate RTL from one module IR is not what Verilator, CXXRTL, CIRCT Arc, essent, FireSim, or MPACT do; industry practice is adapters (TLM↔pins) or two source models (CodAL IA/CA, Spike vs RTL).** Sources: XLS project page; Verilator connecting; CIRCT Sim/Arc; FireSim; Codasip DVCon; IEEE 1666-2023 TLM page. accessed: 2026-08-18. confidence: high. class: pattern | implementation

**CIRCT vitality: llvm/circt ~2,220 stars this run; Sim/DPI landing in Apr 2026; it is Chisel’s production SV backend (Chisel docs).** Source: [llvm/circt](https://github.com/llvm/circt); Chisel testing/FileCheck pages. Publisher: LLVM / Chisel. pub_date: 2026-04 activity; living docs. accessed: 2026-08-18. confidence: medium on commit rate (no pulse/release series fetched). class: ecosystem

**Yosys vitality: monthly 0.61–0.68 from 2025-12 through 2026-08-05 (v0.68 removes flowmap/`abc -fast`, adds `symfpu`).** Source: [Yosys 0.68](https://github.com/YosysHQ/yosys/releases/tag/v0.68); [releases list](https://github.com/YosysHQ/yosys/releases). Publisher: YosysHQ. pub_date: 2026-08-05. accessed: 2026-08-18. confidence: high. class: ecosystem | version/compat

**Yosys↔Verilator lint coupling broke in the field: oss-cad-suite 2026-01-24 + Verilator fatal-errors on Yosys ECP5 `cells_bb.v` after synth_lattice refactor — recommended-linter story is not clean.** Source: [YosysHQ/yosys discussion #5633](https://github.com/YosysHQ/yosys/discussions/5633). Publisher: YosysHQ GitHub. pub_date: discussion of 2026-01-24 suite. accessed: 2026-08-18. confidence: high. class: failure | version/compat

**Five-year regret sketch (from these sources, not speculation beyond them): Vitis = QoR and IP catalog, AMD lock-in + IDE churn; XLS = closest dual-view IR, Google-experimental + no BC; Dynamatic/Bambu = open HLS research, MLIR/LLVM skew and academic staffing; rust-hdl = RTL subset, not HLS architecture; CIRCT = right multi-view compiler bet if you accept cycle-accurate Arc now and TLM as a separate lowering you would have to invent; Verilator+Yosys = safest sim/synth backends by release cadence.** Sources: items above. accessed: 2026-08-18. confidence: medium. class: landscape | ecosystem

## Leads worth chasing

- XLS JIT vs `codegen_main` vs Z3 LEC on a proc with channels — whether “functionally identical” covers timing/throughput or only untimed I/O.
- Whether CIRCT will grow an LT/untimed dialect beside Arc, or remain Sim-as-DPI-for-Verilator/VCS/Arc.
- Dynamatic handshake→XLS dialect: is it dead from LLVM skew (author-reported risk)?
- AMD `libsystemctlm-soc` + Verilator AXI examples as the concrete TLM↔RTL adapter pattern to copy.
- `rust-hdl-hls` crate semantics vs XLS scheduling (name collision risk).
- essent vs CIRCT FIRRTL version skew (not verified this run).
- Bambu LLVM-IR ingest as a Rust HLS off-ramp without owning an HLS scheduler.

## Looked for and did not find

- A production tool that lowers one module IR to TLM-2.0 LT sockets and to cycle-accurate C++/SV (XLS is host-JIT + Verilog, not TLM).
- Independent 2025–2026 QoR bakeoff: XLS vs Vitis vs Bambu vs Dynamatic.
- CIRCT tagged releases / 6-month commit histogram (GitHub homepage fetch was a stub).
- rust-hdl 2026 commit/release date (crate 0.46.0 undated; GitHub fetch stub).
- MPACT consuming XLS or CIRCT hardware IR.
- Evidence that CXXRTL’s old ~8×-vs-Verilator gap (2020 blog; out of pattern window) was fixed.
- Commercial IP catalog that auto-emits both views from a single source (Codasip uses two CodAL models).
- Chisel official “golden model testers” — not in current ChiselSim docs.
