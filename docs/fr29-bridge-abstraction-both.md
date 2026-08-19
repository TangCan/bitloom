# FR29 — handwritten `#[bridge]` / `#[abstraction]` / mixed `both`

Multi-view simulation is **two handwritten models**, compared only on `PortValues`.
The toolchain does **not** lower `FrozenHir` to TLM-2.0 or any untimed SystemC socket.

## Views

| Attribute | Kind | Role |
|-----------|------|------|
| `#[rhdl::functional_model]` | `ViewKind::FunctionalModel` | Host cycle() matching `tick` (CAP-6 / Story 3.3) |
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

Use `rhdl_sim::check_mixed_both` (dev-dependency `rhdl-sim` only). Design `[dependencies]` stay `rhdl-prelude`.

## Non-goals

- No `emit_tlm` / HIR→TLM generator (FR14, FR29).
- No generated TLM sockets from Verilog or FIRRTL.
