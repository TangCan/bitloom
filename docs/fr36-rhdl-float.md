# FR36 — rhdl-float / SoftF16

`SoftF16` is a **host-only** golden model (FR36 minimal contract) with **round-ties-to-even** when converting from `f32`.
There is **no** SoftF16 float-operator → HIR → emit synthesizable path in-tree today.

Emit surfaces that carry SoftF16-shaped values treat them as a 16-bit bitvector (`Bits<16>` surface). That is **not** a synthesizable floating-point operator library.

## FR84 — Option B (explicit defer)

**Epic 35 / FR84 Path B (ADOPTED):** synthesizable SoftF16 lowering (SoftF16 → HIR → emit float ops) is **explicitly deferred**.

- **MUST NOT** claim SoftF16 synthesizable float / 可综合浮点 has been delivered.
- **MUST NOT** market host-only SoftF16 or `Bits<16>` bitvector emit as synthesizable float operators (NFR37).
- Historical Epic 10 / FR36 `done` = host RTE goldens only; **≠** FR84 depth close via Option A.

Close check: `cargo test -p bitloom --test fr84_softf16_explicit_defer`.
