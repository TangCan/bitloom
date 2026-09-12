# FR175 — Broader CIRCT/MLIR/sim (beyond FR169)

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 108 / FR175 in progress** (Story **108.2** implementation).

Phase 20 **FR169** (`--ir-fir` + `--ir-hw` + Verilog), **FR164** sim, and **FR137** compile **remain closed and valid** (NFR78). Those closes alone ≠ FR175.

## Selected shape (NFR14)

| Layer | Role |
|-------|------|
| **FR169** | `--ir-fir` + `--ir-hw` + Verilog — **still valid**; alone ≠ FR175 |
| **FR175 deepen** | AD-9 **firtool-1.158.0**: **`--ir-sv`** (SV dialect) + **`--ir-verilog`** (post-Verilog-lowering IR) + Verilog |
| **Tooling** | `RHDL_FIRTOOL_PATH` or `cargo bitloom firtool ensure` |

**Forbidden closes:** FR169 alone; FR164 alone; FR137 alone; FR129 alone; PATH-random; unpaired HEAD pin as this gate; docs-only; silent-Ok under `BITLOOM_CIRCT_SV_FORCE_MISSING`.

## Reproducible steps

```bash
just circt-external-sv-check
# → bash scripts/circt-external-sv-check.sh

BITLOOM_CIRCT_SV_FORCE_MISSING=1 bash scripts/circt-external-sv-check.sh
# → non-zero; refusing silent success
```

CI: required job **`circt-external-sv`** (no `continue-on-error`).

Artifacts under `target/circt-external-sv-check/`:
- `fr175_sv.ir-sv.mlir` — SV dialect
- `fr175_sv.ir-verilog.mlir` — post-Verilog-lowering IR
- `fr175_sv.firtool.v` — Verilog emit

## Standing honesty

CIRCT/firtool runtime must **not** enter `bitloom-prelude` design-crate deps.
Further dialect/sim deepen beyond this NFR14 subset still needs a new contract (**NFR81**).

```text
cargo test -p bitloom --test fr175_broader_circt_mlir_sim
just circt-external-sv-check
```
