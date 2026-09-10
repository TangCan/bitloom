# FR107 — SystemC TLM-2.0 AT product path

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

This page is the **FR107 product completion surface** (Epic 49 / Story **49.2**).
It delivers a buildable/runnable SystemC TLM-2.0 **AT-style** path per NFR14
(`nfr14-risk-epic49-systemc-tlm-at.md` A1–A4), **beyond** FR101 LT-only MVP.

FR101 LT (`emit_systemc_tlm_lt` / `gen-tlm`) remains closed and must stay green
(**NFR44**). This path is **parallel**, not a rewrite of LT.

## Abstraction contract (documented AT subset)

**MVP = AT documented subset:**

- `tlm::tlm_generic_payload`
- Target `nb_transport_fw` registered on `tlm_utils::simple_target_socket`
- Initiator calls `nb_transport_fw` with `BEGIN_REQ`; target returns
  `TLM_COMPLETED` (collapsed BEGIN_REQ → END_RESP)

**Out of this FR / still need new contract (NFR47):** full multi-phase PEQs,
quantum keeper family, DMI-heavy VP, performance-model certification,
`nb_transport_bw`-only initiator/target pairs beyond the collapsed subset.

## Deliverables (A1–A4)

| # | Item | Bitloom face |
|---|------|--------------|
| A1 | AT-style interface | Generated `nb_transport_fw` in `bitloom_tlm_at.*` |
| A2 | Relation to LT | LT path unchanged; docs state AT ∥ LT |
| A3 | Generator / entry | `bitloom_sim::emit_systemc_tlm_at` + `cargo bitloom gen-tlm-at` |
| A4 | Fixture + deps | `sc_main.cpp` + `make run` → `BITLOOM_TLM_AT_OK`; SystemC **2.3.3** |

## Product API (`bitloom-sim`)

```rust
use bitloom_sim::{emit_systemc_tlm_at, build_and_run_tlm_at_smoke, resolve_systemc};

let out = emit_systemc_tlm_at(&hir, out_dir)?;
build_and_run_tlm_at_smoke(&out)?;
```

### CLI

```bash
cargo bitloom gen-tlm-at --package counter_ports --out-dir target/bitloom-systemc-tlm-at
cd target/bitloom-systemc-tlm-at && make run
```

Default `cargo bitloom gen-tlm` remains **LT-only** (FR101).

## Revised AD-5 (NFR46)

ARCHITECTURE-SPINE **AD-5** (revised 2026-09-10 / Phase 13) **allows** this AT /
`nb_transport_*` product branch. Cycle-accurate remains `FrozenHir` → `Sim::tick`.
Rust FL (**FR47**) is **not** SystemC TLM AT. FR101 LT closeout stays valid.

## Contrast table

| Path | Role | Closes FR107? |
|------|------|---------------|
| `emit_systemc_tlm_lt` / FR101 | LT `b_transport` | **No** |
| `emit_functional_crate` / FR47 | Host Rust FL | **No** |
| Docs slogan only | Docs | **No** |
| This page + `emit_systemc_tlm_at` | AT documented subset | **Yes** (when Epic 49 closes) |

## Recipe

```text
cargo test -p bitloom --test fr107_systemc_tlm_at
```

## Cross-links

| Doc | Role |
|-----|------|
| [`fr101-systemc-tlm.md`](fr101-systemc-tlm.md) | LT product (FR101) — must remain green |
| [`fr47-dual-sim-generation.md`](fr47-dual-sim-generation.md) | Rust FL ≠ TLM |
| NFR14 Epic 49 | `_agile-output/implementation-artifacts/nfr14-risk-epic49-systemc-tlm-at.md` |
