# Code review — Story 17.2

**Verdict: approve**

## Findings

1. **(pass)** Unique fetch strategy **(b) harness `instr`** locked in `examples/rv32_core/SUBSET.md` with explicit no-silent-mix vs SyncReadMem IF (FR69).
2. **(pass)** CPI/teaching limits and deferral conditions for (a) documented; AD-21 / 15.1 referenced; 17.4 bound to (b).
3. **(pass)** No five-stage pipeline implementation in this story (docs + cross-refs only); 17.1 FEASIBILITY PASS preserved.
4. **(info)** On-chip I-fetch remains a later story; residual risk is tutorial drift if Episode II text forgets the harness contract — mitigated by SUBSET + COMPLIANCE pointers.

No blocking defects.
