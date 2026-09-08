# Digest D3 — Implementation reality (r1-1)

**Dimension:** D3 Implementation reality in crates  
**Import:** [`../imports/crates-implementation-inventory.md`](../imports/crates-implementation-inventory.md)  
**Date:** 2026-09-08  
**Named imports only:** `crates/**` (+ Justfile/scripts as test recipes)

---

## Verdict

Bitloom’s **core RTL loop is real in-tree**: design surface → `ElaborateSession` → `FrozenHir` → Verilog / FIRRTL / tick-sim, plus FR28 Chisel Scala emit (no Mem) and FR47 Rust functional/cycle crate generation. **Controlled generic closures / `impl Fn` hardware generators are absent** from the crates surface — a hard gap vs any requirements that assume LUT builders or `const fn` callbacks.

---

## Counts (theme buckets)

| Class | n | Notes |
|-------|--:|-------|
| Implemented | **8** | macros surface, HIR, builder, vlog, sim, FIRRTL subset, Chisel subset, FR47 generators |
| Partial | **9** | HLS external, SVA emit, viz HTML, CDC markers, Bundle flatten, IP stubs, C ABI demo, SoftF16, process macros |
| Absent (design-facing) | **5+** | `impl Fn` generators, `const fn` generators, LUT+callback, nested Bundle derive, real CDC RTL, TLM-from-HIR, LSP |

| Workspace | n |
|-----------|--:|
| Cargo crates under `crates/` | **13** |
| `#[test]` attrs (approx) | **146** |
| `bitloom/tests/*.rs` ATDD/integration | **32** |

---

## Closure / Fn search (critical for this research topic)

| Query | Result |
|-------|--------|
| `impl Fn` / `FnMut` / `FnOnce` / `dyn Fn` / `F: Fn` | **NOT FOUND** under `crates/` |
| `const fn` | **NOT FOUND** |
| LUT builders + callbacks | **NOT FOUND** |
| FR47 `generate_*` APIs | **FOUND** — take `&FrozenHir` + `Path` only (`bitloom-sim/src/generate.rs`, `cycle.rs`); **no** user `Fn` |
| Toolchain-internal closures | **FOUND** e.g. `rhdl-firrtl/src/lib.rs:165` local `flush_processes`; iterator `.map(\|…\|)` — not product generators |

---

## What is shipped (evidence anchors)

1. **Pipeline:** `bitloom-prelude` + `bitloom-macro` → `bitloom-builder` → `bitloom-hir::FrozenHir` → `bitloom-vlog` / `rhdl-firrtl` / `bitloom-sim`.
2. **CLI product:** `crates/bitloom/src/main.rs` — build, import(+chisel), gen-func/gen-cycle, visualize/doc/wave, hls, firtool.
3. **Chisel:** `rhdl-firrtl::emit_chisel` — compilable Scala subset; Mem → `rhdl::E0901`.
4. **Host multi-view:** macros mark bridge/abstraction/functional_model; compare via `PortValues` / `AbstractionView` (no TLM).
5. **Mem:** SyncReadMem latency-1 in sim; FIRRTL/Verilog emit; Chisel unsupported.
6. **Verify recipes:** `just test`; `just hls-smoke`; `just chisel-fr28-jvm` / `chisel-fr28-atdd`; `scripts/*`.

---

## Gaps most relevant to “generic closures” FR plan

- No language API to elaborate hardware from `impl Fn` / closures / `const fn` table builders.
- `#[combinational]`/`#[sequential]` do **not** lower Rust bodies to HIR assigns (markers only); real nets built via `ElaborateSession` / `#[rhdl::module]` ports.
- Bundle is flatten-to-scalars only; no `#[derive(Bundle)]`; nested composites out of MVP scope.
- CDC/IP/formal/float/C-ABI are stubs or host-side — not full requirement depth.

---

## Sources

All paths under `/home/richard/richard/2026/2026/rhdl/crates/` listed in the inventory import; recipes from repo `Justfile` and `scripts/`.
