# Chapter 0 — 工具链与真独立跟练

## MSRV

钉死 **rustc 1.97.1** / edition 2024（见仓库 `rust-toolchain.toml`）。

## 路径 A：真独立（不必 clone monorepo）

```bash
rustup toolchain install 1.97.1
cargo install bitloom
cargo bitloom new my_cpu
# 设计只依赖 bitloom-prelude；仿真：
#   cargo add bitloom-sim --dev
cargo bitloom build --package my_cpu --manifest-dir my_cpu --out-dir out
```

本教程的完整 Episode I 核在上游仓库的 `examples/rv32_core`；你可以对照实现，或在自己的 crate 里逐步抄写。

## 路径 B：贡献者 monorepo

```bash
git clone https://github.com/TangCan/bitloom.git
cd bitloom
cargo test -p rv32_core
cargo run -p bitloom -- build --package rv32_core --manifest-dir . --out-dir /tmp/rv32-out
```

**不得**把 clone 当成唯一入口；日常学习优先路径 A。

## 仿真合同

周期精确仿真在 `cargo test` 里：`bitloom_sim::Sim` + `tick`（`bitloom-sim` 仅 `[dev-dependencies]`）。
