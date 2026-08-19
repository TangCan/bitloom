# Digest: implementation-reality r1-1

## Findings

**crates.io `rhdl` is still 0.1.0, published 2023-09-02, with ~1.7k total downloads and 10 recent; 0 dependents. GitHub default-branch HEAD is later (doc merge 2026-02-03) — published crate ≠ live repo.**
**source:** https://crates.io/crates/rhdl ; https://api.github.com/repos/samitbasu/rhdl/commits?per_page=1
**publisher:** crates.io / GitHub
**pub_date:** 2023-09-02 (crate); 2026-02-03 (commit `c99d5cc`)
**accessed:** 2026-08-18
**confidence:** high
**class:** version/compat

**RHDL is an in-repo rewrite of rust-hdl aiming at four gaps the author says rust-hdl lacked: high-perf sim, “just Rust” syntax, trivial reuse, enums-with-payloads. Compiler path is AST → RHIF IR → SSA/opts → RTL layer → Verilog AST (not string concat). Frontend is a `#[kernel]` / `#[hdl]` proc-macro; native Rust sim + VCD; Yosys not described as first-class.**
**source:** https://raw.githubusercontent.com/samitbasu/rhdl/main/README.md
**publisher:** Samit Basu / GitHub
**pub_date:** README as of HEAD (commit dated 2026-02-03)
**accessed:** 2026-08-18
**confidence:** high (authoritative for intended architecture; not independently measured)
**class:** implementation

**Author-stated sim speed: “roughly 1 to 2 orders of magnitude faster than RustHDL.” No cycle counts, design sizes, or Verilator comparison table. Verilator bridge is explicitly cancelled (`~~Build a verilator bridge…~~`). Remaining open plan items include porting the rust-hdl widget library, FPGA BSPs, wrapping foreign cores, and “revisit precise diagnostics.”**
**source:** https://raw.githubusercontent.com/samitbasu/rhdl/main/README.md
**publisher:** Samit Basu / GitHub
**pub_date:** README as of HEAD
**accessed:** 2026-08-18
**confidence:** medium (self-claim; no second numeric source this run)
**class:** performance/scale

**RHDL author flags proc-macro HDL as a structural risk: rustc not reusable, rust-analyzer “includes only the front end… not the middle parts,” proc-macros cannot share state and lack context. Schedule risk: rust-hdl came from paid commercial firmware; “I no longer get paid to write firmware. So progress is slower.”**
**source:** https://raw.githubusercontent.com/samitbasu/rhdl/main/README.md
**publisher:** Samit Basu / GitHub
**pub_date:** README as of HEAD
**accessed:** 2026-08-18
**confidence:** high for stated risks; n/a for 2026 velocity
**class:** failure

