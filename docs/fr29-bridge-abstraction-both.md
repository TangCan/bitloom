# FR29 — handwritten `#[bridge]` / `#[abstraction]` / mixed `both`

Multi-view simulation starts as **two handwritten models**, compared only on `PortValues`
(`bitloom-hir` / AD-17). Cycle-accurate RTL remains `FrozenHir` + `bitloom_sim::Sim::tick`
(AD-5). SystemC TLM-2.0 is **not** a product contract.

## Handwritten path vs generated path (FR47)

| Path | Status | Notes |
|------|--------|-------|
| **Handwritten** `#[functional_model]` / `#[abstraction]` / `#[bridge]` / `#[both]` | **Supported now (FR29)** | Host-side attributes; never enter `freeze` / HIR. Design crates depend only on `bitloom-prelude`. |
| **Generated** Rust functional-sim crate + cycle-accurate sim artifacts | **Coming (FR47 / Epic 21.3+)** | Toolchain generates artifacts; does **not** remove or replace handwritten annotation capability. |

FR29 no longer forbids generating a functional simulator (see PRD overturn table / AD-5 / FR47).
What remains forbidden as a silent downgrade: claiming FR47 done with **only** handwritten
fixtures, or claiming **SystemC TLM** delivery.

## Views

| Attribute | Kind | Role |
|-----------|------|------|
| `#[rhdl::functional_model]` | `ViewKind::FunctionalModel` | Host `cycle()` matching `tick` |
| `#[rhdl::abstraction]` | `ViewKind::Abstraction` | Untimed / transaction-shaped host model |
| `#[rhdl::bridge]` | `ViewKind::Bridge` | Handwritten adapter between pin-level `PortValues` and the abstraction |
| `#[rhdl::both]` | `ViewKind::Both` | Mixed fixture that owns RTL (`FrozenHir` + `tick`) **and** a handwritten view |

All four expand to `impl HostView`; they **never** enter `freeze` / HIR.

**FR102:** the full multi-view attribute matrix (including `#[functional_state]` soft
fields and illegal-combination gates) is documented in
[`fr102-multiview-attribute-matrix.md`](fr102-multiview-attribute-matrix.md).
FR29 rows remain first-class; FR102 is the completion surface beyond adapter templates.

## Mixed fixture (documented)

See `examples/mixed_both`: a counter RTL `tick` vs a `#[rhdl::abstraction]` + `#[rhdl::bridge]` pair, driven as `#[rhdl::both]`.

```text
inputs ──► Sim::tick(FrozenHir) ──► PortValues (RTL)
       └──► Bridge::to_pins / Abstraction::cycle ──► PortValues (host)
compare_port_values(rtl, host)  // mismatch → test fail
```

Use `bitloom_sim::check_mixed_both` (dev-dependency `bitloom-sim` only). Design
`[dependencies]` stay `bitloom-prelude`.

Regression: `cargo test -p mixed_both` and `cargo test -p bitloom-sim --lib mixed_both`.

## Non-goals (this story / handwritten surface)

- No HIR→TLM *lowering* that replaces cycle-accurate `tick`. SystemC TLM-2.0
  **product** path is **FR101** — see [`fr101-systemc-tlm.md`](fr101-systemc-tlm.md)
  (`emit_systemc_tlm_lt` / `cargo bitloom gen-tlm`; revised **AD-5**). Not contracted
  by FR29.
- No FR47 generator in this story — generation lands in Epic 21.3+.
- Handwritten attributes remain first-class after generation ships.

## Related: FR78 bridge-adapter templates

Reusable host handshake helpers (`start_wait_complete`) live **inside** bridge /
verification code and are **not** HIR attributes. See
[`fr78-bridge-adapter-closures.md`](fr78-bridge-adapter-closures.md) and the
UJ「桥接半程」[`tutorials/bridge-half.md`](tutorials/bridge-half.md).

**FR102 honesty:** FR78 / FR92 adapter templates are **supporting** and **alone do
not** close the multi-view attribute full matrix — see
[`fr102-multiview-attribute-matrix.md`](fr102-multiview-attribute-matrix.md).
