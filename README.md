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

## 快速开始

**诚实面（Phase 17 / 18 / 19 / 20 / 21 / 22 + 工程结项）：** 库 crate（`bitloom-macro` / `hir` / `builder` / `vlog` / `sim` / `prelude`）已在 crates.io **1.0.0**（Phase 17 / FR146）。**公开 CLI `bitloom` 1.0.0 已可 `cargo install`**（**FR151** / Epic 85）。**Phase 18 已关闭**（Epic 84–86）：SemVer / 发版诚实 **FR153**（[`docs/fr153-semver-honesty.md`](docs/fr153-semver-honesty.md)）。CLI crates.io 宣称须引 **FR148–153**。**Phase 19 已关闭**（Epic 87–98 / FR154–165）：闸门 **FR154**；**FR155** lsp 上架；**FR157–165** NFR59 升格子集；宣称诚实 **FR156**（[`docs/fr156-phase19-claim-honesty.md`](docs/fr156-phase19-claim-honesty.md)）。Phase 19 宣称须引 **FR154–165**；超出各 NFR14 钉死子集仍曾归 **NFR71**（**NFR68** / **NFR72**）。**Phase 20 已关闭**（Epic 99–104 / **FR166–FR171** / **NFR73–NFR77**；Correct Course 2026-09-12）：完整 ChiselSim / 多端 IDE 商店 **FR167**；SPI·I2C·AXI 手写 FL **FR168**；更广 CIRCT/MLIR allocation **FR169**（firtool 升钉曾 **NFR76**）；Chisel update-mainline Parser **FR170**；宣称诚实门 **FR171**（[`docs/fr171-phase20-claim-honesty.md`](docs/fr171-phase20-claim-honesty.md)）。**Phase 20 规划+实现故事已齐（Epic 99–104）**；宣称须引 **FR166–171**（**NFR77**）；**不得**宣称「NFR71 账本已空」。**Phase 21 已关闭**（Epic 105–110 / **FR172–FR177** / **NFR78–NFR82**；Correct Course 2026-09-12；`correctCoursePhase21Approved`）：宣称诚实门 **FR177**（[`docs/fr177-phase21-claim-honesty.md`](docs/fr177-phase21-claim-honesty.md)）。**Phase 21 规划+实现故事已齐（Epic 105–110）**；Phase 21 宣称须引 **FR172–177**（**NFR82**）；**不得**用 Phase 20 alone 冒充 FR173–176；**不得**宣称「NFR76 账本已空」；超出各 NFR14 钉死子集曾归 **NFR81**（**NFR78**）。**Phase 22 合同闸门已关闭**（Epic 111–117 / **FR178–FR184** / **NFR83–NFR87**；Correct Course 2026-09-12；`correctCoursePhase22Approved`；闸门 **Epic 111 / FR178** **已关闭** / Story **111.4**；Epic 112–117 可合法开工且须各自 NFR14）：NFR81 leftovers 升格（浮动 HEAD / Handshake / Style / unpaired 产品钉 / 显式 FR142）；宣称须引 **FR178–184**（**FR184** / **NFR87**）；**不得**用 Phase 21 alone 冒充本批五条；**不得**宣称「NFR81 账本已空」；超出各 NFR14 钉死子集仍须新合同（**NFR86**；**NFR83**）。触 AD-9 / AD-25 / AD-27 / FR142 表面须按 **NFR85** 先修订再 story ready。**工程/合同结项收口已批准**（Correct Course 2026-09-14；`engineeringCloseoutApproved` / `engineeringCloseoutStatus: complete`；[`sprint-change-proposal-2026-09-14-engineering-closeout.md`](_agile-output/planning-artifacts/sprint-change-proposal-2026-09-14-engineering-closeout.md)）：sprint backlog（故事/回顾/action items）**曾空**；Phase 12–22 关闭仍有效；**≠** 「NFR86 账本已空」。**Phase 23 合同已批准**（Correct Course 2026-09-14；`correctCoursePhase23Approved`；[`sprint-change-proposal-2026-09-14-phase23-nfr86-leftovers.md`](_agile-output/planning-artifacts/sprint-change-proposal-2026-09-14-phase23-nfr86-leftovers.md)；Epic 118–124 / **FR185–FR191** / **NFR88–NFR92**；闸门 **Epic 118** 关闭前 119–124 不得 ready）：NFR86 leftovers 升格（无界 tip / Handshake lower / Style 全家桶 / firtool 再升钉 / 继续扩 FR142）；宣称须引 **FR185–191**（**FR191**）；**不得**用 Phase 22 / 结项 alone 冒充本批五条；超出各 NFR14 仍 **NFR91**。`git push` **不是** FR。贡献者亦可 clone 本仓库使用 workspace CLI。

```bash
# 推荐：crates.io（FR151）
cargo install bitloom
cargo bitloom new blink
cargo bitloom build --package blink --manifest-dir blink --out-dir out
```

```bash
# 贡献者（workspace）
rustup toolchain install 1.97.1
cargo run -p bitloom -- new blink
cargo run -p bitloom -- build --package blink --manifest-dir blink --out-dir out
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
| Idiomatic / 可维护 Chisel（**FR97** · Epic 42 已关闭；**FR111** 加深 · Epic 53 已关闭；**FR122** 官方风格全家桶 · Epic 63 已关闭） | [`docs/fr97-idiomatic-chisel.md`](docs/fr97-idiomatic-chisel.md) · [`docs/fr111-idiomatic-chisel-depth.md`](docs/fr111-idiomatic-chisel-depth.md) · [`docs/fr122-official-style-chisel.md`](docs/fr122-official-style-chisel.md)（`emit_chisel_idiomatic_fr122`；机械/FR97/FR111 alone 不得关闭 FR122） |
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

默认**不信任** `PATH` 上的 firtool。CLI 钉死 **firtool-1.159.0**（Chisel **7.15.0** · AD-9 / NFR12 / FR182 *unpaired product-pin*；`firrtl-bin-linux-x64.tar.gz` + `.sha256`）：

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

库 crate 表面为 **1.0.0**（Phase 17 / FR146；曾为 **0.x** 直至 FR146）。**CLI 已上架**（**FR151** / Epic 85；`cargo install bitloom`）。**FR153** 发版诚实 / SemVer 跟进仍属 Epic 86。已交付：生成器 elaborate → FrozenHir → `.v` / FIRRTL 互转 / `tick`、firtool 钉死、Mem/CDC、**HLS（树内 FR95/FR96 MVP + 外挂 Bambu 可选）**、**Idiomatic Chisel（FR97 · Epic 42）**、**内置层次/时序可视化入口** 等（见 `epics.md`）。

**路线图阶段五–七「绿 / 全绿」字面绿 MVP 按 Phase 12 验收（FR94–105 / NFR42）：字面项须由对应 FR 关闭后方可勾选；Phase 12 Epic 40–47 已关闭。** 完成定义见 [`docs/requirements/19. 实施路线图.md`](docs/requirements/19.%20实施路线图.md) §19.7–19.9。Phase 11 合同绿（FR87 / NFR38）为**历史已交付里程碑**，**禁止**用合同绿冒充字面全绿。

**Phase 13（MVP→商业加深 · FR106–FR115 / NFR44–NFR47）：** Correct Course 已批准（2026-09-10）。Phase 12 MVP 关闭证据**仍有效**（**NFR44**），**不得**改写为失败。**Phase 13 规划/实现故事已齐（Epic 48–56 已关闭）。** 对外「商业加深 / 非 MVP」类宣称按 **FR115** 勾选（对应 **FR106–114** 均已关闭）；**禁止**用 Phase 12 MVP 冒充商业完整面。延期与边界 ledger：[`_agile-output/implementation-artifacts/deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md)。

