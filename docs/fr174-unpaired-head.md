# FR174 — Unpaired CIRCT / Chisel HEAD (document-pinned unpaired mainline)

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 107 / FR174 closed** (Story **107.3**). Product path: document-pinned unpaired mainline **firtool-1.156.0** (`just circt-unpaired-head-check`; ≠ AD-9 **1.158.0**).

Phase 20 **FR170** update-mainline Parser and Phase 21 **FR173** paired AD-9 pin (**firtool-1.158.0 ↔ Chisel 7.15.0**) **remain closed and valid** (NFR78). Those closes alone ≠ FR174.

## Selected shape (NFR14)

| Layer | Role |
|-------|------|
| **AD-9 product pin** | firtool-**1.158.0** ↔ Chisel **7.15.0** (default `cargo bitloom firtool ensure`) |
| **FR174 unpaired pin** | Document-pinned unpaired mainline **firtool-1.156.0** (≠ product pin; historically never Chisel-paired in Bitloom Stack) |
| **Override** | `BITLOOM_FIRTOOL_HEAD_PATH` → directory containing `firtool` (must still report **1.156.0**) |
| **AD revise** | AD-9 optional HEAD/mainline channel; AD-27 notes unpaired channel ≠ FR170 alone |

**Forbidden closes:** FR170 alone; FR138 alone; FR165 alone; FR173 alone; PATH-random firtool; using AD-9 product binary as “HEAD”; docs-only; silent-Ok under `BITLOOM_FIRTOOL_HEAD_FORCE_MISSING`.

## Reproducible steps

```bash
just circt-unpaired-head-check
# → bash scripts/circt-unpaired-head-check.sh
# downloads/caches firtool-1.156.0 under firtool-head cache (or uses BITLOOM_FIRTOOL_HEAD_PATH)

BITLOOM_FIRTOOL_HEAD_FORCE_MISSING=1 bash scripts/circt-unpaired-head-check.sh
# → non-zero; refusing silent success
```

CI: required job **`circt-unpaired-head`** (no `continue-on-error`).

## Standing honesty

Default product path remains AD-9 **1.158.0**. FR174 does **not** replace the paired pin.
True floating CIRCT/Chisel git HEAD beyond document-pinned **1.156.0** still needs a new contract (**NFR81**).
CIRCT/firtool runtime must **not** enter `bitloom-prelude` design-crate deps.

```text
cargo test -p bitloom --test fr174_unpaired_head
just circt-unpaired-head-check
```
