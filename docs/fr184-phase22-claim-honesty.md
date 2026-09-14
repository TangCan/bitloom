# FR184 — Phase 22 claim honesty gate

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 117 / FR184 in progress** (Story **117.2** honesty surface). Closeout → Story **117.3**.

NFR14: `_agile-output/implementation-artifacts/nfr14-risk-epic117-phase22-claim-honesty-fr184.md`.

**Phase 22 规划故事已齐（Epic 111–117）**；加深面 FR179–183 **已关闭**；本诚实门关闭后宣称须引 **FR178–184**（本 FR / **NFR87**）。超出各 epic NFR14 钉死子集的加深仍 **NFR86**。**不得**宣称「NFR81 账本已空」。

## Rule (NFR87)

External claims about Phase 22 delivery **must** cite the matching closed FR:

| Claim | Must cite |
|-------|-----------|
| Phase 22 contract / gate | **FR178** |
| floating CIRCT git HEAD | **FR179** (≠ FR174 alone; ≠ Phase 21 alone) |
| Handshake dialect deepen | **FR180** (≠ FR129 / FR175 alone) |
| deeper Style Guide / linter | **FR181** (≠ FR176 alone) |
| unpaired firtool product-pin | **FR182** (≠ FR173 / FR174 / FR179 alone) |
| explicit FR142 surface expand | **FR183** (≠ FR142 alone; ≠ silent expand) |
| Full Phase 22 claim set | **FR178–FR184** via this **FR184** honesty gate |

## Forbidden

- Phase 21 alone as “FR179–183 delivered” or “NFR81 leftovers cleared”
- Marking undelivered FRs as done
- Implying beyond-NFR14 deepen is already cleared, or 「NFR81 账本已空」
- Treating `git push` as a product FR

## Closed deepen map (claim pointers)

| FR | Epic | Doc |
|----|------|-----|
| FR178 | 111 | Phase 22 gate (Correct Course 2026-09-12) |
| FR179 | 112 | [`fr179-floating-circt-git-head.md`](fr179-floating-circt-git-head.md) |
| FR180 | 113 | [`fr180-handshake-dialect-deepen.md`](fr180-handshake-dialect-deepen.md) |
| FR181 | 114 | [`fr181-deeper-style-guide-linter.md`](fr181-deeper-style-guide-linter.md) |
| FR182 | 115 | [`fr182-unpaired-firtool-product-pin.md`](fr182-unpaired-firtool-product-pin.md) |
| FR183 | 116 | [`fr183-explicit-fr142-api-expand.md`](fr183-explicit-fr142-api-expand.md) |
| FR184 | 117 | this document (honesty gate; close in Story **117.3**) |

## NFR86 leftovers (still need new contracts)

Beyond each Phase 22 epic NFR14 subset (e.g. unbounded live CIRCT tip; Handshake dialect/lower beyond fork+join; full community Style Guide 全家桶; further unpaired product-pin bumps; further public-API expands) still needs a **new** contract (**NFR86**). Do **not** claim 「NFR81 账本已空」.

## Brand

Public brand **Bitloom**; design crates depend only on **`bitloom-prelude`**.