**Phase 14（NFR47 未选加深升格 · FR116–FR123 / NFR48–NFR51）：** Correct Course 已批准（2026-09-10）。Phase 12/13 关闭证据**仍有效**（**NFR48**），**不得**改写为失败。Phase 14 = 将原 NFR47 未选加深子集升格为显式合同（闸门 **FR116** / Epic 57 **已关闭**；实现 Epic 58–63 **全部已关闭**：**FR117–FR122**），**不是**「Phase 13 AC 未达标」补救。**Phase 14 规划/实现故事已齐（Epic 57–63）。** 对外「Tywaves / syn-scan / SBY / VIP GPIO / Handshake / 官方风格全家桶」类宣称按 **FR123** 勾选（对应 **FR116–122** 关闭后方可）；**禁止**用 Phase 13 商业加深完成面冒充本批加深。原 NFR51 明示剩余已另开 **Phase 15** 合同（见下）。

**Phase 15（NFR51 剩余升格 · FR124–FR132 / NFR52–NFR55）：** Correct Course 已批准（2026-09-10）。Phase 12–14 关闭证据**仍有效**（**NFR52**），**不得**改写为失败。Phase 15 = 将原 NFR51 明示剩余升格为显式合同（闸门 **FR124** / Epic 64 **已关闭**；实现 Epic 65–71 **全部已关闭**：**FR125–FR131**），**不是**「Phase 14 AC 未达标」补救。**Phase 15 规划/实现故事已齐（Epic 64–71）。** 对外「上游 Tywaves 一等 / 更多 IP FL / 强制 sby CI / 全 SoC pad / CIRCT Handshake / Style Guide·Parser / ip.rs 拆分」类宣称按 **FR132** 勾选（对应 **FR124–131** 关闭后方可）；**禁止**用 Phase 14 完成面（含 FR117/119/120/121/122 alone）冒充本批加深。原 NFR55 明示剩余已另开 **Phase 16** 合同（见下）。

**Phase 16（产品终局结项 / NFR55 升格 · FR133–FR140 / NFR56–NFR59）：** Correct Course 已批准（2026-09-11）。Phase 12–15 关闭证据**仍有效**（**NFR56**），**不得**改写为失败。Phase 16 = 将原 NFR55 明示剩余升格为显式终局加深合同（闸门 **FR133** / Epic 72 **已关闭**；**Phase 16 规划故事已齐（Epic 72–78）**；实现关闭态：**Epic 73 / FR134 已关闭**；**Epic 74 / FR135 已关闭**；**Epic 75 / FR136 已关闭**；**Epic 76 / FR137 已关闭**；**Epic 77 / FR138 已关闭**；**Epic 78 / FR139 已关闭**），**不是**「Phase 15 AC 未达标」补救，**也不等于**冲 1.0 / backlog 永久空。对外「终局 / Tywaves GUI·IDE / 更多 IP FL / 全芯片 pad / 外部 CIRCT 门禁 / Parser 恢复 / IP 跨 crate」类宣称按 **FR140** 勾选（对应 **FR133–139** 关闭后方可）；**禁止**用 Phase 15 完成面（含 **FR125–131 alone**）冒充本批 / 终局完成面。未列入本批的 deferred（含更深 IP 布局 / 更深 GUI·IDE 子集 / 未列入协议手写 FL / 更广 CIRCT·MLIR lower / 仿真门禁加深 / 更深 Chisel·Parser 生态）仍须另开合同（**NFR59**）。**禁止**用 Phase 16 终局 alone 冒充 **1.0 / 公开 API 稳定**（→ **Phase 17 / FR141–147**）。

**Phase 17（公开 API 稳定门 / Bitloom 1.0 · FR141–FR147 / NFR60–NFR63）：** Correct Course 已批准（2026-09-11）。Phase 12–16 关闭证据**仍有效**（**NFR60**）。闸门 **FR141** / Epic 79–83 **已关闭**；**Epic 83 / FR146：** workspace **1.0.0** + tag `v1.0.0` + CHANGELOG；库 crate 已上 crates.io **1.0.0**（见 [`docs/fr146-bitloom-1-0-0-release.md`](docs/fr146-bitloom-1-0-0-release.md)）。对外「1.0 / 公开 API 稳定」按 **FR147**。**禁止**用 Phase 16 终局 alone 冒充 1.0。**NFR59** 仍 deferred（**NFR63**）。**CLI 上架 ≠ Phase 17 完成面**（→ **Phase 18 / FR148–153**）。公开表面：[`docs/public-api-1-0-surface.md`](docs/public-api-1-0-surface.md)。

