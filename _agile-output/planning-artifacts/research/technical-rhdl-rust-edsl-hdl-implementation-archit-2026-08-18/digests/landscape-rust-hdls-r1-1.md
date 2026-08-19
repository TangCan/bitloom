# Digest: landscape-rust-hdls r1-1

## Findings

- **claim:** `samitbasu/rhdl` is an active (not archived) Rust-embedded HDL rewrite of `rust-hdl`; GitHub last push 2026-04-21, created 2023-09-02, 324 stars / 24 forks / 9 open issues as of access.
- **source:** https://api.github.com/repos/samitbasu/rhdl
- **publisher:** GitHub (samitbasu/rhdl metadata)
- **pub_date:** 2026-04-21
- **accessed:** 2026-08-18
- **confidence:** high
- **class:** landscape

- **claim:** crates.io `rhdl` 0.1.0 is frozen at first publish (2023-09-02); 1679 total downloads, 10 recent, 0 reverse dependents — GitHub activity is not reflected on crates.io.
- **source:** https://crates.io/crates/rhdl
- **publisher:** crates.io (owner samitbasu)
- **pub_date:** 2023-09-02
- **accessed:** 2026-08-18
- **confidence:** high
- **class:** version/compat

- **claim:** RHDL’s stated design goals vs rust-hdl are: high-performance simulation, “it’s just Rust” syntax (match/if/let/generics/early return; no refs/lambdas), trivial reuse of (state, constants, types, kernel) components, and enums with payloads; rust-hdl is described as structured typed-Verilog with `.val()`/`.next` and C-style enums only.
- **source:** https://raw.githubusercontent.com/samitbasu/rhdl/main/README.md
- **publisher:** Samit Basu / samitbasu/rhdl
- **pub_date:** 2026-04 (README as of last push)
- **accessed:** 2026-08-18
- **confidence:** high
- **class:** implementation

- **claim:** Author claims RHDL simulation is “roughly 1 to 2 orders of magnitude faster than RustHDL.” No method, design size, or independent measurement is given. (single-source performance)
- **source:** https://raw.githubusercontent.com/samitbasu/rhdl/main/README.md
- **publisher:** Samit Basu / samitbasu/rhdl
- **pub_date:** 2026-04
- **accessed:** 2026-08-18
- **confidence:** low
- **class:** implementation

- **claim:** Compiler stack claimed complete on the project checklist: RHIF IR, AST→RHIF, type inference, multi-kernel compiler, SSA, RHIF→Verilog, RTL bridge, flow-graph sim, clock-crossing checks, AXI4-Lite/Stream (minimal), and wrapping RHDL cores for other languages. Still unchecked: wrapping cores in other languages, porting the rust-hdl widget library, porting FPGA BSPs.
- **source:** https://raw.githubusercontent.com/samitbasu/rhdl/main/README.md
- **publisher:** Samit Basu / samitbasu/rhdl
- **pub_date:** 2026-04
- **accessed:** 2026-08-18
- **confidence:** medium
- **class:** implementation

- **claim:** RHDL README names proc-macro limits (no shared state, no context) and nascent rustc-reuse as design risks; progress is described as slower because rust-hdl was commercially sponsored and RHDL is not.
- **source:** https://raw.githubusercontent.com/samitbasu/rhdl/main/README.md
- **publisher:** Samit Basu / samitbasu/rhdl
- **pub_date:** 2026-04
- **accessed:** 2026-08-18
- **confidence:** high
- **class:** failure

- **claim:** crates.io `rust-hdl` latest is 0.46.0 (2023-07-02); 93552 total downloads, 1115 recent, 10 dependents; still advertised as compiling a Rust subset to Verilog with sim/verification and a widget library (FIFOs, RAM/ROM, SPI, PWM).
- **source:** https://crates.io/crates/rust-hdl
- **publisher:** crates.io (owner samitbasu)
- **pub_date:** 2023-07-02
- **accessed:** 2026-08-18
- **confidence:** high
- **class:** version/compat

- **claim:** Same crates.io README claims rust-hdl firmware is “shipping in commercial products” including designs that “use nearly all of a moderately sized FPGA.” Same-author corroboration in RHDL README: “fielded production quality firmware which was built with [RustHDL].” No third-party product name. (two docs, one author)
- **source:** https://crates.io/crates/rust-hdl
- **publisher:** crates.io / Samit Basu
- **pub_date:** 2023-07-02
- **accessed:** 2026-08-18
- **confidence:** medium
- **class:** landscape

