# Rubric Walk — Architecture Spine

- **Subject:** `ARCHITECTURE-SPINE.md`
- **Altitude:** feature (epics/stories below)
- **Paradigm claimed:** staged-runtime-elaboration
- **Date:** 2026-08-18
- **Walker:** good-spine checklist only
- **Sources read:** this spine only (stack versions verified against public tags)

## Verdict: **fail**

The spine is a real decision document, not a template. Paradigm, crate graph, publish identity, freeze uniqueness, dual-model sim, firtool pin, and local-CLI deployment are adopted and mostly enforceable. It still fails the good-spine bar: it leaves process-boundary / HIR handoff, clock/reset, and generator entry as silent operational envelope, so two epics can still fork. AD-3’s “lossy reversible” rule does not prevent the divergence it names. Phase-1 Verilog vs phase-2 FIRRTL export is adopted in AD-8 while `rhdl-firrtl` remains a first-class bind and seed crate.

## Checklist

| Criterion | Result | Notes |
| --- | --- | --- |
| Fixes real divergence points for the level below; misses none | **fail** | Strong on elaboration timing, identity, sim model, deps, freeze. Misses HIR handoff across process/crate boundary, generator entry, clock/reset, Verilog dialect vs `.sv`. |
| Every AD Rule is enforceable and actually prevents its stated divergence | **fail** | AD-1, AD-2, AD-4–AD-7, AD-9–AD-11 hold. AD-3 “有损可逆” is not a single enforceable contract. AD-1+AD-6+AD-7 jointly leave who calls backends unspecified. |
| Nothing under Deferred could let two units diverge | **pass with caveat** | Deferred items are bound to ADs. The phase-2 FIRRTL *export* gap lives in AD-8, not Deferred, and is the real fork. |
| Named tech is verified-current (flag invented versions) | **pass** | All named versions exist. Pin is current-enough; see Stack audit. |
| Brownfield: ratify rather than contradict | **n/a pass** | Greenfield + research. Spine does not fight an existing tree. |
| Covers driving capabilities (Rust eDSL HDL, FIRRTL, sim, Chisel) | **pass with gap** | All four are mapped. Comb/seq is listed but not governed by a rule that constrains it. Reset/mem/entry are not mapped. |
| Every altitude-owned dimension is decided, deferred, or open — no silent envelope | **fail** | No Open list. Silent: OS/arch for firtool fetch, generator discovery, HIR on-disk vs in-process, reset, license, first-run network. |
| No template comments, no placeholders | **pass** | No TODO/TBD/`{placeholder}`/template commentary. `status: draft` is metadata, not a hole. |

## Findings

### F1 — Blocker — HIR handoff / who invokes backends is undecided

**Rubric:** missed divergence + AD rules do not jointly prevent the fork they imply.

AD-1: HIR is built only in the generator process’s `elaborate()`.
AD-6: design crates must not depend on CLI or backends; backends must not depend on each other.
AD-7: backends are pure `&Hir -> Artifact`, owned by CLI in the crate graph.

Those three cannot be true at once without a fourth decision: how frozen HIR moves from the design crate to `rhdl-vlog` / `rhdl-firrtl` / `rhdl-sim`.

Possible forks for `cli` vs `core-language` vs `hir`:

- serialize HIR to a file/stdout, CLI deserializes and calls backends
- `cargo rhdl` injects a runner that links the design lib with backend crates
- design crate `fn main` prints Verilog by depending on backends (violates AD-6)
- dylib / in-process plugin

None is adopted, deferred, or open. Structural seed and capability map assume CLI “runs the generator, then backends” without a contract. This is the load-bearing process boundary for every epic below.

**Fix:** Adopt one handoff (recommended: freeze emits a versioned HIR artifact; CLI is the only backend caller) **or** adopt in-process linking and say how Cargo does it. Put the other options in Deferred. Name the artifact format or the runner crate.

### F2 — Blocker — Clock/reset and seq primitives are silent

**Rubric:** missed divergence + silent envelope + weak capability coverage.

AD-8: phase 1 is single-clock. Deferred: CDC phantom domains, multi-clock HIR. That only closes *how many* clocks.

Unstated, and certain to split `core-language`, `hir`, `verilog-backend`, `native-sim`:

- clock port implicit vs explicit; default name
- reset: none / sync / async; polarity; implicit vs explicit; FIRRTL `reg` vs `regreset`
- whether phase-1 HIR has `mem` (AD-3 bans CHIRRTL-specific mem on *import* only)

Capability map row “类型 / 模块 / comb·seq” is governed by AD-1 and AD-7, which do not constrain comb vs seq semantics.

**Fix:** Adopt a phase-1 clock/reset envelope (e.g. one explicit clock, explicit sync active-high reset, no mem / or FIRRTL `mem` with stated subset). Defer async reset, implicit clock, and memories if they are not phase 1. Do not leave them unlisted.

