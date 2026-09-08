# Code Review: Story 28.2 组合逻辑内联可综合闭包

**Verdict:** Approve

## Findings

1. **Accepted (by design):** MVP API is `ElaborateSession::inline_comb_fn(dst, args, violations, span, |args| CombInline::…)` (+ `inline_comb_fn_marker`). Cap-R-60 check from 28.1 runs first; legal paths expand via existing `assign_*` — no new HIR closure nodes.
2. **Accepted (by design):** `CombInline` is an elaborate-time descriptor only; FrozenHir stores ordinary `AssignExpr` (NFR36). Violation tokens skip expand and still fail finish with E0143–E0145.
3. **Accepted (AD-18):** Incomplete if/else after inline still yields `rhdl::E0110` (builder unit + ATDD).
4. **Accepted (deferred):** Sequential inline → Story 28.3; full FR74/FR75/FR16 matrix → 28.4; automatic rustc body analysis → later.

## AC Trace

| AC | Result |
| ---- | ------ |
| Comb synthesizable closures; elaborate inline to ordinary HIR (FR75 / Cap-R-55) | pass (`inline_comb_fn` → `AssignExpr`) |
| Fixture elaborate → emit `.v` → tick matches handwritten golden | pass (`fr75_comb_inline_closure`) |
| No closure residue after freeze (NFR36) | pass (HIR assigns + emit spot-check) |
| Incomplete-assign comb rules still apply (AD-18) | pass (`rhdl::E0110`) |
| Call/respect SynthesizableClosure check from 28.1 | pass (check before expand; E0145 ATDD) |

## Verification

- `cargo test -p bitloom-builder inline_comb_fn`
- `cargo test -p bitloom --test fr75_comb_inline_closure`
- `cargo test -p bitloom --test fr74_synthesizable_closure_check`
- `cargo test -p bitloom-prelude --lib`

**Accept**
