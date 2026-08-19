# Digest: architecture-patterns r2-1

Cornell `WebFetch` timed out again; **curl retrieved both LATTE’25 PDFs this run** (RHDL 539 KB / 3 pp; cmt2 457 KB / 3 pp). Author-site cmt2 PDF matches. Neither LATTE paper has sim-speed or FPGA-area tables. Independent Cement numbers appear only in a later arXiv paper, not in LATTE’25.

## Findings

**RHDL is a rustc co-compiler, not a syntax-matched Verilog transpiler.** RustHDL “is a transpiler, which generates Verilog syntax that is matched to the allowed subset of Rust… stripping type information out of the AST, and providing small shims” (e.g. `match` → Verilog `case`). RHDL “includes a compiler that treats Verilog like a machine-code target, lowering in steps.” A “co-compiler… runs alongside rustc to analyze the Rust source code and generate a series of HDL-compatible representations that are successively lowered.” Invocation is “adding an annotation to the Rust source code and adding a dependency on the appropriate packages.”
**source** https://capra.cs.cornell.edu/latte25/paper/2.pdf
**publisher** LATTE ’25 workshop (hosted by Cornell CAPRA); Samit Basu
**pub_date** 2025-03-30
**accessed** 2026-08-18
**confidence** high
**class** architecture-pattern

**Named IRs: RHIF → RTL → flow-graph → Verilog.** Lowering steps: (1) **RHDL Hardware Intermediate Form (RHIF)** — “strongly typed, static single assignment (SSA), register-based virtual machine instruction set”; (2) **RTL** — “untyped SSA register-based virtual machine instruction set”; (3) **flow graph** — “netlist representation (which may not be directed if the design has loops).” ADT/`match` example is shown as register SSA opcodes, “later lowered into RTL and then into Verilog.”
**source** https://capra.cs.cornell.edu/latte25/paper/2.pdf
**publisher** LATTE ’25 / Basu
**pub_date** 2025-03-30
**accessed** 2026-08-18
**confidence** high
**class** architecture-pattern

**Sim architecture in the LATTE paper is the RHIF/RTL virtual-machine ISAs, not a published cycle-accurate sim stack with numbers.** RHIF/RTL are described as VM instruction sets; no separate simulator product, no Hz, no cycles/sec, no design-size table. Only numeric example: `MyEnum` “is 21 bits wide.”
**source** https://capra.cs.cornell.edu/latte25/paper/2.pdf
**publisher** LATTE ’25 / Basu
**pub_date** 2025-03-30
**accessed** 2026-08-18
**confidence** high
**class** implementation

**Chisel mention is embedding analogy only; FIRRTL is not in the RHDL LATTE paper.** “RHDL (like its predecessor RustHDL) is embedded in the Rust programming language, much as MyHDL is embedded in Python and Chisel is embedded in Scala.” Ref [7] is `https://www.chisel-lang.org/`. No FIRRTL, firtool, or Chisel codegen path.
**source** https://capra.cs.cornell.edu/latte25/paper/2.pdf
**publisher** LATTE ’25 / Basu
**pub_date** 2025-03-30
**accessed** 2026-08-18
**confidence** high
**class** landscape

**RHDL limitations stated in-paper (no eval).** Only “a very small set of Rust code can be directly mapped to Verilog”; mapping a larger subset “requires significant analysis.” RHDL type inference “must agree with the types inferred by rustc to avoid miscompilation.” Clock-domain `Signal<T, Color>` uses marker types; illegal CDC add fails compile; “Clock domain crossings require special constructs provided in the RHDL core library.” Timing estimator is “simple” / “basic”; built-in heuristic “counts the number of non-trivial operations on every path (after optimization)”; intent is later “closed loop from 3rd party tools.” “Zero-cost abstraction” is asserted, not measured.
**source** https://capra.cs.cornell.edu/latte25/paper/2.pdf
**publisher** LATTE ’25 / Basu
**pub_date** 2025-03-30
**accessed** 2026-08-18
**confidence** high
**class** failure (coverage: no numbers)

