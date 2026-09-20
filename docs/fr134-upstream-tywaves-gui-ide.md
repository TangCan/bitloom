# FR134 — Upstream Tywaves GUI / IDE plugin depth

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 73 / FR134 closed** (Story **73.3**). Product path delivered in Story **73.2** (G1–G4).

Phase 16 **规划故事已齐（Epic 72–78）**；实现关闭态：**Epic 72–78 全部已关闭**（含本 FR / Epic 73；**FR138 / Epic 77 已关闭**）。FR125 T1–T4 **仍有效**（NFR56）and is **not** this face alone. 未列入更深 GUI/IDE 子集仍属 **NFR59**。终局宣称须对应 **FR133–139** 关闭后方可勾选（**FR140**）。

Beyond FR125 `wave.tywaves.json` / `BITLOOM_TYWAVES_BIN` single-binary launch.  
**Forbidden closes:** FR104 alone; FR114 alone; FR117 typed-wave alone; FR125 T1–T4 alone; docs-only.

## Selected shape (NFR14 G1–G4)

| ID | Contract |
|----|----------|
| G1 | `tywaves.gui.install.json` pins GUI package version + channel **and** IDE plugin marketplace/channel (beyond `BITLOOM_TYWAVES_BIN`) |
| G2 | `tywaves.gui.manifest.json` — Bitloom / `schemaVersion` / `tywaves.gui` / `tywaves.ide-plugin` |
| G3 | `BITLOOM_TYWAVES_GUI_FORCE_MISSING=1` or invalid `BITLOOM_TYWAVES_GUI_ROOT` → non-zero + `bitloom.tywaves*` |
| G4 | ATDD locks G1–G3; FR104/117/125 artifacts still emit |

### Pinned upstream (G1)

| Kind | Identity | Channel |
|------|----------|---------|
| GUI package | `surfer-tywaves` `v0.3.2-tywaves-dev-SNAPSHOT` | https://gitlab.com/rameloni/surfer-tywaves-demo/-/releases/v0.3.2-tywaves-SNAPSHOT |
| IDE plugin | `surfer-project.surfer` (VS Code Marketplace) | https://marketplace.visualstudio.com/items?itemName=surfer-project.surfer |

In-tree / CI may use a **stub** GUI root: a directory containing marker file `BITLOOM_TYWAVES_GUI_OK`.

## Reproducible steps

```bash
# Stub GUI root (CI / ATDD)
mkdir -p /tmp/tywaves-gui-stub && echo ok > /tmp/tywaves-gui-stub/BITLOOM_TYWAVES_GUI_OK

BITLOOM_TYWAVES_GUI_ROOT=/tmp/tywaves-gui-stub \
  cargo bitloom wave \
    --input crates/rhdl-firrtl/fixtures/external_wave_counter.fir \
    --out-dir target/wave-tywaves-gui \
    --ticks 8 \
    --tywaves-gui

# Hard fail (must not silent-green)
BITLOOM_TYWAVES_GUI_FORCE_MISSING=1 \
  cargo bitloom wave --input … --out-dir target/wave-tywaves-gui --tywaves-gui
# → non-zero; stderr contains bitloom.tywaves*
```

`--tywaves-gui` also emits FR125 sidecar (`wave.tywaves.json` + `tywaves.launch.sh`).
`--tywaves` alone does **not** emit FR134 manifests (FR125 alone ≠ FR134).

## Standing honesty

Tywaves runtime must **not** enter design-crate (`bitloom-prelude`) dependencies.

## Non-regression (NFR56)

FR117 `typed-wave.html`, FR104 `interactive.html`, and FR125 sidecar still emit.  
Those closes remain valid; FR134 is the GUI/IDE depth claim face.
FR125 T1–T4 remains closed and valid (≠ this FR alone).

## Non-goals (historical for FR134 alone)

Replacing default VCD / `typed-wave.html` as the only wave surface → **FR162** (Epic 94);
full ChiselSim coupling; extra IDE-store multi-target publish beyond the G1-pinned Surfer
marketplace id. Further GUI/IDE subsets not listed in G1–G4 remain **NFR59** unless claimed
by a later FR (FR162 closes the default-surface gap only).

```text
cargo test -p bitloom --test fr134_tywaves_gui_ide
cargo test -p bitloom --test fr134_epic73_closeout
```
