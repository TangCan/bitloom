# Digest: naming landscape collisions — r1

**Source agent:** [Naming landscape](dd91bebe-7b67-48ca-a773-d58dbfd091fa) · accessed 2026-08-19 · confidence high

## Verdict

**Do not use “RHDL” as the public product name.** In the Rust → RTL / FPGA niche, that brand is already owned by Samit Basu’s rewrite of rust-hdl (GitHub, crates.io, LATTE paper, industry writeups). A second RHDL is high confusion / impersonation risk.

## Evidence (compressed)

| claim | URL | confidence |
|-------|-----|------------|
| samitbasu/rhdl is active Rust HDL branded RHDL (~300+★); rewrite of rust-hdl | https://github.com/samitbasu/rhdl | high |
| crates.io `rhdl` / homepage rhdl.org — taken by samitbasu | https://crates.io/crates/rhdl | high |
| rust-hdl predecessor; README points rename → rhdl | https://crates.io/crates/rust-hdl | high |
| Academic: “RHDL: Rust as a Hardware Description Language” (LATTE’25) | https://capra.cs.cornell.edu/latte25/paper/2.pdf | high |
| Industry treats RHDL = Basu’s project | https://www.minres.com/pipelined-riscv-in-rhdl/ | med-high |
| `rhdl-bits` ecosystem crate | https://docs.rs/rhdl-bits/latest/rhdl_bits/ | high |
| Legacy Ruby RHDL (dormant, same acronym) | https://fpga-faq.org/archives/105300.html | medium |
| Working patterns: coined names (kaze) or distinct brands (Spade / spade-lang.org) | kaze GitHub; https://spade-lang.org/ | high |

## Search risk

Query proxy for `RHDL rust` (2026-08-19) clusters on Basu lineage. New RHDL would lose SEO, look like a fork, inherit Ruby RHDL noise, and face “related to samitbasu/rhdl?” as the first question.

## Leads

- Prefer distinctive coinages over `R`+`HDL` acronyms.
- Internal monorepo nickname “rhdl” is OK only if publish name differs and README states unrelated.
- Do not plan to reclaim `rhdl` / rhdl.org.

## Gaps

- No Trends/SERP export; no trademark legal search; incomplete `rhdl-*` crate family enum.
