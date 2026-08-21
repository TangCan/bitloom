# Bitloom

Rust 嵌入式 RTL HDL：设计是**可执行生成器**。`cargo bitloom build` 在本机 elaborate 得到冻结电路图（`FrozenHir`），再降到 Yosys 友好的 Verilog，并可在 `cargo test` 里做周期精确 `tick`。

## 身份与发布（请先读）

**本仓库与 [samitbasu/rhdl](https://github.com/samitbasu/rhdl) 无关。** 那是另一个独立项目。

| | 本项目 |
|---|---|
| Git 仓库名 | 可以叫 `rhdl` |
| 公开产品名 | **Bitloom** |
| crates.io **发布名** | **`bitloom`**（CLI 二进制 `cargo-bitloom` → `cargo bitloom`） |
| **禁止**发布 | `rhdl`、`rhdl-bits`（名称已被占用或保留） |

文档、徽章与发布说明不得暗示本工具链以 crates.io 包名 `rhdl` 发布。

## 快速开始（真独立；不必 clone）

```bash
rustup toolchain install 1.97.1
cargo install bitloom
cargo bitloom new blink
cargo bitloom build --package blink --manifest-dir blink --out-dir out
```

设计 crate 只依赖 **`bitloom-prelude`**（不要把 CLI 包 `bitloom` 加进 `[dependencies]`）。与 **bitbloom** 等无关拼写无关；与 [samitbasu/rhdl](https://github.com/samitbasu/rhdl) 无关。

- **工具链：** `rust-toolchain.toml` 钉死 **rustc 1.97.1** / edition 2024
- **测试（贡献者）：** `just test`（或 `cargo test --workspace`）
- **ClockDomain / CDC（FR52）：** 产品叙事与夹具见 [`examples/clockdomain_skel`](examples/clockdomain_skel)（`bind_domain` / `mark_cdc_bridge` / `rhdl::E0220`；全局 `Sim::tick` 为按域 tick 的 MVP 等价）

### 贡献者：在 monorepo 里跑示例

```bash
git clone https://github.com/TangCan/bitloom.git
cd bitloom
cargo run -p bitloom -- build --package counter_ports --out-dir /tmp/bitloom-out --manifest-dir .
```

多包发布与 Trusted Publishing：见 [`docs/crates-io-publish-bitloom.md`](docs/crates-io-publish-bitloom.md)。

## 教学 RV32

公开产品名 **Bitloom**。与 [samitbasu/rhdl](https://github.com/samitbasu/rhdl) 无关。

| 集 | 教程 | 示例核 | 范围摘要 |
|----|------|--------|----------|
| **Episode I** | [`docs/tutorials/rv32-episode-i/`](docs/tutorials/rv32-episode-i/README.md) | `examples/rv32_core` | 裁剪 RV32I 单周期；无 SoC/MMU/Linux |
| **Episode II** | [`docs/tutorials/rv32-episode-ii/`](docs/tutorials/rv32-episode-ii/README.md) | `examples/rv32_pipe`（可选 CSR：`rv32_priv`） | 立即数冻结 → 五级 + 转发 + load-use + 分支 flush；**可选** Zicsr/M-trap 教学最小集（NFR32） |

**非目标（两集共用）：** cache / MMU / Linux Softcore；动态分支预测；完整 Privileged / arch-test。VexRiscv **仅对照**，不作第一路径。CSR/trap **不**阻塞 Epic 17 流水完成定义。

延伸阅读：Harris DDCA Ch.7；FemtoRV `FROM_BLINKER_TO_RISCV`；对照 PicoRV32/SERV。子集合规是**最小过滤器**（见 `examples/rv32_core/COMPLIANCE.md`），未宣称 arch-test。

### 可选：独立仿真（`bitloom-sim`）

设计 crate 的 `[dependencies]` 仍只有 `bitloom-prelude`。需要 `tick` / VCD 时：

```bash
cargo add bitloom-sim --dev
```

```rust
#[cfg(test)]
mod sim {
    use bitloom_hir::PortValues;
    use bitloom_prelude::Elaboratable;
    use bitloom_sim::Sim;
    use super::MyTop; // your Elaboratable type

    #[test]
    fn tick_once() {
        let mut sim = Sim::new(MyTop::elaborate().unwrap());
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        sim.set_inputs(pv);
        sim.tick();
    }
}
```

勿把 sim 放进 `[dependencies]`。

## 文档在哪

| 文档 | 路径 |
|------|------|
| 阶段一产品合同 | `_agile-output/specs/spec-rhdl/SPEC.md` |
| 阶段二需求（FR21–FR40） | `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md` |
| 架构脊柱（AD-1…AD-26） | `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md` |
| Epic / Story | `_agile-output/planning-artifacts/epics.md` |
| Sprint 状态 | `_agile-output/implementation-artifacts/sprint-status.yaml` |
| HIR→源码再生（仅调试） | [`docs/hir-to-source-debug-only.md`](docs/hir-to-source-debug-only.md) |
| 手写 bridge / abstraction / both | [`docs/fr29-bridge-abstraction-both.md`](docs/fr29-bridge-abstraction-both.md) |
| 双视图等价检查 | [`docs/fr30-dual-view-equiv.md`](docs/fr30-dual-view-equiv.md) |
| 可选 FST | [`docs/fr31-optional-fst.md`](docs/fr31-optional-fst.md) |
| tick 引擎 | [`docs/fr32-tick-engines.md`](docs/fr32-tick-engines.md) |
| C ABI cdylib | [`docs/fr33-c-abi.md`](docs/fr33-c-abi.md) |
| 仿真覆盖率 | [`docs/fr34-sim-coverage.md`](docs/fr34-sim-coverage.md) |
| Chisel 可编译生成（FR28） | [`docs/fr28-chisel-best-effort.md`](docs/fr28-chisel-best-effort.md) |
| Chisel / `.fir` 反向导入（FR46） | [`docs/fr46-chisel-import.md`](docs/fr46-chisel-import.md) |
| `import` CLI + 混合夹具 | [`docs/fr40-cli-verbs.md`](docs/fr40-cli-verbs.md) · [`examples/chisel_mixed`](examples/chisel_mixed) |
| HLS 产品路径（**支持** · FR35/FR50 · Bambu 2024.10） | [`docs/fr35-hls.md`](docs/fr35-hls.md) · 烟测 [`scripts/hls-smoke.sh`](scripts/hls-smoke.sh) |
| Formal/SVA | [`docs/fr39-formal-sva.md`](docs/fr39-formal-sva.md) |
| Analog/InOut | [`docs/fr27-analog-inout.md`](docs/fr27-analog-inout.md) |
| rhdl-float | [`docs/fr36-rhdl-float.md`](docs/fr36-rhdl-float.md) |
| IP / 黑盒 | [`docs/fr37-ip-box.md`](docs/fr37-ip-box.md) |
| HIR HTML / LSP（层次；LSP deferred） | [`docs/fr38-viz-lsp.md`](docs/fr38-viz-lsp.md) |
| 时序 / 波形产品入口 | [`docs/fr38-wave.md`](docs/fr38-wave.md) |
| UJ-6 可视化半程跟练 | [`docs/tutorials/uj6-visualization.md`](docs/tutorials/uj6-visualization.md) |
| 额外 CLI | [`docs/fr40-cli-verbs.md`](docs/fr40-cli-verbs.md) |
| 多平台 firtool | [`docs/nfr11-firtool-platforms.md`](docs/nfr11-firtool-platforms.md) |
| MSRV 1.97.1 (NFR13) | [`docs/nfr13-msrv-1.97.1.md`](docs/nfr13-msrv-1.97.1.md) |

## firtool（NFR3）

默认**不信任** `PATH` 上的 firtool。CLI 钉死 **firtool-1.155.0**（`firrtl-bin-linux-x64.tar.gz` + `.sha256`）：

```bash
cargo run -p bitloom -- firtool info
cargo run -p bitloom -- firtool ensure   # 下载/校验/缓存并打印二进制路径
```

覆盖：`RHDL_FIRTOOL_PATH` 指向含 `firtool` 的目录；缓存根可用 `RHDL_FIRTOOL_CACHE`。

工具链 crate：MIT OR Apache-2.0（见各 crate 的 `Cargo.toml`）。

## HLS（支持功能 · FR35 / FR50）

Bitloom **将 HLS 列为支持功能**：算法级 `#[hls]` / `cargo bitloom hls` 经钉死外挂 **PandA Bambu 2024.10** 产出可综合 RTL。Bitloom **不**实现树内调度器（AD-25）。

跟练：

```bash
cargo run -p bitloom -- hls --help
# 安装 Bambu 2024.10 后：
export BITLOOM_BAMBU_PATH=/path/to/bambu
cargo run -p bitloom -- hls --function add --out-dir target/bitloom-hls
# 或 CI/本地烟测（默认 stub；真机设 BITLOOM_HLS_USE_REAL=1）：
just hls-smoke
```

专章、限制与烟测位置：[`docs/fr35-hls.md`](docs/fr35-hls.md)。

## 可视化（层次 + 时序 · FR38 / FR49）

产品入口（**不是**「请自行打开 GTKWave」）：

```bash
cargo bitloom visualize \
  --input crates/rhdl-firrtl/fixtures/external_hierarchy.fir \
  --out-dir target/viz
# → target/viz/hierarchy.html

cargo bitloom wave \
  --input crates/rhdl-firrtl/fixtures/external_hierarchy.fir \
  --out-dir target/wave --ticks 8
# → target/wave/timing.html + wave.vcd
```

端到端跟练：[`docs/tutorials/uj6-visualization.md`](docs/tutorials/uj6-visualization.md)。  
FST 可选说明：[`docs/fr31-optional-fst.md`](docs/fr31-optional-fst.md)。完整 LSP **deferred**（非本 epic 完成条件）。

## 状态与 deferred（诚实声明）

当前为 **0.x**。已交付：生成器 elaborate → FrozenHir → `.v` / FIRRTL 互转 / `tick`、firtool 钉死、Mem/CDC、**HLS 产品路径（外挂 Bambu）**、**内置层次/时序可视化入口** 等（见 `epics.md`）。

**明确 deferred / 未承诺为产品完整面：**

- 完整 LSP hover/goto（FR38 部分；层次/时序 HTML 入口已交付）
- 部分 CLI 动词（`check` / `build-sim`）
- 自研 HLS 调度器（永不；仅外挂 Bambu——HLS **本身已支持**）
- crates.io 名 `rhdl` / `rhdl-bits`（禁止）

详见 [`docs/semver-0x-policy.md`](docs/semver-0x-policy.md) 与 [`docs/crates-io-publish-bitloom.md`](docs/crates-io-publish-bitloom.md)。
