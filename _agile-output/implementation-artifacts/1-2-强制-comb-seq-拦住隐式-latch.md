# Story 1.2: 强制 comb/seq，拦住隐式 latch

Status: done

## Story

As a 硬件设计者,
I want 必须标注 `#[combinational]` / `#[sequential]`，并且组合路径写全,
so that 不会在综合里变成隐式 latch，也不会把 comb 和 seq 写串。

## Acceptance Criteria

1. 组合块对某输出有的分支没赋值 → `elaborate()` 失败，诊断指向缺口，不生成 latch
2. 组合写 `Reg.d`、时序驱动组合网 → 失败
3. 没有 comb/seq 标注的硬件过程不能进入 HIR

## Tasks / Subtasks

- [x] HIR：`Wire`/`Reg` 声明 + Comb/Seq 过程 AST
- [x] Builder：`begin_combinational` / `begin_sequential` + when/else 赋值集合交
- [x] 诊断：`rhdl::E01xx` latch / cross-drive
- [x] 宏：`#[combinational]` / `#[sequential]` → mark；`#[process]` 拒绝
- [x] 测试：完整赋、缺分支、comb→Reg.d、seq→wire、无标注

## Dev Agent Record

### Completion Notes List

- Latch analysis via begin_then/begin_else/end_if set intersection (E0110).
- Cross-drive: E0116 comb→Reg.d, E0114 seq→net, E0103 unmarked assign.
- trybuild: bare `#[process]` rejected.

### File List

- crates/rhdl-hir/src/lib.rs
- crates/rhdl-builder/src/lib.rs
- crates/rhdl-macro/src/lib.rs
- crates/rhdl-prelude/src/lib.rs
- examples/counter_ports/tests/ui/unmarked_process.rs(.stderr)
