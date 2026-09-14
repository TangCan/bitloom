# FR179 — Floating CIRCT git HEAD (reproducible floating-track pin)

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 112 / FR179 closed** (Story **112.3**). Product path: document-pinned **floating-track** firtool-**1.159.0** (`just circt-floating-git-head-check`; ≠ AD-9 **1.158.0**; ≠ FR174 **1.156.0**).

Phase 21 **FR174** unpaired **1.156.0** and **FR173** paired AD-9 pin (**firtool-1.158.0 ↔ Chisel 7.15.0**) **remain closed and valid** (NFR83). Those closes alone ≠ FR179.

## Selected shape (NFR14)

| Layer | Role |
|-------|------|
| **AD-9 product pin** | firtool-**1.158.0** ↔ Chisel **7.15.0** (default `cargo bitloom firtool ensure`) |
| **FR174 unpaired pin** | Document-pinned unpaired mainline **firtool-1.156.0** |
| **FR179 floating-track pin** | Document-pinned floating CIRCT HEAD **track** **firtool-1.159.0** (readable version; beyond FR174; ≠ product pin) |
| **Override** | `BITLOOM_FIRTOOL_FLOATING_HEAD_PATH` → directory containing `firtool` (must still report **1.159.0**) |
| **AD revise** | AD-9 floating-track channel (NFR85); ≠ FR174 alone; ≠ FR182 unpaired product-pin bump |

**Honesty:** This is a **reproducible floating-track pin** (not PATH-random). Unbounded live tip without a document pin still needs a new contract (**NFR86**).

**Forbidden closes:** FR174 alone; FR173 alone; FR182 alone; PATH-random firtool; using AD-9 product binary or FR174 **1.156.0** as “floating HEAD”; docs-only; silent-Ok under `BITLOOM_FIRTOOL_FLOATING_HEAD_FORCE_MISSING`.

## Reproducible steps

```bash
just circt-floating-git-head-check
# → bash scripts/circt-floating-git-head-check.sh
# downloads/caches firtool-1.159.0 under firtool-floating-head cache (or uses BITLOOM_FIRTOOL_FLOATING_HEAD_PATH)

BITLOOM_FIRTOOL_FLOATING_HEAD_FORCE_MISSING=1 bash scripts/circt-floating-git-head-check.sh
# → non-zero; refusing silent success
```

CI: required job **`circt-floating-git-head`** (no `continue-on-error`).

## Standing honesty

Default product path remains AD-9 **1.158.0**. FR174 unpaired **1.156.0** remains a separate optional channel.
CIRCT/firtool runtime must **not** enter `bitloom-prelude` design-crate deps.

```text
cargo test -p bitloom --test fr179_floating_circt_git_head
just circt-floating-git-head-check
```
