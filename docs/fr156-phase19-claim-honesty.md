# FR156 — Phase 19 claim honesty gate

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 98 / FR156 closed** (Story **98.3**). Honesty surface delivered in Story **98.2**.

NFR14: `_agile-output/implementation-artifacts/nfr14-risk-epic98-phase19-claim-honesty-fr156.md`.

Phase 19 **规划故事已齐（Epic 87–98）**；宣称须引 **FR154–165**（本 FR）。超出各 epic NFR14 钉死子集的加深仍 **NFR71**。

## Rule (NFR72)

External claims about Phase 19 delivery **must** cite the matching closed FR:

| Claim | Must cite |
|-------|-----------|
| Phase 19 contract / gate | **FR154** |
| `bitloom-lsp` live on crates.io | **FR155** (≠ Phase 18 / FR152(b) alone) |
| NFR59 subset delivered | matching **FR157–FR165** |
| Full Phase 19 claim set | **FR154–FR165** via this **FR156** honesty gate |

## Forbidden

- Phase 18 alone as “lsp shipped” or “NFR59 cleared”
- Marking undelivered FRs as done
- Silent expand of FR142 public API surface
- Implying NFR71 leftover deepen is already cleared
- Treating `git push` as a product FR

## Closed deepen map (claim pointers)

| FR | Epic | Doc |
|----|------|-----|
| FR154 | 87 | Phase 19 gate (Correct Course 2026-09-12) |
| FR155 | 88 | [`fr152-bitloom-lsp-publish-policy.md`](fr152-bitloom-lsp-publish-policy.md) |
| FR157 | 89 | [`fr157-auto-fsm-labels.md`](fr157-auto-fsm-labels.md) |
| FR158 | 90 | [`fr158-third-party-lcov-gui.md`](fr158-third-party-lcov-gui.md) |
| FR159 | 91 | [`fr159-memread-full-emit.md`](fr159-memread-full-emit.md) |
| FR160 | 92 | [`fr160-non-cargo-path-scan.md`](fr160-non-cargo-path-scan.md) |
| FR161 | 93 | [`fr161-formal-sby-image-hygiene.md`](fr161-formal-sby-image-hygiene.md) |
| FR162 | 94 | [`fr162-deeper-gui-ide-default-wave.md`](fr162-deeper-gui-ide-default-wave.md) |
| FR163 | 95 | [`fr163-unlisted-protocol-handwritten-fl.md`](fr163-unlisted-protocol-handwritten-fl.md) |
| FR164 | 96 | [`fr164-circt-external-sim-gate.md`](fr164-circt-external-sim-gate.md) |
| FR165 | 97 | [`fr165-deeper-chisel-parser-ecosystem.md`](fr165-deeper-chisel-parser-ecosystem.md) |
| FR156 | 98 | this document |

## NFR71 leftovers (still need new contracts)

Examples: Chisel HEAD Parser migration; broader CIRCT/MLIR allocation; SPI/I2C/AXI handwritten FL; full ChiselSim / extra IDE store ports.

## Brand

Public brand **Bitloom**; design crates depend only on **`bitloom-prelude`**.
