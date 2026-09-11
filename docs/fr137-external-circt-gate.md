# FR137 — External CIRCT compile gate

**Product:** Bitloom. **Status:** **Epic 76 / FR137 closed** (Story **76.3**). E1–E4 compile-gate MVP delivered in Story **76.2**.

Beyond FR129 C1–C4 in-tree Handshake markers / FR121 ready/valid / FR110 / FR95 / FR96 alone.

Phase 16 **规划故事已齐（Epic 72–78）**；实现关闭态：**Epic 72**（闸门 FR133）、**Epic 73**（FR134）、**Epic 74**（FR135）、**Epic 75**（FR136）、**Epic 76**（本 FR）、**Epic 78**（FR139）**已关闭**；**Epic 77** 仍须各自实现关闭。FR129 C1–C4 **仍有效**（NFR56）and is **not** this face alone. 未列更广 CIRCT/MLIR lower（含仿真门禁加深）仍属 **NFR59**。终局宣称须对应 **FR133–139** 关闭后方可勾选（**FR140**）。

See also [`docs/fr129-circt-handshake.md`](fr129-circt-handshake.md).

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
Closeout: `cargo test -p bitloom --test fr137_epic76_closeout`
