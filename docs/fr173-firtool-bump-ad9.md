# FR173 — firtool bump beyond AD-9 (paired Chisel)

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 106 / FR173 closed** (Story **106.3**). Product pin: **Chisel 7.15.0 ↔ firtool-1.158.0** with **AD-9 / Stack revised** (NFR80).

Phase 20 **FR169(A)** multi-lower @ then-current `firtool-1.155.0`, **FR164** sim, and **FR137** compile **remain closed and valid** (NFR78). Those closes alone ≠ FR173.

## Selected shape (NFR14)

| Layer | Role |
|-------|------|
| **Prior pin** | Chisel **7.14.0** ↔ firtool-**1.155.0** (AD-9 until FR173) |
| **FR173 pin** | Chisel **7.15.0** ↔ firtool-**1.158.0** (official Chisel versioning table) |
| **AD-9 / Stack** | Must revise before claiming bump (NFR80) — done in Story 106.2 |
| **Evidence** | https://www.chisel-lang.org/docs/appendix/versioning |

**Forbidden closes:** FR169 alone; FR164 alone; FR137 alone; PATH-random firtool; unpaired CIRCT HEAD; docs-only; silent-Ok on version mismatch.

## Reproducible steps

```bash
cargo bitloom firtool info      # → version=1.158.0
cargo bitloom firtool ensure    # download + sha256 verify firrtl-bin-linux-x64 @ firtool-1.158.0
just circt-external-check
just circt-external-sim-check
just circt-external-alloc-check
just parser-restore-check
```

CI jobs that ensure pinned firtool must resolve **1.158.0** (no `continue-on-error` on version mismatch).

## Standing honesty

CIRCT/firtool runtime must **not** enter `bitloom-prelude` design-crate deps.
Further **paired** bumps beyond 1.158.0 / 7.15.0 require upstream pairing + AD-9 revise.
Live AD-9 product pin later moved to **firtool-1.159.0** (Chisel still 7.15.0) via **FR182** unpaired product-pin exception (**NFR85**); FR173 close @ 1.158.0 remains valid (**NFR83**).
Unpaired HEAD binaries remain Epic **107 / FR174** (≠ this FR).

```text
cargo test -p bitloom --test fr173_firtool_bump_ad9
```
