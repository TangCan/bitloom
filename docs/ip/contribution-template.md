# Bitloom IP contribution template

Use this checklist for every core or external IP contribution. A checked box requires a linked artifact; prose claims alone are not evidence.

## Ownership and behavior contract

- **owner:** Name the long-term maintainer and the implementation reviewer.
- **behavior contract:** Freeze ports, widths, reset priority, ready/valid commit points, register side effects, error codes, and unsupported behavior.
- **independent oracle:** Describe hand-written expected addresses and state transitions that do not call the DUT generator or next-state implementation.
- **two compositions:** Identify two materially different compositions. Renaming one topology, replacing leaves with dummy registers, or compile-only reuse does not qualify.

## Execution matrix

- **direct:** Execute generated Verilog with non-zero assertions, transactions, and waveform.
- **FIRRTL:** Lower through the pinned firtool, then execute the resulting RTL with the same oracle.
- **Chisel:** Compile through the pinned Scala/JVM/Chisel/firtool path, then execute the resulting RTL with the same oracle.
- **formal:** Record bounded prove, independent cover, assumptions/depth, and a negative control that produces a real counterexample.
- **synthesis:** Run the uninstrumented original RTL through Yosys `check -assert`, `synth`, and `stat`; do not call this timing, PPA, or physical signoff.
- **compatibility:** Run the applicable SemVer packages and list every public symbol addition against FR142. State retained legacy ports and APIs.

## Provenance and evidence

- **source:** Record repository, immutable revision, recursive dependency manifest, ordered file list, include paths, macros, and sha256.
- **license:** Archive the exact license/NOTICE text and sha256; name the reviewer responsible for redistribution approval.
- **seed:** Save every deterministic and randomized seed plus the frozen transaction budget.
- **waveform:** Save a non-empty waveform for each behavioral backend and state the inspected top.
- **exit code:** Save command argv, start/end UTC, exit code, timeout, tool identity, and raw log for each row.
- **sha256:** Hash DUT source, wrapper, testbench, evidence manifest, and any fetched external source.

## Support declaration

Choose one honest level from `catalogued`, `locked`, `compiled`, `behavior-tested`, or `maintained`. A higher level requires every lower level. Mark native/generated simulation separately from RTL; unsupported layers must remain explicit. Core acceptance never promotes an external IP row without its own locked source, adapter, behavior, and maintenance evidence.