**Phase 18（CLI / 依赖 crate crates.io 可发布 · FR148–FR153 / NFR64–NFR67）：** Correct Course 已批准（2026-09-11）。Phase 12–17 关闭证据**仍有效**（**NFR64**），**不得**改写为失败。闸门 **FR148** / Epic 84 **已关闭**（Story 84.4）。**Epic 85 / FR149–152 已关闭**（`bitloom-firrtl` / `bitloom-viz` / CLI `bitloom` **1.0.0** 已上 crates.io；lsp 策略 **FR152(b)**）。**FR153** 发版后诚实 / SemVer 跟进 → **Epic 86**（**已关闭**）。**口径：** CLI 上架 ≠ 清空 NFR59（**NFR67**）。公开品牌 **Bitloom**。同源：[`deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md) Phase 18 pointer；[`docs/fr151-bitloom-cli-publish.md`](docs/fr151-bitloom-cli-publish.md)。

**Phase 19（NFR59 全子集升格 + FR152(a) · FR154–FR165 / NFR68–NFR72）：** Correct Course 已批准（2026-09-12；`correctCoursePhase19Approved`）。Phase 12–18 关闭证据**仍有效**（**NFR68**），**不得**改写为失败。闸门 **FR154** / Epic 87–**Epic 98 / FR156** **全部已关闭**（Story 98.3）。**Epic 88 / FR155**、**Epic 89–97 / FR157–165** 加深面已关；宣称诚实见 [`docs/fr156-phase19-claim-honesty.md`](docs/fr156-phase19-claim-honesty.md)。**Phase 19 规划故事已齐（Epic 87–98）**。**禁止**用 Phase 18 alone 冒充 lsp/NFR59；**禁止**暗示超出 NFR14 钉死子集的加深已清（**NFR71**）。`git push` **不是** FR。公开品牌 **Bitloom**。同源：[`deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md) Phase 19 pointer。

**Phase 12 规划/实现故事已齐（Epic 40–47 已关闭）。** Epic 47 / FR104+FR105 **已关闭**（Story 47.3）。

### 永久非目标（FR93）— 历史；已被 Phase 12 推翻