**cmt2 embedding: procedural macros, explicitly not a rustc plugin.** “cmt2 is embedded in Rust using Rust’s powerful procedural macro system, as opposed to modifying the rustc compiler with plug-ins, as done in HazardFlow [5].” Reasons given: (1) “greater flexibility for custom syntax”; (2) “clear distinction between the embedded DSL and the host language.” Macros named: `itfc_declare!`, `#[module]`, `io!`, `instance!`, `method!`, `rule!`, `schedule!`, `for_!`.
**source** https://capra.cs.cornell.edu/latte25/paper/1.pdf (mirror https://uv-xiao.github.io/assets/pdf/cmt2_latte.pdf )
**publisher** LATTE ’25; Youwei Xiao, Zizhang Luo, Yun Liang (Peking University)
**pub_date** 2025-03-30
**accessed** 2026-08-18
**confidence** high
**class** architecture-pattern

**LATTE cmt2 backend path: compiler → FIRRTL / SystemVerilog / Verilator / Khronos; no firtool version, no named project IR.** “We implement a set of backends for cmt2’s compiler, targeting FIRRTL [4], SystemVerilog [3], and simulation tools such as Verilator [7] and Khronos [9].” FIRRTL backend is “a drop-in HDL choice for Chipyard-based SoC development, where generators like Rocket Chip [6] can be reused with cmt2 at the FIRRTL level.” Simulation: “embedded-in-rust rule-based testbench specification.” Paper does **not** name CMTIR/CTIR or firtool.
**source** https://capra.cs.cornell.edu/latte25/paper/1.pdf
**publisher** LATTE ’25 / PKU
**pub_date** 2025-03-30
**accessed** 2026-08-18
**confidence** high
**class** architecture-pattern

**LATTE cmt2 is a 3-page position paper with zero evaluation numbers.** No LUT/FF/MHz/CPI/sim-speed tables. Stated limits: prior rule-based HDLs “rely exclusively on latency-insensitive composition” with “unnecessary overhead”; cmt2 schedule “cannot describe the total order of all rules across multiple modules” but pairs sharing state get a deterministic order (Kôika-style, cycle-accurate serialize). Future work: “stabilize cmt2’s temporal semantics features and make them publicly available in the near future.”
**source** https://capra.cs.cornell.edu/latte25/paper/1.pdf
**publisher** LATTE ’25 / PKU
**pub_date** 2025-03-30
**accessed** 2026-08-18
**confidence** high
**class** failure (no LATTE eval)

**Independent Cement2 numbers exist on arXiv, not in LATTE’25.** Compiler “built around the Cement2 Transaction Intermediate Representation (**CTIR**)”; “translates CTIR into FIRRTL… and generates optimized SystemVerilog using **firtool-1.108.0**” for Vivado FPGA deploy. Sim: Verilator **v5.028**; C++ harness from rule testbenches. Soft-core **CMT2-RV** vs Sodor (Chisel) vs HazardFlow, Vivado **2024.1**, **XCVU9P**, memories excluded:

| | Sodor | HazardFlow | CMT2-RV | CMT2-RV+custom |
|---|---|---|---|---|
| CPI | 1.389 | 1.389 | 1.386 | — |
| Fmax | 367 MHz | 287 MHz | **377 MHz** | 316 MHz |
| LUT | 1974 | 3055 | **1614** (0.82× Sodor) | 2729 |
| FF | 924 | 2829 (3.06× Sodor) | **779** (0.84× Sodor) | 1152 |

Soft processor: 571 SLOC, untimed multi-cycle, latency-insensitive synth interface. Custom ops: 86 SLOC; Edge Detection cycle count **−75%**; +16% Fmax drop vs CMT2-RV. PolyBench 13 kernels: 771 SLOC / 1 PhD-day vs SV 2610 SLOC / 1 week; geomean vs SV: cycle 0.92×, time 1.01×, LUT 0.87×, FF 0.81× (Vivado 2024.1, XCVU9P, 7 ns). Systolic vs Chisel: geomean **−7% LUT, −4% FF, 1.03× frequency** (XCU250, 2.5 ns).
**source** https://arxiv.org/html/2511.15073v1
**publisher** arXiv (Xiao, Luo, Peng, Zou, Liang)
**pub_date** 2025-11 (arXiv 2511.15073)
**accessed** 2026-08-18
**confidence** high (HTML conversion of tables)
**class** implementation

