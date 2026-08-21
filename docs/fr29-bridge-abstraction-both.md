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

- No SystemC TLM-2.0 / `emit_tlm` product API (still absent; not contracted).
- No FR47 generator in this story — generation lands in Epic 21.3+.
- Handwritten attributes remain first-class after generation ships.
