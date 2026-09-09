# Language surface

Catalog for CAP-1…CAP-3、CAP-7、CAP-10、CAP-11。HOW（宏如何展开、freeze 何时跑）见架构脊柱。

## Hardware types (baseline)

- `Bool`, `Bits<N>`, `UInt<N>`, `SInt<N>`, `Clock`, `Reset`
- Ports: `Input<T>`, `Output<T>` — not bare `UInt` at the module boundary
- Builder facade `Bits<N>` lowers to runtime widths on FrozenHir / `PortValues`

## Composite types (CAP-10 / FR51 / FR80)

- `Bundle` 与 `Vec<T,N>`（或文档等价）允许进入可综合路径。
- **文档等价：** 公开类型名为 `HwVec<T,N>`（避免与堆 `Vec` / E0141 冲突）；合同与叙述中的 `Vec<T,N>` 即指 `HwVec`。`N` 须 > 0。
- **展平：** elaborate 时展平为标量 HIR 端口；叶命名 `{field}_{member}`（Bundle ground）、`{field}_{nested}_{leaf}`（一层嵌套 Bundle，FR80）、`{field}_{i}`（HwVec）。公开 HIR 可不含 Bundle/Vector 节点。
- **嵌套 Bundle（FR80）：** 至少**一层**文档化嵌套（子 Bundle 作父成员，经 `Bundle::nested_bundles` → 子 `leaves`）可 elaborate → emit `.v` → tick。夹具：`examples/bundle_vec_skel` 嵌套正/负例。更深嵌套（≥2 层）为本 epic **默认非目标**，不得静默声称任意深度。
- **仍 OUT OF SCOPE：** `HwVec<Bundle,_>` — `HwVec` 元素须 `AsGround`。负向：trybuild `nested_hwvec_bundle`。
- **`#[derive(Bundle)]`（FR80）：** 经 `bitloom-prelude` 可用（AD-6；设计 crate 勿直接依赖 `bitloom-macro` / CLI）。支持：具名字段 struct；ground 字段 `Bool`/`Clock`/`Reset`/`UInt<N>`/`SInt<N>`/`Bits<N>`；其它简单 path 类型作为**一层**嵌套 Bundle。限制/拒绝（稳定 `rhdl::E0180`）：enum、tuple/unit struct、结构体泛型、`HwVec<_>`/`Input<_>`/`Output<_>` 字段、非 path 字段。夹具：`examples/bundle_vec_skel` 正例 + trybuild 负例。手写 `Bundle` 仍可用。
- **叶名碰撞：** `{field}_{member}` / `{field}_{nested}_{leaf}` / `{field}_{i}` 与已有信号冲突 → emit 前失败（`rhdl::E0152`）。
- 位宽/方向不匹配必须在 emit 前失败（含嵌套叶）；不得 silently 可用却无检查。
- HIR ground 是否扩展 Bundle/Vector 节点由实现选择；公开表面与 emit 语义须一致（AD-20）。
- **FR22 边界：** 单时钟表面加厚（FR22）的构造条**不含** Bundle/Vec；复合类型由本节 / FR51 / FR80 交付，不得 silently 算进 FR22 验收。
- **文档收口（Story 32.4）：** `docs/tutorials/nested-bundle.md` · `docs/fr80-nested-bundle.md`；ATDD
  `cargo test -p bitloom --test fr80_nested_bundle`（嵌套正/负例矩阵 + 限制表 + NFR14 勾选）。

## ClockDomain (CAP-11 / FR52) + CDC 真 RTL (FR79 / AD-29)