- **claim:** `samitbasu/rust-hdl` is not archived; last push 2025-06-03, 500 stars / 29 forks / 26 open issues. Successor migration is therefore incomplete: crate frozen 2023-07, repo still receiving some 2025 commits, RHDL still lists widget/BSP ports as TODO.
- **source:** https://api.github.com/repos/samitbasu/rust-hdl
- **publisher:** GitHub (samitbasu/rust-hdl metadata)
- **pub_date:** 2025-06-03
- **accessed:** 2026-08-18
- **confidence:** high
- **class:** ecosystem

- **claim:** LATTE’25 author paper (search snippet; PDF fetch timed out) states RHDL is a complete rewrite after RustHDL learner feedback; RustHDL was a Verilog-matching transpiler stripping types, while RHDL is a compiler over a broader Rust subset, still requiring all RHDL to be valid rustc. Paper accessed-Jan-2025 citation of github.com/samitbasu/rhdl. (single-source vs README)
- **source:** https://capra.cs.cornell.edu/latte25/paper/2.pdf
- **publisher:** Cornell CAPRA / LATTE 2025 (Samit Basu)
- **pub_date:** 2025-01
- **accessed:** 2026-08-18
- **confidence:** medium
- **class:** implementation

- **claim:** `yupferris/kaze` is an archived Rust-embedded builder-API HDL (Module/Signal → Rust sim or Verilog); last push 2023-11-15, 204 stars, 31 open issues. Not a 2026 implementation base.
- **source:** https://api.github.com/repos/yupferris/kaze
- **publisher:** GitHub (yupferris/kaze metadata)
- **pub_date:** 2023-11-15
- **accessed:** 2026-08-18
- **confidence:** high
- **class:** landscape

- **claim:** crates.io `kaze` last published version is 0.1.19 (search/crates index; crate page not fetched this run). (single-source version)
- **source:** https://crates.io/crates/kaze
- **publisher:** crates.io
- **pub_date:** 2021-03 (lib.rs listing in search)
- **accessed:** 2026-08-18
- **confidence:** medium
- **class:** version/compat

- **claim:** `spade-lang/spade` GitHub is a live read-only mirror of gitlab.com/spade-lang/spade; last push 2026-08-18, 80 stars, 1 open issue. Compiler is Rust; language is standalone (not a Rust eDSL).
- **source:** https://api.github.com/repos/spade-lang/spade
- **publisher:** GitHub (spade-lang/spade metadata)
- **pub_date:** 2026-08-18
- **accessed:** 2026-08-18
- **confidence:** high
- **class:** landscape

- **claim:** Spade official site (search snippet; page not fetched) positions Spade vs Chisel as a standalone language whose type system is available in hardware, not only at elaboration; vs Bluespec/HLS it stays RTL; first-class pipelines with latency in the language; `swim` build tool; compiles to Verilog. ACM TRETS paper listed as January 2026. Hackaday 2025-04 quotes maintainers: WIP/breaking changes; best on ice40/ecp5/gowin via OSS FPGA tools.
- **source:** https://spade-lang.org/
- **publisher:** Spade project / Linköping University authors
- **pub_date:** 2026-01
- **accessed:** 2026-08-18
- **confidence:** medium
- **class:** ecosystem

- **claim:** `veryl-lang/veryl` is the largest Rust-written HDL by repo activity: last push 2026-08-18, 1019 stars / 69 forks / 141 open issues. It is a standalone SV-oriented HDL (compiler in Rust), not a Rust eDSL.
- **source:** https://api.github.com/repos/veryl-lang/veryl
- **publisher:** GitHub (veryl-lang/veryl metadata)
- **pub_date:** 2026-08-18
- **accessed:** 2026-08-18
- **confidence:** high
- **class:** landscape

- **claim:** Veryl v0.20.2 released 2026-07-01 (incremental build cache, simulator backends). Project README (search) argues Chisel-style alt-HDLs generate unreadable Verilog and cannot bind SV `interface`/`struct`, blocking ASIC ECO; Veryl’s bet is human-readable SV. (release date from GitHub tag search, not API JSON)
- **source:** https://github.com/veryl-lang/veryl/releases/tag/v0.20.2
- **publisher:** veryl-lang
- **pub_date:** 2026-07-01
- **accessed:** 2026-08-18
- **confidence:** medium
- **class:** version/compat

- **claim:** `pku-liang/Cement` (`cmt2` default branch) is a Rust-embedded rule-based HDL; last push 2025-01-16, 40 stars, 0 open issues — idle >12 months as of access. Search README: `cmtrs` proc-macro frontend → `cmtir` → FIRRTL/`firtool` SystemVerilog, simulators Verilator and Khronos; redesigned from FPGA’24 `cmt1` to Bluespec-like rules. (FIRRTL/backend details single-source search README)
- **source:** https://api.github.com/repos/pku-liang/Cement
- **publisher:** GitHub (pku-liang/Cement metadata)
- **pub_date:** 2025-01-16
- **accessed:** 2026-08-18
- **confidence:** medium
- **class:** landscape

