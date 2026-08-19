# Story 1.1: 声明带方向端口的模块并 elaborate

Status: done

## Story

As a 硬件设计者,
I want 用合法 Rust 声明带 `Input`/`Output` 的模块并 `elaborate()`,
so that 在任何后端之前就有一张冻结电路图。

## Acceptance Criteria

1. **Given** 空的 rhdl 工作区 **When** 写带 `Input<UInt<8>>` / `Output<UInt<8>>`、`Clock`、`Reset` 的模块并 `Elaboratable::elaborate()` **Then** 得到 `FrozenHir`（层次 AST）
2. 未冻结 HIR 不能从 `rhdl-hir` 外拿到
3. 设计 crate 只依赖 `rhdl-prelude`；宏只展开到 builder
4. 裸 `UInt` 当端口无法通过（类型系统拒绝）

## Tasks / Subtasks

- [x] Cargo workspace：`rhdl-hir`, `rhdl-builder`, `rhdl-macro`, `rhdl-prelude`（AD-6 依赖方向）
- [x] 类型：`Bool`/`Bits`/`UInt`/`SInt`/`Clock`/`Reset` + `Input`/`Output`
- [x] `FrozenHir` 层次 AST + 私有 unfrozen + `Diagnostics`
- [x] `Elaboratable` + `#[rhdl::module]` 经 builder 会话
- [x] example 设计 crate + 测试
- [x] 根 `Justfile`：`just test` → `cargo test --workspace`

## Dev Notes

### Architecture compliance (must follow)

- AD-1/7: only `elaborate`/`import` allocate; return `Result<FrozenHir, Diagnostics>`; freeze private
- AD-6: DES→PRE→BLD→HIR；MAC→BLD only；no macro→hir
- AD-12: hierarchical Circuit/Module AST; ground types UInt/SInt/Clock/Reset
- AD-13: builder is session over `&mut` unfrozen Hir
- AD-18: ports are `Input`/`Output` wrappers
- Stack: edition 2024, MSRV 1.97.1
- Do NOT: Verilog emit, CLI, comb/seq latch checks, multi-drive, FIRRTL (later stories)

### File structure

```
rhdl/
  Cargo.toml          # workspace
  Justfile
  rust-toolchain.toml # 1.97.1
  crates/
    rhdl-hir/
    rhdl-builder/
    rhdl-macro/
    rhdl-prelude/
  examples/
    counter_ports/    # design crate → rhdl-prelude only
```

### Testing

- Unit: FrozenHir from elaborate with directed ports
- trybuild: bare UInt port rejected (`PortField` not implemented)

### References

- [Source: `_agile-output/planning-artifacts/epics.md` Story 1.1]
- [Source: `ARCHITECTURE-SPINE.md` AD-1,6,7,12,13,18]
- [Source: `_agile-output/specs/spec-rhdl/language-surface.md`]
- [Source: `AGENTS.md`]

## Dev Agent Record

### Agent Model Used

Composer (Auto)

### Completion Notes List

- Implemented workspace + crates; AD-6 dependency direction honored.
- `Hir` is `pub(crate)`; designs use `#[rhdl::module]` → builder → `seal_from_builder`.
- Code review (lightweight): AC met; no latch/multi-drive yet (deferred to 1.2/1.4).
- Regression: `cargo clean && cargo fmt --all && just test` passed.

### File List

- Cargo.toml, Justfile, rust-toolchain.toml, .gitignore
- crates/rhdl-hir/**
- crates/rhdl-builder/**
- crates/rhdl-macro/**
- crates/rhdl-prelude/**
- examples/counter_ports/**
- _agile-output/implementation-artifacts/1-1-声明带方向端口的模块并-elaborate.md
- _agile-output/implementation-artifacts/sprint-status.yaml
