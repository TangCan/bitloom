# FR79 — DoubleFlop CDC 真 RTL（AD-29）

公开品牌 **Bitloom**；设计 crate 只依赖 **`bitloom-prelude`**。

## 相对 Epic 7 / FR52 的诚实度（NFR37）

| | Epic 7 / FR52 最小合同 | FR79 / AD-29（本故事） |
| --- | --- | --- |
| 完成话术 | phantom 域 + E0220 + `mark_cdc_bridge` | emit **可综合同步器 RTL** + 按域 tick 黄金 |
| `DoubleFlop` | ZST 叙事锚点 | `Elaboratable` → `sync_ff0` / `sync_ff1` |
| 深度关闭 | sprint 可 `done`（最小） | **不得**用历史 done 冒充 |

`examples/clockdomain_skel` 仍演示 FR52 最小合同。真 RTL 夹具：`examples/doubleflop_skel`。  
Epic 31 收口跟练：[`docs/tutorials/cdc-depth.md`](tutorials/cdc-depth.md)（CDC 深度 / FR79）。

## 合同内

- **级数：** `DoubleFlop::STAGES = 2`（`sync_ff0` → `sync_ff1`）。
- **延迟：** 稳定 `din` 后，经 `DoubleFlop::LATENCY_DST_TICKS = 2` 个**目的域** tick，`dout` 才跟随。
- **仿真 MVP：** 全局 `bitloom_sim::Sim::tick` ≡ 目的域 tick（尚无独立 per-domain 引擎）。
- **非法跨域：** 无 bridge 的跨域 `assign_net` / `assign_reg_d_from` → `rhdl::E0220`。
- **API：**
  - 独立模块：`DoubleFlop::elaborate()` / `elaborate_width(w)`
  - 嵌入：`ElaborateSession::declare_double_flop_stages` + `connect_double_flop`

## 合同外

- **不**保证硅片亚稳态消除、MTBF 公式或工艺时序签核。
- 文档区分 **RTL/仿真语义延迟** vs **物理亚稳态风险提示**。
- `SyncFIFO` 真 RTL → 见 `docs/fr79-syncfifo-cdc.md`（Story 31.3）。

## 最小用法

```ignore
use bitloom_prelude::{DoubleFlop, Elaboratable};

let hir = DoubleFlop::elaborate()?;
// emit → DoubleFlop.v with sync_ff0 / sync_ff1
// Sim::tick × 2 after din rises → dout follows
```

权威风险边界：`_agile-output/implementation-artifacts/nfr14-risk-epic31-cdc-true-rtl.md`。
