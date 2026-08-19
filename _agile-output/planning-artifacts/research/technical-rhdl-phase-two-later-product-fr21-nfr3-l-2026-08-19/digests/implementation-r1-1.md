# Digest: HLS attach + surface thicken
accessed: 2026-08-19
assistant: fanout-3
HLS last: emit C/LLVM/XLS IR into Bambu/XLS schedulers; CIRCT HLS non-production.
Sequence: IR+comb/seq/ops+emit+sim → thicken surface → Mem → multi-clock → optional HLS.
rust-hdl .val/.next post-mortem: thin Verilog-shaped surface forced rewrite.
