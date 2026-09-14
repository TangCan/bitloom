# FR191 — Phase 23 claim honesty gate

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 124 / FR191 closed** (Story **124.3**). Honesty surface delivered in Story **124.2**.

NFR14: `_agile-output/implementation-artifacts/nfr14-risk-epic124-phase23-claim-honesty-fr191.md`.

**Phase 23 规划故事已齐（Epic 118–124）**；宣称须引 **FR185–191**（本 FR / **NFR92**）。实现诚实：**FR186–188 / FR190 closed**；**FR189 blocked-upstream**（**不得**宣称已交付）。超出各 epic NFR14 钉死子集的加深仍 **NFR91**。**不得**宣称「NFR86 账本已空」。Phase 12–22 + engineering closeout remain valid (**NFR88**).

## Rule (NFR92)

External claims about Phase 23 delivery **must** cite the matching closed FR (or honest blocked):

| Claim | Must cite |
|-------|-----------|
| Phase 23 contract / gate | **FR185** |
| unbounded CIRCT live tip | **FR186** (≠ FR179 alone; ≠ Phase 22 alone) |
| Handshake lower deepen (`branch`+`merge`) | **FR187** (≠ FR180 alone; ≠ FR129 alone) |
| community Style Guide pack | **FR188** (≠ FR181 alone; ≠ FR176 alone) |
| further firtool product-pin | **FR189** — **blocked-upstream / parked** (Correct Course A 2026-09-14; no published firtool >1.159.0); **must not** claim delivered |
| further explicit FR142 expand | **FR190** (≠ FR183 alone; ≠ silent expand) |
| Full Phase 23 claim set | **FR185–FR191** via this **FR191** honesty gate (includes honest FR189 blocked) |

## Forbidden

- Phase 22 / engineering closeout alone as “FR186–190 delivered” or “NFR86 leftovers cleared”
- Marking undelivered / blocked FRs (especially **FR189**) as done
- Implying beyond-NFR14 deepen is already cleared, or 「NFR86 账本已空」
- Treating `git push` as a product FR

## Claim map (pointers)

| FR | Epic | Status | Doc |
|----|------|--------|-----|
| FR185 | 118 | **closed** | Phase 23 gate (Correct Course 2026-09-14) |
| FR186 | 119 | **closed** | [`fr186-unbounded-circt-tip.md`](fr186-unbounded-circt-tip.md) |
| FR187 | 120 | **closed** | [`fr187-handshake-lower-deepen.md`](fr187-handshake-lower-deepen.md) |
| FR188 | 121 | **closed** | [`fr188-community-style-guide-pack.md`](fr188-community-style-guide-pack.md) |
| FR189 | 122 | **blocked-upstream / parked** | Correct Course **A** approved 2026-09-14: `sprint-change-proposal-2026-09-14-fr189-upstream-block.md` — **not delivered** |
| FR190 | 123 | **closed** | [`fr190-further-fr142-api-expand.md`](fr190-further-fr142-api-expand.md) |
| FR191 | 124 | **closed** | this document |

## NFR91 leftovers

Beyond each Phase 23 epic NFR14 subset still needs a **new** contract (**NFR91**). Closing FR191 does **not** empty the NFR86 / beyond-subset ledger. Do **not** claim 「NFR86 账本已空」.

## Brand

Public brand **Bitloom**; design crates depend only on **`bitloom-prelude`**.
