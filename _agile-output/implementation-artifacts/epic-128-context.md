# Epic 128 Context: 可组合外设

<!-- Compiled from planning artifacts. Edit freely. Regenerate with compile-epic-context if planning docs change. -->

## Goal

Deliver composable Timer, event IRQ, GPIO CSR wrapper and buffered UART for FR197/M3, with explicit asynchronous-input and board-level limits. M0/M1/M2 are closed at baseline `b926629`; their bus/CSR fixtures do not deliver peripheral algorithms. Epic128 remains in progress and FR197/M3 unclaimed until all five stories and their evidence close. The current all-stories seven-step authorization supersedes historical M0-only restrictions; FR189 remains deferred, NFR91 remains open, and no push or publication is authorized.

## Stories

- Story 128.1: 外设 NFR14
- Story 128.2: Timer
- Story 128.3: 事件 IRQ
- Story 128.4: GPIO CSR wrapper
- Story 128.5: 缓冲 UART 与 M3 关闭

## Requirements & Constraints

- Scope: one clock, 16-bit byte addresses, 32-bit data, four byte strobes, GPIO32, Timer32, IRQ5, UART8N1 LSB-first, 32-bit baud divider and separate TX/RX 8-bit × 4 register FIFOs. No extra external IRQ, deep RAM, native hierarchy, new protocols, CPU, external-core integration or physical signoff. The full-system delivery belongs to Epic129.
- NFR14 must establish upstream constraints, owner, supported parameters, estimates, acceptance mapping, tools, failure actions and stop conditions before functional stories become ready. Richard owns maintenance; Codex implements and records evidence. Estimate 10.5–17 effective person-days, excluding optional integration reserve. Stop on unresolved architecture/tool blockers; do not silently reduce scope or reopen settled interface choices as undecided requirements.
- Preserve existing APIs, ports and handwritten functional models. Design crates depend only on `bitloom-prelude`. Register new stable symbols explicitly under FR142/SemVer minor; the risk story adds no product API. Keep Rust 1.97.1/edition2024 and existing firtool/Chisel pins. Each story follows create-story → ATDD → build → independent review → automate → clean/fmt/workspace regression → one commit.
- Save commands, versions, seeds, exit codes and logs. Separate Rust, actual RTL, synthesis, formal proof and cover results; emission, ignored tests and missing tools are not passes. Apply direct/Chisel/FIRRTL RTL coverage where supported, independent references, compatibility and CDC/reset checks. Actual hierarchical RTL is required; native/generated hierarchy stays explicitly unsupported. Safety must survive indefinite stalls; liveness needs stated fairness bounds. Synthesis does not establish PPA; logic simulation does not establish metastability or board reliability.

## Technical Decisions

- Use one ElaborateSession, shared module definitions and one final freeze; no stitched FrozenHir or alternate IR. Validate parameter identity, module graph, widths and directions before freeze. Internal reset is synchronous and high-active; external aresetn must already have synchronous assertion and release. Bridge and peripherals reset together; inversion is not synchronization.
- CSR transactions commit exactly once at non-reset `req_valid && req_ready`. Capture read/response snapshots at that edge; expose response next cycle and hold under backpressure, with no new commit or response-consumption refill. Autonomous state has one peripheral owner, not a duplicate CSR shadow. Candidate values and effective masks must permit rejection before commit.
- CSR is little-endian and four-byte aligned. RW merges selected bytes; reserved bits read zero and ignore writes. RO writes/WO reads fail SLVERR; failed reads return zero. Legal zero-effective-mask writes succeed without side effects or busy rejection. Reset dominates commits; all CSR reset values are zero and controllers disabled. UART/GPIO/Timer/IRQ windows start at 0x0000/0100/0200/0300, each size 0x100. Window holes/alignment/access errors give SLVERR, outside gives DECERR without high-address aliasing. No ID/version registers.
- Timer: CTRL(enable, periodic), COUNT32, COMPARE32, EVENT W1C at offsets 00/04/08/0c. Disabled count holds. Effective CTRL/COUNT/COMPARE writes suppress automatic count/match that edge; COUNT write wins. Otherwise compare incremented modulo-2^32 count; periodic match resets count, one-shot clears enable. COMPARE=0 matches only wraparound. Zero-mask writes and EVENT clears do not suppress counting; new match beats clear.
- IRQ: PENDING W1C, ENABLE RW, TEST WO and RAW RO at 00/04/08/0c, mask 0x1f. Inputs are pulses: Timer match, successful RX arrival, TX dequeue, overflow/framing, GPIO new-edge aggregate. Never feed sticky peripheral EVENT levels. Hardware/software set beats clear; enable only masks output `|(pending & enable)`. RAW snapshots hardware pulses, excluding TEST. Repeated events are not counted.
- GPIO32: DIR/OUT/IN/SET/CLEAR/RISE_EVENT at 00/04/08/0c/10/14. IN is synchronized RO; SET/CLEAR are byte-masked WO write-one operations; events are W1C. Two-stage input synchronization and history reset to zero. Rising edges use pre-edge DIR input selection; initial high generates an event after synchronization. IRQ uses new edges, not sticky state. Cover bit31, direction/write collisions and synchronization latency; no glitch filtering or pad/electrical guarantee.
- UART: CTRL/BAUD_DIV/STATUS/TX_DATA/RX_DATA/EVENT at 00/04/08/0c/10/14. STATUS reports RX nonempty, TX full, TX/RX busy; EVENT records arrival, dequeue, overflow and framing. TX idle is high. Disable prevents new frames but preserves FIFOs; reset clears them. Busy divider writes or enable changes fail; validate merged values, allowing disabled idle DIV 0..2 but requiring DIV≥3 when enabled. One bit lasts DIV+1 clocks; DIV=0xffffffff requires 2^32-cycle arithmetic, not overflowing a 32-bit helper.
- RX uses two-stage synchronization, falling-edge start detection, half-bit confirmation and centered data/stop sampling. Bad stop drops the frame with framing event; full RX drops new data with overflow. TX dequeue generates space event. Pre-commit full/empty controls TX write/RX read errors even with simultaneous dequeue/arrival; unselected TX byte0 never pushes or fails for fullness. Ordinary nonfull/nonempty simultaneous push/pop is allowed. Verify real loopback plus independent decoding, phase/baud boundaries, collisions and stalled side effects; label any arithmetic/formal extreme-divider evidence separately from full-duration simulation.

## Cross-Story Dependencies

- 128.1 requires 125.3/M0; current M1/M2 supply verified composition, FIFO and CSR foundations.
- 128.2 and 128.3 each require 128.1 and 127.2.
- 128.4 requires 128.3 and 126.2; 128.5 requires 128.3 and 126.4.
- M3 closure requires all 128.1–128.5 done, including Timer/GPIO even if UART starts first. Record NFR14 closeout, requirement mapping and actual support matrix.
- 129.2 requires 129.1, 127.4, 128.2, 128.4 and 128.5. State-gate automation does not replace independent verification of these cross-story dependencies.
