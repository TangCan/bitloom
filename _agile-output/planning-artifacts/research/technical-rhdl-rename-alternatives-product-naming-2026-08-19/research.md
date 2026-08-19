---
title: 'technical research: RHDL rename alternatives / product naming'
type: 'technical'
topic: 'RHDL rename alternatives / product naming'
decision: 'Whether and how to replace the RHDL name (product/repo/crates.io/CLI)'
source: 'native-run'
status: complete
decision_locked: 'Bitloom'
preset: 'standard'
validation: 'normal'
created: '2026-08-19'
updated: '2026-08-19'
---

# technical research: RHDL rename alternatives / product naming

**Decision this research serves:** Whether and how to replace the RHDL name (product/repo/crates.io/CLI)

## Decision locked (2026-08-19)

**Public brand: Bitloom.** crates.io / CLI package & binary: **`bitloom`**. Never publish `rhdl` / `rhdl-bits`. Do not use `rhdl-rs` as the publish name. README and crates.io description must state unrelated to `samitbasu/rhdl`. Repo path may remain `rhdl` until a dedicated rename story; public surfaces use Bitloom.

## Executive summary

**Rename the public product away from “RHDL.”** Keep crates.io off `rhdl` / `rhdl-bits` forever; do not plan to reclaim those names.[1][2][3]

In the same niche (Rust → RTL / FPGA HDL), **samitbasu/rhdl** already owns GitHub branding, crates.io `rhdl` (+ `rhdl-bits`), **rhdl.org**, a LATTE paper titled around RHDL, and industry writeups. Search for “RHDL rust” surfaces that lineage first; a second RHDL pays permanent SEO / impersonation tax and answers “are you related?” on every touchpoint.[1][4][5]

crates.io is FCFS: yank does not free a name; transfers are owner-to-owner with **no team mediation** — pick a new name.[2][3][6]

**Chosen path (locked):** Bitloom / `bitloom` for display, package, and CLI.[7]  
Rename **before** first publish; align closeout runbook to `bitloom` (not `rhdl-rs`).[7][8][10]

## Hard gates (all must pass)

| Gate | Result | Evidence |
|------|--------|----------|
| Never publish crates.io `rhdl` / `rhdl-bits` | **Pass by design** — names taken | [2] |
| Lower confusion with samitbasu/rhdl | **Fails if product = “RHDL”** | [1][4][5] |
| crates.io package available (or clear suffix strategy) | **Pass** for shortlist + `rhdl-rs` (404 as of 2026-08-19); re-check at publish | [2][7] |
| Pronounceable CLI | **Pass** for Bitloom / Hirfrost / Gateknit / Wireelab / Rulite / spinedl | [7] |
| No obvious major HDL trademark collision | **Pass** on heuristic (no TM search) | [7]; gap |

## Landscape

**Incumbent RHDL (Rust):** samitbasu/rhdl is an active eDSL / “just Rust” subset story, successor to rust-hdl; crates and docs.rs ecosystem include `rhdl-bits`; academic and industry materials brand that project as RHDL.[1][4][5]

**Legacy RHDL (Ruby):** older Ruby HDL projects share the acronym — extra SERP noise for unqualified “RHDL.”[1][9]

**Names that work in this space:** coined/evocative (kaze) or distinct language brands (Spade / spade-lang.org), not another `R*HDL` acronym.[1]

## crates.io availability & policy

Taken (do not use): `rhdl`, `rhdl-bits`, `rust-hdl`, `kaze`, `netforge` (unrelated domain).[2][7]

Free (404, UA-required API, 2026-08-19) among candidates: `rhdl-rs`, `bitloom`, `hirfrost`, `gateknit`, `wireelab`, `rulite`, `spinedl`, `frozenhir`, `chillrtl`, `bitwire`, `elabrs`, …[2][7]

Policy implications: first publish locks identity; name squatting banned; do not expect crates.io staff to hand over `rhdl`.[3][6]

## Candidate shortlist (select)

Weights: distinctiveness 30% · searchability 25% · short CLI 20% · semantic fit 15% · low obvious TM collision 10%.[7]

| Rank | Brand | Package / bin | Score | Why |
|------|-------|---------------|-------|-----|
| 1 | **Bitloom** | `bitloom` | ~8.3 | Coined, short, searchable token, free |
| 2 | **Hirfrost** | `hirfrost` | ~7.7 | Best FrozenHir semantic fit |
| 3 | **Gateknit** | `gateknit` | ~7.3 | Gate/netlist metaphor; free (confirmed) |
| 4 | **Wireelab** | `wireelab` | ~7.1 | Elaborate-flow cue; slightly clunky |
| 5 | **Rulite** | `rulite` | ~6.6 | Shortest; weaker meaning |

