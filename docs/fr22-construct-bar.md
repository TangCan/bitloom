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
- SynthesizableClosure constraints + Cap-R-60 check hook (FR74): `check_synthesizable_closure` / `reject_unsynthesizable_closure` — E0143 heap / E0144 runtime capture state / E0145 impure
- Comb inline synthesizable closures (FR75 / Cap-R-55): `inline_comb_fn` / `inline_comb_fn_marker` + `CombInline` — Cap-R-60 then expand to ordinary `assign_*` (NFR36); incomplete-assign still `rhdl::E0110`
- Seq inline synthesizable closures (FR75 / Cap-R-56 / Cap-R-70): `inline_seq_fn` / `inline_seq_fn_marker` + `SeqInline` — Cap-R-60 + Cap-R-70 (`rhdl::E0146` illegal mutable borrow / second Reg.d) then expand to ordinary `Reg.d`; cross-process multi-drive still `rhdl::E0140` (AD-4)

# Explicitly deferred (must not silently work)

- `Bundle`, `Vec<T, N>`
- Multi-clock / phantom domains (Epic 7 / AD-22)
- Capturing Wire/Reg into elaborate-time generator/factory → `rhdl::E0142` (`assert_no_hw_capture` / `HwCaptureRef`); FR16 capturing closure stays `rhdl::E0141`
- Full FR74/FR75/FR16 diagnostic matrix ATDD → Story 28.4
