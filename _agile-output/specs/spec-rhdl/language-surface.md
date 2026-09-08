# Language surface

Catalog for CAP-1…CAP-3、CAP-7、CAP-10、CAP-11。HOW（宏如何展开、freeze 何时跑）见架构脊柱。

## Hardware types (baseline)

- `Bool`, `Bits<N>`, `UInt<N>`, `SInt<N>`, `Clock`, `Reset`
- Ports: `Input<T>`, `Output<T>` — not bare `UInt` at the module boundary
- Builder facade `Bits<N>` lowers to runtime widths on FrozenHir / `PortValues`

## Composite types (CAP-10 / FR51)

- `Bundle` 与 `Vec<T,N>`（或文档等价）允许进入可综合路径。
- **文档等价：** 公开类型名为 `HwVec<T,N>`（避免与堆 `Vec` / E0141 冲突）；合同与叙述中的 `Vec<T,N>` 即指 `HwVec`。`N` 须 > 0。
- **展平：** elaborate 时展平为标量 HIR 端口；叶命名 `{field}_{member}`（Bundle）与 `{field}_{i}`（HwVec）。公开 HIR 可不含 Bundle/Vector 节点。
- **OUT OF SCOPE（MVP 锁定）：** 嵌套 `Bundle` 成员与 `HwVec<Bundle,_>` — `Bundle::leaves` 仅为 ground；`HwVec` 元素须 `AsGround`。负向：`examples/bundle_vec_skel` trybuild `nested_hwvec_bundle`。
- **`#[derive(Bundle)]`：不可用（documented defer）** — 无 derive 宏；须手写 `Bundle::leaves`。负向：trybuild `derive_bundle_unavailable`。
- **叶名碰撞：** `{field}_{member}` / `{field}_{i}` 与已有信号冲突 → emit 前失败（`rhdl::E0152`）。
- 位宽/方向不匹配必须在 emit 前失败；不得 silently 可用却无检查。
- HIR ground 是否扩展 Bundle/Vector 节点由实现选择；公开表面与 emit 语义须一致（AD-20）。
- **FR22 边界：** 单时钟表面加厚（FR22）的构造条**不含** Bundle/Vec；复合类型由本节 / FR51 交付，不得 silently 算进 FR22 验收。

## ClockDomain (CAP-11 / FR52)

- **API 映射（产品面；对齐 AD-22）：**
  - 域标记：`ClockDomain::<ID>`（prelude ZST）+ `ElaborateSession::bind_domain(name, id)`（session 域标签；**非**独立 `Signal<D,T>` 包装）
  - 合法 CDC：`mark_cdc_bridge(name)` — 文档等价 `DoubleFlop` / `SyncFIFO`（叙事锚点，非真实同步器 RTL IP）
  - 非法跨域无 bridge：`assign_net` 跨域 → `finish`/`freeze` 失败，诊断码 **`rhdl::E0220`**（诊断文案点名 DoubleFlop/SyncFIFO）
  - 同步/异步复位：`declare_reg_ex(..., async_reset, has_enable, ...)`；**极性** = 默认同步**高有效** `Reset`（AD-15，无 ActiveLow API）
  - 仿真步进：全局 `Sim::tick` 为「按域 tick」的 MVP 等价（尚无独立 per-domain tick 引擎）
  - **MVP 门禁范围：** 跨域检查在 `assign_net`；`assign_reg_d_*` 暂不查域（预存缺口，非 FR52 新引入）
- 夹具：`examples/clockdomain_skel`（非法 E0220 + 同形无 bridge 负向 + 合法 emit/tick + sync/async 并排）。
- 默认模块仍是单时钟：恰好一个 `Clock` + 同步高有效 `Reset`（AD-15），除非显式声明多域。
- 域为 session 标签：多域夹具可仍用一对 `clk`/`rst` 端口（非每域独立时钟端口）。

## Still deferred from this catalog

见 `later-product.md` / PRD：`Analog`, `InOut`、浮点 crate（FR36）等——有独立 FR，不并入 CAP-1。

## Mem / SyncReadMem (AD-21 / FR26)

- 表面：`Mem` / `SyncReadMem`（prelude ZST 标记）+ session `declare_mem` / `declare_sync_read_mem`。
- 互转/降级锚 FIRRTL `mem`；Chisel emit 对 Mem 仍可 E0901（见 FR28 / Epic 33）。

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

## Sequential envelope (default)

Every default module has exactly one `Clock` port and one sync active-high `Reset` port. `tick` is one posedge of that clock. No implicit ports at emit. Multi-clock / async reset / enables：见 PRD FR23–FR25 与脊柱 AD-22/AD-23。

## First-class IP (FR37 / FR48 / FR82)

- Surface：`bitloom_prelude::ip::{SyncFifo, UartTx, SpiMaster, I2cMaster, Axi4LiteSlave, ExtBlackBox}`。
- **FR82（Epic 34.2）：** `SyncFifo` = depth-4 sync FIFO（`wr_en`/`rd_en`/`full`/`empty`）；`UartTx` = 8N1 bit-bang（`tx` + busy-gated write；baud=`clk`）。均无生成器闭包参数（Epic 29 叠加）。
- **黑盒：** `ExtBlackBox` 仅端口、空 body；`vendor_blackbox_v()` 旁路；不内联 vendor 网表进 HIR。
- **Sim：** 同周期输入门控须 `set_inputs` → `Sim::settle` → `tick`（见 `docs/ip/README.md`）。
- SPI/I2C/AXI 在 34.3 前仍为 Epic 22 stub；索引：`docs/ip/README.md`。