**Spine-adjacent free tokens** (from availability pass, not fully scored): `spinedl`, `frozenhir`, `chillrtl`, `bitspine`.[2]

**Continuity option:** display rename + package `rhdl-rs` — crates-safe but still “rhdl”-adjacent in search.[2][7]

## Rename cost

| Timing | Cost |
|--------|------|
| Before first crates.io publish / few external links | ≈ weekend: repo, docs, AGENTS, Cargo package names |
| After publish | New package + optional deprecated shim + CLI forwarder + dual docs window |
| After adopters / papers | SEO + citation long tail; dual-name tax |

Act before second external adopter.[7][8]

## Cross-dimension insights

- Landscape and crates.io agree: **RHDL as product name is already taken in mindshare and registry.**[1][2]
- Closeout research assumed publish name `rhdl-rs`; naming research says that is **technically valid but strategically weak** if the goal is clean product identity — update the publish target when a brand is chosen.[2][10]
- Internal path/crate prefixes can stay `rhdl` temporarily only if public surfaces (README title, crates.io, CLI `--help` brand, paper titles) use the new name.

## Recommendations

| # | Recommendation | Confidence | Downstream |
|---|----------------|------------|------------|
| N1 | ~~Decide brand~~ → **locked Bitloom / `bitloom`** (2026-08-19) | high | done |
| N2 | Re-probe crates.io + GitHub/org/domain for `bitloom` immediately before first publish | high | Cargo.toml, CLI, docs |
| N3 | Never publish `rhdl` / `rhdl-bits`; README state **unrelated to samitbasu/rhdl** | high | policy / AGENTS |
| N4 | Rename public surfaces **before** first `cargo publish` | high | release runbook |
| N5 | Update closeout/publish research & stories: target package **`bitloom`** (not `rhdl-rs`) | high | closeout research R-path |
| N6 | Optional: light TM / domain check for Bitloom | medium | counsel / ops |

## Open questions (remaining)

1. Execute public-surface rename (Cargo package/bin, README, CLI help, AGENTS) this cycle — **yes, planned** once scheduled.
2. Repo directory / GitHub remote rename to `bitloom` this cycle, or keep path `rhdl` for now?

## Gaps

- No USPTO/EUIPO search; no GitHub org / domain availability matrix.
- SERP ranking is tool-proxy, not export.
- Live crates.io `/policies` HTML scrape incomplete; rely on RFCs + Cargo book + re-read at publish.[2]
- Availability is instantaneous — race until publish.

## Citations

[1] Digest `digests/naming-landscape-collisions-r1.md` — [Naming landscape](dd91bebe-7b67-48ca-a773-d58dbfd091fa); primary: https://github.com/samitbasu/rhdl · https://crates.io/crates/rhdl · https://capra.cs.cornell.edu/latte25/paper/2.pdf · https://www.minres.com/pipelined-riscv-in-rhdl/

[2] Digest `digests/crates-io-availability-policy-r1.md` + `digests/crates-io-availability-r1-1.md` — [crates.io availability](f3996350-e42f-45e8-82e2-cbcd03742b23); API probes 2026-08-19 (UA required)

[3] https://doc.rust-lang.org/cargo/reference/publishing.html · https://rust-lang.github.io/rfcs/3463-crates-io-policy-update.html

[4] https://crates.io/crates/rust-hdl · https://github.com/samitbasu/rust-hdl

[5] https://docs.rs/rhdl-bits/latest/rhdl_bits/

[6] https://rust-lang.github.io/rfcs/3646-remove-crate-transfer-mediation-policy.html

[7] Digest `digests/candidates-rename-cost-r1.md` — [Candidates + rename cost](a8320eac-21fb-4c16-b302-f3a1cf605d02); lead re-check bitloom/hirfrost/gateknit/… = 404

[8] Rename cost sources in candidates digest (framesmith, Remnic RENAME.md, forced-rename reporting)

[9] Legacy Ruby RHDL: https://fpga-faq.org/archives/105300.html · also RubyGems / davidsiaw & skryl RHDL projects (web search 2026-08-19)

[10] Sibling research: `_agile-output/planning-artifacts/research/technical-rhdl-clean-product-closeout-and-crates-i-2026-08-19/research.md`
