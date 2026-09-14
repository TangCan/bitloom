# Bitloom 1.0 public API surface (FR142 + FR183 + FR190 expand)

> **Contract:** Phase 17 / **FR142** / Epic 80; Phase 22 / **FR183** / Epic 116; Phase 23 / **FR190** / Epic 123 further expand.  
> **Authority:** Correct Course 2026-09-11 (`correctCoursePhase17Approved`); Phase 22 Correct Course 2026-09-12 (`correctCoursePhase22Approved`); Phase 23 Correct Course 2026-09-14 (`correctCoursePhase23Approved`).  
> **Claim discipline:** 「1.0 / 公开 API 稳定」requires FR141–146 closed (**FR147**). This file alone ≠ 1.0 shipped.  
> **FR183:** Explicit additive expand of in-surface (NFR14 S1–S3). **≠** silent expand. See [`docs/fr183-explicit-fr142-api-expand.md`](fr183-explicit-fr142-api-expand.md).  
> **FR190:** Further additive expand beyond FR183 (FIRRTL text emit/import + documented `check_*` family). **≠** FR183 alone. See [`docs/fr190-further-fr142-api-expand.md`](fr190-further-fr142-api-expand.md).  
> **NFR63 / NFR91:** Listing a surface does **not** clear deferred deepen outside each NFR14 subset.

## Purpose

Pin the **in-surface** vs **out-of-promise / out-of-surface** partition for SemVer **1.0** major stability. Breaking changes **inside** in-surface require a **major** bump once 1.0.0 is published (see [`docs/semver-1-0-policy.md`](semver-1-0-policy.md) / FR143). **Additive** in-surface expands require an explicit doc + story update (**FR183** / **FR190**) and are **minor**-class under FR143.

## Design crate dependency boundary (AD-6)

- Design crates depend **only** on **`bitloom-prelude`**.
- Design crates must **not** depend on `bitloom` (CLI), `bitloom-sim`, `bitloom-hir`, `bitloom-builder`, `bitloom-vlog`, `bitloom-macro` (direct), `bitloom-firrtl`, or `bitloom-lsp`.
- Macros and attributes are consumed via the prelude `rhdl` facade / re-exports.
- **FR183 honesty:** Promoting documented `bitloom-firrtl` interop into in-surface does **not** authorize design crates to depend on `bitloom-firrtl`. That crate remains a **maintainer / toolchain interop** surface.

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
| `coverage` | Coverage artifacts (FR114 LCOV + in-tree GUI; optional `--genhtml` FR158 third-party path) |

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

### `bitloom-firrtl` — FR183 / v1.x expand (maintainer / toolchain interop)

**Promoted (additive · FR183):** the following **documented** interop entries are **in-surface** for SemVer promise on the `bitloom-firrtl` crate:

| Entry | Role |
| --- | --- |
| `emit_chisel` | FrozenHir → compilable Chisel Scala (FR28 mechanical face) |
| `emit_chisel_idiomatic` / documented idiomatic·style·ecosystem variants already productized (`emit_chisel_idiomatic_fr111`, `emit_chisel_idiomatic_fr122`, `emit_chisel_style_guide_fr130`, `emit_chisel_style_guide_fr165`, `emit_chisel_ecosystem_fr176`, `emit_chisel_style_linter_fr181`, `emit_chisel_style_guide_pack_fr188`) | Documented maintainable / Style / deepen faces |
| `CHISEL_TARGET` / `FIRTOOL_TARGET` | Documented AD-9 pin constants |
| `BitloomFirrtlParser.parse` | Product-equivalent Parser path (FR138; `just parser-restore-check`) |
| `BitloomFirrtlParser.parseUpdateMainline` | Update-mainline Parser path (FR170; FIRRTL 6.0.0) |

**Not** in this expand: undocumented `pub` items; whole-crate internal modules; design-crate dependency on `bitloom-firrtl` (forbidden — **AD-6**).

