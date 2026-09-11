# FR137 — External CIRCT compile gate

**Product:** Bitloom. **Status:** Story **76.2** implemented (Epic 76 closeout → **76.3**).

Beyond FR129 C1–C4 in-tree Handshake markers / FR121 ready/valid / FR110 / FR95 / FR96 alone.

## Selected MVP (NFR14 E1–E4)

| Item | Contract |
|------|----------|
| **Selected gate** | **Compile gate** (external `firtool` lowers a representative `.fir`) |
| **Sim gate** | **Not selected** this epic — deferred (**NFR59** / future contract); still satisfies E1–E4 |
| Tool | **firtool-1.155.0** (Chisel 7.14.0 pair; **AD-9**) |
| Channel | GitHub tag `firtool-1.155.0` → `firrtl-bin-linux-x64.tar.gz` + `.sha256` **or** AD-9 CLI cache (`cargo bitloom firtool ensure`) |
| Override | `RHDL_FIRTOOL_PATH` (directory containing `firtool`); **never** trust bare `PATH` firtool as the resolver |
| Local path | `just circt-external-check` → `bash scripts/circt-external-check.sh` |
| CI job | `.github/workflows/ci.yml` → **`circt-external`** (required; **no** `continue-on-error`) |
| Fixture | `crates/rhdl-firrtl/fixtures/fr137_external_circt_gate.fir` |
| Failure | Missing tool / version ≠ 1.155.0 / compile fail → **non-zero** + readable stderr (`BITLOOM_CIRCT_FORCE_MISSING=1` forces missing) |

## Forbidden closes

- FR129 C1–C4 alone；FR121 alone；FR110 alone；FR95 / FR96 alone
- docs-only；`continue-on-error` silent skip
- CIRCT HEAD；PATH-random firtool；rewriting FR95/110/121/129 closed as failed (**NFR56**)

## NFR58 ops sync

- AD-9 / Stack pin remains firtool-1.155.0；CI + `just` path documented here
- Spine Phase 16 pointer: external CIRCT → **FR137** / this doc
- Cross-link: [`fr129-circt-handshake.md`](fr129-circt-handshake.md) (in-tree ≠ external gate)

## Non-regression (NFR56)

FR95 / FR110 / FR121 / FR129 product paths remain valid.

ATDD: `cargo test -p bitloom --test fr137_external_circt_compile_sim_gate`