- **claim:** LATTE’25 `cmt2` paper snippet: embedding via procedural macros rather than rustc plugins (contrasted with HazardFlow); macros draw a boundary between HDL regions and Rust parameterization/construction.
- **source:** https://capra.cs.cornell.edu/latte25/paper/1.pdf
- **publisher:** Cornell CAPRA / LATTE 2025
- **pub_date:** 2025
- **accessed:** 2026-08-18
- **confidence:** medium
- **class:** implementation

- **claim:** Name collision: `VHDL-LS/rust_hdl` is a VHDL language server (releases through 2025–2026), not a Rust HDL. Do not treat crates/repos named `rust_hdl` as rust-hdl successors without checking owner.
- **source:** https://github.com/VHDL-LS/rust_hdl/releases
- **publisher:** VHDL-LS
- **pub_date:** 2025-03-22
- **accessed:** 2026-08-18
- **confidence:** high
- **class:** landscape

- **claim:** Consolidating in this niche: Verilog/SystemVerilog as the synthesis handoff; type-safe RTL (not HLS); compiler IRs (RHIF, cmtir, FIRRTL) instead of string transpile; Rust either as host eDSL or as compiler implementation language. Churning: crates.io vs git for RHDL; unfinished rust-hdl → RHDL widget/BSP port; kaze archived; Cement last push Jan 2025 vs Veryl/Spade daily 2026 activity; competing bets (Rust-subset kernels vs builder APIs vs standalone Spade vs SV-alt Veryl vs rule-based Cement).
- **source:** https://raw.githubusercontent.com/samitbasu/rhdl/main/README.md
- **publisher:** synthesis of fetched repo/crate metadata this run
- **pub_date:** 2026-08
- **accessed:** 2026-08-18
- **confidence:** medium
- **class:** ecosystem

- **claim:** What this generation newly enables vs Verilog: rustc-checked subset + payload enums + kernel/IR compilation (RHDL README); vs Chisel: option of a hardware-resident type system and first-class pipelines without Scala elaboration (Spade site, medium); vs Spinal/Chisel alt-HDL ECO pain: readable SV (Veryl, medium); vs all of them: a FIRRTL/`firtool` path from a Rust proc-macro eDSL (Cement, idle). RHDL itself still lists wrapping foreign-language cores as TODO — Chisel interop is not a delivered RHDL feature.
- **source:** https://raw.githubusercontent.com/samitbasu/rhdl/main/README.md
- **publisher:** mixed primary (RHDL README high; Spade/Veryl/Cement search medium)
- **pub_date:** 2026-04
- **accessed:** 2026-08-18
- **confidence:** medium
- **class:** implementation

- **claim:** Downstream RHDL user `zebreus/a5-1-rhdl` (search) reports rust-hdl development stopped in favor of RHDL; RHDL “not quite finished yet, but should be usable”; crates were not on crates.io so they used path deps; docs were essentially the compiler source. Aligns with crates.io 0.1.0 and unchecked widget/BSP items.
- **source:** https://github.com/zebreus/a5-1-rhdl
- **publisher:** zebreus
- **pub_date:** (undated in snippet; post-RHDL)
- **accessed:** 2026-08-18
- **confidence:** medium
- **class:** ecosystem

## Leads worth chasing

- Full LATTE’25 RHDL PDF (fetch timed out) for compiler vs transpile, sim, and Chisel comparisons.
- GitLab canonical Spade history vs GitHub mirror; `swim` crate versions.
- RHDL workspace crate versions/tags on git vs crates.io 0.1.0; whether `rhdl.org` docs exist.
- Content of rust-hdl push 2025-06-03 (maintenance vs leftover).
- Cement2 arXiv 2511.15073 eval numbers; `cmtrs` crates.io current version.
- HazardFlow (rustc-plugin HDL cited by cmt2) status.
- MINRES “Pipelined RISC-V in RHDL” FPGA eval (Yosys; board BSP not in RHDL).
- Whether any RHDL path emits FIRRTL (checklist shows Verilog assembler, not FIRRTL).

## Looked for and did not find

- Independent (non-author) RHDL vs rust-hdl simulation-speed numbers.
- Any crates.io `rhdl` release after 0.1.0 (2023-09-02).
- Third-party named production chip/product using RHDL (only rust-hdl, same author).
- RHDL Chisel/FIRRTL backend or completed foreign-core wrapping.
- kaze successor or un-archive.
- Live `rhdl.org` documentation contents (homepage listed on crates.io; not retrieved).
- HazardFlow or Silica/tpt-silicon maturity with primary fetch this run.