### F3 — Major — Generator entry / `cargo rhdl build` discovery is silent

**Rubric:** missed divergence for `cli` and `core-language`.

The paradigm says `cargo rhdl build` runs the generator. No rule says what a generator *is*:

- `fn main` binary target
- `#[rhdl::top]` / trait `Elaboratable`
- conventional `src/rhdl.rs`
- examples-as-generators (structural seed hints this for `examples/`, not for user crates)

`cargo test` “must elaborate then tick” (AD-1) has the same hole: how a test finds and calls `elaborate()`.

**Fix:** Adopt one discovery API and one test entry. Defer alternate attributes.

### F4 — Major — AD-3 “有损可逆” does not prevent the named fork; phase vs crate disagrees

**Rubric:** rule not enforceable; Deferred/phase gap can still split units.

AD-3 Prevents: one epic treating Chisel Scala round-trip as the contract vs another only dumping `.fir`.
AD-3 Rule: contract is HIR ↔ text with `FIRRTL version 6.0.0` header; import is “有损可逆子集（标量端口；无 property；无 CHIRRTL 特有 mem）”.

“Lossy” and “reversible” are opposite success criteria. Two `firrtl-interop` stories can both claim compliance: one requires HIR→`.fir`→HIR identity on the subset; the other accepts drop-on-import. The listed subset (scalar ports, no property, no CHIRRTL mem) is a start, not a round-trip oracle (aggregates, probes, unsigned/signed, modules-as-instances, width).

AD-8: FIRRTL **export** is phase 2; phase 1 must emit Yosys-friendly Verilog. `binds`, capability map, and structural seed still include `rhdl-firrtl` as a peer of `rhdl-vlog`. Import timing is not sequenced. `firrtl-interop` and `verilog-backend`/`cli` can disagree on whether phase 1 ships a crate, a CLI flag, or a stub.

**Fix:** Replace “有损可逆” with a testable round-trip predicate (what must survive, what may drop). Move `rhdl-firrtl` export (and import, if delayed) to Deferred with a phase gate, **or** say the crate exists in phase 1 as parse/ast only with no CLI emit.

### F5 — Major — Verilog dialect vs `.sv` and silent firtool OS/arch

**Rubric:** internal contradiction + silent operational envelope.

AD-8: phase 1 artifact is “Yosys 友好 Verilog” (no packed arrays, no `automatic` locals).
Consistency table: backend output is `<module>.sv` / `<module>.fir`.

`.sv` plus “Verilog” plus firtool-fed Yosys discipline will split `verilog-backend` on extension, IEEE dialect, and whether `logic`/`always_ff` are allowed.

AD-9 pins **firtool-1.155.0** and forbids default `PATH`. It does not name:

- host OS/arch matrix (CIRCT ships linux-x64, macos-arm64, macos-x64, windows-x64; no linux-aarch64 in the 1.155.0 asset list)
- which tarball (firrtl-bin vs circt-full-static)
- SHA-256 pin (AD-9’s anti-Cement rule still allows “tag name is current” without hash)
- whether first run requires network

Two `cli` stories can still download different artifacts or silently drop aarch64 Linux.

**Fix:** Adopt output as `.v` or `.sv` and name the allowed construct set. Adopt OS/arch + asset + hash (or explicitly Open them). Defer windows/macos if phase 1 is linux-x64 only.

## Stack audit (verified 2026-08-18)

