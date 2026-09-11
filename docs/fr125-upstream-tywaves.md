# FR125 — Upstream Tywaves first-class integration

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 65 / FR125 closed** (Story **65.3**). T1–T4 delivered in Story **65.2**.
FR117 in-house typed (subset B) remains closed (NFR52) and is **not** this face.

Beyond FR117 in-house `typed-wave.html` / `wave.typed.json` (subset B).  
**Forbidden closes:** FR104 alone; FR114 alone; FR117 typed-wave alone; docs-only.

## Selected shape (NFR14 T1–T4)

| ID | Contract |
|----|----------|
| T1 | `cargo bitloom wave --tywaves` → `wave.tywaves.json` + `tywaves.launch.sh` |
| T2 | `BITLOOM_TYWAVES_BIN` pins upstream (or stub) viewer |
| T3 | `BITLOOM_TYWAVES_FORCE_MISSING=1` → non-zero + `bitloom.tywaves*` message |
| T4 | ATDD locks sidecar schema ≠ FR117 alone; FR104/117 artifacts still emitted |

## Reproducible steps

```bash
# Write sidecar only (no live viewer required)
cargo bitloom wave \
  --input crates/rhdl-firrtl/fixtures/external_hierarchy.fir \
  --out-dir target/wave-tywaves \
  --ticks 8 \
  --tywaves

# Live-open with stub / upstream viewer
BITLOOM_TYWAVES_BIN=/path/to/tywaves-or-stub \
  cargo bitloom wave --input … --out-dir target/wave-tywaves --tywaves
```

`wave.tywaves.json` fields: `product=Bitloom`, `fr=FR125`, `viewer=tywaves`,
`schemaVersion=bitloom-tywaves-1`, `marker=data-bitloom-tywaves`, plus typed signals/samples.

## Non-regression (NFR52)

FR117 `typed-wave.html` / `wave.typed.json` and FR104 `interactive.html` still emit.  
FR117 close remains valid; FR125 is the upstream Tywaves claim face.

## Non-goals (NFR55 → Phase 16)

IDE marketplace plugins / 真实上游 GUI 安装包深度 → **FR134 / Epic 73 已关闭**（Story 73.3；G1–G4）；T1–T4 **仍有效**（alone ≠ FR134）。
更深 GUI/IDE 子集（替换默认 VCD/typed-wave；完整 ChiselSim 耦合；额外 IDE 商店多端）仍 **NFR59**。

ATDD: `cargo test -p bitloom --test fr125_upstream_tywaves`
