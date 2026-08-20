# Digest: Integration — round 1 (crates.io bitloom surface)

**Accessed:** 2026-08-20

## Claims

1. **claim:** crates.io crate `bitloom` exists; newest/default version is `0.1.1` (also `0.1.0`); description states Bitloom CLI / binary `cargo-bitloom`; repository `https://github.com/TangCan/bitloom`; `has_lib: false`; `bin_names: ["cargo-bitloom"]`; MSRV `rust_version: 1.97.1`; crate size ~12KB; downloads still very low (~21 total at fetch time).  
   **source:** https://crates.io/api/v1/crates/bitloom  
   **publisher:** crates.io API  
   **pub_date:** versions created 2026-08-19  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** version

2. **claim:** Published `bitloom` 0.1.1 depends only on `clap ^4` and `sha2 ^0.10` — no published path to `rhdl-prelude`, `rhdl-hir`, or `rhdl-vlog`.  
   **source:** https://crates.io/api/v1/crates/bitloom/0.1.1/dependencies  
   **publisher:** crates.io API  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** version

3. **claim:** The README attached to crates.io `bitloom` 0.1.1 documents quick start as `cargo run -p bitloom -- build --package counter_ports ...` inside a workspace with `rust-toolchain.toml`, `just test`, and design crates depending on `rhdl-prelude` — i.e. monorepo-oriented instructions, not a pure post-`cargo install` tutorial.  
   **source:** https://crates.io/api/v1/crates/bitloom/0.1.1/readme  
   **publisher:** Bitloom README via crates.io  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** process

4. **claim:** crates.io reports crate `rhdl-prelude` does not exist; therefore end users cannot `cargo add rhdl-prelude` from crates.io today.  
   **source:** https://crates.io/api/v1/crates/rhdl-prelude (error detail)  
   **publisher:** crates.io API  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** version

5. **claim:** Search for `rhdl` on crates.io returns unrelated/prior art packages (e.g. `rhdl` / `rhdl-bits` associated with other FPGA/Rust HDL efforts) plus `bitloom` 0.1.1 — reinforcing Bitloom’s published identity is the CLI name `bitloom`, not those other crates.  
   **source:** https://crates.io/api/v1/crates?q=rhdl  
   **publisher:** crates.io API  
   **accessed:** 2026-08-20  
   **confidence:** high  
   **class:** landscape

6. **claim:** Generic web search for “crates.io bitloom” can confuse the name with unrelated crate `bitbloom` (Bloom filter) — name collision risk for users searching casually.  
   **source:** WebSearch results for “crates.io bitloom cargo install” (2026-08-20) surfacing bitbloom  
   **publisher:** search aggregator (secondary)  
   **accessed:** 2026-08-20  
   **confidence:** medium  
   **class:** landscape

## Leads
- docs.rs/crate/bitloom page was nearly empty in fetch — verify whether docs generation is bin-only stub
- GitHub TangCan/bitloom README vs crates.io README drift (`cargo run -p` vs `cargo bitloom`)

## Gaps
- No primary evidence yet of a published user-facing “standalone design crate template” crate from Bitloom.
