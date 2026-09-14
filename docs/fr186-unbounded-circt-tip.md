# FR186 — Unbounded / live CIRCT tip (beyond FR179 floating-track pin)

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 119 / FR186 closed** (Story **119.3**). Product path: **live tip** channel that resolves the latest GitHub `firtool-*` release (readable tip identity) via `just circt-live-tip-check` / CI `circt-live-tip`. **Not** the AD-9 product default pin. **Not** FR179 floating-track alone.

Phase 22 **FR179** floating-track **firtool-1.159.0** and **FR182** product pin **1.159.0** **remain closed and valid** (NFR88). Those closes alone ≠ FR186. **FR174** unpaired **1.156.0** alone ≠ FR186.

## Selected shape (NFR14)

| Layer | Role |
|-------|------|
| **AD-9 product pin** | firtool-**1.159.0** + Chisel **7.15.0** (default product cache) |
| **FR179 floating-track** | Document-pinned floating track **firtool-1.159.0** (`firtool-floating-head`) |
| **FR186 live tip** | Resolve **latest** `firtool-*` release; cache `firtool-live-tip`; report readable tip identity |
| **Override** | `BITLOOM_FIRTOOL_LIVE_TIP_PATH` → directory containing `firtool` (must not be AD-9 or floating-head cache trees) |
| **Offline / CI pin** | `BITLOOM_FIRTOOL_LIVE_TIP_VERSION` may pin the resolved tip tag for cache hits (still live-tip **channel**) |
| **AD revise** | AD-9 live-tip channel (NFR90); ≠ FR179 alone; ≠ product default |

**Honesty:** This channel is **not the product default pin**（**非产品默认钉**）. When the latest tip version coincides with FR179/AD-9 **1.159.0**, distinguish by **cache/path** (`firtool-live-tip`), not version inequality alone.

**Forbidden closes:** FR179 alone; FR174 alone; FR182 alone; PATH-random firtool; using AD-9 product **cache** or FR179 **floating-head** cache as “live tip”; docs-only; silent-Ok under `BITLOOM_FIRTOOL_LIVE_TIP_FORCE_MISSING`.

## Reproducible steps

```bash
just circt-live-tip-check
# → bash scripts/circt-live-tip-check.sh
# resolves latest firtool-* (or BITLOOM_FIRTOOL_LIVE_TIP_VERSION) into firtool-live-tip cache

BITLOOM_FIRTOOL_LIVE_TIP_FORCE_MISSING=1 bash scripts/circt-live-tip-check.sh
# → non-zero; refusing silent success
```

CI: required job **`circt-live-tip`** (no `continue-on-error`).

## Standing honesty

Default product path remains AD-9 **1.159.0**. FR179 floating-track remains a separate **channel**.
CIRCT/firtool runtime must **not** enter `bitloom-prelude` design-crate deps.
Beyond this live-tip subset still needs a newer contract (**NFR91**).

## Verify

```bash
just circt-live-tip-check
cargo test -p bitloom --test fr186_unbounded_circt_tip
```
