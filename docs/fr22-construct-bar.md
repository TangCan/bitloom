//! Phase-2 FR22 construct bar (AD-20). `Bundle` / `Vec` remain out of scope.

# Allowed (must elaborate → emit `.v` → `tick`)

- Forced `#[combinational]` / `#[sequential]` (or builder `begin_combinational` / `begin_sequential`)
- Branching via builder `begin_then` / `begin_else` / `end_if` (latch-complete)
- Same-width binary ops: `assign_add(dst, lhs, rhs)`
- Same-width connect: `assign_net(dst, src)`
- Explicit `pad_to` / `trunc_to`
- Sync `Reg` + `assign_reg_d_inc` / `assign_reg_d_from` under AD-15 reset

# Explicitly deferred (must not silently work)

- `Bundle`, `Vec<T, N>`
- Multi-clock / phantom domains (Epic 7 / AD-22)
