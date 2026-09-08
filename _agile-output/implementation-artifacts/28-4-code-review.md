# Code Review: Story 28.4 诊断矩阵 + FR16 共存 ATDD

**Verdict:** Approve

## Findings

1. **Accepted (by design):** Single ATDD matrix `fr74_fr75_fr16_coexist_matrix.rs` consolidates positive comb/seq inline + Cap-R-60 legal path, negatives E0141–E0146 / Wire capture, and NFR36 emit spot-check; deep goldens remain in sibling `fr74_*` / `fr75_*` / `fr73_hw_capture_diag` fixtures (presence asserted).
2. **Accepted (NFR35):** FR16 capturing closure stays `rhdl::E0141` and is not reclassified as `E0142`; synthesizable-path codes E0143–E0146 coexist.
3. **Accepted (docs):** README + language-surface include comb/seq minimal examples and explicit “do not capture Wire” warning; fr22 marks matrix delivered.
4. **Accepted (deferred):** Automatic rustc closure-body / borrow analysis → later when macros gain typed inline.

## AC Trace

| AC | Result |
| ---- | ------ |
| ATDD matrix: legal pass; heap/capture/illegal IO fail; FR16 still fail (FR74/75, NFR35) | pass |
| Stable under documented recipe / `just test` path | pass (`cargo test -p bitloom --test fr74_fr75_fr16_coexist_matrix`) |
| Spot-check emit has no closure IR (NFR36) | pass (Verilog + FIRRTL on legal comb/seq) |
| User docs: comb/seq examples + do-not-capture-Wire | pass (README + language-surface) |

## Verification

- `cargo test -p bitloom --test fr74_fr75_fr16_coexist_matrix`
- `cargo test -p bitloom --test fr74_synthesizable_closure_check --test fr75_comb_inline_closure --test fr75_seq_inline_closure --test fr73_hw_capture_diag`

**Accept**
