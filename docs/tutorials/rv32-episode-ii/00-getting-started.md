# Chapter 0 — 工具链与包路径

## MSRV

钉死 **rustc 1.97.1** / edition 2024（见仓库 `rust-toolchain.toml`）。

## 本集用到的包

| 包 | 角色 |
|----|------|
| `examples/rv32_core` | Episode I 单周期；**ISA / 立即数冻结**仍以 `SUBSET.md` 为准 |
| `examples/rv32_pipe` | Episode II 五级核（`EpisodeIIPipe`）；见 `PIPE.md` |

设计 `[dependencies]` 仅 **`bitloom-prelude`**。仿真用 **`bitloom-sim`**（仅 `[dev-dependencies]`）。

## 路径 A：真独立（不必 clone）

```bash
rustup toolchain install 1.97.1
cargo install bitloom
cargo bitloom new my_pipe
# 设计只依赖 bitloom-prelude；仿真：
#   cargo add bitloom-sim --dev
cargo bitloom build --package my_pipe --manifest-dir my_pipe --out-dir out
```

完整流水核在上游 `examples/rv32_pipe`；可对照抄写。**不得**把 clone 当成唯一入口。

## 路径 B：贡献者 monorepo

```bash
git clone https://github.com/TangCan/bitloom.git
cd bitloom
cargo test -p rv32_pipe
cargo test -p rv32_core
cargo run -p bitloom -- build --package rv32_pipe --manifest-dir . --out-dir /tmp/rv32-pipe-out
```

## 仿真合同

与 Episode I 相同：`bitloom_sim::Sim` + `tick`。取指仍是 **harness `instr`**（见 `SUBSET.md` / `PIPE.md`），不是片上 SyncReadMem I-fetch。

验收：

```bash
cargo test -p rv32_pipe
cargo run -p bitloom -- build --package rv32_pipe --manifest-dir . --out-dir /tmp/rv32-pipe-out
```
