# Digest: landscape — round 1

agent: 7c5cee26-17fa-483e-90a4-6b10c466ad94
accessed: 2026-09-09

## Findings (bulleted claims)

- Peer Scala/Python RTL eDSLs publicly define product “done” around **RTL/netlist generation + host-language tooling + thin stdlib/build/sim**, and explicitly **exclude HLS** as a core goal | https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Introduction/SpinalHDL.html | SpinalHDL project docs | undated page (“master”; site states SpinalHDL started Dec 2014) | accessed 2026-09-09 | high | primary_doc

- SpinalHDL scopes **IDE as host Scala IDE** (IntelliJ Scala / VSCodium Metals: completion, rename, navigation)—not a first-party HDL LSP product | https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Introduction/SpinalHDL.html + https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Getting%20Started/Install%20and%20setup.html | SpinalHDL project docs | undated (master) | accessed 2026-09-09 | high | primary_doc

- SpinalHDL treats **IP completeness** as (a) reusable generators in `spinal.lib` and (b) **BlackBox** for existing VHDL/Verilog—not a curated commercial IP catalog as a maturity gate | https://github.com/SpinalHDL/SpinalHDL (README “Not an HLS…” / lib deps) + https://spinalhdl.github.io/SpinalDoc-RTD/dev/SpinalHDL/Structuring/blackbox.html | SpinalHDL | README live; blackbox page undated (dev) | accessed 2026-09-09 | medium-high | primary_repo/docs

- SpinalHDL docs pair simulation “done” with **Verilator (+ other sims) and Gtkwave**—standard EDA waves, not a typed/source-level viewer as stock product | Install/setup docs above | SpinalHDL | undated (master) | accessed 2026-09-09 | medium-high | primary_doc

- Amaranth’s published toolchain boundary is **language + optional stdlib + Python simulator + FPGA build/board integration** = “all steps of a typical FPGA development workflow,” while still allowing Verilog/VHDL interop both ways | https://amaranth-lang.org/docs/amaranth/latest/intro.html | Amaranth project (docs marked 0.6.0.dev138) | undated page; version string implies pre-0.6.0 | accessed 2026-09-09 | high | primary_doc

- Amaranth still marks **native-code / co-simulation as deferred** (“not support … at the moment … will be added in near future”), so multi-engine sim parity is not a freeze criterion for current “toolchain complete” messaging | same Amaranth intro | Amaranth | as above | accessed 2026-09-09 | high | primary_doc

- Amaranth release “done” is scoped as **blocker RFCs vs nice-to-haves** (e.g. full `hdl` reference docs framed as credibility/nice-to-have once guides exist)—maturity is gated on language/stdlib contracts, not IDE/HLS/IP marketplace | https://github.com/amaranth-lang/amaranth/issues/1225 | amaranth-lang (whitequark) | created 2024-03-22; closed 2024-06-14 | accessed 2026-09-09 | medium (roadmap age >12mo; still useful for “how done is defined”) | primary_issue

- Clash’s public feature surface is **typed Haskell HDL + stdlib + interactive REPL + low-level HDL templating**; homepage does **not** claim first-party IDE/LSP, HLS, or a productized IP library as stock | https://clash-lang.org/ | Clash / QBayLogic | undated live homepage | accessed 2026-09-09 | high | primary_site

- Search for “Clash LSP” hits **unrelated** `clash-lsp` (policy files), reinforcing that **HDL-Clash LSP is not a visible core product artifact** in public naming | https://docs.rs/crate/clash-lsp/latest | docs.rs | crate 0.7.2 listed as “latest” at fetch time | accessed 2026-09-09 | medium | secondary_index (negative evidence)

- PyMTL3 publicly scopes “done” as a **multi-level modeling / sim / verify framework** (elaborate + passes; FL↔RTL testing patterns; line traces; optional Verilator cosim)—visualization is **line-trace / VCD+external viewers**, not a first-party IDE | https://pymtl.github.io/ + https://pymtl3.readthedocs.io/_/downloads/en/latest/pdf/ + Cornell ECE materials citing Gtkwave | PyMTL3 / Cornell courseware | docs undated; ECE tutorial PDF live | accessed 2026-09-09 | medium-high | primary_docs + academic_secondary

- Chisel **v7.0.0** (2025-09-08) release notes emphasize language/FIRRTL/CIRCT/`firtool` integration and APIs—**not** HLS, IP catalog, or IDE as ship criteria | https://github.com/chipsalliance/chisel/releases/tag/v7.0.0 | chipsalliance/chisel | published 2025-09-08 | accessed 2026-09-09 | high | primary_release

