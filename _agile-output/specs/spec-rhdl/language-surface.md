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
- **MVP 边界：** `Bundle::leaves` 仅为 ground 叶；不支持嵌套 Bundle / `HwVec<Bundle,_>`；手写 `leaves()`（无 `#[derive(Bundle)]`）。
- 位宽/方向不匹配必须在 emit 前失败；不得 silently 可用却无检查。
- HIR ground 是否扩展 Bundle/Vector 节点由实现选择；公开表面与 emit 语义须一致（AD-20）。
- **FR22 边界：** 单时钟表面加厚（FR22）的构造条**不含** Bundle/Vec；复合类型由本节 / FR51 交付，不得 silently 算进 FR22 验收。

## ClockDomain (CAP-11 / FR52)

- **API 映射（产品面）：**
  - 域标记：`ClockDomain::<ID>`（prelude ZST）+ `ElaborateSession::bind_domain(name, id)`
  - 合法 CDC：`mark_cdc_bridge(name)` — 文档等价 `DoubleFlop` / `SyncFIFO`（叙事锚点，非真实同步器 RTL IP）
  - 非法跨域无 bridge：`assign_net` 跨域 → `finish`/`freeze` 失败，诊断码 **`rhdl::E0220`**（诊断文案点名 DoubleFlop/SyncFIFO）
  - 同步/异步复位：`declare_reg_ex(..., async_reset, has_enable, ...)`；**极性** = 默认同步**高有效** `Reset`（AD-15，无 ActiveLow API）
  - 仿真步进：全局 `Sim::tick` 为「按域 tick」的 MVP 等价（尚无独立 per-domain tick 引擎）
  - **MVP 门禁范围：** 跨域检查在 `assign_net`；`assign_reg_d_*` 暂不查域（预存缺口，非 FR52 新引入）
- 夹具：`examples/clockdomain_skel`（非法 E0220 + 同形无 bridge 负向 + 合法 emit/tick + sync/async 并排）。
- 默认模块仍是单时钟：恰好一个 `Clock` + 同步高有效 `Reset`（AD-15），除非显式声明多域。
- 域为 session 标签：多域夹具可仍用一对 `clk`/`rst` 端口（非每域独立时钟端口）。

## Still deferred from this catalog

见 `later-product.md` / PRD：`Analog`, `InOut`, `Mem`/`SyncReadMem`（FR26/AD-21）、浮点 crate（FR36）等——有独立 FR，不并入 CAP-1。

## Comb / seq

- `#[combinational]` and `#[sequential]` are mandatory.
- Comb may drive `Wire` / `Output` only; incomplete assignment is an error (no inferred latch).
- Only seq writes `Reg.d`. Comb must not write `Reg.d`; seq must not drive combinational nets.
- Stage-2 surface thickening (FR22 / AD-20): `if`/`match`（或等价）、严格同位宽二元运算与连接、显式 pad/trunc、同步复位赋值语义。Bundle/Vec 不在 FR22 构造条内——见上文 Composite types / FR51。

## Width

- Surface arithmetic and connections are strict same-width.
- Extend/truncate only via explicit pad/trunc nodes.
- FIRRTL `add` n+1 is allowed only as those explicit nodes, never as silent prelude truncation.

## Synthesizable subset (cycle-accurate / generate path)

Allowed: hardware types and their ops; `if` / `match`; statically bounded loops that fully unroll; inlined functions; const generics; arrays / structs / enums / Bundle / Vec used as hardware aggregates in-scope.

Rejected on this path: heap `Vec`/`Box`/`String`（软件堆，非硬件 `Vec<T,N>`）；unbounded recursion; `dyn Trait`; capturing closures; file/net/threads; default `f32`/`f64`（可综合浮点见 FR36）。

Functional view（手写 `#[functional_model]` 或 CAP-13 生成的 Rust crate）may use rejected constructs. Fields marked `#[functional_state]` never enter HIR.

## FR47 dual-sim generators (MVP subset)

`generate_functional_sim` / `generate_cycle_accurate_sim`（及 CLI `gen-func` / `gen-cycle`）当前 **MVP = 扁平单模块**：只消费顶层一个 module；**不**保证层次实例（`instances`）或 `MemDecl` 的周期精确 emit。扩到层次/mem 须单独故事并先改本段与 `docs/fr47-dual-sim-generation.md` / `deferred-work.md`——禁止静默扩子集。

## Sequential envelope (default)

Every default module has exactly one `Clock` port and one sync active-high `Reset` port. `tick` is one posedge of that clock. No implicit ports at emit. Multi-clock / async reset / enables：见 PRD FR23–FR25 与脊柱 AD-22/AD-23。
