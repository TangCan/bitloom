//! Phase-2 FR22 construct bar (AD-20). `Bundle` / `Vec` remain out of scope.

# Allowed (must elaborate → emit `.v` → `tick`)

- Forced `#[combinational]` / `#[sequential]` (or builder `begin_combinational` / `begin_sequential`)
- Branching via builder `begin_then` / `begin_else` / `end_if` (latch-complete analysis; not data-dependent HIR)
- Data-dependent select: `assign_mux(dst, sel, t, f)`, compare `assign_eq`, constants `assign_lit`
- Same-width binary ops: `assign_add(dst, lhs, rhs)`
- Same-width connect: `assign_net(dst, src)`
- Explicit `pad_to` / `trunc_to`
- Sync `Reg` + `assign_reg_d_inc` / `assign_reg_d_from` under AD-15 reset
- SyncReadMem: `declare_sync_read_mem` + `assign_mem_write` / `assign_reg_d_mem_read`
- Elaborate-time Mem/ROM init (FR73): `declare_mem_with_init_fn` / `generate_mem_init` — not comb/seq inlined closures
- Elaborate-time module factory (FR73 / Cap-R-53): `generate_instances` / `generate_instances_from` + `GeneratedInstance` — dissolves to Instance/Connect before freeze

# Explicitly deferred (must not silently work)

- `Bundle`, `Vec<T, N>`
- Multi-clock / phantom domains (Epic 7 / AD-22)
- Capturing Wire/Reg into elaborate-time generator/factory → `rhdl::E0142` (`assert_no_hw_capture` / `HwCaptureRef`); FR16 capturing closure stays `rhdl::E0141`