**Issue-tracker pain (RHDL, still open): `bits`/`Digital` UX — `u16` const inside `#[kernel]` fails with E0277 (`Digital` only for i128/u128/usize). Workaround: move consts outside kernel (`const …: b16 = b16(...)`). Other open issues include Stream API (2025-08), wingfoil composition RFC (2025-12). Issue list shows ~5 listed / search reported 9 open; activity is sparse vs rust-hdl.**
**source:** https://github.com/samitbasu/rhdl/issues/12 ; https://github.com/samitbasu/rhdl/issues
**publisher:** GitHub (brandonros, samitbasu)
**pub_date:** 2025-07-09 (#12); list retrieved 2026-08-18
**accessed:** 2026-08-18
**confidence:** high
**class:** failure

**rust-hdl published version is 0.46.0 (2023-07-02), ~93.5k downloads, 10 dependents. Architecture: `#[derive(LogicBlock)]` + `#[hdl_gen]` kernel → Verilog; built-in Rust sim; `generate_verilog`. Last crate release is >3 years old; GitHub issues still arrive in 2025.**
**source:** https://crates.io/crates/rust-hdl
**publisher:** crates.io
**pub_date:** 2023-07-02
**accessed:** 2026-08-18
**confidence:** high
**class:** version/compat

**rust-hdl docs: no Verilator/external-sim integration — “Currently, RustHDL can’t help much there… testbench won’t go with the design.” Yosys is recommended via `yosys_validate` as more thorough than RustHDL’s own checks. Sim “pretty fast, especially in release mode” — no Hz/cycle numbers. Macro cannot detect read-before-write; deferred to runtime static analyzer.**
**source:** https://docs.rs/rust-hdl/latest/rust_hdl/ ; https://docs.rs/rust-hdl
**publisher:** docs.rs / samitbasu
**pub_date:** docs for 0.46.0 (crate 2023-07-02)
**accessed:** 2026-08-18
**confidence:** high
**class:** implementation

**Proc-macro error quality still broken in rust-hdl 0.46: naming a signal `output`/`input` yields `error: custom attribute panicked` / assertion `left: "output"`. Open since 2023-07, still open 2026-08-18 — not fixed in current published version.**
**source:** https://github.com/samitbasu/rust-hdl/issues/29
**publisher:** GitHub (PoignardAzur)
**pub_date:** 2023-07-22 (open; last comment 2023-07-23)
**accessed:** 2026-08-18
**confidence:** high
**class:** failure

**Post-release sim bug still open: Signed<8> sim overflows on −128; −127..127 works. Filed 2025-04-26, no maintainer reply on the issue. Separate 2025 issues: “State machine that doesn’t work” (#46), BSP compile error (#48). rust-hdl is effectively unmaintained as a crate while users still hit sim/signed-edge cases.**
**source:** https://github.com/samitbasu/rust-hdl/issues/47 ; https://github.com/samitbasu/rust-hdl/issues
**publisher:** GitHub (OscarLopezCentenera et al.)
**pub_date:** 2025-04-26 (#47); list retrieved 2026-08-18
**accessed:** 2026-08-18
**confidence:** high that the report exists and is unanswered; medium that the root cause is a tool limitation vs user error (no maintainer diagnosis)
**class:** failure

**Author production claim (rust-hdl only): “firmware that is shipping in commercial products… nearly all of a moderately sized FPGA.” No part number, LUT/FF counts, clock, volume, or third-party confirmation this run. RHDL README: that fielded firmware was rust-hdl, not RHDL; RHDL itself has no shipping claim.**
**source:** https://crates.io/crates/rust-hdl (README); https://raw.githubusercontent.com/samitbasu/rhdl/main/README.md
**publisher:** Samit Basu
**pub_date:** rust-hdl README as of 0.46.0 (2023-07-02); RHDL README as of HEAD
**accessed:** 2026-08-18
**confidence:** low for “production with numbers” (self-claim, zero numbers)
**class:** landscape

**kaze is a non-macro builder eDSL (`Context`/`Module`/`Signal`) generating Rust sim code or Verilog. crates.io 0.1.19 last published 2021-03-14; 0 dependents; ~32k downloads. No 2024–2026 user retrospective found this run. Last crate age (~5 years) is the implementation reality.**
**source:** https://crates.io/crates/kaze
**publisher:** crates.io / yupferris
**pub_date:** 2021-03-14
**accessed:** 2026-08-18
**confidence:** high for version/staleness; low for “months of use” (no such threads retrieved)
**class:** version/compat

**Spade is not a Rust eDSL: standalone HDL + Swim. First `swim` build downloads and compiles a git-pinned compiler. Board templates: ecpix5, go-board, icesugar-nano, tinyfpga-bx, ulx3s_85k; author (TheZoq2) says Swim works best on ice40/ecp5/gowin. Hackaday (2025-04): “work in progress… missing features and breaking changes.” Verilog interop exists (`swim build` → `build/spade.sv`) but packing/`#[no_mangle]` constraints. No r/rust or r/FPGA “after months” thread found this run (site:reddit.com search returned none).**
**source:** https://docs.spade-lang.org/ws2812/project.html ; https://hackaday.com/2025/04/13/the-spade-hardware-description-language/ ; https://docs.spade-lang.org/external_verilog.html
**publisher:** spade-lang.org / Hackaday
**pub_date:** docs undated (retrieved 2026-08-18); Hackaday 2025-04-13
**accessed:** 2026-08-18
**confidence:** high for tooling burden; medium for learning-curve (advocacy + WIP warning, not long-term user diaries)
**class:** ecosystem

**Proc-macro / rust-analyzer (general, not HDL-specific): IDEs must compile macros to dylibs then expand via a separate proc-macro server; expansions can be sequential; users see “cannot expand proc macro because crate hasn’t been compiled yet.” JetBrains (2026-05) restates crash isolation and host-side arbitrary code. Combined with RHDL’s own “macros lack context / RA has no middle-end” and rust-hdl’s still-open `custom attribute panicked`, a `#[kernel]` HDL in 2026 inherits RA latency, expansion-before-check, and panic-shaped diagnostics unless the project invests in rustc-level diagnostics (open RHDL item).**
**source:** https://blog.jetbrains.com/rust/2026/05/29/how-rust-ides-understand-code/ ; https://fasterthanli.me/articles/proc-macro-support-in-rust-analyzer-for-nightly-rustc-versions ; RHDL README
**publisher:** JetBrains; Amos (fasterthanli); samitbasu
**pub_date:** 2026-05-29; article undated (retrieved 2026-08-18); RHDL README
**accessed:** 2026-08-18
**confidence:** high for RA architecture; medium for HDL-specific compile-time magnitudes (no HDL timing numbers this run)
**class:** implementation

**Adjacent Verilator+Rust-HDL experiment (not RHDL): zebreus/rust_hls found proc-macro-at-compile-time Verilator “performance is horrible”; build-script still “too slow” and “compatibility nightmare” (bambu + old Verilator v4.108). Useful as operational warning: in-process Verilator is a compile-time tax, not a free sim backend.**
**source:** https://github.com/zebreus/rust_hls ; https://crates.io/crates/rust_hls
**publisher:** zebreus / crates.io
**pub_date:** repo/docs retrieved 2026-08-18 (crate docs describe experimental status)
**accessed:** 2026-08-18
**confidence:** medium (single-author HLS experiment, not RHDL)
**class:** performance/scale

## Leads worth chasing

- LATTE’25 PDF `capra.cs.cornell.edu/latte25/paper/2.pdf` — fetch timed out this run; likely the only architecture paper with RHIF/sim numbers.
- GitHub `pushed_at` vs default-branch HEAD: profile listed rhdl “Last Updated 2026-03-27” vs API HEAD 2026-02-03 — check other branches/wiki.
- rust-hdl #46 (state machine) and #42 (embed VHDL/Verilog) for BSP/black-box reality.
- rhdl.org book vs crates.io 0.1.0 — whether docs document git-dep-only usage.
- Independent rust-hdl shipping product (company, FPGA family) — README claim is unattributed.
- Spade ACM TRETS Jan 2026 PDF (`spade-lang.org/spade.typ.pdf` was search-fetched) for compiler/error-message evaluation vs marketing.

## Looked for and did not find

- r/rust or r/FPGA threads on rust-hdl, RHDL, kaze, or Spade after months of use (`site:reddit.com rust-hdl OR rhdl FPGA` → no results this run).
- Tapeout claims or FPGA shipping **numbers** (LUTs, MHz, units, node, date) for RHDL, rust-hdl, kaze, or Spade. rust-hdl README is qualitative only; RHDL has none; kaze last crate 2021; Spade academic/WIP.
- Verilator or Yosys first-class integration in current RHDL (bridge cancelled).
- HDL-specific rust-analyzer compile-time measurements (only general RA proc-macro architecture).
- kaze 2024–2026 maintenance or production use.
- Confirmation that rust-hdl #29 / #47 were fixed in any release after 0.46.0 (none exists on crates.io).
