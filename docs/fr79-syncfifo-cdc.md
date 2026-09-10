# FR79 — SyncFIFO CDC 真 RTL（AD-29）

公开品牌 **Bitloom**；设计 crate 只依赖 **`bitloom-prelude`**。

## 相对 Epic 7 / FR52 / IP SyncFifo 的诚实度（NFR37）

| | Epic 7 / FR52 最小合同 | `ip::SyncFifo`（FR82） | FR79 `SyncFIFO`（本故事） |
| --- | --- | --- | --- |
| 完成话术 | `mark_cdc_bridge` 叙事 | 单时钟一级 IP 非 stub | emit **跨域 FIFO 真 RTL** + tick 黄金 |
| 时钟 | 单 clk 叙事 | 单 clk | 单物理 `clk` + **phantom 双域** MVP |
| 模块名 | — | `SyncFifo` | `SyncFIFO` |

`examples/clockdomain_skel` 仍演示 FR52 最小合同。真 RTL 夹具：`examples/syncfifo_skel`。DoubleFlop 见 `docs/fr79-doubleflop-cdc.md`。  
Epic 31 收口跟练：[`docs/tutorials/cdc-depth.md`](tutorials/cdc-depth.md)（CDC 深度 / FR79）。

## 合同内（文档化最小子集）

- **参数：** `DEPTH = 4`、`WIDTH = 8`（`SyncFIFO::<4, 8>`）；指针宽 `PTR_WIDTH = 3`（`clog2(DEPTH)+1`）。
- **结构：** `ram` mem + `wr_ptr`/`rd_ptr` + 灰码 + `w2r_*`/`r2w_*` DoubleFlop 同步 + `full`/`empty`。
- **域：** 写侧（`wr_en`/`data_in`/`full`）D0；读侧（`rd_en`/`data_out`/`empty`）D1。
- **延迟：**
  - `LATENCY_PTR_SYNC_TICKS = 2` — 灰码经 DoubleFlop 后，读侧 `empty` 才反映写；
  - `LATENCY_REG_READ_TICKS = 1` — 接受 `rd_en` 后 `data_out` 注册读。
- **满/空：** Cummings 式灰码比较（`empty`: `rd_gray == wr_gray_sync`；`full`: `wr_gray == {~rd_sync[2:1], rd_sync[0]}`）。写满后忽略写；读空忽略读。
- **仿真 MVP：** 全局 `Sim::tick` ≡ 按域 tick（尚无独立双物理时钟引擎）。
- **非法跨域：** 无 bridge → `rhdl::E0220`。

## 合同外

- 不保证硅片亚稳态 / MTBF。
- 非 `wr_clk`/`rd_clk` 双物理时钟产品矩阵；非任意 DEPTH/WIDTH 全家桶（Ask First）。
- **不得**把本原语与 [`ip::SyncFifo`](../crates/bitloom-prelude/src/ip/sync_fifo.rs) 混为同一完成话术。

## 最小用法

```ignore
use bitloom_prelude::{Elaboratable, SyncFIFO};

let hir = SyncFIFO::<4, 8>::elaborate()?;
// emit → SyncFIFO.v with ram + w2r_ff*/r2w_ff* + full/empty
```

权威风险边界：`_agile-output/implementation-artifacts/nfr14-risk-epic31-cdc-true-rtl.md`。