- **API 映射（产品面；对齐 AD-22 / AD-29）：**
  - 域标记：`ClockDomain::<ID>`（prelude ZST）+ `ElaborateSession::bind_domain(name, id)`（session 域标签；**非**独立 `Signal<D,T>` 包装）
  - **FR79 DoubleFlop 真 RTL：** `DoubleFlop::elaborate()` / `elaborate_width` → HIR 含 `sync_ff0`/`sync_ff1`；session
    `declare_double_flop_stages` + `connect_double_flop`。emit `.v` 须可见两级 reg + `always @(posedge …)`。
    **级数=2**；`dout` 延迟 = **2** 目的域 tick（`DoubleFlop::LATENCY_DST_TICKS`；MVP = 全局 `Sim::tick`）。
    **非**物理亚稳态/MTBF 合同。夹具：`examples/doubleflop_skel`；ATDD：`fr79_doubleflop_rtl`。
  - **FR79 SyncFIFO 真 RTL：** `SyncFIFO::<4, 8>::elaborate()` → mem + 灰码指针 + `w2r_*`/`r2w_*` DoubleFlop +
    `full`/`empty`。**DEPTH=4 / WIDTH=8** 文档化最小子集；`LATENCY_PTR_SYNC_TICKS=2`；注册读 +1。
    **≠** `ip::SyncFifo`（FR82 单时钟）。夹具：`examples/syncfifo_skel`；ATDD：`fr79_syncfifo_rtl`；
    文档：`docs/fr79-syncfifo-cdc.md`。
  - **历史最小合同（Epic 7 / FR52）仍有效：** `mark_cdc_bridge(name)` 允许跨域（诊断文案仍点名 DoubleFlop/SyncFIFO）；
    **不得**用该最小合同冒充 FR79 深度（NFR37）。
  - 非法跨域无 bridge：`assign_net` **与** `assign_reg_d_from` 跨域 → `finish`/`freeze` 失败，诊断码 **`rhdl::E0220`**
  - 同步/异步复位：`declare_reg_ex(..., async_reset, has_enable, ...)`；**极性** = 默认同步**高有效** `Reset`（AD-15，无 ActiveLow API）
  - 仿真步进：全局 `Sim::tick` 为「按域 tick」的 MVP 等价（尚无独立 per-domain tick 引擎）；RegD 按 **NBA** 提交（双 FF 延迟不折叠）
- 夹具：`examples/clockdomain_skel`（FR52）；`examples/doubleflop_skel` / `examples/syncfifo_skel`（FR79）。
- **文档收口（Story 31.4）：** `docs/tutorials/cdc-depth.md`；ATDD
  `cargo test -p bitloom --test fr79_cdc_depth_closeout`（黄金矩阵 +「仅 ZST/bridge」对照 + NFR14 勾选）。
- 默认模块仍是单时钟：恰好一个 `Clock` + 同步高有效 `Reset`（AD-15），除非显式声明多域。
- 域为 session 标签：多域夹具可仍用一对 `clk`/`rst` 端口（非每域独立时钟端口）。

## Still deferred from this catalog

见 `later-product.md` / PRD：`Analog`, `InOut`、浮点 crate（FR36）等——有独立 FR，不并入 CAP-1。

## Mem / SyncReadMem (AD-21 / FR26)

- 表面：`Mem` / `SyncReadMem`（prelude ZST 标记）+ session `declare_mem` / `declare_sync_read_mem`。
- 互转/降级锚 FIRRTL `mem`；`emit_chisel`：**Path A 子集内可编译**（`Mem`/`SyncReadMem` + 可选常量 init）；**子集外仍 E0901**（FR81 / Epic 33）。
- **Story 33.4 收口：** ATDD `fr81_path_a_mem_chisel_emit` + `fr81_mem_chisel_atdd_fr71`；Mem 合同夹具 `testdata/fr81_path_a_sync_read_mem.scala`（可选 `just chisel-fr81-mem-jvm`）；**FR71 required** 仍为无 Mem 的 `fr28_golden_counter.scala` / `just chisel-fr28-jvm` / GHA `fr28-chisel-jvm`。用户文档边界见 `docs/fr28-chisel-compilable.md`。

## Elaborate-time Mem init generators (FR73 / Epic 27)

