# 桥接半程 — FR78 `start_wait_complete` follow-along

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

This walkthrough is the **UJ-level「桥接半程」**: one fixture path from host-side
bridge-adapter closures to ordinary cycle-accurate signals (FR78 / Cap-R-65…68).
It does **not** replace full UJ-6 visualization, FR47 crate generation alone, or
Epic 27/28 hardware closures.

Design crates depend only on **`bitloom-prelude`**. SystemC TLM-2.0 is **not** a
product contract (AD-5).

## What you are practicing

| Step | View | Closures |
|------|------|----------|
| 1. Host / bridge | Functional / verification | Free Rust closures OK (`start_fn`) |
| 2. Template expand | `start_wait_complete` | Closure finishes on the host stack |
| 3. Cycle-accurate | `Sim::tick` / `PortValues` | **No** `Fn` objects (NFR36) |

## Fixture A — template API (Story 30.2)

Minimal host handshake + UART-driven ordinary signals:

```bash
cargo test -p bitloom --test fr78_bridge_adapter_start_wait_complete
```

Expect 4 tests green: trait form, free-function form, UART cycle-accurate pins,
and docs/surface presence checks.

Source: `crates/bitloom/tests/fr78_bridge_adapter_start_wait_complete.rs`.

## Fixture B — FR47 dual-view co-verify (Story 30.3)

Same template records `PortValues` stimuli into the FR47 generated-path bridge
(`check_functional_equiv_generated`). Matching views pass; a deliberate wrong
abstraction fails (FR30 spirit). Also samples FR16 capture reject + NFR36 emit.

```bash
cargo test -p bitloom --test fr78_fr47_dual_view_coverify
```

Source: `crates/bitloom/tests/fr78_fr47_dual_view_coverify.rs`.

## Cross-links (Phase 9 map)

| Topic | Doc |
|-------|-----|
| Bridge template API + view boundary | [`docs/fr78-bridge-adapter-closures.md`](../fr78-bridge-adapter-closures.md) |
| Dual-view sim crate generation (≠ FR78) | [`docs/fr47-dual-sim-generation.md`](../fr47-dual-sim-generation.md) |
| Handwritten `#[bridge]` / HostView | [`docs/fr29-bridge-abstraction-both.md`](../fr29-bridge-abstraction-both.md) |
| Epic 27 generator closures (FR73) | README「Elaborate-time Mem init」· [`docs/fr22-construct-bar.md`](../fr22-construct-bar.md) |
| Epic 28 synthesizable closures (FR74/75) | README「可综合闭包」· [`docs/fr22-construct-bar.md`](../fr22-construct-bar.md) |
| Terminology (decision table §3) | `closure-decision-table-2026-09-08.md` |
| Risk gate | `nfr14-risk-epic30-bridge-adapter-closures.md` |

## Out of scope here

- Comb/seq `SynthesizableClosure` inline (Epic 28) — not a bridge template.
- Elaborate-time Mem/factory generator `Fn` (Epic 27) — not host handshake.
- Claiming Phase 7 overview「闭环」or FR47 sim generators as FR78 done.
