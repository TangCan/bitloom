# FR169 — Broader CIRCT/MLIR allocation / multi-lower (beyond FR164)

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 102 / FR169 in progress** (Story **102.2** implementation). Story **102.3** closeout pending.

Phase 19 **FR164** external sim gate and **FR137** compile gate **remain closed and valid** (NFR73).
This FR delivers NFR14 option **(A)** — multi-lower / HW dialect allocation product path at the **same AD-9 pin** (`firtool-1.155.0`). Option **(B)** firtool bump is **not** selected in this story (requires Chisel formal pairing + AD-9 revise).

## Selected shape (NFR14 · A)

| Layer | Role |
|-------|------|
| **FR137** | External firtool **compile** → Verilog — **still valid**; alone ≠ FR169 |
| **FR164** | External firtool compile + **cycle-sim** predicate — **still valid**; alone ≠ FR169 |
| **FR129** | In-tree Handshake markers — **still valid**; alone ≠ FR169 |
| **FR169 (A)** | External firtool **multi-lower**: `--ir-fir` + `--ir-hw` (HW dialect allocation) + Verilog |

**Forbidden closes:** FR164 alone; FR137 alone; FR129 alone; PATH-random firtool; unpaired CIRCT HEAD; docs-only; silent-Ok under `BITLOOM_CIRCT_ALLOC_FORCE_MISSING`.

## Reproducible steps

```bash
just circt-external-alloc-check
# → bash scripts/circt-external-alloc-check.sh

BITLOOM_CIRCT_ALLOC_FORCE_MISSING=1 bash scripts/circt-external-alloc-check.sh
# → non-zero; refusing silent success
```

CI: required job **`circt-external-alloc`** (no `continue-on-error`).

Artifacts under `target/circt-external-alloc-check/`:
- `fr169_alloc.ir-fir.mlir` — FIR dialect IR
- `fr169_alloc.ir-hw.mlir` — HW dialect (`hw.module`) allocation evidence
- `fr169_alloc.firtool.v` — Verilog emit

## Standing honesty

CIRCT/firtool runtime must **not** enter `bitloom-prelude` design-crate deps.
firtool bump beyond 1.155.0 remains deferred until Chisel pairing + AD-9 revise (**NFR76** / option B).

```text
cargo test -p bitloom --test fr169_circt_mlir_allocation
just circt-external-alloc-check
```
