# FR101 — SystemC TLM-2.0 product path

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

This page is the **FR101 product completion surface**. **Epic 46 / FR101 is
closed** (Story **46.3** closeout; product face delivered in Story **46.2**).
It delivers a buildable/runnable SystemC TLM-2.0 path per NFR14
(`nfr14-risk-epic46-systemc-tlm.md` D1–D4).

「不承诺 SystemC TLM-2.0」is **no longer** a product completion exclusion
(revised **AD-5** / Phase 12). Contracted MVP remains **LT-only**; AT
(`nb_transport_*`) is delivered under **FR107** / Epic 49 (see
[`fr107-systemc-tlm-at.md`](fr107-systemc-tlm-at.md)), not by this epic.

## Abstraction contract (L3)

**MVP = LT-only** (loosely-timed):

- `tlm::tlm_generic_payload`
- `b_transport` on `tlm_utils::simple_*_socket`

**Out of MVP / deferred from FR101:** AT (`nb_transport_fw` / `nb_transport_bw`)
→ **FR107**; full SoC VP; default TLM≡CA formal proof (that is **FR100** / Epic 45 —
already closed).

## Deliverables (D1–D4)

| # | Item | Bitloom face |
|---|------|--------------|
| D1 | Library / includeable TLM side | Generated `bitloom_tlm_lt.hpp` / `.cpp` |
| D2 | Generator or first-class entry | `bitloom_sim::emit_systemc_tlm_lt` + `cargo bitloom gen-tlm` |
| D3 | ≥1 runnable fixture | Emitted `sc_main.cpp` + `make run` → `BITLOOM_TLM_LT_OK` |
| D4 | Documented deps / versions | SystemC **2.3.3** via `pkg-config systemc` / `libsystemc-dev` |

## Product API (`bitloom-sim`)

Design crates stay on **`bitloom-prelude` only**. SystemC emission lives in the
toolchain:

```rust
use bitloom_sim::{emit_systemc_tlm_lt, build_and_run_tlm_lt_smoke, resolve_systemc};

let out = emit_systemc_tlm_lt(&hir, out_dir)?;
let _tc = resolve_systemc()?; // readable Err if missing
build_and_run_tlm_lt_smoke(&out)?;
```

### CLI

```bash
cargo bitloom gen-tlm --package counter_ports --out-dir target/bitloom-systemc-tlm
cd target/bitloom-systemc-tlm && make run
```

## Toolchain pin (D4)

| Dep | Pin | Notes |
|-----|-----|-------|
| SystemC | **2.3.3** (Accellera; Debian `libsystemc-dev`) | `pkg-config --modversion systemc` |
| C++ | g++ with `-std=c++17` | Linked via Makefile |
| TLM headers | bundled with SystemC (`<tlm>`, `tlm_utils/*`) | IEEE 1666 TLM-2.0 |

Missing SystemC must **fail readably** (message names `systemc` / `libsystemc-dev`
and points here). Silent skip that still claims FR101 green is forbidden.

Install (Debian/Ubuntu):

```bash
sudo apt-get install -y libsystemc-dev pkg-config g++
```

CI installs the same package so `just test` / workspace ATDD can reproduce the LT smoke.

## Revised AD-5 (NFR41)

ARCHITECTURE-SPINE **AD-5** (revised 2026-09-09 / Path B) **allows** this SystemC
TLM-2.0 product path. Cycle-accurate simulation remains only `FrozenHir` →
`Sim::tick`. Rust functional sim (**FR47**) remains a legal functional view and is
**not** SystemC TLM.

## FR47 ≠ FR101

| Path | Role | Closes FR101? |
|------|------|---------------|
| `emit_functional_crate` / `GeneratedFunctional` | Host Rust FL | **No** |
| README slogan / empty stub headers | Docs only | **No** |
| This page + `emit_systemc_tlm_lt` ATDD | SystemC TLM-2.0 LT product | **Yes** (Epic 46 **closed**) |

## Recipe

```text
cargo test -p bitloom --test fr101_systemc_tlm_product
cargo test -p bitloom --test fr101_epic46_closeout
```

## Cross-links

| Doc | Role |
|-----|------|
| [`fr47-dual-sim-generation.md`](fr47-dual-sim-generation.md) | Rust FL / CA generators — **Not** SystemC |
| [`fr100-formal-equiv.md`](fr100-formal-equiv.md) | FL≡RTL formal product — **≠** TLM product |
| NFR14 Epic 46 | `_agile-output/implementation-artifacts/nfr14-risk-epic46-systemc-tlm.md` (**closed**) |
| AD-5 | `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md` |
