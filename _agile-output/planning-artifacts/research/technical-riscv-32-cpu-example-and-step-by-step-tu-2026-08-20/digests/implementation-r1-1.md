### Findings

1. **Progressive RV32I (no Linux/MMU) is the dominant teaching shape; 1–2 semester packaging is the stated course cadence.** RVfpga is documented as typically two semesters (use/program SoC → microarchitecture/hazards), with labs 1–10 junior/senior and 11–20 upper-division/master’s; expected digital-design + architecture prerequisites. Barriers called out: toolchain/SoC piece-gathering and extending a non-trivial commercial core—not “build a toy ISA.”  
   claim/source: ASEE paper “RVfpga: Computer Architecture Course and MOOC…” (Harris et al.) / ASEE / ~2023 (MOOC “by summer 2023”) / accessed:2026-08-20 / confidence: high / class: learning-curve

2. **Successful “build a core” tutorials change one thing at a time and run every step in sim + (recommended) FPGA; Episode II defers pipeline/hazards; interrupts/priv ISA marked WIP.** FemtoRV “From Blinker to RISC-V”: Steps 1–3 blink/ROM → Step 4 decoder → … → Step 20 GNU asm → Step 21 C binaries; Verilator introduced at Step 19 after Mandelbrot; Icarus sufficient earlier; author explicitly warns against device-only development.  
   claim/source: BrunoLevy/learn-fpga `FemtoRV/TUTORIALS/FROM_BLINKER_TO_RISCV/README.md` / Bruno Levy / tutorial logbook era 2020–2022 (repo pushed_at 2025-11-18) / accessed:2026-08-20 / confidence: high / class: tutorial-packaging

3. **Course monorepos package numbered labs that share one verification spine; first “real” programs appear at single-cycle (fibonacci/quicksort), not at SoC.** ca2025-mycpu: `0-minimal` → `1-single-cycle` → traps → pipeline variants → `4-soc`; ChiselTest + RISCOF; Verilator via Make/SBT; MIT license.  
   claim/source: sysprog21/ca2025-mycpu README / NTU sysprog21 / course year CA2025 / accessed:2026-08-20 / confidence: high / class: integration

4. **Verification stack seen in teaching/minimal-core repos (this run): Verilator ≫ cocotb/ChiselTest ≫ riscv-tests or RISCOF/riscv-arch-test ≫ riscv-formal (reference/advanced).**  
   - RVfpga: Verilator backends (Trace/ViDBo/Pipeline) + Whisper ISS; board optional.  
   - PicoRV32: Makefile `tests/` from riscv-tests; Icarus default benches; RVFI/`riscv-formal` bindings.  
   - ca2025-mycpu: ChiselTest + `make compliance` (RISCOF).  
   - NEORV32: CI badge for riscv-arch-test.  
   - Hobby teaching SV cores: cocotb+Verilator (+ sometimes riscv-tests).  
   claim/sources: ASEE RVfpga PDF; YosysHQ/picorv32 README; ca2025-mycpu README; stnolting/neorv32 README badges; YosysHQ/riscv-formal quickstart / various / 2015–2026 / accessed:2026-08-20 / confidence: high / class: tooling

5. **Generator-style eDSL packaging is real and shippable without teaching the HDL of the core:** SpinalHDL VexRiscv — `sbt "runMain …Gen*"` → `VexRiscv.v`, Murax SoC demos, Verilator regression makefile (`RUN_HEX`/`TRACE`); MIT; ~3232★, pushed_at 2026-02-11, open_issues 130. Good for “use a softcore + firmware,” weaker as “write the datapath yourself.”  
   claim/source: SpinalHDL/VexRiscv README + GitHub API / SpinalHDL / ongoing / accessed:2026-08-20 / confidence: high / class: integration

6. **Docs packaging patterns observed:** (a) single long progressive Markdown tutorial in-repo (learn-fpga); (b) numbered lab directories + top-level Make (ca2025); (c) downloadable lab package + PlatformIO/VSCode chapters (RVfpga); (d) all-in-one CPU+SoC+SW + hosted user guide / AsciiDoc site + Vivado IP packaging (NEORV32). Cargo-style flows for *building the soft CPU* were not evidenced as the teaching-package norm; Make/SBT dominate. Rust appears mainly as *software* on the core (e.g. VexRiscv badge writeup), not as the primary teaching HDL for RV32 cores in this sample.  
   claim/sources: learn-fpga tutorial; ca2025 README; Digikey/RVfpga webinar PDF; neorv32 GitHub + docs links; craigjb.com 2020-01-22 / various / accessed:2026-08-20 / confidence: medium-high / class: integration

7. **Ecosystem health metrics (cite pages, not vibes) for pedagogical reference selection:**  
   | Core / package | Stars | Last push (API) | Open issues | License | Notes |  
   |---|---|---|---|---|---|  
   | PicoRV32 | 4343 | 2026-07-31 | 87 | ISC | High reuse + formal story; README notes IRQ features **do not follow** RISC-V IRQ conventions |  
   | learn-fpga | 3642 | 2025-11-18 | 58 | BSD-3 | Teaching-first; Episode III interrupts WIP |  
   | VexRiscv | 3232 | 2026-02-11 | 130 | MIT | Generator/plugin complexity; issue backlog elevated |  
   | NEORV32 | 2238 | 2026-08-19 | 11 | BSD-3 | Active; CI arch-tests; beginner+SoC oriented |  
   Red flags evidenced: large open-issue counts relative to push cadence (VexRiscv 130); intentional non-spec features that students may treat as “golden” (PicoRV32 IRQ note); incomplete advanced chapters (learn-fpga Episode III WIP).  
   claim/source: GitHub API + project READMEs / GitHub / accessed:2026-08-20 / confidence: high / class: ecosystem

8. **Feasible ship path consistent with evidence (not Linux SoC unless forced):** Standalone teaching repo (or numbered monorepo labs) with (1) blinker→minimal RV32I steps, (2) Verilator/Icarus from day one, (3) first hand-assembled then GNU C binary once load/store + MMIO exist (~learn-fpga step 20–21 pattern), (4) pipeline as Episode II, (5) riscv-tests or RISCOF gate before claiming “ISA complete,” (6) optional riscv-formal/RVFI for the reference core only. Prefer permissive, actively pushed, low-issue packages when *emulating* a reference; prefer FemtoRV/ca2025-style progression when *authoring* the tutorial core.  
   claim/source: synthesis of findings 1–7 / this run / accessed:2026-08-20 / confidence: medium / class: other

### Leads worth chasing
- TechRxiv “The RISC-V FPGA (RVfpga) Teaching Package” (full text) for assessment outcomes / tool install failure modes.
- NEORV32 user guide “Application Makefile” + riscv-arch-test CI workflow YAML (exact suite scope).
- RISC-KC/basic_rv32s `guidelines/tutorials.md` (listed on RISC-V Learn as intermediate; fetch was thin via GitHub HTML).
- Whether any Rust-HDL eDSL ships a comparable RV32 *tutorial core* (rust-hdl docs this run = blinky/workflow only).

### Looked for but not found
- Longitudinal “6–12 months into teaching a soft CPU” instructor retrospectives with persistent vs fixed pain points (only setup/access barriers and progressive-lab design, not time-horizon surveys).
- Widespread cargo/crates packaging of *teaching RTL cores* as the primary student flow (Make/SBT/PlatformIO dominate this sample).
- Teaching repos routinely shipping riscv-arch-test *and* cocotb *and* formal together as the default beginner path (combinations appear, not the full stack).