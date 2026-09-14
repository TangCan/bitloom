# FR182 — Unpaired firtool product-pin bump (AD-9 exception)

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 115 / FR182 closed** (Story **115.3**). Product path: AD-9 default product pin **firtool-1.159.0** with Chisel remaining **7.15.0** (**no** upstream official Chisel↔firtool pairing for 1.159.0). **AD-9 revised** with *unpaired product-pin* exception (**NFR85**).

Phase 21 **FR173** paired close (**firtool-1.158.0 ↔ Chisel 7.15.0**), **FR174** unpaired optional **1.156.0**, and **FR179** floating-track optional channel **1.159.0** **remain closed and valid** (NFR83). Those closes alone ≠ FR182.

## Selected shape (NFR14)

| Layer | Role |
|-------|------|
| **Prior product pin (FR173)** | Chisel **7.15.0** ↔ firtool-**1.158.0** (official pairing; close evidence retained) |
| **FR182 product pin** | firtool-**1.159.0** + Chisel **7.15.0** (**unpaired**; AD-9 exception) |
| **FR174 optional channel** | Document-pinned unpaired mainline **firtool-1.156.0** (≠ product pin) |
| **FR179 optional channel** | Document-pinned floating-track **firtool-1.159.0** via `firtool-floating-head` cache / `BITLOOM_FIRTOOL_FLOATING_HEAD_PATH` (≠ product *channel*; version may coincide) |
| **AD-9 / Stack** | Must revise *unpaired product-pin* exception before claiming bump (NFR85) — Story 115.2 |

**Forbidden closes:** FR173 alone; FR174 alone; FR179 alone; PATH-random firtool; treating FR174/FR179 optional channels as the default product pin; docs-only; silent-Ok on version mismatch / missing tool.

## Reproducible steps

```bash
cargo bitloom firtool info      # → version=1.159.0
cargo bitloom firtool ensure    # download + sha256 verify firrtl-bin-linux-x64 @ firtool-1.159.0
just circt-external-check
just circt-external-sim-check
just circt-external-alloc-check
just parser-restore-check
```

CI jobs that ensure the AD-9 product pin must resolve **1.159.0** (no `continue-on-error` on version mismatch).

## Standing honesty

CIRCT/firtool runtime must **not** enter `bitloom-prelude` design-crate deps.
Further bumps beyond this NFR14 subset require a new contract (**NFR86**).
FR179 floating-track remains a **separate channel** even when the readable version equals the product pin.
Do **not** claim FR173/FR174/FR179 alone deliver this FR.

```text
cargo test -p bitloom --test fr182_unpaired_firtool_product_pin
```
