# Buffered UART CSR

`bitloom_prelude::ip::UartCsr` implements fixed 8N1, LSB first, with independent four-byte TX and RX register FIFOs. It reuses `CsrBlock` and two instances of the same `ParamSyncFifo<8,4>` definition. Designs depend only on `bitloom-prelude`. Bitloom is unrelated to `samitbasu/rhdl`.

The exact [standalone/shared definition example](uart-csr-example.rs) runs with only the prelude and generates the checked-in [C header](uart-csr-registers.h) and [register table](uart-csr-registers.md). Call `define_module` for each desired name in the same `ElaborateSession`, instantiate with the normal builder, then call `finish` once. Repeated definitions reuse their complete identity; existing name/width/direction/clock-type diagnostics apply. `Elaboratable::elaborate` is an independent top-level convenience with the same definition body.

The fifteen ports are `clk: Clock`, `rst: Reset`, CSR inputs `req_valid:1`, `write:1`, `addr:16`, `wdata:32`, `wstrb:4`, `rsp_ready:1`; CSR outputs `req_ready:1`, `rsp_valid:1`, `rdata:32`, `error:2`; and `rx:1` input, `tx:1`, `raw_events:4` outputs. Numeric ports are UInt. All state uses the same clock and synchronous active-high reset.

| Local offset | Register | Meaning |
|---|---|---|
| 00 | CTRL RW | bit0 enable |
| 04 | BAUD_DIV RW | all32 bits; bit period P=DIV+1 ACLKs |
| 08 | STATUS RO | bits0 RX nonempty,1 TX full,2 TX busy,3 RX busy |
| 0c | TX_DATA WO | low byte enqueue |
| 10 | RX_DATA RO | low byte read and dequeue |
| 14 | EVENT W1C | bits0 RX arrival,1 TX dequeue,2 overflow,3 framing |

CSR configuration and events reset to zero. Holes, high aliases, unaligned accesses, RO writes and WO reads return SLVERR with zero response data. Writes use little-endian WSTRB; reserved bits read zero. A legal zero effective mask returns OKAY without side effects, including a full TX FIFO with byte0 unselected. Commit is exactly a nonreset edge with req_valid && req_ready. Its response is a stored snapshot, held under backpressure; response consumption cannot refill the slot that edge. FIFO side effects occur on commit, never on response consumption.

Enable requires DIV>=3. While either serial engine is active, effective DIV writes are rejected, even to the same value; effective CTRL writes that change enable are rejected. Same-enable CTRL and zero effective masks remain legal. Checks use the merged candidate. Successful idle configuration applies on that edge to a new TX launch or RX falling-edge detection. Active frames retain their own divider. Disabling prevents launches without flushing queues; software can prefill TX or drain RX while disabled.

TX idle is high. Dequeue emits the start bit that edge, then eight data bits and one stop bit, each held for P clocks. A subsequent queued frame may have one extra idle cycle. Full-width countdown reloads DIV, so ffffffff means 2^32 clocks per bit without overflow. Tests at the extreme values distinguish bounded launch and injected near-endpoint state from billions of cycles of serial execution.

RX always consumes the second synchronizer stage. Both stages and history reset to zero. Let e be the edge whose pre-edge sync2/history detects falling, H=floor(P/2). Confirm start at e+H, sample data[k] at e+H+(k+1)P, and stop at e+H+9P. An invalid start cancels without event; low stop discards and reports framing. Reset release at high or sustained low does not invent a falling edge. DIV3/4/5 and raw phases0.1/0.5/0.9 ACLK are covered by independent serial vectors, including frames with only one stop and no extra idle.

FIFO decisions use pre-edge full/empty. TX full rejects a write even with simultaneous dequeue; RX empty reads fail even when a byte arrives that edge. RX full drops an arriving valid frame and reports overflow even with a successful simultaneous software pop. A bad stop reports framing instead of RX arrival or overflow. Normal nonfull/nonempty simultaneous push/pop retains FIFO order.

`raw_events` is the pre-edge, reset-gated four-event pulse vector. The CSR leaf performs EVENT=(old & ~clear)|new, with set winning clear. Connect raw bit0 to IRQ1, raw bit1 to IRQ2, and raw bit2|bit3 to IRQ3 of `Irq`; leave IRQ0/4 for their other producers. Connecting sticky EVENT would incorrectly retrigger after IRQ clear. Events keep operating during bus backpressure.

Reset cancels pending responses, queued bytes and active frames and returns TX idle high. The verification ledger distinguishes queued cancellation, serial dequeue, completed transmission and active cancellation: already transmitted physical bits cannot be undone. External `aresetn` must be synchronously asserted and released by the system reset controller before conversion; inversion alone is not synchronization.

Verification uses real direct/FIRRTL/Chisel RTL, independent serial transmit/decode, actual RTL wire loopback, shared/renamed dual instances and actual UART→IRQ composition. Native Interpreter/Compiled and generated functional hierarchy remain explicitly unsupported; no separate nonhierarchical UART core is exposed. Formal safety/cover and original synthesis are separate evidence classes, not a complete UART protocol proof or PPA signoff. Logic simulation does not establish board MTBF, electrical margins or baud mismatch tolerance. Parity, flow control, fractional division, async FIFOs and BRAM are outside this interface. Old `UartTx`/`UartRx`, their eight-bit baud and VIP/handwritten FL semantics remain unchanged.

The four public symbols are explicitly recorded in [FR142](../public-api-1-0-surface.md) as a SemVer minor addition. No package version, tool pin or publication is changed. M3 closure requires the final seven-step story evidence; this UART does not deliver Epic129/130 or all Phase24, and FR189 remains deferred with NFR91 retained.