- **API：** `ElaborateSession::declare_mem_with_init_fn` /
  `declare_sync_read_mem_with_init_fn`（或 `generate_mem_init` + `declare_*_with_init`）。
- **语义：** 在 `ElaborateSession` 内执行非捕获 `Fn(usize) -> u64`，写入 `MemDecl.init: Option<Vec<u64>>`；freeze 后 **无**闭包节点（NFR36 / AD-18）。
- **Emit：** Verilog `initial` 块可见初值；FIRRTL 以 `; mem-init …` 注释记录。
- **捕获：** 见下节 NFR35；`rhdl::E0142`。
- **ATDD golden（Story 27.4）：** 同一 CRC/LUT 算法闭包生成 vs 手写常量表 → emit / `tick` 等价；抽检 `.v`/FIRRTL **无** closure/callback IR（NFR36）。夹具：`crates/bitloom/tests/fr73_crc_lut_golden.rs`（`cargo test -p bitloom --test fr73_crc_lut_golden`）。
- **最小用法（Bitloom / `bitloom-prelude`）：**

```ignore
use bitloom_prelude::{ElaborateSession, GroundType, Span};

let mut s = ElaborateSession::new("CrcRom");
s.begin_module("CrcRom", Span::default());
s.add_input("clk", GroundType::Clock, Span::default());
s.add_input("rst", GroundType::Reset, Span::default());
// Elaborate-time Fn → plain MemDecl.init words (no closure in emit).
s.declare_mem_with_init_fn("crc", 16, 8, |i| /* crc8(i) */ i as u64, Span::default());
s.end_module();
let frozen = s.finish()?;
```

- **非目标（本 epic）：** comb/seq 内联可综合闭包（→ Epic 28）。
- 设计 crate 仅依赖 `bitloom-prelude`（AD-2）。

## Elaborate-time module factory (FR73 / Cap-R-53)

- **API：** `ElaborateSession::generate_instances(n, |i, s| { s.add_instance(...); })` 或
  `generate_instances_from(n, |i| GeneratedInstance::new(...), span)`（prelude 再导出 `GeneratedInstance`）。
- **语义：** 在 elaborate 内批量实例化子模块并完成类型安全 connect；freeze 后仅普通 `Stmt::Instance` / `PortConnect`（NFR36）。
- **错误：** 宽度/方向等既有实例校验仍在 `finish` 前失败（FR8 / E0203 等）；硬件引用捕获 → `rhdl::E0142`。
- **非目标：** comb/seq 内联工厂。

## Elaborate-time vs capturing (NFR35 / AD-18)

| 类别 | 含义 | 诊断 |
|------|------|------|
| **Elaborate-time 非捕获 `Fn`** | 在 session 内生成 Mem init / 工厂 Instance，freeze 前消解为普通 HIR（FR73） | 正例：空 `assert_no_hw_capture(&[])` + generator/factory |
| **捕获硬件引用** | 闭包环境持有 Wire / Reg / 端口等硬件句柄（文档标记 `HwCaptureRef`） | **`rhdl::E0142`** via `reject_hw_capture` / `assert_no_hw_capture` — **不得** silent 成功 |
| **周期精确捕获闭包（FR16）** | 进入 cycle-accurate / `tick` 路径的捕获闭包等不可综合构造 | **`rhdl::E0141`** via `reject_unsynthesizable("capturing closure", …)`（与 E0142 分立） |

- **Wire/Reg 映射：** `HwCaptureRef::wire(name)` / `::reg(name)` / `::signal(name)`（prelude 再导出）。
- **夹具：** `crates/bitloom/tests/fr73_hw_capture_diag.rs`。

## SynthesizableClosure constraints + check hook (FR74 / Cap-R-48…50 / Cap-R-60)

