# Digest: crates.io availability probe — r1 (lead)

**Method:** `GET https://crates.io/api/v1/crates/<name>` with browser-like User-Agent · accessed 2026-08-19  
**Note:** bare curl without UA returned HTTP 403.

## Table

| name | status | notes |
|------|--------|-------|
| rhdl | TAKEN | samitbasu lineage |
| rhdl-bits | TAKEN | support crate |
| rhdl-rs | FREE | previous planned CLI publish name |
| rust-hdl | TAKEN | predecessor |
| kaze | TAKEN | Rust eDSL HDL |
| rtlrs | FREE | |
| firrtl-rs | FREE | FIRRTL signal — may confuse with CIRCT |
| yoshdl | FREE | Yosys-adjacent coinage |
| bitwire | FREE | |
| spinedl | FREE | |
| chillrtl | FREE | |
| circt-rs | FREE | CIRCT association risk |
| hardwire-rs | FREE | |
| rtlgen | FREE | generic |
| elabrs | FREE | elaborate signal |
| elabors | FREE | |
| frozenhir | FREE | project-specific IR term |
| gated | FREE | very generic English |
| netlist-rs | FREE | |
| myhdl-rs | FREE | MyHDL association |
| literal-rs | FREE | |
| rhdl2 | FREE | still RHDL-branded |
| spinrtl | FREE | |
| firrs | FREE | |

## Findings

1. **Hard gate confirmed:** `rhdl` and `rhdl-bits` are taken on crates.io (owner samitbasu per public crate pages).  
2. **`rhdl-rs` is free** — viable publish name but still carries “rhdl” search collision with the established project.  
3. Multiple distinctive FREE names exist (elabrs, spinrtl, bitwire, yoshdl, frozenhir, …).

## Gaps

- Availability is instantaneous; claim before publish (race).  
- Trademark/search engine ranking not measured here.
