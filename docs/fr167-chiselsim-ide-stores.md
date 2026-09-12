# FR167 — Full ChiselSim coupling + multi IDE-store publish

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 100 / FR167 in progress** (Story **100.2** implementation). Story **100.3** closeout pending.

Phase 19 **FR162** default Tywaves GUI primary and **FR134** G1–G4 **remain closed and valid** (NFR73).
This FR deepens **both**:

| Layer | Role |
|-------|------|
| **(a) ChiselSim** | Product path beyond FR162 GUI — `chiselsim.manifest.json` + install/check |
| **(b) IDE stores** | Beyond FR134 G1 VS Code alone — **Open VSX** + **JetBrains** (+ retained G1) |

**Forbidden closes:** FR162 alone; FR134 G1–G4 alone; only (a) or only (b); docs-only;
silent-Ok under missing ChiselSim root / missing store tokens / `FORCE_MISSING`.

## (a) ChiselSim coupling

Pinned: `chisel3-chiselsim` **7.14.0** (pairs AD-9 Chisel 7.14.0 / firtool-1.155.0).
Peek/poke on generated SV — **not** Bitloom `tick` golden.

```bash
mkdir -p /tmp/chiselsim-stub && echo ok > /tmp/chiselsim-stub/BITLOOM_CHISELSIM_OK

BITLOOM_CHISELSIM_ROOT=/tmp/chiselsim-stub \
  cargo bitloom wave \
    --input crates/rhdl-firrtl/fixtures/external_hierarchy.fir \
    --out-dir target/wave-fr167 \
    --ticks 4 \
    --chiselsim

# Hard fail (must not silent-green)
BITLOOM_CHISELSIM_FORCE_MISSING=1 \
  cargo bitloom wave ... --chiselsim
# → non-zero; stderr contains bitloom.chiselsim*
```

Artifacts: `chiselsim.manifest.json`, `chiselsim.install.json`, `chiselsim.check.sh`.

## (b) Extra IDE stores (Open VSX + JetBrains)

| Store | Id | Channel |
|-------|----|---------|
| VS Code (FR134 G1 retained) | `surfer-project.surfer` | marketplace.visualstudio.com |
| **Open VSX (FR167)** | `surfer-project.surfer` | open-vsx.org |
| **JetBrains (FR167)** | `org.surferproject.surfer` | plugins.jetbrains.com |

```bash
# Descriptors + token gate (requires both tokens for success path)
OVSX_PAT=… JETBRAINS_TOKEN=… \
  cargo bitloom wave ... --ide-stores

# Hard fail
BITLOOM_IDE_STORE_PUBLISH_FORCE_MISSING=1 \
  cargo bitloom wave ... --ide-stores
# → non-zero; bitloom.ide-store-missing-token

# Publish helper (also fails without tokens)
./scripts/publish-tywaves-ide-stores.sh dry-run
```

Artifacts: `tywaves.ide-stores.manifest.json`, `tywaves.ide-stores.install.json`.
Live store publish must revise docs/CI first (**NFR75**); missing tokens → **non-zero**, never silent-Ok.

## Standing honesty

- ChiselSim / IDE publish runtimes must **not** enter `bitloom-prelude` design-crate deps.
- FR162 default wave and FR134 G1–G4 still emit / remain valid.
- Both (a) **and** (b) required for FR167 claim.

```text
cargo test -p bitloom --test fr167_chiselsim_ide_stores
```