- **约束（文档化 / 标记 trait `SynthesizableClosure`）：**
  - **纯函数**（无副作用 / I/O / 线程）— Cap-R-50
  - **无堆**（软件 `Box` / `Vec` / `String` 等；非硬件 `HwVec`）— Cap-R-48
  - **无运行时捕获状态**（非 `const` 捕获；Wire/Reg 硬件引用仍走 **E0142**）— Cap-R-49
  - 冻前消解为普通 HIR；**不得**作为 Rust 闭包对象进入 `tick` / FIRRTL / Chisel（NFR36）
- **检查钩子（Cap-R-60）：** `ElaborateSession::check_synthesizable_closure` /
  `reject_unsynthesizable_closure` / `check_synthesizable_closure_marker`；自由函数
  `diagnose_synthesizable_closure_violations`（供 `cargo bitloom check` 或等价包装）。
  空 violation 列表 / `LegalEmptyClosure` / `LegalSimpleClosure` = 合法。
- **稳定诊断码：**

| 违规 | 码 | API |
|------|----|-----|
| 堆分配 | **`rhdl::E0143`** | `SynthesizableClosureViolation::heap` |
| 运行时捕获状态 | **`rhdl::E0144`** | `::runtime_capture_state` |
| 不纯 / 副作用 | **`rhdl::E0145`** | `::impure` |
| 时序闭包非法可变借用（Cap-R-70） | **`rhdl::E0146`** | `SeqOwnershipViolation::illegal_mutable_borrow` |
| 周期精确捕获闭包（FR16） | **`rhdl::E0141`**（分立） | `reject_unsynthesizable` |
| 硬件引用捕获（FR73） | **`rhdl::E0142`**（分立） | `reject_hw_capture` |

- **非目标（本故事段已交付检查；内联见下）：** 不在 FIRRTL/Chisel 增加闭包 IR（NFR36）。
- **夹具：** `crates/bitloom/tests/fr74_synthesizable_closure_check.rs`。

## Comb inline synthesizable closures (FR75 / Cap-R-55)

- **API（elaborate 期内联）：** `ElaborateSession::inline_comb_fn(dst, args, violations, span, |args| CombInline::…)`
  与 `inline_comb_fn_marker`；设计 crate 经 `bitloom-prelude` 使用 `CombInline`。
- **语义：** 先跑 Cap-R-60 `check_synthesizable_closure`；合法则执行闭包一次，将
  `CombInline`（`Ref` / `Lit` / `Add` / `Sub` / `And` / `Or` / `Xor` / `Eq` / `Mux`）
  降到既有 `assign_*` / Wire 赋值；**不得**把 `Fn` 写入 FrozenHir（NFR36）。
- **范围：** 仅组合过程（`begin_combinational` / `#[combinational]`）；时序内联见下节 Cap-R-56。
- **AD-18：** 不完整 if/else 赋值仍报 **`rhdl::E0110`**（与手写 `assign_*` 相同）。
- **夹具：** `crates/bitloom/tests/fr75_comb_inline_closure.rs`。

## Seq inline synthesizable closures (FR75 / Cap-R-56 / Cap-R-70)

- **API（elaborate 期内联）：** `ElaborateSession::inline_seq_fn(dst_reg, args, synth_violations, ownership_violations, span, |args| SeqInline::…)`
  与 `inline_seq_fn_marker`；设计 crate 经 `bitloom-prelude` 使用 `SeqInline` / `SeqOwnershipViolation`。
- **语义：** 先跑 Cap-R-60，再跑 Cap-R-70（token + 自动检测同过程二次 `Reg.d` 写）；合法则执行闭包一次，将
  `SeqInline`（`Inc` / `Comb(CombInline)`）降到普通时序 `AssignExpr` 目标 `Reg.d`；**不得**把 `Fn` 写入 FrozenHir（NFR36）。
- **Cap-R-70 稳定码：** **`rhdl::E0146`** — illegal mutable signal borrow / second `Reg.d` in the same sequential process（`SeqOwnershipViolation::illegal_mutable_borrow`）。
- **AD-4：** 跨过程多驱动仍为 **`rhdl::E0140`**（展开后的网照常进 freeze）。
- **夹具：** `crates/bitloom/tests/fr75_seq_inline_closure.rs`。