| Name | Spine | Verified | Invented? |
| --- | --- | --- | --- |
| rustc 1.97.1 | MSRV | [Rust 1.97.1](https://blog.rust-lang.org/2026/07/16/Rust-1.97.1/) (2026-07-16) | no |
| edition 2024 | listed | stable | no |
| FIRRTL spec 6.0.0 | interop header | [firrtl-spec v6.0.0](https://github.com/chipsalliance/firrtl-spec/releases/tag/v6.0.0) (2026-05-12); latest *spec* tag | no |
| firtool 1.155.0 | AD-9, “2026-08-11 CIRCT” | [firtool-1.155.0](https://github.com/llvm/circt/releases/tag/firtool-1.155.0) published 2026-08-11 | no |
| Chisel 7.14.0 | pairing, not a crate dep | [chisel v7.14.0](https://github.com/chipsalliance/chisel/releases/tag/v7.14.0) (2026-08-13); notes associated firtool **1.155.0** | no |
| Yosys 0.68 | optional | Yosys 0.68 tagged 2026-08-05 | no |
| Verilator 5.050 | optional | Verilator 5.050 (2026-07-01) | no |
| rustc 1.98.0 (Deferred) | plan 2026-08-20 | beta; stable date 2026-08-20 matches | no |

**Currency note (not a fail):** [firtool-1.156.0](https://github.com/llvm/circt/releases/tag/firtool-1.156.0) exists (2026-08-16). AD-9 pins 1.155.0 as the Chisel 7.14.0 pair, which is a valid current pin, not an invented one. Do not “upgrade” in the spine without breaking that pair.

Chisel 7.13+ emits FIRRTL version 7.0.0 in places; the *language spec* latest release remains 6.0.0. Pinning spec 6.0.0 is coherent with AD-3 (do not require Chisel to parse RHDL `.fir`).

## What already meets the bar

- **AD-2** publish name `rhdl-rs` vs git `rhdl` vs `samitbasu/rhdl` — checkable in crate manifests and docs homepage.
- **AD-4 / AD-7** freeze as the only multi-drive/width/dir gate; HIR immutable after freeze; backends pure. Ownership-as-proof is both forbidden and Deferred.
- **AD-5** cycle-accurate `tick` from frozen HIR vs handwritten `#[functional_model]`; TLM-2.0 banned.
- **AD-6** downward deps with a testable graph; macros/backends split.
- **AD-9 / AD-11** local CLI, cached firtool, no cloud control plane, Yosys/Verilator optional.
- **AD-10** user errors → structured diagnostics / `compile_error`; no user-facing panic.
- Capability map hits Rust eDSL, FIRRTL, sim, and Chisel (explicit non-contract).
- Deferred CDC, HLS, IP/LSP, SVA, multi-clock, TLM, Chisel Scala — none of those rows is an ungoverned phase-1 fork.
- No placeholders.

## AD enforceability (per rule)

| AD | Prevents (stated) | Prevents in practice? |
| --- | --- | --- |
| AD-1 | rustc-time HIR vs generator process | **Partial.** Blocks compile-time netlist if macros stay sugar. Does not define `elaborate()` signature or test harness. |
| AD-2 | crates.io `rhdl`; docs aliasing samitbasu/rhdl | **Yes.** |
| AD-3 | Chisel Scala as contract vs `.fir` dump | **Partial.** Bans Scala. Round-trip predicate is contradictory. |
| AD-4 | linear types as freeze vs graph checks | **Yes** for freeze. `Wire` move remains MAY → API can still fork (covered by F2/F3 more than AD-4). |
| AD-5 | TLM/untimed gen vs `tick` only | **Yes.** |
| AD-6 | design/macro → backend; backend ↔ backend | **Yes** as a graph. Combined with AD-1/7 opens F1. |
| AD-7 | post-elaborate mutation; backend as second owner | **Yes** if freeze returns a distinct type. |
| AD-8 | FIRRTL-only phase 1; HIR semantics drifting from `.fir` | **Partial.** Verilog-first is clear. FIRRTL crate still in seed. Dialect vs `.sv` (F5). |
| AD-9 | PATH firtool; stale tarball labeled current | **Yes** for version name. Hash/OS silent (F5). |
| AD-10 | user input → panic / `custom attribute panicked` | **Yes** if CI forbids unwrap on user paths. Diagnostic crate unstated (minor). |
| AD-11 | hosted service; assume global Yosys/Verilator/firtool | **Yes.** |

## Silent envelope (feature altitude)

Decided: paradigm, crate layout, publish identity, freeze/mutability, sim dual-model, diag code shape, `RHDL_FIRTOOL_PATH`, local CLI, MSRV/stack, single-clock phase 1, logging = tracing.

Deferred: CDC, ownership-as-proof, Chisel Scala, TLM, HLS, IP/IDE/LSP, SVA, multi-clock, MSRV 1.98.0.

**Silent (must become Adopted, Deferred, or Open):**

- HIR handoff / process boundary (F1)
- clock/reset/mem phase-1 envelope (F2)
- generator and test entry (F3)
- FIRRTL import/export phase gate vs crate presence (F4)
- Verilog dialect and file extension (F5)
- OS/arch + firtool asset + hash + offline-after-cache (F5)
- HIR identifier/width/signedness builder API (runtime `u32` vs const generics — paradigm hints runtime but does not bind it)
- license of crate and of generated HDL
- `rhdl-macro` may depend on `rhdl-hir` / `syn` versions (graph omits MAC outgoing edges)

There is no Open section. Silent items are not acknowledged.

## Required to re-walk as pass

1. Adopt HIR handoff (artifact or runner) so AD-1, AD-6, and AD-7 compose.
2. Adopt or defer phase-1 clock/reset/mem.
3. Adopt generator/test entry.
4. Make AD-3 round-trip testable; put FIRRTL export (and import if delayed) in Deferred or a phase rule that matches the seed.
5. Resolve `.v` vs `.sv`; list firtool OS/arch or Open it.

Do not add template prose. Keep the current AD style.
