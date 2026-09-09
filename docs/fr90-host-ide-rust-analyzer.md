# FR90 — 宿主 IDE / rust-analyzer 工作流

**Product:** Bitloom（`cargo bitloom`）。与 [samitbasu/rhdl](https://github.com/samitbasu/rhdl) 无关。

**合同：** Wave D **必做**宿主路径——用 **rust-analyzer**（或等价宿主 LSP）为 Bitloom **设计 crate** 提供 Rust 语言智能（补全、跳转、诊断）。**不**替代 Epic 44 / **FR99** 自研 Bitloom language-server（**Epic 44 closed** — `bitloom-lsp`；见 [`fr99-bitloom-lsp.md`](fr99-bitloom-lsp.md)）。Epic 39 **FR91 Path B** 仍为该 epic 的**历史**关闭路径（显式 defer）；Path B defer **不再**作为 Phase 12 FR99 完成口径。见 [`fr38-viz-lsp.md`](fr38-viz-lsp.md)。

## 前置

| 项 | 钉死值 |
| --- | --- |
| 工具链 | 仓库根 [`rust-toolchain.toml`](../rust-toolchain.toml) → **rustc 1.97.1**（见 [`nfr13-msrv-1.97.1.md`](nfr13-msrv-1.97.1.md)） |
| 设计依赖 | `[dependencies]` **仅** [`bitloom-prelude`](../crates/bitloom-prelude)（勿把 CLI 包 `bitloom` 加进设计依赖） |
| 宿主 LSP | **rust-analyzer**（VS Code / Cursor「Rust Analyzer」扩展，或同等编辑器宿主） |

安装提示（高阶）：本机已装 `rustup` 时，打开本仓库根目录即可按 `rust-toolchain.toml` 选用 1.97.1。编辑器侧安装并启用 **rust-analyzer**（VS Code / Cursor 扩展 id：`rust-lang.rust-analyzer`，或 JetBrains Rust 插件内置分析器），并将 workspace 根对准本仓库。

## 可复现步骤

1. **打开仓库根**为编辑器 workspace（含 `Cargo.toml` workspace 与 `rust-toolchain.toml`）。
2. 确认工具链：终端执行 `rustc --version`，应看到 **1.97.1**。
3. 打开任一 Bitloom **设计 crate** 源文件（见下方夹具），等待 rust-analyzer 索引结束：状态栏显示检查已完成 / idle（无持续红色 cargo check / proc-macro 失败）。
4. **验证补全（completion）：** 在 `use bitloom_prelude::` 或模块体内键入符号前缀，应出现 prelude / 本地符号补全。
5. **验证跳转（goto）：** 对 `Input`、`UInt`、`#[module]` 所用类型或本地 `fn` 使用「转到定义 / Go to Definition」，应跳到定义处。
6. **验证诊断（diagnostics）：** 故意引入类型错误或未使用项后保存，应看到 rustc / rust-analyzer **诊断**；修复后诊断消失。设计侧可综合诊断（`rhdl::E0xxx`）仍以 elaborate / `cargo bitloom build` / `cargo test` 为准——宿主 IDE 展示的是 **Rust 宿主** 诊断，不是硬件 netlist 语义服务。

可选对照：

```bash
cargo test -p counter_ports
cargo bitloom build --package counter_ports
```

## 夹具工程：`examples/counter_ports`

| 项 | 说明 |
| --- | --- |
| 路径 | [`examples/counter_ports`](../examples/counter_ports) |
| 角色 | 最小 Bitloom 设计 crate：`#[module]` + 时钟/复位/数据端口；只依赖 `bitloom-prelude` |
| 入口 | `src/lib.rs` 中 `CounterPorts` 与 `rhdl_elaborate()`（CLI elaborate 宿主入口；历史名 `rhdl_*` / `bitloom_prelude::rhdl::module`，产品品牌仍为 **Bitloom**） |
| IDE 用法 | 在 workspace 内打开该包的 `src/lib.rs`，按上节步骤验证补全 / 跳转 / 诊断 |

脚手架替代路径：树外可用 `cargo bitloom new <name>` 生成同样「仅 `bitloom-prelude`」的最小设计 crate，再用同一 rust-analyzer 步骤验证。

## 边界（宿主智能 ≠ 硬件 LSP）

| 能力 | 本故事（FR90） | 非本故事 |
| --- | --- | --- |
| Rust 补全 / 跳转 / rustc 诊断 | ✅ rust-analyzer 宿主路径 | — |
| 硬件语义 / netlist 符号与 elaborate 语义 LSP | ❌（非 FR90） | ✅ **FR99** / Epic 44 **closed** — [`fr99-bitloom-lsp.md`](fr99-bitloom-lsp.md) |
| Bitloom 自研 language-server 二进制 | ❌（非 FR90；本页不交付） | ✅ **FR99** `bitloom-lsp`（Epic 44 closed）；Epic 39 FR91 Path B 仅为历史 defer |
| 层次 / 时序 HTML | ❌ 不计入 LSP | 见 [`fr38-viz-lsp.md`](fr38-viz-lsp.md) / [`fr38-wave.md`](fr38-wave.md) |

**不得**把本页或层次 HTML 写成「装了 rust-analyzer 就等于 FR99 完成」。FR90 宿主路径**仍可用**，且 **does not substitute** for FR99。

## 相关链接

- [`fr38-viz-lsp.md`](fr38-viz-lsp.md) — 层次 HTML；HTML ≠ LSP；FR99 / Epic 44 closed
- [`fr99-bitloom-lsp.md`](fr99-bitloom-lsp.md) — 自研 Bitloom LSP（FR99）
- [`nfr13-msrv-1.97.1.md`](nfr13-msrv-1.97.1.md) — MSRV / 工具链
- [`../README.md`](../README.md) — 快速开始与文档索引
- NFR14 Epic 39：`_agile-output/implementation-artifacts/nfr14-risk-epic39-ide-multiview.md`（FR91 Path B 历史）
- NFR14 Epic 44：`_agile-output/implementation-artifacts/nfr14-risk-epic44-full-elaborate-lsp.md`（closed）
- deferred ledger：[`../_agile-output/implementation-artifacts/deferred-work.md`](../_agile-output/implementation-artifacts/deferred-work.md)

## 验证（ATDD）

```bash
cargo test -p bitloom --test fr90_host_ide_rust_analyzer
```
