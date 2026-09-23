# Story130.3 upstream parallel execution independent review

Reviewed `scripts/phase24-external-ip-upstream.py`, the exact locked `common_cells/test/fifo_tb.sv`, `common_verification/src/clk_rst_gen.sv`, `rand_verif_pkg.sv`, and `130-3-upstream-parallel-diagnostic1000.json`.

No blocking implementation finding for equivalent configuration/oracle/count coverage:

- The original `fifo_inst_tb` source, stimulus drivers, randomization assertions, and queue comparison assertion remain unchanged. Each compile includes `--timing --assert`; the scoped CONTASSINIT lint control does not disable assertions.
- Six independent instances retain depths 8/1/9 × FALL_THROUGH 0/1, DATA_WIDTH=8, and acceptance N_CHECKS=100000 each. No reduced count is eligible for acceptance.
- The harness uses the original clock/reset generator, TCLK=10ns, reset parameter10, TA=TCLK*1/4 and TT=TCLK*3/4. It wires the original instance and finishes only after its done output. No new driver, checker, or behavioral DUT is introduced.
- Source input hashes, Verilator/runtime hashes, tool hashes, generated harness text/hash, simulator executable hash, configuration, seed1303 and command results are recorded.
- Calibration records six cases ×1000 checks with successful assertions enabled, but explicitly diagnosticOnly=true and acceptanceEligible=false. This is timing evidence only.

Coverage equivalence does not mean identical randomized traces: each process starts its own RNG stream; the original monolithic run interleaves streams and keeps already-completed instances running until the last done. Parallel runs guarantee each original instance reaches the same required100000 queue comparisons, with the disclosed different RNG scheduling boundary. Verilator still excludes upstream fall-through SVA via the original source conditional; this limitation is unchanged. Width8 upstream verification supplements the separate width32 Bitloom wrapper oracle.

At review time the full parallel100000 and original monolithic100000 runs are active. This review does not claim their execution passed. Choosing parallel mode for acceptance requires successful full evidence for every case; retain the original monolithic execution record as a distinct run rather than relabeling it.