## FR74/FR75 + FR16 coexistence matrix (Story 28.4 / NFR35)

- **正例：** 合法 `inline_comb_fn` / `inline_seq_fn`（及 Cap-R-60 空 violation）elaborate 成功；emit `.v`/FIRRTL **无** closure/callback IR（NFR36）。
- **负例（稳定码并存，互不吞并）：**

| 场景 | 码 |
|------|----|
| 堆 | `rhdl::E0143` |
| 运行时捕获状态 | `rhdl::E0144` |
| 不纯 / 非法 IO | `rhdl::E0145` |
| 捕获 Wire/Reg（**勿捕获 Wire**） | `rhdl::E0142` |
| 周期精确捕获闭包（FR16） | `rhdl::E0141` |
| 时序非法可变借用（Cap-R-70） | `rhdl::E0146` |

- **警告：** 可综合 / 生成器闭包**不得**捕获 `Wire`/`Reg`；将信号名以 `&str` 传给 `inline_*_fn` / generator API。
- **最小示例：** 见仓库根 `README.md`（comb add + seq Inc）。
- **夹具：** `crates/bitloom/tests/fr74_fr75_fr16_coexist_matrix.rs`（汇总；深度行仍见 `fr74_*` / `fr75_*` / `fr73_hw_capture_diag`）。
- **配方：** `cargo test -p bitloom --test fr74_fr75_fr16_coexist_matrix`（亦由 `just test` 覆盖）。

## HLS dataflow closures (FR76 / Cap-R-62 / Cap-R-71 / Epic 29.2)

- **路径：** AD-25 外挂 HLS only（`bitloom::hls` / `cargo bitloom hls`）；**不是**可综合 comb/seq 表面。
- **约束类（D1）：** `HlsClosureConstraintClass::HlsFree` — 自由闭包仅外挂 HLS / 功能侧；可综合腿仍用 `SynthesizableClosure`（FR74）。
- **API：** `dissolve_dataflow_transform(name, violations, || HlsDataflowOp::…)` 在发射 prep 执行一次闭包，展开为 C 运算；`run_hls_dissolved` / CLI `--dataflow add|identity|add1|xor_a5`。
- **语义：** 调度/降低**前**消解；C / RTL **无** `Fn` / closure 残留（NFR36）；无树内 scheduler（AD-25）。
- **检查：** 非空 `HlsDataflowClosureViolation`（捕获 / 错路径）→ 可读失败，不 silent 进后端。
- **消歧：** ≠ **FR47** dual-sim generators（下节）；≠ Phase 7「闭环」。
- **ATDD：** `cargo test -p bitloom --test fr76_hls_dataflow_closure`（emit-only + bambu-ci-stub RTL）。
- **透明矩阵（29.4）：** `cargo test -p bitloom --test fr76_fr77_nfr36_transparency_matrix`。
- **文档：** `docs/fr35-hls.md`；NFR14：`nfr14-risk-epic29-hls-ip-closures.md`。

## Comb / seq

- `#[combinational]` and `#[sequential]` are mandatory.
- Comb may drive `Wire` / `Output` only; incomplete assignment is an error (no inferred latch).
- Only seq writes `Reg.d`. Comb must not write `Reg.d`; seq must not drive combinational nets.
- Stage-2 surface thickening (FR22 / AD-20): `if`/`match`（或等价）、严格同位宽二元运算与连接、显式 pad/trunc、同步复位赋值语义。Bundle/Vec 不在 FR22 构造条内——见上文 Composite types / FR51。
- Comb 可综合闭包内联：见上节 FR75 / Cap-R-55。
- Seq 可综合闭包内联：见上节 FR75 / Cap-R-56 / Cap-R-70。

