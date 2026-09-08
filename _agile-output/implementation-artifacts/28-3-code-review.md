# Code Review: Story 28.3 时序逻辑内联 + 所有权

**Verdict:** Approve

## Findings

1. **Accepted (by design):** MVP API is `ElaborateSession::inline_seq_fn(dst_reg, args, synth_violations, ownership_violations, span, |args| SeqInline::…)` (+ `inline_seq_fn_marker`). Cap-R-60 then Cap-R-70; legal paths expand to ordinary `Reg.d` / `AssignExpr` — no new HIR closure nodes.
2. **Accepted (by design):** `SeqInline` (`Inc` / `Comb(CombInline)`) is elaborate-time only; FrozenHir stores ordinary sequential assigns (NFR36). Cap-R-70 uses stable **`rhdl::E0146`** (token + auto-detect second `Reg.d` in the same sequential process).
3. **Accepted (AD-4):** Cross-process multi-drive after expand still yields `rhdl::E0140`.
4. **Accepted (deferred):** Full FR74/FR75/FR16 diagnostic matrix → Story 28.4; automatic rustc borrow analysis → later.

## AC Trace

| AC | Result |
| ---- | ------ |
| Seq synthesizable closures; elaborate inline to ordinary HIR (FR75 / Cap-R-56) | pass (`inline_seq_fn` → `Reg.d` AssignExpr) |
| Fixture tick matches handwritten equivalent | pass (`fr75_seq_inline_closure` counter + acc) |
| Illegal extra mutable borrows diagnosed (Cap-R-70) | pass (`rhdl::E0146`) |
| Multi-drive freeze checks still cover expanded nets (AD-4) | pass (`rhdl::E0140`) |
| No closure residue (NFR36); SynthesizableClosure check respected | pass (emit spot-check + Cap-R-60 before expand) |

## Verification

- `cargo test -p bitloom-builder inline_seq_fn`
- `cargo test -p bitloom --test fr75_seq_inline_closure`
- `cargo test -p bitloom --test fr75_comb_inline_closure`
- `cargo test -p bitloom-prelude --lib`

**Accept**
