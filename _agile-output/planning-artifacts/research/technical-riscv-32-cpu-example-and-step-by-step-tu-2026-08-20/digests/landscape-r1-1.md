# Digest: landscape-r1-1

dimension: landscape-maturity
round: 1
assistant: 14fb2bd5-5c04-4822-a471-8ca1e37d4739
accessed: 2026-08-20

## Findings

- claim: PicoRV32 is an open HDL (Verilog) RV32IMC-capable CPU, configurable as RV32E/I/IC/IM/IMC, shipped as a single-file core with native/AXI4-Lite/Wishbone variants; maturity signals include ~4340 GitHub stars and long-lived ISC-licensed docs (created 2015-06).
  source: https://github.com/YosysHQ/picorv32
  publisher: YosysHQ / Clifford Wolf lineage
  pub_date: unknown
  confidence: high
  class: landscape

- claim: SERV is an open HDL bit-serial RV32I core (optional C/M/Zicsr), positioned as “world’s smallest” RISC-V CPU; Servant is a minimal FPGA reference platform.
  source: https://serv.readthedocs.io/en/latest/
  publisher: Olof Kindgren / SERV project
  pub_date: unknown
  confidence: high
  class: landscape

- claim: SERV states it is verified with RISC-V compliance tests for RV32I via RISCOF.
  source: https://github.com/olofk/serv
  publisher: Olof Kindgren / SERV project
  pub_date: unknown
  confidence: medium
  class: maturity

- claim: VexRiscv is an open SpinalHDL FPGA-oriented pipelined RV32I[M][A][F[D]][C] core; last push observed 2026-02-11; ~3156 stars.
  source: https://github.com/SpinalHDL/VexRiscv
  publisher: SpinalHDL
  pub_date: 2026-02
  confidence: high
  class: landscape

- claim: VexiiRiscv is presented as VexRiscv’s successor, WIP, with step-by-step tutorial and Spike lock-step — churn at Linux-capable end.
  source: https://github.com/SpinalHDL/VexiiRiscv
  publisher: SpinalHDL
  pub_date: unknown
  confidence: high
  class: landscape

- claim: Spike (`riscv-isa-sim`) is a functional ISA simulator (not HDL); last push 2026-05-16.
  source: https://github.com/riscv-software-src/riscv-isa-sim
  publisher: riscv-software-src
  pub_date: 2026-05
  confidence: high
  class: landscape

- claim: `riscv-formal` is formal verification (RVFI + SymbiYosys), not a teaching CPU microarchitecture.
  source: https://github.com/YosysHQ/riscv-formal
  publisher: YosysHQ
  pub_date: unknown
  confidence: high
  class: landscape

- claim: Harris & Harris DDCA RISC-V Edition companion materials teach single-cycle → multicycle → pipelined RISC-V processors.
  source: https://pages.hmc.edu/harris/ddca/ddcarv.html
  publisher: Harvey Mudd College / Harris
  pub_date: unknown
  confidence: high
  class: landscape

- claim: Pedagogy consolidates on single→multi→pipe; community cores split size (PicoRV32/SERV) vs configurable pipeline (VexRiscv) with succession churn toward VexiiRiscv.
  source: https://pages.hmc.edu/harris/class/e85/DDCArv_Ch7.pdf
  publisher: HMC/Harris et al. (composite)
  pub_date: mixed
  confidence: medium
  class: landscape

- claim: Current generation enables teaching via ratified specs (docs.riscv.org), RISCOF/arch-test, riscv-formal, and common RISC-V GCC packaging.
  source: https://riscv.org/technical/specifications/
  publisher: RISC-V International
  pub_date: 2026-08
  confidence: high
  class: other

- claim: RISCOF architectural tests are a minimal filter, not full DV.
  source: https://riscof.readthedocs.io/en/stable/intro.html
  publisher: RISC-V Software / RISCOF
  pub_date: unknown
  confidence: high
  class: maturity

## Leads
- PicoRV32 maintenance cadence
- Harris HDL.zip as teaching sequence
- riscv-arch-test RV32I as tutorial CI gate
- Avoid Linux-on-Vex as first tutorial scope

## Not found
- Authoritative curriculum market-share of teaching cores
- PicoRV32 last-push date in this run’s metadata
- Fresh ≤12mo comparative survey of teaching microarchitectures