**MINRES FPGA’d a 5-stage pipelined RISC-V in RHDL; no numeric results on the eval writeup.** Harris & Harris five stages + hazard unit; VHDL→RHDL; extra memory router + MMIO GPIO; RISC-V binary counter on 8 LEDs. Board: **Trenz TEC0117**. Flow: RHDL Verilog **out-of-crate** through **Yosys**; “complete board toolchain was not implemented directly inside the RHDL crate.” Limits: early docs; combinational subcircuits had to be restructured; **no negative-edge RF write-back** in RHDL so the VHDL design was changed. No LUT/FF/MHz/sim figures.
**source** https://www.minres.com/pipelined-riscv-in-rhdl/
**publisher** MINRES Technologies GmbH
**pub_date** undated on page
**accessed** 2026-08-18
**confidence** high
**class** implementation

**MINRES “RISC-V Tournament” has no RHDL core and therefore no RHDL FPGA numbers.** README last updated **2026-06-05**. Cores present: `bluespec` (5216 CPE_LT, 16.39 MHz) and `verilog` (11558 CPE_LT, 17.15 MHz) on **GateMate CCGM1A1**. Layout lists `cores/verilog/` and `_template/` only.
**source** https://raw.githubusercontent.com/Minres/riscv-tournament/main/README.md
**publisher** Minres/GitHub
**pub_date** 2026-06-05 (README “Last updated”)
**accessed** 2026-08-18
**confidence** high
**class** version/compat

**RHDL LATTE PDF is not on OpenReview/arXiv under the searched titles.** Queries `RHDL Samit Basu LATTE 2025` + `site:arxiv.org` / `site:openreview.net` returned unrelated LATTE/VERT hits, not this paper. Official copy is the workshop PDF.
**source** WebSearch this run; PDF retrieved from https://capra.cs.cornell.edu/latte25/paper/2.pdf
**publisher** n/a (negative search)
**pub_date** n/a
**accessed** 2026-08-18
**confidence** medium (search completeness)
**class** landscape

## Leads worth chasing

- **arXiv:2511.15073** is the independent Cement2 architecture/sim/FPGA table source; LATTE’25 cmt2 is a preview without numbers. Confirm PDF (not HTML) table column labels and whether CTIR ≡ GitHub `cmtir`.
- **RHDL sim speed / design size:** not in LATTE’25. Next: `samitbasu/rhdl` docs (`doc/osda2024/osda2024.pdf`, `doc/latte24/latte.pdf` linked from third-party notes), crate benches, GitHub README “high performance simulation.”
- **RHDL rustc plugin vs proc-macro:** LATTE says only “co-compiler alongside rustc” + “annotation”; does not say plugin vs `syn`. Repo/compiler crate is the next source.
- **MINRES numbers:** TEC0117 Yosys logs / bitstream reports if they exist internally; tournament `cores/` has no `rhdl` yet — watch for a PR.
- Workshop homepage https://capra.cs.cornell.edu/latte25/ timed out this run; CFP search snippets say 2-page position papers, PDFs on site, **not in proceedings**.

## Looked for and did not find

- RHDL LATTE’25: sim speed, design-size (LUTs/gates/LOC), FIRRTL/firtool, rustc-plugin vs proc-macro, any RISC-V/FPGA eval.
- cmt2 LATTE’25: evaluation tables; IR name CMTIR/CTIR; firtool version.
- OpenReview or arXiv copy of Basu LATTE’25 RHDL.
- MINRES RHDL FPGA LUT/FF/Fmax/sim-speed; RHDL row in riscv-tournament (as of 2026-06-05 README).
- Cornell workshop HTML index (fetch timeout); PDFs obtained by direct curl.
