# FR177 — Phase 21 claim honesty gate

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 110 / FR177 closed** (Story **110.3**). Honesty surface delivered in Story **110.2**.

NFR14: `_agile-output/implementation-artifacts/nfr14-risk-epic110-phase21-claim-honesty-fr177.md`.

**Phase 21 规划故事已齐（Epic 105–110）**；宣称须引 **FR172–177**（本 FR / **NFR82**）。超出各 epic NFR14 钉死子集的加深仍 **NFR81**。**不得**宣称「NFR76 账本已空」。

## Rule (NFR82)

External claims about Phase 21 delivery **must** cite the matching closed FR:

| Claim | Must cite |
|-------|-----------|
| Phase 21 contract / gate | **FR172** |
| firtool bump (paired AD-9) | **FR173** (≠ FR169 alone; ≠ Phase 20 alone) |
| unpaired CIRCT/Chisel HEAD | **FR174** (≠ FR170 alone; ≠ AD-9 product pin) |
| broader CIRCT SV / ir-verilog | **FR175** (≠ FR169 fir+hw alone) |
| deeper Parser/Chisel ecosystem | **FR176** (≠ FR170 / FR165 alone) |
| Full Phase 21 claim set | **FR172–FR177** via this **FR177** honesty gate |

## Forbidden

- Phase 20 alone as “FR173–176 delivered” or “NFR76 leftovers cleared”
- Marking undelivered FRs as done
- Silent expand of FR142 public API surface
- Implying beyond-NFR14 deepen is already cleared, or 「NFR76 账本已空」
- Treating `git push` as a product FR

## Closed deepen map (claim pointers)

| FR | Epic | Doc |
|----|------|-----|
| FR172 | 105 | Phase 21 gate (Correct Course 2026-09-12) |
| FR173 | 106 | [`fr173-firtool-bump-ad9.md`](fr173-firtool-bump-ad9.md) |
| FR174 | 107 | [`fr174-unpaired-head.md`](fr174-unpaired-head.md) |
| FR175 | 108 | [`fr175-broader-circt-mlir-sim.md`](fr175-broader-circt-mlir-sim.md) |
| FR176 | 109 | [`fr176-deeper-parser-chisel-ecosystem.md`](fr176-deeper-parser-chisel-ecosystem.md) |
| FR177 | 110 | this document |

## NFR81 leftovers (still need new contracts)

Examples historically listed here are now under **Phase 22** Correct Course（FR178–184 / Epic 111–117）：floating CIRCT git HEAD → **FR179**；Handshake dialect beyond FR129/FR175 → **FR180**；further Style Guide/linter beyond FR176 → **FR181**；firtool bumps without upstream Chisel pairing → **FR182**；explicit expand of public API surface (FR142) → **FR183**. See PRD addendum「Phase 22」and `sprint-change-proposal-2026-09-12-phase22-nfr81-leftovers.md`. Beyond each Phase 22 NFR14 subset still needs a **new** contract (**NFR86**).

## Brand

Public brand **Bitloom**; design crates depend only on **`bitloom-prelude`**.
