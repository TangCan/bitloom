# FR180 — Handshake dialect deepen (fork + join beyond FR129)

**Product:** Bitloom (`cargo bitloom` / `bitloom::hls`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 113 / FR180 closed** (Story **113.3**). Product path: Handshake dialect deepen with **`handshake.fork` + `handshake.join`** beyond FR129 C1–C4 (`schedule_circt_handshake_deepen` / `--circt-handshake-deepen`).

Phase 15 **FR129** CIRCT Handshake C1–C4 and Phase 21 **FR175** `--ir-sv`/`--ir-verilog` **remain closed and valid** (NFR83). Those closes alone ≠ FR180.

## Selected shape (NFR14)

| Layer | Role |
|-------|------|
| **FR129 C1–C4** | `handshake.func` / `handshake.buffer` + multi-clock elastic (`clock_domains≥2`) |
| **FR180 deepen** | Same elastic gates **plus** `handshake.fork` + `handshake.join` + `"fr180": true` |
| **API / CLI** | `schedule_circt_handshake_deepen` / `meets_fr180_handshake_deepen`; `--circt-handshake-deepen` |
| **AD revise** | AD-25 FR180 deepen channel (NFR85) |
| **Force missing** | `BITLOOM_HANDSHAKE_DEEPEN_FORCE_MISSING=1` → non-zero readable failure |

**Forbidden closes:** FR129 alone; FR175 alone; FR121 alone; docs-only; unrevised AD-25; silent-Ok under FORCE_MISSING.

## Reproducible steps

```bash
cargo bitloom hls --circt-handshake-deepen --function hs_deepen --dataflow add1 \
  --channels 1 --clock-domains 2 --elastic-depth 2

BITLOOM_HANDSHAKE_DEEPEN_FORCE_MISSING=1 cargo bitloom hls --circt-handshake-deepen ...
# → non-zero; refusing silent success
```

## Standing honesty

FR129 `--circt-handshake` path remains regressable. Handshake lower deepen beyond fork+join (`handshake.branch`+`handshake.merge`) is **FR187** / Epic 120 (Phase 23). Fuller CIRCT Handshake lower suite beyond that NFR14 subset still needs a new contract (**NFR91**).

```text
cargo test -p bitloom --test fr180_handshake_dialect_deepen
```
