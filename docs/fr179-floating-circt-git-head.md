# FR179 — Floating CIRCT git HEAD (reproducible floating-track pin)

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 112 / FR179 closed** (Story **112.3**). Product path: document-pinned **floating-track** firtool-**1.159.0** (`just circt-floating-git-head-check`; ≠ AD-9 product *channel*; ≠ FR174 **1.156.0**). After **FR182**, AD-9 product pin is also **1.159.0** — distinguish by **cache/path** (`firtool-floating-head` / `BITLOOM_FIRTOOL_FLOATING_HEAD_PATH`), not version inequality alone.

Phase 21 **FR174** unpaired **1.156.0** and **FR173** paired AD-9 close (**firtool-1.158.0 ↔ Chisel 7.15.0**) **remain closed and valid** (NFR83). Those closes alone ≠ FR179. **FR182** unpaired product-pin alone ≠ FR179.

## Selected shape (NFR14)

| Layer | Role |
|-------|------|
| **AD-9 product pin** | firtool-**1.159.0** + Chisel **7.15.0** (FR182 unpaired; default `cargo bitloom firtool ensure` / AD-9 product cache) |
| **FR174 unpaired pin** | Document-pinned unpaired mainline **firtool-1.156.0** |
| **FR179 floating-track pin** | Document-pinned floating CIRCT HEAD **track** **firtool-1.159.0** (readable version; beyond FR174; **channel** ≠ product cache) |
| **Override** | `BITLOOM_FIRTOOL_FLOATING_HEAD_PATH` → directory containing `firtool` (must still report **1.159.0**; must not be the AD-9 product cache tree) |
| **AD revise** | AD-9 floating-track channel (NFR85); ≠ FR174 alone; ≠ FR182 unpaired product-pin bump alone |

**Honesty:** This is a **reproducible floating-track pin** (not PATH-random). Unbounded / live tip beyond this floating-track pin is **FR186** (`docs/fr186-unbounded-circt-tip.md`); alone ≠ FR186.

**Forbidden closes:** FR174 alone; FR173 alone; FR182 alone; PATH-random firtool; using AD-9 product **cache** or FR174 **1.156.0** as “floating HEAD”; docs-only; silent-Ok under `BITLOOM_FIRTOOL_FLOATING_HEAD_FORCE_MISSING`.

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

Default product path remains AD-9 **1.159.0** (product cache). FR174 unpaired **1.156.0** remains a separate optional channel.
FR179 floating-track remains a separate **channel** even when the readable version equals the product pin.
CIRCT/firtool runtime must **not** enter `bitloom-prelude` design-crate deps.

```text
cargo test -p bitloom --test fr179_floating_circt_git_head
just circt-floating-git-head-check
```
