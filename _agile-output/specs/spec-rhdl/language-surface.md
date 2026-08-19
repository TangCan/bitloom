# Language surface

Catalog for CAP-1…CAP-3 and CAP-7. HOW（宏如何展开、freeze 何时跑）见架构脊柱。

## Hardware types (in-scope now)

- `Bool`, `Bits<N>`, `UInt<N>`, `SInt<N>`, `Clock`, `Reset`
- Ports: `Input<T>`, `Output<T>` — not bare `UInt` at the module boundary
- Builder facade `Bits<N>` lowers to runtime widths on FrozenHir / `PortValues`

## Deferred types (not CAP-1)

See `later-product.md`: `Analog`, `InOut`, `Bundle`, `Vec<T,N>`, `Mem<T,DEPTH>`, `ClockDomain` / `Polarity` / `ResetKind` as multi-clock types.

## Comb / seq

- `#[combinational]` and `#[sequential]` are mandatory.
- Comb may drive `Wire` / `Output` only; incomplete assignment is an error (no inferred latch).
- Only seq writes `Reg.d`. Comb must not write `Reg.d`; seq must not drive combinational nets.

## Width

- Surface arithmetic and connections are strict same-width.
- Extend/truncate only via explicit pad/trunc nodes.
- FIRRTL `add` n+1 is allowed only as those explicit nodes, never as silent prelude truncation.

## Synthesizable subset (cycle-accurate / generate path)

Allowed: hardware types and their ops; `if` / `match`; statically bounded loops that fully unroll; inlined functions; const generics; arrays / structs / enums used as hardware aggregates in-scope.

Rejected on this path: `Vec`/`Box`/`String` heap; unbounded recursion; `dyn Trait`; capturing closures; file/net/threads; default `f32`/`f64`.

Functional view (`#[functional_model]`) may use rejected constructs. Fields marked `#[functional_state]` never enter HIR.

## Sequential envelope (in-scope now)

Every module has exactly one `Clock` port and one sync active-high `Reset` port. `tick` is one posedge of that clock. No implicit ports at emit.
