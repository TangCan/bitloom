# Code Review: Story 27.3 捕获硬件引用诊断 + FR16 回归

**Verdict:** Approve

## Findings

1. **Accepted (by design):** Option C — documented `HwCaptureRef` + `assert_no_hw_capture` / `reject_hw_capture` rather than rustc capture analysis; Wire/Reg map via `HwCaptureRef::wire` / `::reg`.
2. **Accepted (by design):** New stable code **`rhdl::E0142`** for hardware-ref capture in generator/factory context; FR16 capturing closure remains **`rhdl::E0141`** (not reclassified).
3. **Accepted (deferred):** Automatic typed Wire handle capture detection → later when surface gains typed refs; comb/seq inlined closures → Epic 28; CRC golden → 27.4.

## AC Trace

| AC | Result |
| ---- | ------ |
| Negative: capturing hardware signal refs (or documented illegal capture) → stable diagnostic failure (FR73, NFR35, AD-18) | pass (`rhdl::E0142`, `fr73_hw_capture_diag`) |
| FR16 cycle-accurate “capturing closure” negative still red | pass (`E0141` regression test) |
| Docs explain elaborate-time non-capturing vs capturing (NFR35) | pass (`language-surface.md` section + fr22 bar) |

## Verification

- `cargo test -p bitloom-builder hw_capture`
- `cargo test -p bitloom-builder fr16_capturing`
- `cargo test -p bitloom --test fr73_hw_capture_diag`
- `cargo test -p bitloom --test fr73_mem_init_generator --test fr73_module_factory`
- `cargo test -p bitloom-prelude --lib`

**Accept**
