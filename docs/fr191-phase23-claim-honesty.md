# FR191 — Phase 23 claim honesty gate

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 124 / FR191 closed** (Story **124.3**). Honesty surface delivered in Story **124.2**.

NFR14: `_agile-output/implementation-artifacts/nfr14-risk-epic124-phase23-claim-honesty-fr191.md`.

**Phase 23 规划故事已齐（Epic 118–124）**；宣称须引 **FR185–191**（本 FR / **NFR92**）。实现诚实：**FR186–188 / FR190 closed**；**FR189 deferred / 未交付**（Correct Course 延期关账 2026-09-14；**不得**宣称已升钉；后续 → **NFR91**）。超出各 epic NFR14 钉死子集的加深仍 **NFR91**。**不得**宣称「NFR86 账本已空」。Phase 12–22 + engineering closeout remain valid (**NFR88**).

**Phase 23 诚实结项（2026-09-14）：** Correct Course `sprint-change-proposal-2026-09-14-phase23-honest-closeout.md` / `phase23CloseoutStatus: complete`。证据：`just circt-live-tip-check` → tip **firtool-1.159.0**（与 FR182 产品钉重合）。**结项 ≠ FR189 已交付**——合同要求严格 **>1.159.0**；「已是最新」只说明上游暂无可升版本。可停 sprint；**不得**宣称 NFR86/NFR91 账本已空。

## Rule (NFR92)

External claims about Phase 23 delivery **must** cite the matching closed FR (or honest deferred/blocked):

| Claim | Must cite |
|-------|-----------|
| Phase 23 contract / gate | **FR185** |
| unbounded CIRCT live tip | **FR186** (≠ FR179 alone; ≠ Phase 22 alone) |
| Handshake lower deepen (`branch`+`merge`) | **FR187** (≠ FR180 alone; ≠ FR129 alone) |
| community Style Guide pack | **FR188** (≠ FR181 alone; ≠ FR176 alone) |
| further firtool product-pin | **FR189** — **deferred / 未交付** (Correct Course defer-close 2026-09-14); **must not** claim delivered; further bump → **NFR91** |
| further explicit FR142 expand | **FR190** (≠ FR183 alone; ≠ silent expand) |
| Full Phase 23 claim set | **FR185–FR191** via this **FR191** honesty gate (includes honest FR189 deferred) |

## Forbidden

- Phase 22 / engineering closeout alone as “FR186–190 delivered” or “NFR86 leftovers cleared”
- Phase 23 honest closeout alone as “FR189 delivered” or “product pin bumped above 1.159.0”
- Treating “live tip is latest (==1.159.0)” as FR189 acceptance
- Marking undelivered / deferred FRs (especially **FR189**) as done
- Implying beyond-NFR14 deepen is already cleared, or 「NFR86 账本已空」/ 「NFR91 账本已空」
- Treating `git push` as a product FR

## Claim map (pointers)

| FR | Epic | Status | Doc |
|----|------|--------|-----|
| FR185 | 118 | **closed** | Phase 23 gate (Correct Course 2026-09-14) |
| FR186 | 119 | **closed** | [`fr186-unbounded-circt-tip.md`](fr186-unbounded-circt-tip.md) |
| FR187 | 120 | **closed** | [`fr187-handshake-lower-deepen.md`](fr187-handshake-lower-deepen.md) |
| FR188 | 121 | **closed** | [`fr188-community-style-guide-pack.md`](fr188-community-style-guide-pack.md) |
| FR189 | 122 | **deferred / 未交付** | Correct Course: `sprint-change-proposal-2026-09-14-fr189-defer-close.md` — **not delivered**; → **NFR91** |
| FR190 | 123 | **closed** | [`fr190-further-fr142-api-expand.md`](fr190-further-fr142-api-expand.md) |
| FR191 | 124 | **closed** | this document |

## NFR91 leftovers

Beyond each Phase 23 epic NFR14 subset still needs a **new** contract (**NFR91**), including any future firtool product-pin bump beyond FR182 **1.159.0**. Closing FR191 does **not** empty the ledger. Do **not** claim 「NFR86 账本已空」.

## Brand

Public brand **Bitloom**; design crates depend only on **`bitloom-prelude`**.
