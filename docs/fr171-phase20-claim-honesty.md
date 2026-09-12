# FR171 — Phase 20 claim honesty gate

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 104 / FR171 closed** (Story **104.3**). Honesty surface delivered in Story **104.2**.

NFR14: `_agile-output/implementation-artifacts/nfr14-risk-epic104-phase20-claim-honesty-fr171.md`.

**Phase 20 规划故事已齐（Epic 99–104）**；宣称须引 **FR166–171**（本 FR / **NFR77**）。超出各 epic NFR14 钉死子集的加深仍 **NFR76**。**不得**宣称「NFR71 账本已空」。

## Rule (NFR77)

External claims about Phase 20 delivery **must** cite the matching closed FR:

| Claim | Must cite |
|-------|-----------|
| Phase 20 contract / gate | **FR166** |
| ChiselSim / multi IDE stores | **FR167** (≠ FR162 / FR134 alone; ≠ Phase 19 alone) |
| SPI·I2C·AXI handwritten FL | **FR168** (≠ FR163 alone; ≠ Phase 19 alone) |
| CIRCT multi-lower / allocation | **FR169** (≠ FR164 alone; firtool bump still **NFR76**) |
| update-mainline / HEAD Parser | **FR170** (≠ FR138 / FR165 alone; unpaired HEAD still **NFR76**) |
| Full Phase 20 claim set | **FR166–FR171** via this **FR171** honesty gate |

## Forbidden

- Phase 19 alone as “FR167–170 delivered” or “NFR71 four leftovers cleared”
- Marking undelivered FRs as done
- Silent expand of FR142 public API surface
- Implying beyond-NFR14 deepen is already cleared, or 「NFR71 账本已空」
- Treating `git push` as a product FR

## Closed deepen map (claim pointers)

| FR | Epic | Doc |
|----|------|-----|
| FR166 | 99 | Phase 20 gate (Correct Course 2026-09-12) |
| FR167 | 100 | [`fr167-chiselsim-ide-stores.md`](fr167-chiselsim-ide-stores.md) |
| FR168 | 101 | [`fr168-spi-i2c-axi-handwritten-fl.md`](fr168-spi-i2c-axi-handwritten-fl.md) |
| FR169 | 102 | [`fr169-circt-mlir-allocation.md`](fr169-circt-mlir-allocation.md) |
| FR170 | 103 | [`fr170-chisel-head-parser.md`](fr170-chisel-head-parser.md) |
| FR171 | 104 | this document |

## NFR76 leftovers (still need new contracts)

Examples: firtool bump beyond AD-9 without Chisel pairing; unpaired CIRCT/Chisel HEAD binaries; broader CIRCT/MLIR/sim suites beyond FR169 NFR14; deeper Parser/Chisel ecosystem beyond FR170 NFR14.

## Brand

Public brand **Bitloom**; design crates depend only on **`bitloom-prelude`**.