## Width

- Surface arithmetic and connections are strict same-width.
- Extend/truncate only via explicit pad/trunc nodes.
- FIRRTL `add` n+1 is allowed only as those explicit nodes, never as silent prelude truncation.

## Synthesizable subset (cycle-accurate / generate path)

Allowed: hardware types and their ops; `if` / `match`; statically bounded loops that fully unroll; inlined functions; const generics; arrays / structs / enums / Bundle / Vec used as hardware aggregates in-scope; **elaborate-time non-capturing generator `Fn`** that dissolves to Mem/ROM init or factory Instance/Connect before freeze (FR73 / AD-18); **SynthesizableClosure-constrained closures** checked via Cap-R-60 (FR74) and **comb-inlined** via `inline_comb_fn` → ordinary AssignExpr (FR75 / Cap-R-55) or **seq-inlined** via `inline_seq_fn` → ordinary `Reg.d` (FR75 / Cap-R-56) with Cap-R-70 ownership (`rhdl::E0146`).

Rejected on this path: heap `Vec`/`Box`/`String`（软件堆，非硬件 `Vec<T,N>`）→ FR74 **`rhdl::E0143`** when diagnosed via SynthesizableClosure check；unbounded recursion; `dyn Trait`; **capturing** closures / runtime capture state → **`rhdl::E0141`** / **`rhdl::E0144`** / **`rhdl::E0142`** as documented；file/net/threads / impure → **`rhdl::E0145`**；seq illegal mutable borrow → **`rhdl::E0146`**；default `f32`/`f64`（可综合浮点见 FR36）；Rust 闭包对象进入 `tick` / 后端 IR（NFR36）。

Functional view（手写 `#[functional_model]` 或 CAP-13 生成的 Rust crate）may use rejected constructs. Fields marked `#[functional_state]` never enter HIR.

## FR47 dual-sim generators (MVP subset)

`generate_functional_sim` / `generate_cycle_accurate_sim`（及 CLI `gen-func` / `gen-cycle`）当前 **MVP = 扁平单模块**：只消费顶层一个 module；**不**保证层次实例（`instances`）或 `MemDecl` 的周期精确 emit。扩到层次/mem 须单独故事并先改本段与 `docs/fr47-dual-sim-generation.md` / `deferred-work.md`——禁止静默扩子集。

**消歧（Epic 29）：** FR47「sim generators」= 从 FrozenHir **生成**功能/周期精确 **Rust crate**。**不是** FR73/FR76/FR77 的 elaborate-time / HLS 数据流 **闭包**定制。不得用 FR47 完成话术冒充 Epic 29 关闭。

## Sequential envelope (default)

Every default module has exactly one `Clock` port and one sync active-high `Reset` port. `tick` is one posedge of that clock. No implicit ports at emit. Multi-clock / async reset / enables：见 PRD FR23–FR25 与脊柱 AD-22/AD-23。

## First-class IP (FR37 / FR48 / FR82)

- Surface：`bitloom_prelude::ip::{SyncFifo, UartTx, SpiMaster, I2cMaster, Axi4LiteSlave, ExtBlackBox}`。
- **FR82（Epic 34）：** `SyncFifo` = depth-4 sync FIFO；`UartTx` = 8N1 bit-bang；`SpiMaster` = Mode-0-ish MSB byte shifter；`I2cMaster` = START+8data+STOP bit-bang；`Axi4LiteSlave` = 单寄存器 AXI4-Lite write/read 握手玩具（ADDR=8, DATA=32）。均无生成器闭包参数。
- **Epic 29 handoff / FR77：** Epic 34 = 无闭包可综合基线；Epic 29 / FR77 = 闭包定制 **overlay** — `bitloom_prelude::ip::Crc8Lut`（`elaborate_with_table_fn` / 默认 poly `0x07`；Story 29.3）。相对 Epic 22 stub：**NFR37** 规划 done ≠ FR82 深度。索引：`docs/ip/README.md`。
- **黑盒：** `ExtBlackBox` 仅端口、空 body；`vendor_blackbox_v()` 旁路；不内联 vendor 网表进 HIR。
- **Sim：** 同周期输入门控须 `set_inputs` → `Sim::settle` → `tick`（见 `docs/ip/README.md`）。
- **ATDD 配方：** `cargo test -p bitloom --test fr82_ip_baseline_matrix`（+ sibling `fr82_*_baseline`）；FR77：`cargo test -p bitloom --test fr77_ip_generator_closure`。
- 索引与已知限制：`docs/ip/README.md`（NFR37：基线 ≠ 全协议 / VIP / Full AXI）。

