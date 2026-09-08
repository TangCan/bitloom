# Code Review: Story 29.3 IP 生成器闭包定制

**Verdict:** Approve

**Scope:** `Crc8Lut` FR77 overlay API + `fr77_ip_generator_closure` ATDD + IP/docs handoff

## Findings

1. **Accepted (by design):** New `Crc8Lut` IP rather than adding Fn params to FR82 SyncFifo/UartTx — five-class baseline stays closure-free; overlay matches Epic 34→29.3 sequencing.
2. **Accepted (Epic 27 reuse):** Customization dissolves via `declare_sync_read_mem_with_init_fn` / plain `MemDecl.init` — no new HIR closure nodes.
3. **Accepted (D1):** `elaborate_with_table_fn(violations, f)` gates synthesizable leg with `SynthesizableClosureViolation` tokens; non-empty → clear E014x, no silent default.
4. **Accepted (NFR36):** ATDD spot-checks `.v` / FIRRTL / FrozenHir debug for closure/`Fn`/`||` residue.
5. **Accepted (deferred):** 29.4 viz sweep; streaming CRC / filter-coeff IP; not extending FR82 five with closures.

## AC Trace

| AC | Result |
| ---- | ------ |
| IP/generator API accepts closure customization (FR77 / Cap-R-63) | pass (`Crc8Lut::elaborate_with_table_fn`) |
| Fixture elaborate → emit/tick proves customization | pass (poly `0x1D` ≠ default; SyncReadMem latency-1 tick) |
| Without closure: documented default or clear error | pass (`DEFAULT_POLY=0x07`; violations → diagnostics) |
| No closure residue after freeze (NFR36) | pass (ATDD spot-check) |

## Verification

- `cargo test -p bitloom-prelude --lib crc8_lut`
- `cargo test -p bitloom --test fr77_ip_generator_closure`
- `cargo test -p bitloom --test fr82_ip_baseline_matrix`

**Approve** — mark `29-3-ip-生成器闭包定制: done`.
