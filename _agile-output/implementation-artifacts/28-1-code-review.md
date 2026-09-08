# Code Review: Story 28.1 SynthesizableClosure 约束与检查钩子

**Verdict:** Approve

## Findings

1. **Accepted (by design):** Cap-R-60 is the session/prelude check hook (`check_synthesizable_closure` / `diagnose_synthesizable_closure_violations`) — equivalent to `cargo bitloom check` per FR74; full CLI `check` verb remains deferred (FR40 historical).
2. **Accepted (by design):** Stable codes **`rhdl::E0143`** (heap), **`rhdl::E0144`** (runtime capture state), **`rhdl::E0145`** (impure); FR16 stays **E0141**, hw-capture stays **E0142**.
3. **Accepted (deferred):** Comb/seq inline → 28.2/28.3; full diagnostic matrix → 28.4; automatic rustc body analysis → later.
4. **Accepted (NFR14):** Phase 9 record `nfr14-risk-phase9-closures.md` already covers Epic 26–30; Epic 28 has no separate NFR14 story (unlike Epic 29/30).

## AC Trace

| AC | Result |
| ---- | ------ |
| Document SynthesizableClosure: pure, no heap, no runtime capture state (FR74 / Cap-R-48…50) | pass (`language-surface.md` + trait docs) |
| Prelude/builder or check-reachable Cap-R-60 hook | pass (`check_synthesizable_closure` / `diagnose_*`) |
| Violating examples → stable diagnostic codes | pass (E0143/E0144/E0145 ATDD) |
| Legal empty/simple closures pass check (paves 28.2) | pass (`LegalEmptyClosure` / `LegalSimpleClosure`) |
| No new closure IR in FIRRTL/Chisel (NFR36) | pass (FrozenHir spot-check; check is diagnostics-only) |

## Verification

- `cargo test -p bitloom-builder synthesizable_closure`
- `cargo test -p bitloom-builder diagnose_free_fn_cap_r60`
- `cargo test -p bitloom --test fr74_synthesizable_closure_check`
- `cargo test -p bitloom-prelude --lib`

**Accept**
