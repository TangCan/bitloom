# Phase 24 IP support matrix

Support levels are cumulative: `catalogued` means identified, `locked` adds immutable source/tool identity, `compiled` adds a real tool invocation, `behavior-tested` adds an independent executable oracle, and `maintained` adds a required CI gate and named owner. `unsupported` and `not delivered` are product states, not skips.

| Scope | catalogued | locked | compiled | behavior-tested | maintained | Native / generated hierarchy | Evidence and boundary |
|---|---:|---:|---:|---:|---:|---|---|
| Bitloom core leaves: UART/GPIO32/Timer32/IRQ5 | yes | yes | yes | yes | yes | unsupported | Epic128 closeout and the FR201 required matrix |
| FR198 AXI-Lite four-peripheral system | yes | yes | yes | yes | yes | unsupported | direct, FIRRTL and Chisel RTL execute the same independent AXI oracle; finite formal and original synthesis are separate rows in `129-3-latest-results.json` |
| FR198 direct-CSR four-peripheral system | yes | yes | yes | yes | yes | unsupported | materially distinct no-bridge topology; direct/FIRRTL/Chisel each execute the original independent CSR oracle with 16,000 random transactions; see the Phase24 closeout evidence |
| External IP pilot: PULP `fifo_v3` (Epic130 / FR199-FR200) | yes | yes | yes | yes | yes | unsupported | Fixed 32-bit/depth8/fall-through0; real Bitloom HIR parent, independent RTL oracle, six original upstream cases ×100000 and isolated replay. See `epic-130-closeout.md` / `130-3-final-verification.md`. |

The core owner is Richard; Codex records the current execution evidence. The required gate is `just fr198-fr201-core-check`. Missing tools, timeout, formal UNKNOWN, zero transactions/assertions, empty waveform, or absent source hashes fail the row. These rows do not claim PPA, timing closure, physical signoff, multi-clock support, native hierarchical simulation, delivery of FR189 or clearance of NFR91. Phase24 scoped completion is recorded in [the closeout](phase24-closeout.md), with finite-formal and physical-signoff limits preserved.

External pilot owner: Richard. The required workflow job is `fr200-external-ip-pilot`; local entry is `just fr200-external-ip-pilot-check`. Source/wrapper/tool identities, license review and upgrade/deprecation obligations are described in [external FIFO pilot](external-fifo-pilot.md). The actual local commands passed; this is not a claim that remote CI or branch-protection configuration was executed. No native/generated hierarchical model or arbitrary parameter support is implied.

2026-09-23 supplemental core verification: both topologies across all three backends passed; request/reset checks and fresh independent evidence validation passed. Durable main/isolated artifacts survive cargo clean. Local commands were executed; remote CI was not run in this closeout.
