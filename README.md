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
- **ClockDomain / CDC：**
  - **FR52（历史最小合同）：** [`examples/clockdomain_skel`](examples/clockdomain_skel) — `bind_domain` / `mark_cdc_bridge` / `rhdl::E0220`；全局 `Sim::tick` 为按域 tick 的 MVP 等价。**不得**单独交差 FR79 深度（NFR37）。
  - **FR79（真 RTL / AD-29）：** [`examples/doubleflop_skel`](examples/doubleflop_skel) · [`examples/syncfifo_skel`](examples/syncfifo_skel)；文档 [`docs/fr79-doubleflop-cdc.md`](docs/fr79-doubleflop-cdc.md) · [`docs/fr79-syncfifo-cdc.md`](docs/fr79-syncfifo-cdc.md)；跟练 [`docs/tutorials/cdc-depth.md`](docs/tutorials/cdc-depth.md)（`cargo test -p bitloom --test fr79_cdc_depth_closeout`）
- **Bundle / 嵌套 + derive（FR51 → FR80）：**
  - **FR51（历史最小）：** ground-leaf `Bundle` / `HwVec` flatten；nested 曾 OUT OF SCOPE。**不得**单独交差 FR80 深度（NFR37）。
  - **FR80（一层嵌套 + `#[derive(Bundle)]`）：** [`examples/bundle_vec_skel`](examples/bundle_vec_skel)；文档 [`docs/fr80-nested-bundle.md`](docs/fr80-nested-bundle.md)；跟练 [`docs/tutorials/nested-bundle.md`](docs/tutorials/nested-bundle.md)（`cargo test -p bitloom --test fr80_nested_bundle`）
- **Elaborate-time Mem init（FR73）：** 非捕获 `Fn` 在 `ElaborateSession` 内生成 ROM/LUT 初值，freeze 前消解为普通字表（后端无闭包 IR）。最小面：

```rust
use bitloom_prelude::{ElaborateSession, GroundType, Span};

let mut s = ElaborateSession::new("Lut");
s.begin_module("Lut", Span::default());
s.add_input("clk", GroundType::Clock, Span::default());
s.add_input("rst", GroundType::Reset, Span::default());
s.declare_mem_with_init_fn("rom", 16, 8, |i| ((i * i) & 0xff) as u64, Span::default());
s.end_module();
let _frozen = s.finish().unwrap();
```

  ATDD：`cargo test -p bitloom --test fr73_crc_lut_golden`（CRC 闭包表 vs 手写 golden）。

- **可综合闭包进 comb/seq（FR74/FR75）：** 满足 `SynthesizableClosure` 约束的闭包在 elaborate 期内联为普通赋值；**勿捕获 `Wire`/`Reg`**（硬件引用 → `rhdl::E0142`；周期精确捕获闭包仍为 `rhdl::E0141`）。最小面：

```rust
use bitloom_prelude::{CombInline, ElaborateSession, GroundType, SeqInline, Span};

// Comb: y = a + b（Cap-R-55）
let mut s = ElaborateSession::new("Add");
s.begin_module("Add", Span::default());
s.add_input("clk", GroundType::Clock, Span::default());
s.add_input("rst", GroundType::Reset, Span::default());
s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
s.add_input("b", GroundType::UInt { width: 8 }, Span::default());
s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
s.begin_combinational(Span::default());
s.inline_comb_fn("y", &["a", "b"], &[], Span::default(), |args| {
    CombInline::Add(args[0].into(), args[1].into())
});
s.end_process();
s.end_module();
let _ = s.finish().unwrap();

// Seq: count.d = count + 1（Cap-R-56）
let mut s = ElaborateSession::new("Cnt");
s.begin_module("Cnt", Span::default());
s.add_input("clk", GroundType::Clock, Span::default());
s.add_input("rst", GroundType::Reset, Span::default());
s.add_output("q", GroundType::UInt { width: 8 }, Span::default());
s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
s.begin_sequential(Span::default());
s.inline_seq_fn("count", &[], &[], &[], Span::default(), |_args| SeqInline::Inc);
s.end_process();
s.begin_combinational(Span::default());
s.assign_net("q", "count", Span::default());
s.end_process();
s.end_module();
let _ = s.finish().unwrap();
```

  警告：**不要**在闭包环境捕获 `Wire`/`Reg` 等硬件句柄；用端口名字符串传给 `inline_*_fn`，不要把信号句柄关进闭包。共存矩阵 ATDD：`cargo test -p bitloom --test fr74_fr75_fr16_coexist_matrix`。

