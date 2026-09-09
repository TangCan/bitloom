# Digest: Architecture patterns — round 1

agent: 3bf5ed68-a3a0-41f2-9ced-b24788b9c8ee
accessed: 2026-09-09

## Findings (claims with source|publisher|pub_date|accessed|confidence|class)

1. Mature Chisel IP is **generator-first**: a thin language core plus in-tree `chisel3.util` (interfaces + parametric FIFO/arbiter/etc.); large SoC IP lives **out-of-tree** (Rocket/BOOM-class generators), not as a fat stdlib. | chipsalliance/chisel README + HN Baaij/Chisel comparison | ~2019–2024 (README snapshot via GitHub; HN 2019-10) | 2026-09-09 | high | pattern
2. SpinalHDL mirrors that split: `spinal.lib` goals are **common generators, bus defs, Stream/Flow methodology, light peripherals/tools**—explicitly not “complete IP catalog,” and marked under-construction for depth. | SpinalDoc RTD Libraries | v1.8.0 docs | 2026-09-09 | high | pattern
3. Ecosystem pressure is to **extract frameworks from megaprojects**: Rocket Chip PRs split Diplomacy (and utils) into standalone libs so users can depend on parameterization/bus negotiation **without** pulling the whole core generator. | chipsalliance/rocket-chip PR #2741, #3571 | 2020–2024 | 2026-09-09 | high | pattern
4. Hand/vendor RTL in these stacks is typically **BlackBox/primitives + mapping**, not the preferred form of reusable library IP; reusable value is parameterized generators + negotiated interfaces (Diplomacy/TileLink style). | Chisel architecture notes (DeepWiki/README); Rocket Diplomacy notes | 2022–2024 | 2026-09-09 | medium | pattern
5. HDL LSPs that work at scale use **tiered/shallow analysis**: keystroke path = parse/symbols/shallow elaborations; full design hierarchy is opt-in / slower. HRT’s slang-server: “shallow compilation” so users need not select a top for core IDE features. | Hudson River Trading “Designing a SystemVerilog Language Server” | undated post (HRTBeat; content current as fetched) | 2026-09-09 | high | pattern
6. Community SV LSP architecture (sv-lsp): **fast CST tier (tree-sitter) + slow semantic tier (slang)** with query caching; deep type/param/UVM work deferred behind FFI maturity. | nktkt/sv-lsp README | undated (GitHub) | 2026-09-09 | medium | pattern
7. Verible LS defaults to **edited-file lint/syntax**; project-wide goto/refs need an explicit filelist—i.e. workspace wiring is a product feature, not automatic. | chipsalliance/verible `verilog/tools/ls/README` | undated | 2026-09-09 | high | pattern
8. eDSL “IDE” usually **piggybacks the host LSP**, not a custom netlist LSP: Chisel→Metals (fragile on Chipyard-scale monorepos / missing deps / cross-file gaps); Amaranth→Pyright via stubs; Spade (non-eDSL) invests in its own rudimentary LSP because it owns the language. | chipyard#986; amaranth-stubs; Spade arXiv | 2021 / 2024–2026 / 2023 | 2026-09-09 | medium–high | pattern
9. What is usually deferred for HDL/eDSL IDE: **full elaborations every keystroke**, deep semantic/UVM/macro fidelity, reliable monorepo cross-project navigation without build-system integration, and **hardware-semantic** (post-elaboration) nav vs host-language nav. | HRT shallow-compile; Verible filelist; Chipyard Metals issue; Spade contrast | mixed | 2026-09-09 | medium | gap
10. Dual-sim product pattern that dominates in Clash: **one behavioral/spec sim in the host (REPL/`sampleN`)**, then **compiler-generated HDL testbench** from the same stimuli/expected vectors (`stimuliGenerator` / `outputVerifier` / `TestBench` ANN)—explicitly to check HDL ≡ Haskell and for post-synth / power. | Clash Book test-bench tutorial; Clash FAQ | current Clash docs | 2026-09-09 | high | pattern
11. Clash FAQ taxonomy of peers: interactive native sim is rare among eDSLs; typical alternatives are **emit HDL then Verilator/VCS**, or **interpret IR** (historical Chisel/FIRRTL interpreter; Migen IR). Older Lava used **dual-embedding** (primitives carry host-eval functions). | Clash FAQ “difference vs Chisel/Spinal/Migen…” | current | 2026-09-09 | high | taxonomy
12. Current Chisel product path emphasizes **one peek/poke stimulus API over external cycle-accurate SV sims (ChiselSim→Verilator/VCS)** plus complementary FileCheck on emitted text—not a separate first-class “functional simulator product.” Verification libs (ChiselVerify) layer UVM-like agents **on the same ChiselTest/ChiselSim interface**. | Chisel Testing docs; ChiselVerify PDF; EECS-2021-132 | 2021 / current docs | 2026-09-09 | high | pattern
13. **Shared stimulus / generated TB** dominates over maintaining two independent testbench codebases; where “functional” exists, it is host eval or IR interpret for latency, with HDL sim as the accuracy/vendor gate. | Clash TB docs; Clash FAQ; ChiselSim docs | mixed | 2026-09-09 | medium–high | decision_relevant

## Leads

- chipsalliance/diplomacy + rocket-utils split history (packaging playbook for “stdlib vs megaproject”).
- SpinalHDL `spinal.lib` chapter set (what they consider “stdlib-complete enough”).
- HRT slang-server design post (shallow vs full design as productization recipe).
- Clash FAQ dual-embedding vs IR/Verilator matrix (positioning dual-view sims without boiling ocean).
- Spade LSP note as counterfactual: custom LSP pays when you own syntax; eDSLs usually shouldn’t.

## Looked for but not found

- Quantitative post-mortems on “stdlib IP depth vs adoption” (LOC/% reuse)—marketing/docs only.
- A rust-analyzer-grade **eDSL-netlist** LSP (Chisel/Spinal/Amaranth) with published architecture post-mortem.
- Clear industry winner metrics for dual-embedding vs generated-TB vs peek/poke-over-Verilator on QoR/dev-time (Clash FAQ states hard to quantify).

## Sources read (count)

**8 primary** (Chisel README/API+Testing; Spinal lib index; Clash FAQ + test-bench; HRT slang-server article; Verible LS README / sv-lsp; Rocket diplomacy PRs; ChiselVerify/EECS-2021-132 or Chipyard Metals / Spade / amaranth-stubs as supporting).