Phase 11 曾将下列五项公开锁定为**永久非目标**，并写「须新 PRD 才能推翻」。**Correct Course + FR94（2026-09-09 Path B）已批准推翻**该锁定。下列项现为 Phase 12 **交付目标**（**须由对应 FR 关闭后方可宣称完成** / NFR42）；实现 epic 须引用已修订 AD（**NFR41**）。同源：PRD addendum「Phase 12 字面绿」与 [`deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md)。

1. **树内 / 自研 HLS 调度器** → **FR95** / **FR96**（**Epic 41 已关闭** — MVP 已交付；修订后 **AD-25**）；**商业深度 → FR110 / Epic 52 已关闭**（Story 52.3）；外挂 Bambu 等可保留为可选，不得单独满足 FR95/FR110
2. **FIRRTL→idiomatic Scala / idiomatic Chisel** → **FR97**（**Epic 42 已关闭** — MVP 已交付；修订后 **AD-27**）；**可维护加深 → FR111 / Epic 53 已关闭**（Story 53.3）；**官方风格全家桶 → FR122 / Epic 63 已关闭**（Story 63.3）；机械可编译仍满足 FR28/FR46，不得冒充 FR97/FR111/FR122；完成面见 [`docs/fr97-idiomatic-chisel.md`](docs/fr97-idiomatic-chisel.md) / [`docs/fr111-idiomatic-chisel-depth.md`](docs/fr111-idiomatic-chisel-depth.md) / [`docs/fr122-official-style-chisel.md`](docs/fr122-official-style-chisel.md)
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

### Phase 14 加深面（合同已批准；闸门 Epic 57 已关闭；Epic 58–63 / FR117–FR122 已关闭；规划/实现故事已齐）

下列原 Phase 13 **NFR47 未选加深**已由 Correct Course + **FR116** 升格为 Phase 14 显式 FR（Epic 57–63）。**FR117 / Epic 58 已关闭**（自研 typed IDE 波形；Tywaves A deferred）；**FR118 / Epic 59 已关闭**（无 metadata `#[bitloom::top]` syn-scan）；**FR119 / Epic 60 已关闭**（SymbiYosys/`sby` F1-(ii)；分支 C 更多 IP 手写 FL 仍 deferred）；**FR120 / Epic 61 已关闭**（商业 VIP GPIO / `GpioVip` C1–C4；全 SoC pad 等仍 deferred）；**FR121 / Epic 62 已关闭**（Handshake 默认可综合；AD-25 修订；见 [`docs/fr121-handshake-default.md`](docs/fr121-handshake-default.md)）；**FR122 / Epic 63 已关闭**（官方风格 Chisel 全家桶；AD-27 修订；见 [`docs/fr122-official-style-chisel.md`](docs/fr122-official-style-chisel.md)）；对外宣称按 **FR123**。同源：PRD addendum「Phase 14」与 [`deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md)。

| 加深面 | FR / Epic | 相对 Phase 13 关闭面 |
| --- | --- | --- |
| Tywaves 级 typed IDE 波形 | FR117 / 58 | vs FR104 / FR114 LCOV+树内 GUI — **Epic 58 已关闭**（Story 58.3；自研 typed；Tywaves A deferred） |
| 无 metadata 全树 `#[bitloom::top]` syn-scan | FR118 / 59 | vs FR99 DesignFixture / FR113 Cargo-graph+metadata — **Epic 59 已关闭**（Story 59.3） |
| SymbiYosys/SMT 形式路径 | FR119 / 60 | vs FR100 F1-(i) / FR112 分支 B — **Epic 60 已关闭**（Story 60.3；`just formal-sby-check`） |
| 商业 VIP GPIO 全家桶 | FR120 / 61 | vs FR98 四类近 VIP / FR108 P1–P4 — **Epic 61 已关闭**（Story 61.3；`GpioVip`；见 [`docs/fr120-commercial-vip-gpio.md`](docs/fr120-commercial-vip-gpio.md)） |
| Handshake / 动态数据流默认可综合 | FR121 / 62 | vs FR95/96 / FR110 — **Epic 62 已关闭**（Story 62.3；AD-25 修订；见 [`docs/fr121-handshake-default.md`](docs/fr121-handshake-default.md)） |
| 官方风格 Chisel 全家桶 | FR122 / 63 | vs FR97 / FR111 D1+D3 — **Epic 63 已关闭**（Story 63.3；AD-27 修订；见 [`docs/fr122-official-style-chisel.md`](docs/fr122-official-style-chisel.md)） |

**FR123 宣称纪律：** 「Tywaves / syn-scan / SBY / VIP GPIO / Handshake / 官方风格全家桶」**仅**在对应 **FR116–122** 关闭后方可勾选；**禁止**用 Phase 13 完成面冒充。**FR117 已关闭**仅授权宣称 **自研 typed IDE 波形（子集 B）**；**不得**据此宣称上游 Tywaves 一等集成（→ **FR125 / Phase 15**）。**FR118 已关闭**授权宣称无 metadata workspace `#[bitloom::top]` syn-scan（≠ DesignFixture / metadata alone）。**FR119 已关闭**授权宣称 SymbiYosys/`sby` F1-(ii) 形式路径（≠ FR100 F1-(i) / FR112-B / FR85 alone）；**分支 C 更多 IP 手写 FL → FR126**。**FR120 已关闭**授权宣称商业 VIP GPIO（`GpioVip` C1–C4；≠ FR108 P1–P4 alone / FR98 四类 alone）；**全 SoC pad → FR128**。**FR121 已关闭**授权宣称 Handshake / 动态数据流默认可综合（ready/valid；≠ FR95/96 MVP alone / FR110 alone）；**CIRCT Handshake 全家桶 → FR129**。**FR122 已关闭**授权宣称官方风格 Chisel 全家桶（O1–O4；≠ FR97 alone / FR111 D1+D3 alone / 机械 emit）；**完整 Style Guide / Parser → FR130**。强制 sby CI → **FR127**；`ip.rs` 拆分 → **FR131**。未列入 Phase 15 的 deferred 仍须另开合同（**NFR55**）。

### Phase 15 加深面（合同已批准；闸门 Epic 64 / FR124 已关闭；Phase 15 规划故事已齐 Epic 64–71；Epic 65–71 / FR125–131 均已关闭）

下列原 Phase 14 **NFR51 明示剩余**已由 Correct Course + **FR124** 升格为 Phase 15 显式 FR（Epic 64–71）。对外宣称按 **FR132**。同源：PRD addendum「Phase 15」与 [`deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md)。**不得**把 FR117/119/120/121/122 alone 写成 Phase 15 完成面。

| 加深面 | FR / Epic | 相对 Phase 14 关闭面 |
| --- | --- | --- |
| 上游 Tywaves 一等集成 | FR125 / 65 | vs FR117 自研 typed — **Epic 65 已关闭**（Story 65.3；`--tywaves` / `wave.tywaves.json`） |
| 更多 IP 手写 FL | FR126 / 66 | vs FR103/112-B / FR119-(A) — **Epic 66 已关闭**（Story 66.3；`GpioFunctional`） |
| 默认 CI 强制真 sby | FR127 / 67 | vs 本机 `formal-sby-check` / FR119 — **Epic 67 已关闭**（Story 67.3；CI `formal-sby`） |
| 全 SoC pad / 商业对拍 | FR128 / 68 | vs FR120 GpioVip C1–C4 — **Epic 68 已关闭**（Story 68.3；`GpioSocPad` D1–D4） |
| CIRCT Handshake / 多时钟 | FR129 / 69 | vs FR121 ready/valid（须 AD-25）— **Epic 69 已关闭**（Story 69.3；`schedule_circt_handshake`） |
| 完整 Style Guide ± Parser | FR130 / 70 | vs FR122 O1–O4（可能 AD-27）— **Epic 70 已关闭**（Story 70.3；Style Guide；Parser 未恢复） |
| `ip.rs` 按协议拆分 | FR131 / 71 | 拆分本身；软序先于 FR128 — **Epic 71 已关闭**（Story 71.3；`ip/` 协议模块） |

**FR132 宣称纪律：** 「上游 Tywaves 一等 / 更多 IP FL / 强制 sby CI / 全 SoC pad / CIRCT Handshake / Style Guide·Parser / ip.rs 拆分」**仅**在对应 **FR124–131** 关闭后方可勾选；**禁止**用 Phase 14 完成面冒充。**不得**把 **FR125–131 alone** 写成 Phase 16 / 终局完成面（→ **FR133–140 / Phase 16**）。公开品牌 **Bitloom**。

### Phase 16 加深面（合同已批准；闸门 Epic 72 / FR133 已关闭；Phase 16 规划故事已齐 Epic 72–78；Epic 73 / FR134 已关闭；Epic 74 / FR135 已关闭；Epic 75 / FR136 已关闭；Epic 76 / FR137 已关闭；Epic 77 / FR138 已关闭；Epic 78 / FR139 已关闭）

下列原 Phase 15 **NFR55 明示剩余**已由 Correct Course + **FR133** 升格为 Phase 16 显式 FR（Epic 72–78）。对外宣称按 **FR140**。同源：PRD addendum「Phase 16」与 [`deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md)。**不得**把 FR125–131 alone 写成 Phase 16 / 终局完成面。

| 加深面 | FR / Epic | 相对 Phase 15 关闭面 |
| --- | --- | --- |
| 真实上游 Tywaves GUI / IDE 插件 | FR134 / 73 | vs FR125 `--tywaves` / JSON — **Epic 73 已关闭**（Story 73.3；G1–G4 `--tywaves-gui` / GUI·IDE manifest） |
| 更多 IP 手写 FL（超出 GpioFunctional） | FR135 / 74 | vs FR126 `GpioFunctional` — **Epic 74 已关闭**（Story 74.3；`UartTxFunctional` F1–F3） |
| 全芯片 pad / 多外设对拍 | FR136 / 75 | vs FR128 `GpioSocPad` D1–D4 — **Epic 75 已关闭**（Story 75.3；`ChipPadRing` R1–R4） |
| 外部 CIRCT 真机门禁 | FR137 / 76 | vs FR129 树内 `schedule_circt_handshake` — **Epic 76 已关闭**（Story 76.3；编译门禁 MVP `just circt-external-check` / CI `circt-external`；[`docs/fr137-external-circt-gate.md`](docs/fr137-external-circt-gate.md)） |
| 恢复 Parser + AD-27 再修订 | FR138 / 77 | vs FR130 Style Guide（Parser 未恢复）— **Epic 77 已关闭**（Story 77.3；`BitloomFirrtlParser.parse` P1–P4；[`docs/fr138-parser-restore.md`](docs/fr138-parser-restore.md)） |
| VIP/SocPad 再拆 / IP 跨 crate | FR139 / 78 | vs FR131 `ip/` 协议模块 — **Epic 78 已关闭**（Story 78.3；C1 `ip/gpio/{base,vip,socpad}`；C2 未选） |

**FR140 宣称纪律：** 「终局 / Tywaves GUI·IDE / 更多 IP FL / 全芯片 pad / 外部 CIRCT 门禁 / Parser 恢复 / IP 跨 crate」**仅**在对应 **FR133–139** 关闭后方可勾选；**禁止**用 Phase 15 完成面（含 FR125–131 alone）冒充。终局口径 = 本批关闭 + 诚实 **NFR59** deferred；**不等于**冲 1.0 / 公开 API 稳定（→ **Phase 17**）。公开品牌 **Bitloom**。

### Phase 17 稳定门（合同已批准；闸门 Epic 79 / FR141 已关闭；Epic 79–83 已关闭）

下列 **公开 API 稳定门 / Bitloom 1.0** 项已由 Correct Course + **FR141** 升格为 Phase 17 显式 FR（Epic 79–83）。对外宣称按 **FR147**。同源：PRD addendum「Phase 17」与 [`deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md)。**不得**把 Phase 16 终局 alone 写成 1.0 / 公开 API 稳定。

| 稳定门面 | FR / Epic | 相对 Phase 16 关闭面 |
| --- | --- | --- |
| 公开 API 表面清单 | FR142 / 80 | vs Phase 16 终局完成面；钉死 in/out-of-surface（含 Q1 `bitloom-sim` IN；Q2 hir/builder/vlog publish OK 不进 1.0 承诺）— **Epic 80 已关闭**（Story 80.3；[`docs/public-api-1-0-surface.md`](docs/public-api-1-0-surface.md)） |
| SemVer 1.0 政策 | FR143 / 81 | vs `docs/semver-0x-policy.md` / NFR15；表面内 breaking → major — **Epic 81 已关闭**（Story 81.4；[`docs/semver-1-0-policy.md`](docs/semver-1-0-policy.md)） |
| 破坏性变更 CI 门禁 | FR144 / 81 | `cargo-semver-checks` / `just semver-check`；失败非零 — **Epic 81 已关闭**（同 Story 81.4；CI job `semver-check`） |
| 预 1.0 表面卫生（可选） | FR145 / 82 | 仅阻塞 breaking；无阻塞可 skip（Q3）— **Epic 82 已关闭**（Story 82.3；**FR145-skip** — [`docs/fr145-pre-1-0-hygiene-skip.md`](docs/fr145-pre-1-0-hygiene-skip.md)） |
| 发布 Bitloom 1.0.0 | FR146 / 83 | 版本 / tag `v1.0.0` / CHANGELOG / publish 或 dry-run+清单 — **Epic 83 已关闭**（[`docs/fr146-bitloom-1-0-0-release.md`](docs/fr146-bitloom-1-0-0-release.md)；库 crate 已上架；**CLI → Phase 18 / FR151**） |

**FR147 宣称纪律：** 「1.0 / 公开 API 稳定」**仅**在对应 **FR141–146** 关闭后方可勾选 — **Phase 17 / Epic 79–83 已关闭**（FR146：`v1.0.0` / [`docs/fr146-bitloom-1-0-0-release.md`](docs/fr146-bitloom-1-0-0-release.md)）。**禁止**用 Phase 16 终局 alone 冒充。1.0 口径 = 对 FR142 钉死表面的 SemVer major 承诺 + 诚实 **NFR59** deferred（**NFR63**）；**不等于**清空 NFR59。公开品牌 **Bitloom**。**不等于** CLI 已可 `cargo install`（→ Phase 18）。

### Phase 18 CLI 可发布（**complete** — Epic 84–86 / FR148–153）

下列 **CLI / 依赖 crate crates.io 可发布** 项已由 Correct Course + **FR148** 升格为 Phase 18 显式 FR（Epic 84–86）并**全部关闭**。`bitloom-firrtl` / `bitloom-viz` / `bitloom` **1.0.0** 已上 crates.io；`cargo install bitloom` 可用（**FR151**）；SemVer 默认 minor（**FR153** / [`docs/fr153-semver-honesty.md`](docs/fr153-semver-honesty.md)）。同源：PRD addendum「Phase 18」与 [`deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md)。**不得**把 Phase 17 库 1.0 alone 写成 CLI 已上架（须 FR151）。

| 可发布面 | FR / Epic | 相对 Phase 17 关闭面 |
| --- | --- | --- |
| Phase 18 合同闸门 | FR148 / 84 | Correct Course + README/deferred/AD 诚实边界 — **已关闭** |
| `bitloom-firrtl` 可发布 | FR149 / 85 | rename `rhdl-firrtl` → `bitloom-firrtl`；`publish=true` — **已关闭** |
| `bitloom-viz` 可发布 | FR150 / 85 | rename `rhdl-viz` → `bitloom-viz`；`publish=true` — **已关闭** |
| `bitloom` CLI 1.0.0 上架 | FR151 / 85 | `cargo install bitloom` — **已关闭** |
| `bitloom-lsp` 发布策略 (b) | FR152 / 85 | 默认可 `publish=false`，不挡 CLI 打包 — **已关闭** |
| SemVer / 发版诚实跟进 | FR153 / 86 | assume-published / 移除 1.0.0 特例 — **已关闭** |

**FR148–153 宣称纪律：** 「CLI 已可从 crates.io 安装」对应 **FR151**；SemVer 默认已按已发布 1.0 处理 → **FR153**；完整 Phase 18 宣称须引 **FR148–153**。CLI 上架 ≠ 清空 NFR59（**NFR67**）。公开品牌 **Bitloom**。

### Phase 19 NFR59 + FR152(a)（**complete** — Epic 87–98 / FR154–165；宣称门 **FR156 已关闭**）

下列原 **NFR59** 账本 + **FR152(a)** 已由 Correct Course + **FR154** 升格为 Phase 19 显式 FR（Epic 87–98）。**对外宣称须引对应已关 FR**（**FR156** / **NFR72** / [`docs/fr156-phase19-claim-honesty.md`](docs/fr156-phase19-claim-honesty.md)）。Phase 12–18 关闭面**仍有效**（**NFR68**）。**Phase 19 规划故事已齐（Epic 87–98）**。超出各 epic NFR14 钉死子集的加深仍须新合同（**NFR71**）。同源：PRD addendum「Phase 19」与 [`deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md)。

| 加深 / 上架面 | FR / Epic | 状态（诚实） |
| --- | --- | --- |
| Phase 19 合同闸门 | FR154 / 87 | Correct Course + README/deferred/AD — **已关闭** |
| `bitloom-lsp` FR152(a) live 上架 | FR155 / 88 | `cargo install bitloom-lsp`（1.0.0）— **已关闭**（须引 **FR155**；≠ Phase 18 alone） |
| 自动 FSM 标签提取 | FR157 / 89 | `#[bitloom::fsm]` / `FsmLabels` — **已关闭** |
| 第三方 LCOV GUI 一等集成 | FR158 / 90 | `coverage --genhtml`（`genhtml`/lcov）— **已关闭** |
| MemRead stub→完整生成 | FR159 / 91 | `generate_functional_sim` MemRead（SyncReadMem latency-1）— **已关闭** |
| 非 Cargo monorepo 路径扫描 | FR160 / 92 | `discover_design_roots_under` — **已关闭** |
| formal-sby 镜像卫生 | FR161 / 93 | `ci-sby-pins.env` + hygiene check — **已关闭** |
| 更深 GUI/IDE | FR162 / 94 | 默认 `wave` GUI 主表面（≠ typed-wave alone；≠ FR134 alone）— **已关闭** |
| 未列协议手写 FL | FR163 / 95 | `UartRxFunctional` ≡ tick（≠ FR135 alone；SPI/I2C/AXI 仍 NFR71）— **已关闭** |
| 更广 CIRCT/MLIR / 仿真门禁 | FR164 / 96 | 外部 firtool 仿真门禁（≠ FR137 compile alone；更广 MLIR 仍 NFR71）— **已关闭** |
| 更深 Chisel/Parser 生态 | FR165 / 97 | Style Guide/linter 加深（≠ FR138 alone；HEAD Parser 仍 NFR71）— **已关闭** |
| Phase 19 宣称诚实门 | FR156 / 98 | [`docs/fr156-phase19-claim-honesty.md`](docs/fr156-phase19-claim-honesty.md) — **已关闭**（Story 98.3） |

**FR154–165 宣称纪律：** 「lsp 已上架」须引 **FR155**；各 NFR59 条须引 **FR157–165**；完整 Phase 19 宣称须引 **FR154–165**（**FR156**）。**不得宣称**超出已关 FR / NFR14 钉死子集的完成面（**NFR71** → Phase 20）。`git push` **不是** FR。公开品牌 **Bitloom**。

### Phase 20 加深面（NFR71 四条升格 · FR166–FR171 / NFR73–NFR77；**Epic 99–104 已关闭** / Phase 20 规划故事已齐）

下列原 **NFR71** 账本已由 Correct Course + **FR166** 升格为 Phase 20 显式 FR（Epic 99–104）。**对外宣称须引对应已关 FR**（**FR171** / **NFR77**）。Phase 12–19 关闭面**仍有效**（**NFR73**）。同源：PRD addendum「Phase 20」与 [`deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md)。

| 加深面 | FR / Epic | 状态（诚实） |
| --- | --- | --- |
| Phase 20 合同闸门 | FR166 / 99 | Correct Course + README/deferred/AD — **已关闭** |
| 完整 ChiselSim / 多端 IDE 商店 | FR167 / 100 | `--chiselsim` + Open VSX/JetBrains（≠ FR162/FR134 alone）— **已关闭**（[`docs/fr167-chiselsim-ide-stores.md`](docs/fr167-chiselsim-ide-stores.md)） |
| SPI·I2C·AXI 手写 FL | FR168 / 101 | 三者手写 FL ≡ tick（≠ FR163 alone）— **已关闭**（[`docs/fr168-spi-i2c-axi-handwritten-fl.md`](docs/fr168-spi-i2c-axi-handwritten-fl.md)） |
| 更广 CIRCT/MLIR / firtool 升钉 | FR169 / 102 | multi-lower / HW dialect @ AD-9 pin（≠ FR164 alone；升钉仍 NFR76）— **已关闭**（[`docs/fr169-circt-mlir-allocation.md`](docs/fr169-circt-mlir-allocation.md)） |
| Chisel HEAD Parser 回迁 | FR170 / 103 | update-mainline `parseUpdateMainline` / FIRRTL 6.0.0（≠ FR138/FR165 alone；AD-27 已修订）— **已关闭**（[`docs/fr170-chisel-head-parser.md`](docs/fr170-chisel-head-parser.md)） |
| Phase 20 宣称诚实门 | FR171 / 104 | [`docs/fr171-phase20-claim-honesty.md`](docs/fr171-phase20-claim-honesty.md) — **已关闭**（Story **104.3**；Phase 20 规划故事已齐 Epic 99–104） |

**FR166–171 宣称纪律（FR171 / NFR77）：** 「ChiselSim/多商店已交付」须引 **FR167**；「SPI·I2C·AXI 手写 FL」须引 **FR168**；「CIRCT multi-lower / allocation」须引 **FR169**；「update-mainline / HEAD Parser」须引 **FR170**；完整 Phase 20 宣称须引 **FR166–171**（本 **FR171** 诚实门）。**不得**用 Phase 19 alone 冒充 FR167–170；**不得**宣称「NFR71 账本已空」；超出各 NFR14 钉死子集仍须新合同（**NFR76** → Phase 21）。`git push` **不是** FR。详见 [`docs/fr171-phase20-claim-honesty.md`](docs/fr171-phase20-claim-honesty.md)。

### Phase 21 加深面（NFR76 leftovers 升格 · FR172–FR177 / NFR78–NFR82；合同闸门已关闭 / Epic 105 已关闭）

下列原 **NFR76** leftover 已由 Correct Course + **FR172** 升格为 Phase 21 显式 FR（Epic 105–110）。**对外宣称须引对应已关 FR**（**FR177** / **NFR82**）。Phase 12–20 关闭面**仍有效**（**NFR78**）。同源：PRD addendum「Phase 21」与 [`deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md)。触 **AD-9 / AD-27** / firtool·HEAD 须按 **NFR80** 先修订再 story ready。

| 加深面 | FR / Epic | 状态（诚实） |
| --- | --- | --- |
| Phase 21 合同闸门 | FR172 / 105 | Correct Course + README/deferred/AD — **已关闭**（Story **105.4**；闸门已开） |
| firtool 升钉（配对 AD-9） | FR173 / 106 | Chisel **7.15.0** ↔ firtool-**1.158.0**；AD-9 已修订（≠ FR169 alone）— **已关闭**（[`docs/fr173-firtool-bump-ad9.md`](docs/fr173-firtool-bump-ad9.md)；Story **106.3**） |
| unpaired CIRCT/Chisel HEAD | FR174 / 107 | 文档钉死 unpaired mainline firtool-**1.156.0**（≠ AD-9 **1.159.0**；≠ FR170 alone）— **已关闭**（[`docs/fr174-unpaired-head.md`](docs/fr174-unpaired-head.md)；Story **107.3**） |
| 更广 CIRCT/MLIR/sim | FR175 / 108 | `--ir-sv` + `--ir-verilog` @ AD-9 firtool（现 **1.159.0**；关闭时 **1.158.0**）— **已关闭**（[`docs/fr175-broader-circt-mlir-sim.md`](docs/fr175-broader-circt-mlir-sim.md)；Story **108.3**） |
| 更深 Parser/Chisel 生态 | FR176 / 109 | 组合 FR165+FR170+ecosystem 标记 @ AD-9（≠ FR170/165 alone）— **已关闭**（[`docs/fr176-deeper-parser-chisel-ecosystem.md`](docs/fr176-deeper-parser-chisel-ecosystem.md)；Story **109.3**） |
| Phase 21 宣称诚实门 | FR177 / 110 | [`docs/fr177-phase21-claim-honesty.md`](docs/fr177-phase21-claim-honesty.md) — **已关闭**（Story **110.3**；Phase 21 规划故事已齐 Epic 105–110） |

**FR172–177 宣称纪律（FR177 / NFR82）：** 「firtool 升钉」须引 **FR173**；「unpaired HEAD」须引 **FR174**；「更广 CIRCT/sim」须引 **FR175**；「更深 Parser 生态」须引 **FR176**；完整 Phase 21 宣称须引 **FR172–177**（本 **FR177** 诚实门 — [`docs/fr177-phase21-claim-honesty.md`](docs/fr177-phase21-claim-honesty.md)）。**不得**用 Phase 20 alone 冒充 FR173–176；**不得宣称**未关闭前已交付；**不得**宣称「NFR76 账本已空」；超出各 NFR14 钉死子集仍须新合同（**NFR81** → Phase 22）。`git push` **不是** FR。

### Phase 22 加深面（NFR81 leftovers 升格 · FR178–FR184 / NFR83–NFR87；合同闸门已关闭 / Epic 111 已关闭）

下列原 **NFR81** leftover 已由 Correct Course + **FR178** 升格为 Phase 22 显式 FR（Epic 111–117）。**对外宣称须引对应已关 FR**（**FR184** / **NFR87**）。Phase 12–21 关闭面**仍有效**（**NFR83**）。同源：PRD addendum「Phase 22」与 [`deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md)。触 **AD-9 / AD-25 / AD-27 / FR142** 须按 **NFR85** 先修订再 story ready。**Epic 112–117 在 Epic 111 关闭前不得 ready**；须各自 NFR14。

| 加深面 | FR / Epic | 状态（诚实） |
| --- | --- | --- |
| Phase 22 合同闸门 | FR178 / 111 | Correct Course + README/deferred/AD — **已关闭**（Story **111.4**；闸门已开） |
| 浮动 CIRCT git HEAD | FR179 / 112 | 文档钉死浮动轨 firtool-**1.159.0**（渠道 ≠ AD-9 产品 cache；≠ FR174 **1.156.0**）— **已关闭**（[`docs/fr179-floating-circt-git-head.md`](docs/fr179-floating-circt-git-head.md)；Story **112.3**） |
| Handshake dialect 加深 | FR180 / 113 | `handshake.fork`+`handshake.join` beyond FR129 C1–C4 — **已关闭**（[`docs/fr180-handshake-dialect-deepen.md`](docs/fr180-handshake-dialect-deepen.md)；Story **113.3**） |
| 更深 Style Guide / linter | FR181 / 114 | wartremover + fatal-warnings beyond FR176 — **已关闭**（[`docs/fr181-deeper-style-guide-linter.md`](docs/fr181-deeper-style-guide-linter.md)；Story **114.3**） |
| unpaired firtool 产品钉再升钉 | FR182 / 115 | AD-9 默认钉 **1.159.0** + Chisel **7.15.0**（*unpaired product-pin*；≠ FR173/174/179 alone）— **已关闭**（[`docs/fr182-unpaired-firtool-product-pin.md`](docs/fr182-unpaired-firtool-product-pin.md)；Story **115.3**） |
| 显式扩大 FR142 公开 API | FR183 / 116 | 文档化 `bitloom-firrtl` interop 升入表面（≠ 静默扩大；AD-6 仍 prelude-only）— **已关闭**（[`docs/fr183-explicit-fr142-api-expand.md`](docs/fr183-explicit-fr142-api-expand.md)；Story **116.3**） |
| Phase 22 宣称诚实门 | FR184 / 117 | 宣称须引 FR178–184 — **已关闭**（[`docs/fr184-phase22-claim-honesty.md`](docs/fr184-phase22-claim-honesty.md)；Story **117.3**；Phase 22 规划+实现故事已齐 Epic **111–117**） |

**FR178–184 宣称纪律（FR184 / NFR87）：** 「浮动 HEAD」须引 **FR179**；「Handshake」须引 **FR180**；「Style/linter」须引 **FR181**；「unpaired 产品钉」须引 **FR182**；「FR142 显式扩展」须引 **FR183**；完整 Phase 22 宣称须引 **FR178–184**（**FR184** 诚实门）。**不得**用 Phase 21 alone 冒充 FR179–183；**不得宣称**未关闭前已交付；**不得**宣称「NFR81 账本已空」；超出各 NFR14 钉死子集仍须新合同（**NFR86** → Phase 23）。`git push` **不是** FR。

### Phase 23 加深面（NFR86 leftovers 升格 · FR185–FR191 / NFR88–NFR92；合同闸门已关闭 / Epic 118 已关闭）

下列原 **NFR86** leftover 已由 Correct Course + **FR185** 升格为 Phase 23 显式 FR（Epic 118–124）。**对外宣称须引对应已关 FR**（**FR191** / **NFR92**）。Phase 12–22 + 工程结项关闭面**仍有效**（**NFR88**）。同源：PRD addendum「Phase 23」与 [`deferred-work.md`](_agile-output/implementation-artifacts/deferred-work.md)。触 **AD-9 / AD-25 / AD-27 / FR142** 须按 **NFR90** 先修订再 story ready。**Epic 119–124 在 Epic 118 关闭前不得 ready**（闸门现已开）；须各自 NFR14。

| 加深面 | FR / Epic | 状态（诚实） |
| --- | --- | --- |
| Phase 23 合同闸门 | FR185 / 118 | Correct Course + README/deferred/AD — **已关闭**（Story **118.4**；闸门已开） |
| 无界 CIRCT tip | FR186 / 119 | 超 FR179 浮动轨；live tip 渠道 `circt-live-tip` — **已关闭**（[`docs/fr186-unbounded-circt-tip.md`](docs/fr186-unbounded-circt-tip.md)；Story **119.3**） |
| Handshake lower/dialect | FR187 / 120 | 超 FR180 fork+join；`branch`+`merge` — **已关闭**（[`docs/fr187-handshake-lower-deepen.md`](docs/fr187-handshake-lower-deepen.md)；Story **120.3**） |
| Style Guide 全家桶 | FR188 / 121 | 超 FR181 wartremover deepen；community-style-guide+scalafmt — **已关闭**（[`docs/fr188-community-style-guide-pack.md`](docs/fr188-community-style-guide-pack.md)；Story **121.3**） |
| 继续 firtool 产品钉升钉 | FR189 / 122 | 超 FR182 1.159.0 — **blocked-upstream**（无已发布 firtool >1.159.0；Correct Course pending） |
| 继续显式扩 FR142 | FR190 / 123 | 超 FR183；emit/import/check_* — **已关闭**（[`docs/fr190-further-fr142-api-expand.md`](docs/fr190-further-fr142-api-expand.md)；Story **123.3**） |
| Phase 23 宣称诚实门 | FR191 / 124 | 宣称须引 FR185–191 — **backlog** |

**FR185–191 宣称纪律（FR191 / NFR92）：** 「无界 tip」须引 **FR186**；「Handshake lower」须引 **FR187**；「Style 全家桶」须引 **FR188**；「firtool 再升钉」须引 **FR189**；「继续扩 FR142」须引 **FR190**；完整 Phase 23 宣称须引 **FR185–191**（**FR191** 诚实门）。**不得**用 Phase 22 / 结项 alone 冒充本批五条；**不得宣称**未关闭前已交付；超出各 NFR14 钉死子集仍须新合同（**NFR91**）。`git push` **不是** FR。

**（历史口径）NFR59 曾 deferred（NFR63 / NFR67）— 现已由 Phase 19 合同升格；未关 FR 前仍不得冒充已清：**

- 自动 FSM 标签提取 → **FR157**（**Epic 89 已关闭** — [`docs/fr157-auto-fsm-labels.md`](docs/fr157-auto-fsm-labels.md)）
- 第三方 LCOV GUI 一等集成 → **FR158**（**Epic 90 已关闭** — [`docs/fr158-third-party-lcov-gui.md`](docs/fr158-third-party-lcov-gui.md)）
- emit MemRead stub → 完整生成 → **FR159**（**Epic 91 已关闭** — [`docs/fr159-memread-full-emit.md`](docs/fr159-memread-full-emit.md)）
- 非 Cargo 全 monorepo 任意路径扫描 → **FR160**（**Epic 92 已关闭** — [`docs/fr160-non-cargo-path-scan.md`](docs/fr160-non-cargo-path-scan.md)）
- GHA `formal-sby` 镜像卫生跟踪 → **FR161**（**Epic 93 已关闭** — [`docs/fr161-formal-sby-image-hygiene.md`](docs/fr161-formal-sby-image-hygiene.md)）
- 更深 GUI/IDE 子集（替换默认 VCD/`typed-wave.html` 唯一波形面）— ≠ FR134 G1–G4 alone → **FR162**（**Epic 94 已关闭** — [`docs/fr162-deeper-gui-ide-default-wave.md`](docs/fr162-deeper-gui-ide-default-wave.md)；完整 ChiselSim / 额外 IDE 商店多端 → **FR167** / **Epic 100 已关闭**）
- 未列入协议手写 FL（`UartRx` beyond `UartTx`）— ≠ FR135 `UartTx` alone；FR126 Gpio 仍有效 → **FR163**（**Epic 95 已关闭** — [`docs/fr163-unlisted-protocol-handwritten-fl.md`](docs/fr163-unlisted-protocol-handwritten-fl.md)；SPI / I2C / AXI 手写 → **FR168** / **Epic 101 已关闭**）
- 更广 CIRCT/MLIR lower / 仿真门禁加深 — ≠ FR137 编译门禁 MVP alone；FR129 C1–C4 仍有效 → **FR164**（**Epic 96 已关闭** — [`docs/fr164-circt-external-sim-gate.md`](docs/fr164-circt-external-sim-gate.md)；更广 MLIR allocation → **FR169** / **Epic 102 已关闭**；firtool 升钉超 AD-9 仍须新合同 **NFR76**）
- 更深 Chisel/Parser 生态（社区 Style Guide/linter 全家桶；任意 Chisel HEAD Parser 回迁）— ≠ FR138 P1–P4 alone；FR130 Style Guide 仍有效 → **FR165**（**Epic 97 已关闭** — [`docs/fr165-deeper-chisel-parser-ecosystem.md`](docs/fr165-deeper-chisel-parser-ecosystem.md)；任意 Chisel HEAD Parser → **FR170** / **Epic 103 已关闭**；unpaired HEAD/firtool 升钉仍须新合同 **NFR76**）

**明确 deferred / 未承诺为产品完整面（可延期；上列五项已不再是永久非目标）：**

- 完整 / 按键全 elaborate Bitloom LSP（**FR99 / Epic 44 已关闭**。Epic 39 **FR91 Path B** 显式 defer 仅为**历史**关闭路径，**不得**再当作 Phase 12 完成口径。宿主 rust-analyzer（FR90）仍可用，**不替代** FR99。层次/时序 HTML **≠ LSP**；见 [`docs/fr99-bitloom-lsp.md`](docs/fr99-bitloom-lsp.md)、[`docs/fr90-host-ide-rust-analyzer.md`](docs/fr90-host-ide-rust-analyzer.md)、[`docs/fr38-viz-lsp.md`](docs/fr38-viz-lsp.md)）
- 部分 CLI 动词（`check` / `build-sim`）
- crates.io 名 `rhdl` / `rhdl-bits`（禁止）
- 自动等价证明 / SystemC TLM-2.0 产品（**FR100 / Epic 45 已关闭** — 形式等价产品 + IP 双模型；**FR101 / Epic 46 已关闭** — LT-only MVP；「不承诺 SystemC TLM」不再是完成排除项；AT → **FR107**；见 [`docs/fr101-systemc-tlm.md`](docs/fr101-systemc-tlm.md) / doc-19）

详见 [`docs/semver-0x-policy.md`](docs/semver-0x-policy.md) 与 [`docs/crates-io-publish-bitloom.md`](docs/crates-io-publish-bitloom.md)。