- **IP 生成器闭包定制（FR77）：** `Crc8Lut` 用 elaborate-time `Fn` 定制 CRC 表（默认 poly `0x07`；`elaborate_with_table_fn`）。叠在 Epic 34 无闭包基线 + Epic 27 Mem-init 之上；freeze 后无闭包残留。见 [`docs/ip/README.md`](docs/ip/README.md)。ATDD：`cargo test -p bitloom --test fr77_ip_generator_closure`。
- **HLS / IP 闭包透明（FR76+FR77 / NFR36）：** 消解后 viz / Verilog / FIRRTL / HLS C·RTL **不**感知闭包 IR（Cap-R-64）。**≠** FR47「sim generators」（双视图 crate 生成）。约束类：外挂 HLS = D1 `HlsFree`；可综合 IP = `SynthesizableClosure`。矩阵 ATDD：`cargo test -p bitloom --test fr76_fr77_nfr36_transparency_matrix`。
- **桥接适配器闭包模板（FR78）：** host/bridge `start_wait_complete` 把事务形启动映射到周期精确握手；功能侧自由闭包 OK，周期侧仅见普通信号（NFR36）。**≠** FR73 生成器闭包 / FR74–75 可综合闭包 / FR47 sim generators / Phase 7「闭环」。跟练：[`docs/tutorials/bridge-half.md`](docs/tutorials/bridge-half.md)。专章：[`docs/fr78-bridge-adapter-closures.md`](docs/fr78-bridge-adapter-closures.md)。ATDD：`cargo test -p bitloom --test fr78_bridge_adapter_start_wait_complete` · `fr78_fr47_dual_view_coverify` · `fr78_bridge_half_followalong`。

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
| 桥接适配器闭包模板（FR78） | [`docs/fr78-bridge-adapter-closures.md`](docs/fr78-bridge-adapter-closures.md) · UJ「桥接半程」[`docs/tutorials/bridge-half.md`](docs/tutorials/bridge-half.md) |
| 嵌套 Bundle + derive（FR80） | [`docs/fr80-nested-bundle.md`](docs/fr80-nested-bundle.md) · UJ「嵌套 Bundle」[`docs/tutorials/nested-bundle.md`](docs/tutorials/nested-bundle.md) |
| 双视图 sim 生成（FR47） | [`docs/fr47-dual-sim-generation.md`](docs/fr47-dual-sim-generation.md) · **≠** SystemC TLM |
| 双视图等价检查 | [`docs/fr30-dual-view-equiv.md`](docs/fr30-dual-view-equiv.md) |
| 可选 FST | [`docs/fr31-optional-fst.md`](docs/fr31-optional-fst.md) |
| tick 引擎 | [`docs/fr32-tick-engines.md`](docs/fr32-tick-engines.md) |
| C ABI cdylib | [`docs/fr33-c-abi.md`](docs/fr33-c-abi.md) |
| 仿真覆盖率（FR34 基线 + **FR105** 扩展 + **FR109** C3） | [`docs/fr34-sim-coverage.md`](docs/fr34-sim-coverage.md) · [`docs/fr105-sim-coverage-ext.md`](docs/fr105-sim-coverage-ext.md)（Mux 分支；**Epic 47 已关闭**）· [`docs/fr109-fsm-state-visit-coverage.md`](docs/fr109-fsm-state-visit-coverage.md)（FSM/state-visit；**Epic 51 已关闭** / Story 51.3） |
| Chisel 可编译生成（FR28 / FR88）+ Mem Path A（FR81） | [`docs/fr28-chisel-compilable.md`](docs/fr28-chisel-compilable.md)（**可编译 ≠ idiomatic**；[钉死运维清单](docs/fr28-chisel-compilable.md#firtool--chisel-钉死运维清单fr88--nfr3--nfr12)）· FR71：`just chisel-fr28-jvm` · 可选 Mem：`just chisel-fr81-mem-jvm` · [维护者合并清单](docs/fr28-chisel-compilable.md#维护者合并前检查清单fr28--emit_chisel) |
| Idiomatic / 可维护 Chisel（**FR97** · Epic 42 已关闭；**FR111** 加深 · Epic 53 已关闭） | [`docs/fr97-idiomatic-chisel.md`](docs/fr97-idiomatic-chisel.md) · [`docs/fr111-idiomatic-chisel-depth.md`](docs/fr111-idiomatic-chisel-depth.md)（`emit_chisel_idiomatic_fr111`；机械面不得单独关闭 FR97/FR111） |
| Chisel / `.fir` 反向导入（FR46） | [`docs/fr46-chisel-import.md`](docs/fr46-chisel-import.md) |
| `import` CLI + 混合夹具 | [`docs/fr40-cli-verbs.md`](docs/fr40-cli-verbs.md) · [`examples/chisel_mixed`](examples/chisel_mixed) |
| HLS 产品路径（**支持** · FR35/FR50/FR95/FR96 + **FR110** 深度） | [`docs/fr35-hls.md`](docs/fr35-hls.md) · [`docs/fr110-hls-commercial-depth.md`](docs/fr110-hls-commercial-depth.md)（pipeline/II；**Epic 52 已关闭** / Story 52.3）· 烟测 [`scripts/hls-smoke.sh`](scripts/hls-smoke.sh) |
| Formal/SVA | [`docs/fr39-formal-sva.md`](docs/fr39-formal-sva.md) |
| Analog/InOut | [`docs/fr27-analog-inout.md`](docs/fr27-analog-inout.md) |
| rhdl-float | [`docs/fr36-rhdl-float.md`](docs/fr36-rhdl-float.md) |
| IP / 黑盒 | [`docs/fr37-ip-box.md`](docs/fr37-ip-box.md) |
| HIR HTML / LSP（层次；**FR99** / Epic 44 **已关闭** · `bitloom-lsp`） | [`docs/fr38-viz-lsp.md`](docs/fr38-viz-lsp.md) · [`docs/fr99-bitloom-lsp.md`](docs/fr99-bitloom-lsp.md) |
| 宿主 IDE / rust-analyzer（FR90） | [`docs/fr90-host-ide-rust-analyzer.md`](docs/fr90-host-ide-rust-analyzer.md) · 夹具 [`examples/counter_ports`](examples/counter_ports) |
| 多视图同刺激 + adapter 模板（FR92） | [`docs/fr92-shared-stimulus-adapter.md`](docs/fr92-shared-stimulus-adapter.md) · `SharedStimulusScoreboard` · FR78 adapter |
| 自动 FL≡RTL / 形式等价产品（**FR100**） | [`docs/fr100-formal-equiv.md`](docs/fr100-formal-equiv.md) · `FormalEquivProduct`（超出 FR92 记分板）；**Epic 45 已关闭** |
| SystemC TLM-2.0 产品路径（**FR101**） | [`docs/fr101-systemc-tlm.md`](docs/fr101-systemc-tlm.md) · `emit_systemc_tlm_lt` / `cargo bitloom gen-tlm`（LT-only MVP；**Epic 46 已关闭**） |
| SystemC TLM-2.0 AT 产品支（**FR107**） | [`docs/fr107-systemc-tlm-at.md`](docs/fr107-systemc-tlm-at.md) · `emit_systemc_tlm_at` / `cargo bitloom gen-tlm-at`（AT `nb_transport_fw` 子集；**Epic 49 已关闭** / Story 49.3；∥ FR101 LT） |
| 多视图属性全矩阵（**FR102**） | [`docs/fr102-multiview-attribute-matrix.md`](docs/fr102-multiview-attribute-matrix.md) · 超出仅 adapter 模板 |
| 一级 IP 双模型齐全（**FR103**） | [`docs/fr103-ip-dual-model.md`](docs/fr103-ip-dual-model.md) · `IpDualModelMatrix`；FIFO/UART/SPI/I2C/AXI |
| 时序 / 波形产品入口 | [`docs/fr38-wave.md`](docs/fr38-wave.md) |
| 交互式富波形（**FR104**） | [`docs/fr104-interactive-wave.md`](docs/fr104-interactive-wave.md) · `interactive.html`（≠ 仅静态 `timing.html`；**Epic 47 / FR104+FR105 已关闭**） |
| UJ-6 可视化半程跟练 | [`docs/tutorials/uj6-visualization.md`](docs/tutorials/uj6-visualization.md) |
| 额外 CLI | [`docs/fr40-cli-verbs.md`](docs/fr40-cli-verbs.md) |
| 多平台 firtool | [`docs/nfr11-firtool-platforms.md`](docs/nfr11-firtool-platforms.md) |
| MSRV 1.97.1 (NFR13) | [`docs/nfr13-msrv-1.97.1.md`](docs/nfr13-msrv-1.97.1.md) |

## firtool（NFR3）

默认**不信任** `PATH` 上的 firtool。CLI 钉死 **firtool-1.155.0**（与 Chisel **7.14.0** 配对 · AD-9 / NFR12；`firrtl-bin-linux-x64.tar.gz` + `.sha256`）：

```bash
cargo run -p bitloom -- firtool info
cargo run -p bitloom -- firtool ensure   # 下载/校验/缓存并打印二进制路径
```

覆盖：`RHDL_FIRTOOL_PATH` 指向含 `firtool` 的目录；缓存根可用 `RHDL_FIRTOOL_CACHE`。

完整钉死运维清单 + `emit_chisel` **可编译 ≠ idiomatic** 诚实声明（FR88）：[`docs/fr28-chisel-compilable.md`](docs/fr28-chisel-compilable.md)。

工具链 crate：MIT OR Apache-2.0（见各 crate 的 `Cargo.toml`）。

## HLS（支持功能 · FR35 / FR50 / FR76 / FR95 / FR96 / FR110）

Bitloom **将 HLS 列为支持功能**，含诚实路径（修订 **AD-25**；**Epic 41** + **Epic 52 已关闭**）：

- **树内** `schedule_in_tree` / `cargo bitloom hls --in-tree`（loop-unroll MVP）= **FR95** 完成面；**FR96** = 调度前闭包 dissolve/inline 后进入树内路径（`--in-tree --dataflow`）。
- **树内商业深度** `schedule_in_tree_fr110` / `--in-tree --pipeline --ii N --stages N`（`pipeline_stages≥2` + `ii`）= **FR110**（**Epic 52 已关闭** / Story 52.3；≠ 完整商业 HLS 全家桶）。
- **外挂** 钉死 **PandA Bambu 2024.10**（stub / `BITLOOM_HLS_USE_REAL`）= **FR35 / FR76 / FR86** 可选对照；**不得单独满足 FR95 或 FR110**。

数据流闭包：外挂侧 FR76 dissolve→C；树内侧 FR96 dissolve→树内 schedule（D1 **HlsFree**；可综合 comb/seq/IP 仍走 **SynthesizableClosure**）。

**消歧：** FR76/FR77 的「生成器/数据流闭包」≠ **FR47**「sim generators」（`gen-func` / `gen-cycle` 双视图 crate）。后者不消解用户 `Fn` 进硬件。

跟练：

```bash
cargo run -p bitloom -- hls --help
# FR95 树内调度（不调用 Bambu）：
cargo run -p bitloom -- hls --function map_add1 --in-tree --out-dir target/bitloom-hls-in-tree
# FR96 闭包→树内：
cargo run -p bitloom -- hls --function map_xor --dataflow xor_a5 --in-tree --out-dir target/bitloom-hls-fr96
# 外挂路径（安装 Bambu 2024.10 后）：
export BITLOOM_BAMBU_PATH=/path/to/bambu
cargo run -p bitloom -- hls --function add --out-dir target/bitloom-hls
# FR76 数据流变换（emit-only 检查溶解产物）：
cargo run -p bitloom -- hls --function map_xor --dataflow xor_a5 --emit-only --out-dir target/bitloom-hls
# 或 CI/本地烟测（默认 stub；真机设 BITLOOM_HLS_USE_REAL=1）：
just hls-smoke
```

**FR88 Path B：** 外挂侧保持 stub 默认（stub 绿 ≠ HLS 调度质量，亦 ≠ FR95）；**无**夜间真机 CI job；真机仅显式 `BITLOOM_HLS_USE_REAL=1`。详见 [`docs/fr35-hls.md`](docs/fr35-hls.md)。

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
FST 可选说明：[`docs/fr31-optional-fst.md`](docs/fr31-optional-fst.md)。按键全 elaborate LSP（**FR99**）— **Epic 44 已关闭**（[`docs/fr99-bitloom-lsp.md`](docs/fr99-bitloom-lsp.md)）；宿主 rust-analyzer（FR90）仍可用但不替代 FR99。

## 状态与 deferred（诚实声明）

当前为 **0.x**。已交付：生成器 elaborate → FrozenHir → `.v` / FIRRTL 互转 / `tick`、firtool 钉死、Mem/CDC、**HLS（树内 FR95/FR96 MVP + 外挂 Bambu 可选）**、**Idiomatic Chisel（FR97 · Epic 42）**、**内置层次/时序可视化入口** 等（见 `epics.md`）。

**路线图阶段五–七「绿 / 全绿」字面绿 MVP 按 Phase 12 验收（FR94–105 / NFR42）：字面项须由对应 FR 关闭后方可勾选；Phase 12 Epic 40–47 已关闭。** 完成定义见 [`docs/requirements/19. 实施路线图.md`](docs/requirements/19.%20实施路线图.md) §19.7–19.9。Phase 11 合同绿（FR87 / NFR38）为**历史已交付里程碑**，**禁止**用合同绿冒充字面全绿。

**Phase 13（MVP→商业加深 · FR106–FR115 / NFR44–NFR47）：** Correct Course 已批准（2026-09-10）。Phase 12 MVP 关闭证据**仍有效**（**NFR44**），**不得**改写为失败。**Phase 13 规划/实现故事已齐（Epic 48–56 已关闭）。** 对外「商业加深 / 非 MVP」类宣称按 **FR115** 勾选（对应 **FR106–114** 均已关闭）；**禁止**用 Phase 12 MVP 冒充商业完整面。延期与边界 ledger：[`_agile-output/implementation-artifacts/deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md)。

**Phase 14（NFR47 未选加深升格 · FR116–FR123 / NFR48–NFR51）：** Correct Course 已批准（2026-09-10）。Phase 12/13 关闭证据**仍有效**（**NFR48**），**不得**改写为失败。Phase 14 = 将原 NFR47 未选加深子集升格为显式合同（闸门 **FR116** / Epic 57 **已关闭**；实现 Epic 58–63，其中 **FR117 / Epic 58 已关闭**、**FR118 / Epic 59 已关闭**、**FR119 / Epic 60 已关闭**），**不是**「Phase 13 AC 未达标」补救。对外「Tywaves / syn-scan / SBY / VIP GPIO / Handshake / 官方风格全家桶」类宣称按 **FR123** 勾选（对应 **FR116–122** 关闭后方可）；**禁止**用 Phase 13 商业加深完成面冒充本批加深。

**Phase 12 规划/实现故事已齐（Epic 40–47 已关闭）。** Epic 47 / FR104+FR105 **已关闭**（Story 47.3）。

### 永久非目标（FR93）— 历史；已被 Phase 12 推翻

Phase 11 曾将下列五项公开锁定为**永久非目标**，并写「须新 PRD 才能推翻」。**Correct Course + FR94（2026-09-09 Path B）已批准推翻**该锁定。下列项现为 Phase 12 **交付目标**（**须由对应 FR 关闭后方可宣称完成** / NFR42）；实现 epic 须引用已修订 AD（**NFR41**）。同源：PRD addendum「Phase 12 字面绿」与 [`deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md)。

1. **树内 / 自研 HLS 调度器** → **FR95** / **FR96**（**Epic 41 已关闭** — MVP 已交付；修订后 **AD-25**）；**商业深度 → FR110 / Epic 52 已关闭**（Story 52.3）；外挂 Bambu 等可保留为可选，不得单独满足 FR95/FR110
2. **FIRRTL→idiomatic Scala / idiomatic Chisel** → **FR97**（**Epic 42 已关闭** — MVP 已交付；修订后 **AD-27**）；**可维护加深 → FR111 / Epic 53 已关闭**（Story 53.3）；机械可编译仍满足 FR28/FR46，不得冒充 FR97/FR111；完成面见 [`docs/fr97-idiomatic-chisel.md`](docs/fr97-idiomatic-chisel.md) / [`docs/fr111-idiomatic-chisel-depth.md`](docs/fr111-idiomatic-chisel-depth.md)
3. 默认 **TLM≡CA 形式证明** → **FR100**（**Epic 45 已关闭** — FR100 形式等价产品 + FR102 属性全矩阵 + FR103 一级 IP 双模型 MVP；见 [`docs/fr100-formal-equiv.md`](docs/fr100-formal-equiv.md)、[`docs/fr103-ip-dual-model.md`](docs/fr103-ip-dual-model.md)）；**形式/双模型加深 → Phase 13 FR112 / Epic 54 已关闭**（Story 54.3；GeneratedFunctional MemRead≡tick；见 [`docs/fr112-generated-functional-memread-equiv.md`](docs/fr112-generated-functional-memread-equiv.md)）；**SystemC TLM-2.0 产品** → **FR101**（**Epic 46 已关闭** — LT-only MVP / Story 46.3；修订后 **AD-5**；**AT 加深 → Phase 13 FR107 / Epic 49**；见 [`docs/fr101-systemc-tlm.md`](docs/fr101-systemc-tlm.md)）
4. **VIP 级全协议 IP** → **FR98**（**Epic 43 已关闭** — UART/SPI/I2C/AXI4-Lite 近 VIP MVP 已交付；**GPIO 近 VIP → Phase 13 FR108 / Epic 50 已关闭** / Story 50.3；边界见 [`docs/ip/README.md`](docs/ip/README.md)）
5. **按键全设计 elaborate** 的 netlist LSP → **FR99**（**Epic 44 已关闭** — `bitloom-lsp` 全设计 elaborate 诊断/符号 MVP；**根发现加深 → Phase 13 FR113 / Epic 55 已关闭** / Story 55.3；见 [`docs/fr99-bitloom-lsp.md`](docs/fr99-bitloom-lsp.md)、[`docs/fr113-lsp-design-root-discovery.md`](docs/fr113-lsp-design-root-discovery.md)）

### Phase 13 加深面（合同已批准；Epic 48–56 实现已关闭）

下列原 Phase 12 **optional product** 已由 Correct Course + **FR106** 升格为 Phase 13 显式 FR（Epic 48–56）。**全部加深 FR（FR107–114）已关闭**；对外商业加深宣称按 **FR115**。同源：PRD addendum「Phase 13」与 [`deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md)。

| 加深面 | FR / Epic | 相对 Phase 12 MVP |
| --- | --- | --- |
| SystemC TLM AT / `nb_transport` | FR107 / 49 | vs FR101 LT-only — **Epic 49 已关闭**（Story 49.3） |
| GPIO 近 VIP | FR108 / 50 | vs FR98 G0 可选 — **Epic 50 已关闭**（Story 50.3） |
| FSM / state-visit 覆盖率（C3） | FR109 / 51 | vs FR105 Mux v2 — **Epic 51 已关闭**（Story 51.3） |
| 树内 HLS 商业深度 | FR110 / 52 | vs FR95/96 MVP stub — **Epic 52 已关闭**（Story 52.3） |
| Idiomatic Chisel 可维护深度 | FR111 / 53 | vs FR97 MVP — **Epic 53 已关闭**（Story 53.3） |
| 形式等价 / 双模型深度 | FR112 / 54 | vs FR100/103 MVP — **Epic 54 已关闭**（Story 54.3） |
| LSP 设计根发现加深 | FR113 / 55 | vs FR99 DesignFixture — **Epic 55 已关闭**（Story 55.3） |
| Tywaves / LCOV GUI | FR114 / 56 | vs FR104/105 MVP — **Epic 56 已关闭**（Story 56.3；LCOV+树内 GUI；typed IDE → **FR117 / Epic 58 已关闭**；Tywaves A deferred） |

### Phase 14 加深面（合同已批准；闸门 Epic 57 已关闭；FR117/Epic 58、FR118/Epic 59、FR119/Epic 60 已关闭；Epic 61–63 仍 deferred）

下列原 Phase 13 **NFR47 未选加深**已由 Correct Course + **FR116** 升格为 Phase 14 显式 FR（Epic 57–63）。**FR117 / Epic 58 已关闭**（自研 typed IDE 波形；Tywaves A deferred）；**FR118 / Epic 59 已关闭**（无 metadata `#[bitloom::top]` syn-scan）；**FR119 / Epic 60 已关闭**（SymbiYosys/`sby` F1-(ii)；分支 C 更多 IP 手写 FL 仍 deferred）；**FR120–122 实现关闭前仍为 deferred 交付态**；对外宣称按 **FR123**。同源：PRD addendum「Phase 14」与 [`deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md)。

| 加深面 | FR / Epic | 相对 Phase 13 关闭面 |
| --- | --- | --- |
| Tywaves 级 typed IDE 波形 | FR117 / 58 | vs FR104 / FR114 LCOV+树内 GUI — **Epic 58 已关闭**（Story 58.3；自研 typed；Tywaves A deferred） |
| 无 metadata 全树 `#[bitloom::top]` syn-scan | FR118 / 59 | vs FR99 DesignFixture / FR113 Cargo-graph+metadata — **Epic 59 已关闭**（Story 59.3） |
| SymbiYosys/SMT 形式路径 | FR119 / 60 | vs FR100 F1-(i) / FR112 分支 B — **Epic 60 已关闭**（Story 60.3；`just formal-sby-check`） |
| 商业 VIP GPIO 全家桶 | FR120 / 61 | vs FR98 四类近 VIP / FR108 P1–P4 |
| Handshake / 动态数据流默认可综合 | FR121 / 62 | vs FR95/96 / FR110（须修订 AD-25） |
| 官方风格 Chisel 全家桶 | FR122 / 63 | vs FR97 / FR111 D1+D3（可能修订 AD-27） |

**FR123 宣称纪律：** 「Tywaves / syn-scan / SBY / VIP GPIO / Handshake / 官方风格全家桶」**仅**在对应 **FR116–122** 关闭后方可勾选；**禁止**用 Phase 13 完成面冒充。**FR117 已关闭**仅授权宣称 **自研 typed IDE 波形（子集 B）**；**不得**据此宣称上游 Tywaves 一等集成（子集 A 仍 **NFR51 deferred**）。**FR118 已关闭**授权宣称无 metadata workspace `#[bitloom::top]` syn-scan（≠ DesignFixture / metadata alone）。**FR119 已关闭**授权宣称 SymbiYosys/`sby` F1-(ii) 形式路径（≠ FR100 F1-(i) / FR112-B / FR85 alone）；**分支 C 更多 IP 手写 FL 仍 deferred（NFR51）**。未列入本批的 deferred（自动 FSM 标签、更多 IP FL、第三方 LCOV GUI、emit MemRead 完整生成等）仍须另开合同（**NFR51**）。

**明确 deferred / 未承诺为产品完整面（可延期；上列五项已不再是永久非目标）：**

- 完整 / 按键全 elaborate Bitloom LSP（**FR99 / Epic 44 已关闭**。Epic 39 **FR91 Path B** 显式 defer 仅为**历史**关闭路径，**不得**再当作 Phase 12 完成口径。宿主 rust-analyzer（FR90）仍可用，**不替代** FR99。层次/时序 HTML **≠ LSP**；见 [`docs/fr99-bitloom-lsp.md`](docs/fr99-bitloom-lsp.md)、[`docs/fr90-host-ide-rust-analyzer.md`](docs/fr90-host-ide-rust-analyzer.md)、[`docs/fr38-viz-lsp.md`](docs/fr38-viz-lsp.md)）
- 部分 CLI 动词（`check` / `build-sim`）
- crates.io 名 `rhdl` / `rhdl-bits`（禁止）
- 自动等价证明 / SystemC TLM-2.0 产品（**FR100 / Epic 45 已关闭** — 形式等价产品 + IP 双模型；**FR101 / Epic 46 已关闭** — LT-only MVP；「不承诺 SystemC TLM」不再是完成排除项；AT → **FR107**；见 [`docs/fr101-systemc-tlm.md`](docs/fr101-systemc-tlm.md) / doc-19）

详见 [`docs/semver-0x-policy.md`](docs/semver-0x-policy.md) 与 [`docs/crates-io-publish-bitloom.md`](docs/crates-io-publish-bitloom.md)。