## IP generator closures (FR77 / Cap-R-63 / Cap-R-64 / Epic 29.3–29.4)

- **API：** `Crc8Lut::elaborate()`（文档默认 poly `DEFAULT_POLY = 0x07`）或 `Crc8Lut::elaborate_with_table_fn(violations, f)`（`Fn(usize) -> u64` → SyncReadMem init）。
- **约束类（D1）：** 可综合腿走 `SynthesizableClosure`；非空 `violations` → 明确诊断，不静默回退默认表。
- **NFR36 / Cap-R-64：** 闭包在 elaborate 内消解；FrozenHir / Verilog / FIRRTL / **viz** 无闭包 IR。
- **透明矩阵（29.4）：** `cargo test -p bitloom --test fr76_fr77_nfr36_transparency_matrix`（+ sibling `fr77_ip_generator_closure`）。
- **文档：** `docs/ip/README.md`；风险：`nfr14-risk-epic29-hls-ip-closures.md`。
- **非目标：** 不要求 IP 内 comb/seq 可综合闭包内联（Epic 28）；不把 HLS `HlsFree` 用到可综合 IP 腿；≠ FR47 sim generators。

## Bridge adapter closure templates (FR78 / Cap-R-65…68 / Epic 30.2–30.4)

- **API（prelude / 验证辅助）：** `bitloom_prelude::StartWaitComplete`（默认 `start_wait_complete`）与自由函数 `bitloom_prelude::start_wait_complete`（文档等价）。
- **语义：** host/桥接侧「启动 → 等待完成」握手模板；`start_fn` 为**自由** Rust 闭包（Cap-R-66/67），仅设置信号级字段；循环调用 `tick` / `is_busy`。
- **NFR36：** 模板展开后周期精确侧仅见普通信号 / `PortValues`；`Fn` **不得**进入 FrozenHir / `Sim::tick` / Verilog / FIRRTL / Chisel。
- **消歧（术语表）：** 生成器闭包（FR73）≠ FR47 sim generators ≠ Phase 7「闭环」≠ **本模板 FR78**；亦 ≠ FR74/FR75 SynthesizableClosure；≠ SystemC TLM（AD-5）。见 `docs/fr78-bridge-adapter-closures.md` 与决策表 §3。
- **交叉链接：** Epic 27 生成器 / Epic 28 可综合闭包 → README + `docs/fr22-construct-bar.md`；双视图 → `docs/fr47-dual-sim-generation.md`。
- **文档：** `docs/fr78-bridge-adapter-closures.md`；UJ「桥接半程」：`docs/tutorials/bridge-half.md`；风险门禁：`nfr14-risk-epic30-bridge-adapter-closures.md`（Epic 30 关闭条件已勾选）。
- **ATDD：** `cargo test -p bitloom --test fr78_bridge_adapter_start_wait_complete`。
- **FR47 联验（Story 30.3）：** `cargo test -p bitloom --test fr78_fr47_dual_view_coverify` — 模板录制激励 × `check_functional_equiv_generated` / `generate_*`；故意破坏 fail；FR16 + NFR36 抽检。
- **文档收口（Story 30.4）：** `cargo test -p bitloom --test fr78_bridge_half_followalong`。
