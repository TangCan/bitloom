# Phase 24 IP support matrix

Support levels are cumulative: `catalogued` means identified, `locked` adds immutable source/tool identity, `compiled` adds a real tool invocation, `behavior-tested` adds an independent executable oracle, and `maintained` adds a required CI gate and named owner. `unsupported` and `not delivered` are product states, not skips.

| Scope | catalogued | locked | compiled | behavior-tested | maintained | Native / generated hierarchy | Evidence and boundary |
|---|---:|---:|---:|---:|---:|---|---|
| Bitloom core leaves: UART/GPIO32/Timer32/IRQ5 | yes | yes | yes | yes | yes | unsupported | Epic128 closeout and the FR201 required matrix |
| FR198 AXI-Lite four-peripheral system | yes | yes | yes | yes | yes | unsupported | direct, FIRRTL and Chisel RTL execute the same independent AXI oracle; finite formal and original synthesis are separate rows in `129-3-latest-results.json` |
| FR198 direct-CSR four-peripheral system | yes | yes | yes | yes | yes | unsupported | materially distinct no-bridge topology, executed by `fr198_direct_csr_four_leaf_hierarchy_emits_and_runs` |
| External IP pilot (Epic130 / FR199-FR200) | no | no | no | no | no | not delivered | Epic130 remains open/backlog; Story129.3 and FR201 **核心部分** do not promote this row |

The core owner is Richard; Codex records the current execution evidence. The required gate is `just fr198-fr201-core-check`. Missing tools, timeout, formal UNKNOWN, zero transactions/assertions, empty waveform, or absent source hashes fail the row. These rows do not claim PPA, timing closure, physical signoff, multi-clock support, native hierarchical simulation, FR189, NFR91, or complete Phase24 delivery.