- In ~12 months (through mid-2026 discourse), **Chisel→CIRCT/`firtool` remains the consolidated backend path**, while **source-level debug/wave metadata is still churning**: Tywaves-era forks → upstream Chisel `circt_debug_*` intrinsics → proposed **UHDI** second debug format because **HGLDD cannot be freely extended** (Synopsys VCS/Verdi coupling) | https://discourse.llvm.org/t/uhdi-unified-hardware-debug-info-structured-debug-info-export-for-chisel-firrtl/90973 | LLVM Discourse / CIRCT (fkhaidari et al.) | posts 2026-06-03 and 2026-07-24 | accessed 2026-09-09 | high | primary_discourse

- UHDI v1 explicitly **does not** replace HGLDD, promise cross-version ID stability, or model memories—i.e. **typed/source-level debug is not treated as a finished core-product checkbox** | same UHDI thread | CIRCT | 2026-07-24 doc update | accessed 2026-09-09 | high | primary_discourse

- Typed waveform viewing for Chisel (Tywaves/Surfer) is framed as **bridging an abstraction gap left by open-source wave tools after lowering**—i.e. peers treat rich visualization as **adjacent research/integration**, historically via forks pending upstream | https://arxiv.org/html/2408.10082v1 + https://github.com/rameloni/tywaves-chisel | Meloni et al. / Tywaves repo | arXiv 2024-08; IEEE NorCAS pub meta 2024-10-29; repo notes “under development” / forks | accessed 2026-09-09 | high | primary_paper + primary_repo

- CIRCT **probe dialect** work (2026) still stages **read-only probes first**; RWProbe/force/release deferred—observability IR is **incremental**, not a single “visualization done” milestone | https://discourse.llvm.org/t/rfc-a-core-level-probe-dialect/91315 | LLVM Discourse / CIRCT | RFC active; related PR noted opened 2026-07-11 | accessed 2026-09-09 | medium-high | primary_discourse

- Cross-peer pattern (Spinal explicit; Amaranth/Clash by omission): **claiming HLS as core-product “done” is publicly rejected or unclaimed**; “done” is **predictable RTL elaboration**, not algorithmic synthesis | Spinal “What SpinalHDL is not” + Clash/Amaranth homepage/intro scopes | SpinalHDL; Clash; Amaranth | as above | accessed 2026-09-09 | high | synthesis_from_primaries

- Cross-peer pattern: **first-party IDE/LSP is rarely a maturity gate**; Scala HDLs lean on Metals/IntelliJ; Clash leans on Haskell tooling + REPL; Python HDLs lean on Python tooling + REPL/pytest | Spinal install docs; Clash homepage; Amaranth/PyMTL intros | multiple | as above | accessed 2026-09-09 | medium-high | synthesis_from_primaries

## Leads

- CIRCT UHDI PR stack status vs merge (follow firtool release notes ≤1mo for “format landed”)
- Chisel `spinal.lib`-class ecosystems (rocket-chip, diplomacy) vs language-core “done” boundary
- Amaranth RFCs post-0.5 / 0.6 for streams, sim, and any IDE mentions
- Surfer upstream typed-view support vs surfer-tywaves fork lifetime
- Clash book’s reusable-component library as de-facto IP vs compiler package
- PyMTL3 maintenance cadence / whether multi-view remains marketed after Cornell course refresh

## Looked for but not found

- A single peer **product roadmap** that treats **HLS + IP marketplace + first-party LSP + multi-view sim** as one coherent late-stage “done” definition
- Official **Clash HDL Language Server** product page (homonymous crates are unrelated)
- Fresh (≤12mo) Amaranth statement that **HLS is planned** as core
- Consolidated public claim that **Tywaves/UHDI is stock default** in released Chisel/`firtool` user workflows (as of sources read, still proposal/series + prior forks)

## Sources read (count)

**8 distinct sources fully/substantively read:** SpinalHDL About + Install (counted as Spinal docs set), Amaranth latest intro, Clash homepage, Chisel v7.0.0 release notes, CIRCT UHDI Discourse thread, CIRCT Probe Dialect RFC (skimmed via search+snippet), Tywaves arXiv HTML + Tywaves-chisel README (counted as Tywaves set), PyMTL3 quickstart/PDF + related course notes (counted as PyMTL set).