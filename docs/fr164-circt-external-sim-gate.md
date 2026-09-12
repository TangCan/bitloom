# FR164 — External CIRCT simulation gate (beyond FR137 compile MVP)

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 96 / FR164 closed** (Story **96.3**). Implementation Story **96.2**.

Phase 16 **FR137** external CIRCT **compile** gate **remains closed and valid** (NFR68). This FR
adds the **simulation / execution** deepen that FR137 explicitly deferred.

## Selected face (NFR14)

| Item | Contract |
|------|----------|
| **Gate** | External firtool **compile** (AD-9) **+** cycle-sim **execution predicate** |
| Tool | **firtool-1.155.0** (same AD-9 pin as FR137) |
| Local | `just circt-external-sim-check` → `bash scripts/circt-external-sim-check.sh` |
| CI | `.github/workflows/ci.yml` → **`circt-external-sim`** (required; **no** `continue-on-error`) |
| Compile fixture | `crates/rhdl-firrtl/fixtures/fr137_external_circt_gate.fir` (firtool dialect) |
| Sim fixture | `crates/rhdl-firrtl/fixtures/fr164_external_circt_sim_gate.fir` (Bitloom FIRRTL 6 import) |
| Failure | `BITLOOM_CIRCT_SIM_FORCE_MISSING=1` / missing tool / version ≠ 1.155.0 / sim fail → **non-zero** readable |

## Forbidden closes

FR137 compile alone；FR129 C1–C4 alone；FR121 alone；docs-only；`continue-on-error` silent skip；CIRCT HEAD / PATH-random firtool.

## Deferred (NFR71)

更广 CIRCT/MLIR dialect allocation / multi-lower 全家桶；firtool 升钉超 AD-9 — require a new contract.

## Recipe

```text
just circt-external-sim-check
# or
bash scripts/circt-external-sim-check.sh

BITLOOM_CIRCT_SIM_FORCE_MISSING=1 bash scripts/circt-external-sim-check.sh   # must fail

cargo test -p bitloom --test fr164_circt_external_sim_gate
```

## Cross-links

| Doc | Role |
|-----|------|
| [`fr137-external-circt-gate.md`](fr137-external-circt-gate.md) | FR137 compile MVP (still closed; ≠ FR164 alone) |
| [`fr129-circt-handshake.md`](fr129-circt-handshake.md) | in-tree Handshake (≠ external gate) |
| NFR14 | `_agile-output/implementation-artifacts/nfr14-risk-epic96-broader-circt-mlir-sim-gate-fr164.md` |
