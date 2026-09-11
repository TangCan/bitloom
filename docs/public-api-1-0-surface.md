# Bitloom 1.0 public API surface (FR142)

> **Contract:** Phase 17 / **FR142** / Epic 80.  
> **Authority:** Correct Course 2026-09-11 (`correctCoursePhase17Approved`); Q1–Q5 defaults.  
> **Claim discipline:** 「1.0 / 公开 API 稳定」requires FR141–146 closed (**FR147**). This file alone ≠ 1.0 shipped.  
> **NFR63:** Listing a surface does **not** clear **NFR59** deferred product deepen.

## Purpose

Pin the **in-surface** vs **out-of-promise / out-of-surface** partition for SemVer **1.0** major stability. Breaking changes **inside** in-surface require a **major** bump once 1.0.0 is published (see [`docs/semver-1-0-policy.md`](semver-1-0-policy.md) / FR143).

## Design crate dependency boundary (AD-6)

- Design crates depend **only** on **`bitloom-prelude`**.
- Design crates must **not** depend on `bitloom` (CLI), `bitloom-sim`, `bitloom-hir`, `bitloom-builder`, `bitloom-vlog`, `bitloom-macro` (direct), or `bitloom-lsp`.
- Macros and attributes are consumed via the prelude `rhdl` facade / re-exports.

## In-surface (1.0 SemVer promise)

### `bitloom` (CLI / crates.io package)

Documented `cargo bitloom` / `cargo-bitloom` subcommands (names may gain aliases; removals/renames of documented verbs are breaking once 1.0):

| Verb | Role |
| --- | --- |
| `build` | Elaborate → Verilog |
| `new` | Scaffold design crate (prelude-only) |
| `firtool` (`ensure` / `info`) | Pinned firtool (AD-9) |
| `sim-engines` | List tick engines |
| `hls` | Product HLS paths |
| `import` | FIRRTL / Chisel import path |
| `gen-func` | Functional-sim crate emit |
| `gen-cycle` | Cycle-accurate emit path |
| `gen-tlm` / `gen-tlm-at` | SystemC TLM emit |
| `visualize` | Hierarchy / timing viz |
| `doc` | Doc emit helper |
| `wave` | Waveform artifacts |
| `coverage` | Coverage artifacts |

Library modules under `bitloom` that are **not** documented as CLI surface are **not** automatically in-surface.

### `bitloom-prelude`

Primary design-facing path:

- Re-exports used by designs: elaboratable / HIR handles as documented in crate docs (`FrozenHir`, ports, diagnostics via prelude).
- `bitloom_prelude::rhdl` attribute facade (`module`, `top`, `sequential`, `combinational`, `process`, `hls`, multi-view attrs).
- `bitloom_prelude::ip` first-class IP surface as documented.
- `Bundle` derive (via prelude) and related documented traits / helpers in crate docs.

Undocumented `pub` items are **not** promised.

### `bitloom-macro`

Documented proc-macro attributes and derives **as reached through `bitloom-prelude`** (design crates do not depend on this crate directly). Direct dependency on `bitloom-macro` is **out of the design-crate contract** even if the crate remains publishable.

### `bitloom-sim` (Q1 — IN)

Public simulation / dual-model API used by maintainers and product paths:

- Cycle-accurate `tick` / engine selection surface.
- VCD / waveform-related public helpers documented in crate docs.
- Dual-model / functional-equiv / IP dual-model public types used by product docs (e.g. `UartTxFunctional`, `GpioFunctional`, scoreboard helpers as documented).
- Coverage report helpers that are part of the documented product path.

Internal modules and undocumented `pub` items are **not** promised.

## Out-of-promise (may publish; not 1.0-stable) — Q2

| Crate | Note |
| --- | --- |
| `bitloom-hir` | May stay on crates.io; **no** 1.0 SemVer stability promise |
| `bitloom-builder` | Same |
| `bitloom-vlog` | Same |

Breaking changes in these crates do **not** by themselves require a Bitloom **1.0** major, but must not silently break the **prelude** / **sim** in-surface contracts.

## Out-of-surface (not in 1.0 promise)

| Item | Note |
| --- | --- |
| `bitloom-lsp` / LSP | Product exists; not part of 1.0 SemVer surface promise |
| `rhdl-*` workspace crates (`rhdl-firrtl`, `rhdl-formal`, …) | Internal / non-Bitloom publish names; never 1.0 surface |
| Undocumented internal `pub` APIs | Forbidden from silent promotion to in-surface |
| NFR59 deferred deepen | Remains deferred; 1.0 ≠ clear NFR59 |

## Blocking hygiene candidates (FR145 / Epic 82)

At FR142 lock time, **no blocking breaking items** are listed against the in-surface partition above. Epic 82 may **skip** FR145 with documentation if semver baseline agrees (Q3).

## Change process

- Expanding in-surface requires an explicit doc + story update (do not silent-expand).
- After 1.0.0, in-surface breaking → major (FR143).
- Brand: **Bitloom** / `bitloom` / `bitloom-*`; never publish `rhdl` / `rhdl-bits` as the product name.
