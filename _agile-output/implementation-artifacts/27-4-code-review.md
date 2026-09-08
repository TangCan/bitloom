# Code Review: Story 27.4 ATDD golden + 后端无感知抽检

**Verdict:** Approve

## Findings

1. **Accepted (by design):** Golden is handwritten CRC-8/SMBUS-style poly `0x07` depth-16 literal vs `declare_*_mem_with_init_fn` / `generate_mem_init` (27.1 APIs).
2. **Accepted (by design):** Optional Chisel — Mem still `rhdl::E0901` (Epic 33); NFR36 covered on Verilog/FIRRTL emit text.
3. **Accepted (non-blocking):** NFR36 token check uses boundary-aware `Fn(`/`Fn (` to avoid false positives on identifiers like `…Fn (`.

## AC Trace

| AC | Result |
| ---- | ------ |
| Closure CRC/LUT vs handwritten → tick or emit equivalence (FR73) | pass (`.v` initial + FIRRTL mem-init + SyncReadMem tick) |
| Tests stable under `just test` / documented recipe | pass (`cargo test -p bitloom --test fr73_crc_lut_golden`; recipe in test module + language-surface + README) |
| Emit Verilog/FIRRTL (optional Chisel) has no closure/callback IR (NFR36) | pass (spot-check; Chisel Mem → E0901 only) |
| README / language-surface minimal Bitloom / `bitloom-prelude` example | pass |

## Verification

- `cargo test -p bitloom --test fr73_crc_lut_golden`
- `cargo test -p bitloom --test fr73_mem_init_generator --test fr73_module_factory --test fr73_hw_capture_diag`

**Accept**