**SemVer honesty (FR143):** this expand is **additive**. The next crates.io publish of `bitloom-firrtl` that cuts after this surface revision is a **minor** bump (e.g. 1.0.0 → 1.1.0), not a silent major and not “already expanded without a doc/story”. Tree may remain at 1.0.0 until that publish; surface honesty is this file + [`docs/fr183-explicit-fr142-api-expand.md`](fr183-explicit-fr142-api-expand.md).

### `bitloom-firrtl` — FR190 / v1.x expand (beyond FR183)

**Promoted (additive · FR190):** the following **documented** entries are **also in-surface** (superset of FR183):

| Entry | Role |
| --- | --- |
| `emit` | FrozenHir → FIRRTL 6.0.0 text (AD-3) |
| `import` | FIRRTL text → FrozenHir |
| `ports_roundtrip_ok` / `instance_graph_roundtrip_ok` | Documented interop roundtrip predicates |
| `check_idiomatic_chisel` / `check_idiomatic_chisel_fr111` / `check_idiomatic_chisel_fr122` | Documented acceptance checks paired with idiomatic emit faces |
| `check_chisel_style_guide_fr130` / `check_chisel_style_guide_fr165` | Style Guide check faces |
| `check_chisel_ecosystem_fr176` / `check_chisel_style_linter_fr181` / `check_chisel_style_guide_pack_fr188` | Ecosystem / deepen / pack check faces |

**Not** in this expand: undocumented `pub`; design-crate dependency on `bitloom-firrtl` (**AD-6**); promoting `bitloom-lsp` / `bitloom-hir` / `bitloom-builder` / `bitloom-vlog` to 1.0-stable.

**SemVer honesty (FR143):** FR190 is **additive** beyond FR183 → **minor** on the next `bitloom-firrtl` publish that cuts after this revision. See [`docs/fr190-further-fr142-api-expand.md`](fr190-further-fr142-api-expand.md). Workspace **1.1.0** (2026-09-14) is that minor cut — see [`docs/bitloom-1-1-0-release.md`](bitloom-1-1-0-release.md).

## Out-of-promise (may publish; not 1.0-stable) — Q2

| Crate | Note |
| --- | --- |
| `bitloom-hir` | May stay on crates.io; **no** 1.0 SemVer stability promise |
| `bitloom-builder` | Same |
| `bitloom-vlog` | Same |

Breaking changes in these crates do **not** by themselves require a Bitloom **1.0** major, but must not silently break the **prelude** / **sim** / **FR183/FR190 firrtl interop** in-surface contracts.

## Out-of-surface (not in 1.0 promise)

| Item | Note |
| --- | --- |
| `bitloom-lsp` / LSP | Product exists; not part of 1.0 SemVer surface promise |
| `bitloom-firrtl` **undocumented** `pub` / non-listed modules | Publishable crate; only the FR183 + FR190 tables above are in-surface — **no** silent promotion |
| remaining `rhdl-*` (`rhdl-formal`, …) | Stay unpublished internal names |
| Undocumented internal `pub` APIs | Forbidden from silent promotion to in-surface |
| NFR59 / NFR91 deferred deepen | Remains deferred; 1.0 ≠ clear leftovers |

## Blocking hygiene candidates (FR145 / Epic 82)

At FR142 lock time, **no blocking breaking items** are listed against the in-surface partition above. Epic 82 may **skip** FR145 with documentation if semver baseline agrees (Q3).

## Change process

- Expanding in-surface requires an explicit doc + story update (do not silent-expand). **FR183** (Phase 22) and **FR190** (Phase 23) are the contracts for the firrtl interop promotes above.
- After 1.0.0, in-surface breaking → major (FR143); additive documented expands → minor.
- Brand: **Bitloom** / `bitloom` / `bitloom-*`; never publish `rhdl` / `rhdl-bits` as the product name.
