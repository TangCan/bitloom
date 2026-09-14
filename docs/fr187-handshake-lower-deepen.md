# FR187 — Handshake lower deepen (branch + merge beyond FR180)

**Product:** Bitloom (`cargo bitloom` / `bitloom::hls`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 120 / FR187 closed** (Story **120.3**). Product path: Handshake lower deepen with **`handshake.branch` + `handshake.merge`** beyond FR180 fork+join (`schedule_circt_handshake_lower_deepen` / `--circt-handshake-lower-deepen`).

Phase 22 **FR180** fork+join deepen, Phase 15 **FR129** C1–C4, and Phase 21 **FR175** `--ir-sv`/`--ir-verilog` **remain closed and valid** (NFR88). Those closes alone ≠ FR187.

## Selected shape (NFR14)

| Layer | Role |
|-------|------|
| **FR129 C1–C4** | `handshake.func` / `handshake.buffer` + multi-clock elastic (`clock_domains≥2`) |
| **FR180 deepen** | Same elastic gates **plus** `handshake.fork` + `handshake.join` + `"fr180": true` |
| **FR187 lower deepen** | Same + **`handshake.branch` + `handshake.merge`** + `"fr187": true` (keeps fork/join markers) |
| **API / CLI** | `schedule_circt_handshake_lower_deepen` / `meets_fr187_handshake_lower_deepen`; `--circt-handshake-lower-deepen` |
| **AD revise** | AD-25 FR187 lower deepen channel (NFR90) |
| **Force missing** | `BITLOOM_HANDSHAKE_LOWER_FORCE_MISSING=1` → non-zero readable failure |

**Forbidden closes:** FR180 alone; FR129 alone; FR175 alone; docs-only; unrevised AD-25; silent-Ok under FORCE_MISSING.

## Reproducible steps

```bash
cargo bitloom hls --circt-handshake-lower-deepen --function hs_lower --dataflow add1 \
  --channels 1 --clock-domains 2 --elastic-depth 2

BITLOOM_HANDSHAKE_LOWER_FORCE_MISSING=1 cargo bitloom hls --circt-handshake-lower-deepen ...
# → non-zero; refusing silent success
```

## Standing honesty

FR180 `--circt-handshake-deepen` and FR129 `--circt-handshake` remain regressable. Fuller CIRCT Handshake lower suite beyond this NFR14 subset still needs a new contract (**NFR91**).

```text
cargo test -p bitloom --test fr187_handshake_lower_deepen
```
